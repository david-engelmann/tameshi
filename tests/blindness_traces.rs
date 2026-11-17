//! The pins of slice 10, the blindness traces at the ruling
//! doors, driving the binary over synthetic packs the suite
//! writes itself, each pack assembled above the floor and
//! carrying the traces a blindness document names for its
//! check, and each tampered pack otherwise sound.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::render::render_json;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-blindness-traces-{}-{name}",
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
/// the two traces where the caller asks for them, and read
/// it back as a document.
fn assemble(dir: &Path, with_traces: bool) -> JsonValue {
    write(dir, "cl-pack.json", &cl_pack());
    write(dir, "canary.json", &canary());
    let mut args = vec![
        "assemble",
        "--catchledger",
        "cl-pack.json",
        "--canaryfield",
        "canary.json",
    ];
    if with_traces {
        write(dir, "blindgate.json", &blindgate());
        args.push("--blindgate");
        args.push("blindgate.json");
    }
    args.push("--packs-dir");
    args.push("packs");
    let (code, err) = run(dir, &args);
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
fn test_a_trace_entry_missing_a_field_is_refused_at_the_rule_door() {
    let dir = scratch("missing-rule");
    let pack = assemble(&dir, true);
    let entry = without_field(&first_entry(&pack), "detail");
    write_pack(&dir, &forged_pack(&pack, entry));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "a trace entry missing a field refuses: {err}");
    assert!(
        err.contains("the blindness trace has the wrong fields (missing detail)"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_trace_entry_carrying_a_fifth_field_is_refused_at_the_rule_door() {
    let dir = scratch("fifth-rule");
    let pack = assemble(&dir, true);
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
fn test_a_bare_string_in_place_of_a_trace_entry_is_refused_at_the_rule_door() {
    let dir = scratch("bare-rule");
    let pack = assemble(&dir, true);
    let entry = JsonValue::Text("a bare string standing in place of a trace entry".to_string());
    write_pack(&dir, &forged_pack(&pack, entry));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "a bare string standing as an entry refuses: {err}");
    assert!(
        err.contains("the blindness trace must be an object"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_trace_entry_missing_a_field_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("missing-validate");
    let pack = assemble(&dir, true);
    let entry = without_field(&first_entry(&pack), "detail");
    rule_over_sound_then_write(&dir, &pack, &forged_pack(&pack, entry));
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 1, "a trace entry missing a field refuses: {err}");
    assert!(
        err.contains("the blindness trace has the wrong fields (missing detail)"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_trace_entry_carrying_a_fifth_field_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("fifth-validate");
    let pack = assemble(&dir, true);
    let entry = add_field(
        &first_entry(&pack),
        "note",
        JsonValue::Text("a field the discipline does not carry".to_string()),
    );
    rule_over_sound_then_write(&dir, &pack, &forged_pack(&pack, entry));
    let (code, err) = validate_ruling(&dir);
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
fn test_a_bare_string_in_place_of_a_trace_entry_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("bare-validate");
    let pack = assemble(&dir, true);
    let entry = JsonValue::Text("a bare string standing in place of a trace entry".to_string());
    rule_over_sound_then_write(&dir, &pack, &forged_pack(&pack, entry));
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 1, "a bare string standing as an entry refuses: {err}");
    assert!(
        err.contains("the blindness trace must be an object"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_trace_entry_with_an_emptied_field_is_refused_at_the_rule_door() {
    let dir = scratch("emptied-rule");
    let pack = assemble(&dir, true);
    let entry = replace_field(
        &first_entry(&pack),
        "surface",
        JsonValue::Text(String::new()),
    );
    write_pack(&dir, &forged_pack(&pack, entry));
    let (code, err) = rule(&dir);
    assert_eq!(
        code, 1,
        "a trace entry with an emptied field refuses: {err}"
    );
    assert!(
        err.contains("the blindness trace surface must be nonempty text"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_trace_entry_with_an_emptied_field_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("emptied-validate");
    let pack = assemble(&dir, true);
    let entry = replace_field(
        &first_entry(&pack),
        "surface",
        JsonValue::Text(String::new()),
    );
    rule_over_sound_then_write(&dir, &pack, &forged_pack(&pack, entry));
    let (code, err) = validate_ruling(&dir);
    assert_eq!(
        code, 1,
        "a trace entry with an emptied field refuses: {err}"
    );
    assert!(
        err.contains("the blindness trace surface must be nonempty text"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_trace_entry_naming_another_check_is_refused_at_the_rule_door() {
    let dir = scratch("foreign-rule");
    let pack = assemble(&dir, true);
    let entry = replace_field(
        &first_entry(&pack),
        "check",
        JsonValue::Text("other checks".to_string()),
    );
    write_pack(&dir, &forged_pack(&pack, entry));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "a trace entry naming another check refuses: {err}");
    assert!(
        err.contains("the evidence pack blindness traces must name the pack's own check"),
        "the refusal keeps the words of BT-3: {err}"
    );
}

#[test]
fn test_a_trace_entry_naming_another_check_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("foreign-validate");
    let pack = assemble(&dir, true);
    let entry = replace_field(
        &first_entry(&pack),
        "check",
        JsonValue::Text("other checks".to_string()),
    );
    rule_over_sound_then_write(&dir, &pack, &forged_pack(&pack, entry));
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 1, "a trace entry naming another check refuses: {err}");
    assert!(
        err.contains("the evidence pack blindness traces must name the pack's own check"),
        "the refusal keeps the words of BT-3: {err}"
    );
}

#[test]
fn test_a_pack_carrying_two_sound_traces_rules_and_its_ruling_validates() {
    let dir = scratch("sound");
    let pack = assemble(&dir, true);
    assert_eq!(
        traces_of(&pack).len(),
        2,
        "the two traces stand in the pack"
    );
    write_pack(&dir, &pack);
    let (code, err) = rule(&dir);
    assert_eq!(code, 0, "the sound pack rules: {err}");
    let body = std::fs::read_to_string(dir.join("out/ruling-unit-checks.md"))
        .expect("the ruling body reads");
    assert!(
        body.contains("| Blindness traces | 2 |"),
        "the body counts the two traces: {body}"
    );
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
fn test_a_pack_carrying_an_empty_traces_list_rules_as_it_does_today() {
    let dir = scratch("empty");
    let pack = assemble(&dir, false);
    assert!(
        traces_of(&pack).is_empty(),
        "the pack carries the member as an empty list"
    );
    write_pack(&dir, &pack);
    let (code, err) = rule(&dir);
    assert_eq!(code, 0, "the pack rules as it does today: {err}");
    let body = std::fs::read_to_string(dir.join("out/ruling-unit-checks.md"))
        .expect("the ruling body reads");
    assert!(
        body.contains("| Blindness traces | 0 |"),
        "the body counts no traces: {body}"
    );
}

#[test]
fn test_a_pack_assembled_with_its_traces_validates_against_its_inputs() {
    let dir = scratch("rebuild");
    let _pack = assemble(&dir, true);
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
