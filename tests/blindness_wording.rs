//! The pins of slice 11, the blindness evidence refusal
//! wording, driving the binary over synthetic blindness
//! documents the suite writes itself, each document
//! standing over the same catchledger pack and the same
//! canaryfield document, and each refusal read at the
//! assemble door the reading serves first and at the
//! validate door whose rebuild passes through it again.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::render::render_json;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-blindness-wording-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch directory stands");
    dir
}

fn write(dir: &Path, name: &str, text: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the fixture writes");
    path
}

fn run(dir: &Path, args: &[&str]) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the binary runs");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

const SCORECARD: &str = r#"{"fires": 40, "blocks": 6, "judged_commits": 20, "flaky_commits": 1, "flake_rate": 0.05, "runtime_seconds": 900, "runtime_minutes": 15.0, "catches": 3, "false_blocks": 1, "unattributed_blocks": 2, "false_block_rate": 0.1667, "total_cost_usd": 0.12, "cost_per_catch_usd": 0.04}"#;

fn catch_record(days: std::ops::RangeInclusive<i32>) -> String {
    let entries: Vec<String> = days
        .map(|day| {
            format!(
                r#"{{"block_run_id": "{}", "blocked_commit": "0000000000000000000000000000000000000001", "evidence": {{"fix_commits": ["ffffffffffffffffffffffffffffffffffffffff"], "reason": null, "type": "flagged_fix_then_success", "validating_run_id": "900"}}, "flagged_paths": ["src/synthetic.py"], "outcome": "catch", "started_at": "2026-01-{:02}T00:10:00Z"}}"#,
                100 + day,
                day
            )
        })
        .collect();
    format!("[{}]", entries.join(", "))
}

fn cl_pack() -> String {
    format!(
        r#"{{"format": "catchledger-pack/1", "repo": "synthetic-repo", "check": "unit checks", "scorecard": {SCORECARD}, "catch_record": {}, "canaryfield_results": null, "false_block_rate": 0.1667, "pricing": {{"currency": "USD", "cost_per_minute": "0.008"}}, "source": {{"ci_history_sha256": null, "git_history_sha256": null, "canaryfield_results_sha256": null}}}}"#,
        catch_record(1..=14)
    )
}

fn canary() -> String {
    let rows: Vec<String> = (0..30)
        .map(|i| {
            format!(
                r#"{{"seed": "seed-{i:02}", "name": "synthetic defect {i}", "defect_class": "synthetic class", "planted_files": ["src/synthetic.py"], "gates": {{"unit checks": "caught"}}}}"#
            )
        })
        .collect();
    format!(
        r#"{{"gates": ["unit checks"], "rows": [{}]}}"#,
        rows.join(", ")
    )
}

/// The two traces the sound blindness document carries,
/// each naming the one check in the four field shape the
/// entry discipline demands.
fn traces() -> String {
    r#"[{"check": "unit checks", "surface": "settings file", "observation": "read", "detail": "synthetic trace"}, {"check": "unit checks", "surface": "settings file", "observation": "read", "detail": "second synthetic trace"}]"#
        .to_string()
}

/// The sound blindness document, carrying exactly the
/// format field and the traces field.
fn sound_document() -> String {
    format!(
        r#"{{"format": "blindgate-evidence/1", "traces": {}}}"#,
        traces()
    )
}

/// The blindness document carrying the two named fields
/// and the further field publisher_note beside them.
fn extra_field_document() -> String {
    format!(
        r#"{{"format": "blindgate-evidence/1", "traces": {}, "publisher_note": "recorded by the publisher"}}"#,
        traces()
    )
}

/// Run the assemble door over the blindness document the
/// caller hands in, beside the standing pack and canary
/// inputs, and hand back the exit and the error stream.
fn assemble(dir: &Path, blindness_text: &str) -> (i32, String) {
    write(dir, "cl-pack.json", &cl_pack());
    write(dir, "canary.json", &canary());
    write(dir, "blindgate.json", blindness_text);
    run(
        dir,
        &[
            "assemble",
            "--catchledger",
            "cl-pack.json",
            "--canaryfield",
            "canary.json",
            "--blindgate",
            "blindgate.json",
            "--packs-dir",
            "packs",
        ],
    )
}

