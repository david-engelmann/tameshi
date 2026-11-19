//! The pins of slice 12, the sources record field set,
//! driving the binary over synthetic packs the suite
//! writes itself, each pack assembled above the floor and
//! carrying the sources record the assembly constructs,
//! and each tampered pack otherwise sound, its sources
//! record forged in one standing alone.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::render::render_json;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-sources-record-{}-{name}",
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

/// The blindness document naming two traces for the one
/// check, in the shape the probes pack carries.
fn blindgate() -> String {
    r#"{"format": "blindgate-evidence/1", "traces": [{"check": "unit checks", "surface": "settings file", "observation": "read", "detail": "synthetic trace"}, {"check": "unit checks", "surface": "settings file", "observation": "read", "detail": "second synthetic trace"}]}"#
        .to_string()
}

/// Assemble the pack standing above the floor, carrying
/// the two traces and the sources record the assembly
/// constructs for them, and read it back as a document.
fn assemble(dir: &Path) -> JsonValue {
    write(dir, "cl-pack.json", &cl_pack());
    write(dir, "canary.json", &canary());
    write(dir, "blindgate.json", &blindgate());
    let (code, err) = run(
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
    );
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

fn without_field(document: &JsonValue, field: &str) -> JsonValue {
    let pairs: Vec<(String, JsonValue)> = document
        .as_object()
        .expect("the document stands as an object")
        .iter()
        .filter(|(key, _)| key != field)
        .map(|(key, held)| (key.clone(), held.clone()))
        .collect();
    JsonValue::Object(pairs)
}

/// The sources record the pack carries, standing as the
/// object the assembly constructed.
fn sources_of(pack: &JsonValue) -> JsonValue {
    pack.get("sources")
        .expect("the sources record stands")
        .clone()
}

/// The pack with its sources record replaced by the
/// record the caller builds, every other member standing
/// sound.
fn forged_pack(pack: &JsonValue, sources: JsonValue) -> JsonValue {
    replace_field(pack, "sources", sources)
}

/// The pack whose sources record carries a nested
/// blindness_traces member holding a bare string, the
/// member the blindness discipline governs one level up
/// and nothing reads one level down.
fn nested_pack(pack: &JsonValue) -> JsonValue {
    let sources = add_field(
        &sources_of(pack),
        "blindness_traces",
        JsonValue::Text("a bare string standing inside the sources record".to_string()),
    );
    forged_pack(pack, sources)
}

/// The pack whose sources record carries a further
/// member of an innocent name beside the four fields.
fn further_pack(pack: &JsonValue) -> JsonValue {
    let sources = add_field(
        &sources_of(pack),
        "publisher_note",
        JsonValue::Text("recorded by the publisher".to_string()),
    );
    forged_pack(pack, sources)
}

/// The pack whose sources record stands missing the
/// unmatched trace count, one of its four fields.
fn missing_pack(pack: &JsonValue) -> JsonValue {
    let sources = without_field(&sources_of(pack), "blindgate_unmatched_trace_count");
    forged_pack(pack, sources)
}

/// The pack whose sources record stands empty, every
/// one of its four fields missing.
fn empty_pack(pack: &JsonValue) -> JsonValue {
    forged_pack(pack, JsonValue::Object(Vec::new()))
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

/// Rule over the sound pack so its record stands in the
/// out directory, then leave the tampered pack in its
/// place for the validate-ruling door.
fn rule_over_sound_then_write(dir: &Path, pack: &JsonValue, forged: &JsonValue) {
    write_pack(dir, pack);
    let (code, err) = rule(dir);
    assert_eq!(code, 0, "the sound pack rules: {err}");
    write_pack(dir, forged);
}

fn validate_ruling(dir: &Path) -> (i32, String) {
    run(
        dir,
        &[
            "validate-ruling",
            "--pack",
            "pack.json",
            "--ruling",
            "out/ruling-unit-checks.json",
        ],
    )
}

#[test]
fn test_a_sources_record_carrying_a_nested_blindness_traces_member_is_refused_at_the_rule_door() {
    let dir = scratch("nested-rule");
    let pack = assemble(&dir);
    write_pack(&dir, &nested_pack(&pack));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "a nested blindness traces member refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack sources record has the wrong fields (unexpected blindness_traces)"
        ),
        "the refusal names the nested member as unexpected: {err}"
    );
}

#[test]
fn test_a_sources_record_carrying_a_further_member_is_refused_at_the_rule_door() {
    let dir = scratch("further-rule");
    let pack = assemble(&dir);
    write_pack(&dir, &further_pack(&pack));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "a further member refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack sources record has the wrong fields (unexpected publisher_note)"
        ),
        "the refusal names the further member as unexpected: {err}"
    );
}

