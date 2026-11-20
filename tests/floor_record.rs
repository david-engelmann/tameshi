//! The pins of slice 13, the floor record field set,
//! driving the binary over synthetic packs the suite
//! writes itself, each pack assembled above the floor and
//! carrying the floor record the assembly constructs,
//! and each tampered pack otherwise sound, its floor
//! record forged in one standing alone.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::render::render_json;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-floor-record-{}-{name}",
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
/// the two traces and the floor record the assembly
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

fn rename_field(document: &JsonValue, field: &str, name: &str) -> JsonValue {
    let pairs: Vec<(String, JsonValue)> = document
        .as_object()
        .expect("the document stands as an object")
        .iter()
        .map(|(key, held)| {
            if key == field {
                (name.to_string(), held.clone())
            } else {
                (key.clone(), held.clone())
            }
        })
        .collect();
    JsonValue::Object(pairs)
}

/// The floor record the pack carries, standing as the
/// object the assembly constructed.
fn floor_of(pack: &JsonValue) -> JsonValue {
    pack.get("floor").expect("the floor record stands").clone()
}

/// The pack with its floor record replaced by the
/// record the caller builds, every other member standing
/// sound.
fn forged_pack(pack: &JsonValue, floor: JsonValue) -> JsonValue {
    replace_field(pack, "floor", floor)
}

/// The pack whose floor record carries a further member
/// of an innocent name beside the seven fields.
fn extra_pack(pack: &JsonValue) -> JsonValue {
    let floor = add_field(
        &floor_of(pack),
        "publisher_note",
        JsonValue::Text("recorded by the publisher".to_string()),
    );
    forged_pack(pack, floor)
}

/// The pack whose floor record stands missing the meets
/// floor mark, one of its seven fields.
fn dropped_pack(pack: &JsonValue) -> JsonValue {
    let floor = without_field(&floor_of(pack), "meets_floor");
    forged_pack(pack, floor)
}

/// The pack whose floor record carries the canary defects
/// count under a changed name, its constructed name
/// standing missing and the changed name standing in its
/// place.
fn renamed_pack(pack: &JsonValue) -> JsonValue {
    let floor = rename_field(&floor_of(pack), "canary_defects", "canary_defect_count");
    forged_pack(pack, floor)
}

/// The pack whose floor record stands empty, every one
/// of its seven fields missing.
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
fn test_a_floor_record_carrying_an_extra_field_is_refused_at_the_rule_door() {
    let dir = scratch("extra-rule");
    let pack = assemble(&dir);
    write_pack(&dir, &extra_pack(&pack));
    let (code, err) = rule(&dir);
    assert_eq!(
        code, 1,
        "a floor record carrying an extra field refuses: {err}"
    );
    assert!(
        err.contains(
            "the evidence pack floor accounting has the wrong fields (unexpected publisher_note)"
        ),
        "the refusal names the extra field as unexpected: {err}"
    );
}

#[test]
fn test_a_floor_record_missing_meets_floor_is_refused_at_the_rule_door() {
    let dir = scratch("dropped-rule");
    let pack = assemble(&dir);
    write_pack(&dir, &dropped_pack(&pack));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "a floor record missing meets floor refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack floor accounting has the wrong fields (missing meets_floor)"
        ),
        "the refusal names the dropped field as missing: {err}"
    );
}

#[test]
fn test_a_floor_record_carrying_a_renamed_field_is_refused_at_the_rule_door() {
    let dir = scratch("renamed-rule");
    let pack = assemble(&dir);
    write_pack(&dir, &renamed_pack(&pack));
    let (code, err) = rule(&dir);
    assert_eq!(
        code, 1,
        "a floor record carrying a renamed field refuses: {err}"
    );
    assert!(
        err.contains(
            "the evidence pack floor accounting has the wrong fields (missing canary_defects; unexpected canary_defect_count)"
        ),
        "the refusal names the constructed field as missing and the changed name as unexpected: {err}"
    );
}

#[test]
fn test_an_empty_floor_record_is_refused_at_the_rule_door() {
    let dir = scratch("empty-rule");
    let pack = assemble(&dir);
    write_pack(&dir, &empty_pack(&pack));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "an empty floor record refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack floor accounting has the wrong fields (missing canary_defects, canary_shortfall, ci_history_days, ci_history_shortfall, meets_floor, required_canary_defects, required_ci_history_days)"
        ),
        "the refusal names all seven fields as missing: {err}"
    );
}

#[test]
fn test_a_floor_record_carrying_an_extra_field_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("extra-validate");
    let pack = assemble(&dir);
    let forged = extra_pack(&pack);
    rule_over_sound_then_write(&dir, &pack, &forged);
    let (code, err) = validate_ruling(&dir);
    assert_eq!(
        code, 1,
        "a floor record carrying an extra field refuses: {err}"
    );
    assert!(
        err.contains(
            "the evidence pack floor accounting has the wrong fields (unexpected publisher_note)"
        ),
        "the refusal names the extra field as unexpected: {err}"
    );
}