/// Assemble over the sound document and read the pack
/// back as a document.
fn assemble_sound(dir: &Path) -> JsonValue {
    let (code, err) = assemble(dir, &sound_document());
    assert_eq!(code, 0, "the assembly stands: {err}");
    let text =
        std::fs::read_to_string(dir.join("packs/001-unit-checks.json")).expect("the pack reads");
    parse(&text).expect("the pack parses")
}

fn replace_field(document: &JsonValue, field: &str, value: JsonValue) -> JsonValue {
    let pairs: Vec<(String, JsonValue)> = document
        .as_object()
        .expect("the document stands as an object")
        .iter()
        .map(|(key, held)| {
            if key == field {
                (key.clone(), value.clone())
            } else {
                (key.clone(), held.clone())
            }
        })
        .collect();
    JsonValue::Object(pairs)
}

fn add_field(document: &JsonValue, field: &str, value: JsonValue) -> JsonValue {
    let mut pairs: Vec<(String, JsonValue)> = document
        .as_object()
        .expect("the document stands as an object")
        .clone();
    pairs.push((field.to_string(), value));
    JsonValue::Object(pairs)
}

fn traces_of(pack: &JsonValue) -> Vec<JsonValue> {
    pack.get("blindness_traces")
        .and_then(|value| value.as_list())
        .expect("the traces stand as a list")
        .to_vec()
}

fn first_entry(pack: &JsonValue) -> JsonValue {
    traces_of(pack)
        .first()
        .expect("the first trace entry stands")
        .clone()
}

/// The pack with its first trace entry replaced by the
/// entry the caller builds, the second entry standing
/// sound.
fn forged_pack(pack: &JsonValue, entry: JsonValue) -> JsonValue {
    let mut traces = traces_of(pack);
    traces[0] = entry;
    replace_field(pack, "blindness_traces", JsonValue::List(traces))
}

fn write_pack(dir: &Path, pack: &JsonValue) {
    write(dir, "pack.json", &render_json(pack));
}

fn rule(dir: &Path) -> (i32, String) {
    run(
        dir,
        &[
            "rule",
            "--pack",
            "pack.json",
            "--ruling",
            "keep",
            "--out",
            "out",
        ],
    )
}

#[test]
fn test_a_blindness_document_carrying_a_further_field_is_refused_with_the_field_named_unexpected() {
    let dir = scratch("extra-field");
    let (code, err) = assemble(&dir, &extra_field_document());
    assert_eq!(
        code, 1,
        "a document carrying a further field refuses: {err}"
    );
    assert!(
        err.contains("the blindness evidence has the wrong fields (unexpected publisher_note)"),
        "the refusal names the further field as unexpected: {err}"
    );
}