#[test]
fn test_a_sources_record_missing_a_field_is_refused_at_the_rule_door() {
    let dir = scratch("missing-rule");
    let pack = assemble(&dir);
    write_pack(&dir, &missing_pack(&pack));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "a sources record missing a field refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack sources record has the wrong fields (missing blindgate_unmatched_trace_count)"
        ),
        "the refusal names the missing field: {err}"
    );
}

#[test]
fn test_an_empty_sources_record_is_refused_at_the_rule_door() {
    let dir = scratch("empty-rule");
    let pack = assemble(&dir);
    write_pack(&dir, &empty_pack(&pack));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "an empty sources record refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack sources record has the wrong fields (missing blindgate_evidence_supplied, blindgate_unmatched_trace_count, canary_origin, catchledger_source)"
        ),
        "the refusal names all four fields as missing: {err}"
    );
}

#[test]
fn test_a_sources_record_carrying_a_nested_blindness_traces_member_is_refused_at_the_validate_ruling_door(
) {
    let dir = scratch("nested-validate");
    let pack = assemble(&dir);
    let forged = nested_pack(&pack);
    rule_over_sound_then_write(&dir, &pack, &forged);
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 1, "a nested blindness traces member refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack sources record has the wrong fields (unexpected blindness_traces)"
        ),
        "the refusal names the nested member as unexpected: {err}"
    );
}

#[test]
fn test_a_sources_record_carrying_a_further_member_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("further-validate");
    let pack = assemble(&dir);
    let forged = further_pack(&pack);
    rule_over_sound_then_write(&dir, &pack, &forged);
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 1, "a further member refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack sources record has the wrong fields (unexpected publisher_note)"
        ),
        "the refusal names the further member as unexpected: {err}"
    );
}

#[test]
fn test_a_sources_record_missing_a_field_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("missing-validate");
    let pack = assemble(&dir);
    let forged = missing_pack(&pack);
    rule_over_sound_then_write(&dir, &pack, &forged);
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 1, "a sources record missing a field refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack sources record has the wrong fields (missing blindgate_unmatched_trace_count)"
        ),
        "the refusal names the missing field: {err}"
    );
}

#[test]
fn test_an_empty_sources_record_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("empty-validate");
    let pack = assemble(&dir);
    let forged = empty_pack(&pack);
    rule_over_sound_then_write(&dir, &pack, &forged);
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 1, "an empty sources record refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack sources record has the wrong fields (missing blindgate_evidence_supplied, blindgate_unmatched_trace_count, canary_origin, catchledger_source)"
        ),
        "the refusal names all four fields as missing: {err}"
    );
}

#[test]
fn test_a_pack_with_a_sound_sources_record_rules_and_its_ruling_validates() {
    let dir = scratch("sound");
    let pack = assemble(&dir);
    let names: Vec<&str> = sources_of(&pack)
        .as_object()
        .expect("the sources record stands as an object")
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(
        names,
        [
            "catchledger_source",
            "canary_origin",
            "blindgate_evidence_supplied",
            "blindgate_unmatched_trace_count"
        ],
        "the sources record carries exactly the four constructed fields"
    );
    write_pack(&dir, &pack);
    let (code, err) = rule(&dir);
    assert_eq!(code, 0, "the sound pack rules: {err}");
    let (code, err) = run(
        &dir,
        &[
            "validate-ruling",
            "--pack",
            "pack.json",
            "--ruling",
            "out/ruling-unit-checks.json",
            "--body",
            "out/ruling-unit-checks.md",
        ],
    );
    assert_eq!(code, 0, "the ruling validates: {err}");
}

#[test]
fn test_a_sources_record_standing_as_a_list_is_refused_as_not_an_object() {
    let dir = scratch("list");
    let pack = assemble(&dir);
    write_pack(&dir, &forged_pack(&pack, JsonValue::List(Vec::new())));
    let (code, err) = rule(&dir);
    assert_eq!(
        code, 1,
        "a sources record standing as a list refuses: {err}"
    );
    assert!(
        err.contains("the evidence pack sources record must be an object"),
        "the refusal keeps the standing object sentence: {err}"
    );
}

#[test]
fn test_a_pack_whose_sources_record_carries_a_null_catchledger_source_rules_and_validates() {
    let dir = scratch("null-source");
    let pack = assemble(&dir);
    let sources = replace_field(&sources_of(&pack), "catchledger_source", JsonValue::Null);
    write_pack(&dir, &forged_pack(&pack, sources));
    let (code, err) = rule(&dir);
    assert_eq!(code, 0, "the pack carrying the null rules: {err}");
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 0, "the pack carrying the null validates: {err}");
}

#[test]
fn test_a_pack_assembled_with_a_sound_sources_record_validates_against_its_inputs() {
    let dir = scratch("rebuild");
    let _pack = assemble(&dir);
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