#[test]
fn test_a_floor_record_missing_meets_floor_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("dropped-validate");
    let pack = assemble(&dir);
    let forged = dropped_pack(&pack);
    rule_over_sound_then_write(&dir, &pack, &forged);
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 1, "a floor record missing meets floor refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack floor accounting has the wrong fields (missing meets_floor)"
        ),
        "the refusal names the dropped field as missing: {err}"
    );
}

#[test]
fn test_a_floor_record_carrying_a_renamed_field_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("renamed-validate");
    let pack = assemble(&dir);
    let forged = renamed_pack(&pack);
    rule_over_sound_then_write(&dir, &pack, &forged);
    let (code, err) = validate_ruling(&dir);
    assert_eq!(
        code, 1,
        "a floor record carrying a renamed field refuses: {err}"
    );
    assert!(
        err.contains(
            "the evidence pack floor accounting has the wrong fields (missing canary_defects; unexpected canary_defect_count)"
        ),
        "the refusal names the constructed field as missing and the changed name as unexpected: {err}"
    );
}

#[test]
fn test_an_empty_floor_record_is_refused_at_the_validate_ruling_door() {
    let dir = scratch("empty-validate");
    let pack = assemble(&dir);
    let forged = empty_pack(&pack);
    rule_over_sound_then_write(&dir, &pack, &forged);
    let (code, err) = validate_ruling(&dir);
    assert_eq!(code, 1, "an empty floor record refuses: {err}");
    assert!(
        err.contains(
            "the evidence pack floor accounting has the wrong fields (missing canary_defects, canary_shortfall, ci_history_days, ci_history_shortfall, meets_floor, required_canary_defects, required_ci_history_days)"
        ),
        "the refusal names all seven fields as missing: {err}"
    );
}

#[test]
fn test_a_pack_with_a_sound_floor_record_rules_and_its_ruling_validates() {
    let dir = scratch("sound");
    let pack = assemble(&dir);
    let floor = floor_of(&pack);
    let mut names: Vec<&str> = floor
        .as_object()
        .expect("the floor record stands as an object")
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "canary_defects",
            "canary_shortfall",
            "ci_history_days",
            "ci_history_shortfall",
            "meets_floor",
            "required_canary_defects",
            "required_ci_history_days"
        ],
        "the floor record carries exactly the seven constructed fields"
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
fn test_a_floor_record_standing_as_a_list_is_refused_as_not_an_object() {
    let dir = scratch("list");
    let pack = assemble(&dir);
    write_pack(&dir, &forged_pack(&pack, JsonValue::List(Vec::new())));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "a floor record standing as a list refuses: {err}");
    assert!(
        err.contains("the evidence pack floor accounting must be an object"),
        "the refusal keeps the standing object sentence: {err}"
    );
}

#[test]
fn test_a_floor_record_standing_as_null_is_refused_as_not_an_object() {
    let dir = scratch("null");
    let pack = assemble(&dir);
    write_pack(&dir, &forged_pack(&pack, JsonValue::Null));
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "a floor record standing as null refuses: {err}");
    assert!(
        err.contains("the evidence pack floor accounting must be an object"),
        "the refusal keeps the standing object sentence: {err}"
    );
}

#[test]
fn test_a_pack_assembled_with_a_sound_floor_record_validates_against_its_inputs() {
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

/// The pack whose floor record carries one held count
/// lowered by one inside a sound field set, the value
/// tamper the first port battery records at its twenty
/// sixth case.
fn tampered_pack(pack: &JsonValue) -> JsonValue {
    let floor = floor_of(pack);
    let held = match floor.get("canary_defects") {
        Some(JsonValue::Int(value)) => *value,
        _ => panic!("the floor record carries its canary defects as a whole count"),
    };
    let floor = replace_field(&floor, "canary_defects", JsonValue::Int(held - 1));
    forged_pack(pack, floor)
}

#[test]
fn test_a_floor_record_carrying_a_tampered_value_in_a_sound_field_set_is_refused_at_the_rule_door()
{
    let dir = scratch("tampered-rule");
    let pack = assemble(&dir);
    write_pack(&dir, &tampered_pack(&pack));
    let (code, err) = rule(&dir);
    assert_eq!(
        code, 1,
        "a floor record carrying a tampered value refuses: {err}"
    );
    assert!(
        err.contains(
            "the pack floor accounting disagrees with the accounting recomputed from the pack evidence"
        ),
        "the refusal keeps the disagreement wording of the recompute: {err}"
    );
}