#[test]
fn test_a_blindness_document_without_its_format_field_is_refused_with_the_field_named_missing() {
    let dir = scratch("no-format");
    let document = format!(r#"{{"traces": {}}}"#, traces());
    let (code, err) = assemble(&dir, &document);
    assert_eq!(
        code, 1,
        "a document without its format field refuses: {err}"
    );
    assert!(
        err.contains("the blindness evidence has the wrong fields (missing format)"),
        "the refusal names the format field as missing: {err}"
    );
}

#[test]
fn test_a_blindness_document_without_its_traces_field_is_refused_with_the_field_named_missing() {
    let dir = scratch("no-traces");
    let (code, err) = assemble(&dir, r#"{"format": "blindgate-evidence/1"}"#);
    assert_eq!(
        code, 1,
        "a document without its traces field refuses: {err}"
    );
    assert!(
        err.contains("the blindness evidence has the wrong fields (missing traces)"),
        "the refusal names the traces field as missing: {err}"
    );
}

#[test]
fn test_an_empty_blindness_document_is_refused_with_both_fields_named_missing() {
    let dir = scratch("empty");
    let (code, err) = assemble(&dir, "{}");
    assert_eq!(code, 1, "an empty document refuses: {err}");
    assert!(
        err.contains("the blindness evidence has the wrong fields (missing format, traces)"),
        "the refusal names both fields as missing: {err}"
    );
}

#[test]
fn test_a_blindness_document_standing_as_a_list_is_refused_as_not_an_object() {
    let dir = scratch("list");
    let document = traces();
    let (code, err) = assemble(&dir, &document);
    assert_eq!(code, 1, "a document standing as a list refuses: {err}");
    assert!(
        err.contains("the blindness evidence must be an object"),
        "the refusal speaks the object words: {err}"
    );
}

#[test]
fn test_a_blindness_document_without_traces_and_carrying_a_further_field_is_refused_with_both_named(
) {
    let dir = scratch("note-no-traces");
    let document =
        r#"{"format": "blindgate-evidence/1", "publisher_note": "recorded by the publisher"}"#;
    let (code, err) = assemble(&dir, document);
    assert_eq!(
        code, 1,
        "a document missing traces and carrying a further field refuses: {err}"
    );
    assert!(
        err.contains(
            "the blindness evidence has the wrong fields (missing traces; unexpected publisher_note)"
        ),
        "the refusal names the missing field and the unexpected field together: {err}"
    );
}

#[test]
fn test_a_blindness_document_carrying_a_further_field_is_refused_at_the_validate_door_with_the_field_named_unexpected(
) {
    let dir = scratch("extra-field-validate");
    let _pack = assemble_sound(&dir);
    write(&dir, "blindgate.json", &extra_field_document());
    let (code, err) = run(
        &dir,
        &[
            "validate",
            "--pack",
            "packs/001-unit-checks.json",
            "--catchledger",
            "cl-pack.json",
            "--canaryfield",
            "canary.json",
            "--blindgate",
            "blindgate.json",
        ],
    );
    assert_eq!(code, 1, "the rebuild refuses the document: {err}");
    assert!(
        err.contains("the blindness evidence has the wrong fields (unexpected publisher_note)"),
        "the refusal names the further field as unexpected: {err}"
    );
}

#[test]
fn test_a_sound_blindness_document_assembles_and_its_traces_stand_in_the_pack() {
    let dir = scratch("sound");
    let pack = assemble_sound(&dir);
    assert_eq!(
        traces_of(&pack).len(),
        2,
        "the two traces stand in the pack"
    );
}

#[test]
fn test_a_blindness_document_carrying_the_wrong_format_name_is_refused_in_its_standing_words() {
    let dir = scratch("wrong-name");
    let document = format!(
        r#"{{"format": "blindgate-evidence/2", "traces": {}}}"#,
        traces()
    );
    let (code, err) = assemble(&dir, &document);
    assert_eq!(
        code, 1,
        "a document carrying the wrong format name refuses: {err}"
    );
    assert!(
        err.contains("the blindness evidence carries the wrong format name"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_blindness_document_whose_traces_stand_as_an_object_is_refused_in_its_standing_words() {
    let dir = scratch("traces-object");
    let document = r#"{"format": "blindgate-evidence/1", "traces": {"check": "unit checks", "surface": "settings file", "observation": "read", "detail": "synthetic trace"}}"#;
    let (code, err) = assemble(&dir, document);
    assert_eq!(
        code, 1,
        "a document whose traces stand as an object refuses: {err}"
    );
    assert!(
        err.contains("the blindness traces must be a list"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_pack_whose_first_trace_entry_carries_a_fifth_field_is_refused_at_the_rule_door_in_its_standing_words(
) {
    let dir = scratch("fifth-entry");
    let pack = assemble_sound(&dir);
    let entry = add_field(
        &first_entry(&pack),
        "note",
        JsonValue::Text("a field the discipline does not carry".to_string()),
    );
    write_pack(&dir, &forged_pack(&pack, entry));
    let (code, err) = rule(&dir);
    assert_eq!(
        code, 1,
        "a trace entry carrying a fifth field refuses: {err}"
    );
    assert!(
        err.contains("the blindness trace has the wrong fields (unexpected note)"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_pack_assembled_from_a_sound_blindness_document_validates_against_its_inputs() {
    let dir = scratch("rebuild");
    let _pack = assemble_sound(&dir);
    let (code, err) = run(
        &dir,
        &[
            "validate",
            "--pack",
            "packs/001-unit-checks.json",
            "--catchledger",
            "cl-pack.json",
            "--canaryfield",
            "canary.json",
            "--blindgate",
            "blindgate.json",
        ],
    );
    assert_eq!(code, 0, "the pack validates against its inputs: {err}");
}
