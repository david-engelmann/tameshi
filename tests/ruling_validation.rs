//! The pins of slice 4, the ruling record validation, over
//! synthetic packs and records the suite writes itself.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::rulings::{generate_ruling, validate_ruling};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-ruling-validation-{}-{name}",
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

fn cl_pack(days: std::ops::RangeInclusive<i32>) -> String {
    format!(
        r#"{{"format": "catchledger-pack/1", "repo": "synthetic-repo", "check": "unit checks", "scorecard": {SCORECARD}, "catch_record": {}, "canaryfield_results": null, "false_block_rate": 0.1667, "pricing": {{"currency": "USD", "cost_per_minute": "0.008"}}, "source": {{"ci_history_sha256": null, "git_history_sha256": null, "canaryfield_results_sha256": null}}}}"#,
        catch_record(days)
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

/// Assemble the pack standing above the floor and read it
/// back as a document.
fn pack_above_floor(dir: &Path) -> JsonValue {
    write(dir, "cl-pack.json", &cl_pack(1..=14));
    write(dir, "canary.json", &canary());
    let (code, err) = run(
        dir,
        &[
            "assemble",
            "--catchledger",
            "cl-pack.json",
            "--canaryfield",
            "canary.json",
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

#[test]
fn test_a_generated_ruling_record_validates_against_its_pack() {
    let dir = scratch("generated");
    let pack = pack_above_floor(&dir);
    let (record, _body) =
        generate_ruling(&pack, "demote", None, None).expect("the ruling generates");
    assert!(
        validate_ruling(&record, &pack).expect("the record validates"),
        "a record the generator wrote validates against its pack"
    );
}

#[test]
fn test_a_signed_ruling_record_validates_against_its_pack() {
    let dir = scratch("signed");
    let pack = pack_above_floor(&dir);
    let (record, _body) = generate_ruling(&pack, "keep", None, None).expect("the ruling generates");
    let signature = JsonValue::Object(vec![
        ("status".to_string(), JsonValue::Text("signed".to_string())),
        (
            "signed_by".to_string(),
            JsonValue::Text("a synthetic signer".to_string()),
        ),
        (
            "signed_at".to_string(),
            JsonValue::Text("2026-02-01T00:00:00Z".to_string()),
        ),
    ]);
    let signed = replace_field(&record, "signature", signature);
    assert!(
        validate_ruling(&signed, &pack).expect("the signed record validates"),
        "a record a human signed validates with its signature unread in value"
    );
}

#[test]
fn test_a_ruling_record_with_a_tampered_floor_is_refused() {
    let dir = scratch("tampered-floor");
    let pack = pack_above_floor(&dir);
    let (record, _body) =
        generate_ruling(&pack, "demote", None, None).expect("the ruling generates");
    let floor = record.get("floor").expect("the floor stands").clone();
    let tampered_floor = replace_field(&floor, "canary_defects", JsonValue::Int(999));
    let tampered = replace_field(&record, "floor", tampered_floor);
    let error = validate_ruling(&tampered, &pack).expect_err("the tampered floor refuses");
    assert_eq!(
        error.message,
        "the ruling record disagrees with the pack it was ruled from"
    );
}

#[test]
fn test_a_ruling_record_with_a_tampered_verdict_is_refused() {
    let dir = scratch("tampered-verdict");
    let pack = pack_above_floor(&dir);
    let (record, _body) =
        generate_ruling(&pack, "demote", None, None).expect("the ruling generates");
    let tampered = replace_field(&record, "ruling", JsonValue::Text("quarantine".to_string()));
    assert!(
        validate_ruling(&tampered, &pack).is_err(),
        "a verdict word swapped after generation refuses"
    );
    let unknown = replace_field(&record, "ruling", JsonValue::Text("shred".to_string()));
    let error = validate_ruling(&unknown, &pack).expect_err("an unknown verdict refuses");
    assert_eq!(
        error.message,
        "the ruling record disagrees with the pack it was ruled from"
    );
    let mismatched_state = replace_field(
        &record,
        "state_after_ruling",
        JsonValue::Text("required".to_string()),
    );
    assert!(
        validate_ruling(&mismatched_state, &pack).is_err(),
        "a verdict standing with another verdict's state refuses"
    );
}

#[test]
fn test_a_ruling_record_whose_signature_claims_signed_without_a_signer_is_refused() {
    let dir = scratch("bad-signature");
    let pack = pack_above_floor(&dir);
    let (record, _body) = generate_ruling(&pack, "keep", None, None).expect("the ruling generates");
    let signature = JsonValue::Object(vec![
        ("status".to_string(), JsonValue::Text("signed".to_string())),
        ("signed_by".to_string(), JsonValue::Null),
        (
            "signed_at".to_string(),
            JsonValue::Text("2026-02-01T00:00:00Z".to_string()),
        ),
    ]);
    let tampered = replace_field(&record, "signature", signature);
    assert!(
        validate_ruling(&tampered, &pack).is_err(),
        "a signature claiming signed with no signer refuses"
    );
}

#[test]
fn test_a_quarantine_record_validates_after_its_expiry_day_has_passed() {
    let dir = scratch("expired-quarantine");
    let pack = pack_above_floor(&dir);
    let expiry = JsonValue::Text("2026-01-20".to_string());
    let today = JsonValue::Text("2026-01-10".to_string());
    let (record, _body) = generate_ruling(&pack, "quarantine", Some(&expiry), Some(&today))
        .expect("the quarantine generates on its ruling day");
    assert!(
        validate_ruling(&record, &pack).expect("the record validates"),
        "the record validates on a later day exactly as on its ruling day"
    );
}

#[test]
fn test_the_validate_ruling_door_refuses_a_tampered_record() {
    let dir = scratch("door");
    let pack = pack_above_floor(&dir);
    let (record, _body) =
        generate_ruling(&pack, "demote", None, None).expect("the ruling generates");
    let pack_path = write(&dir, "pack.json", &tameshi::render::render_json(&pack));
    let record_path = write(&dir, "ruling.json", &tameshi::render::render_json(&record));
    let (code, err) = run(
        &dir,
        &[
            "validate-ruling",
            "--pack",
            pack_path.to_str().expect("the path is text"),
            "--ruling",
            record_path.to_str().expect("the path is text"),
        ],
    );
    assert_eq!(code, 0, "the standing record passes the door: {err}");
    let floor = record.get("floor").expect("the floor stands").clone();
    let tampered_floor = replace_field(&floor, "canary_defects", JsonValue::Int(999));
    let tampered = replace_field(&record, "floor", tampered_floor);
    write(
        &dir,
        "ruling.json",
        &tameshi::render::render_json(&tampered),
    );
    let (code, err) = run(
        &dir,
        &[
            "validate-ruling",
            "--pack",
            pack_path.to_str().expect("the path is text"),
            "--ruling",
            record_path.to_str().expect("the path is text"),
        ],
    );
    assert_eq!(code, 1, "the tampered record refuses at the door");
    assert!(
        err.contains("the ruling record disagrees with the pack it was ruled from"),
        "the door speaks the disagreement words: {err}"
    );
}
