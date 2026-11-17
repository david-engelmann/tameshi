//! The pins of slice 9, the emitted canaryfield document,
//! driving the binary over synthetic documents the suite
//! writes itself in the shape the publisher's binary emits,
//! the seven top level fields and the emitted row fields
//! standing as they stand in its committed example.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-canary-document-{}-{name}",
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

/// One row in the publisher's emitted shape, carrying the
/// planted files list and the singular planted file and the
/// planted exits beside the result for the one gate.
fn emitted_row(index: usize, word: &str) -> String {
    format!(
        r#"{{"seed": "seed-{index:02}", "name": "synthetic defect {index}", "defect_class": "synthetic class", "planted_files": ["src/synthetic.py"], "planted_file": "src/synthetic.py", "planted_exits": {{"unit checks": 1}}, "gates": {{"unit checks": "{word}"}}}}"#
    )
}

/// A document in the publisher's emitted shape over the
/// given rows, its summary claiming the caught count the
/// caller names, standing beside the gates and the rows as
/// the publisher's own record.
fn emitted_document(rows: &[String], summary_caught: i64) -> String {
    format!(
        r#"{{"target": "synthetic-target", "target_unchanged": true, "clean_exits": {{"unit checks": 0}}, "summary": {{"unit checks": {{"caught": {summary_caught}, "waved-through": 0, "not-applicable": 0, "gate-broken": 0}}}}, "unparseable_files": [], "gates": ["unit checks"], "rows": [{}]}}"#,
        rows.join(", ")
    )
}

fn caught_rows(count: usize) -> Vec<String> {
    (0..count).map(|i| emitted_row(i, "caught")).collect()
}

fn assemble(dir: &Path, canary_text: &str) -> (i32, String) {
    write(dir, "cl-pack.json", &cl_pack());
    write(dir, "canary.json", canary_text);
    run(
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
    )
}

fn pack_document(dir: &Path) -> JsonValue {
    let text =
        std::fs::read_to_string(dir.join("packs/001-unit-checks.json")).expect("the pack reads");
    parse(&text).expect("the pack parses")
}

fn performance(dir: &Path) -> JsonValue {
    pack_document(dir)
        .get("canary_performance")
        .expect("the performance stands")
        .clone()
}

#[test]
fn test_a_canaryfield_document_in_the_emitted_shape_is_read() {
    let dir = scratch("emitted");
    let (code, err) = assemble(&dir, &emitted_document(&caught_rows(30), 30));
    assert_eq!(code, 0, "the assembly stands: {err}");
    let performance = performance(&dir);
    assert_eq!(performance.get("scored"), Some(&JsonValue::Int(30)));
    let counts = performance.get("counts").expect("the counts stand");
    assert_eq!(counts.get("caught"), Some(&JsonValue::Int(30)));
    let results = performance
        .get("results")
        .and_then(|value| value.as_list())
        .expect("the results stand");
    assert_eq!(results.len(), 30);
    let pack = pack_document(&dir);
    let floor = pack.get("floor").expect("the floor accounting stands");
    assert_eq!(floor.get("canary_defects"), Some(&JsonValue::Int(30)));
    assert_eq!(pack.get("below_floor"), Some(&JsonValue::Bool(false)));
}

#[test]
fn test_an_emitted_shape_document_tallies_from_its_rows_and_not_its_summary() {
    let dir = scratch("summary");
    let rows = vec![
        emitted_row(0, "caught"),
        emitted_row(1, "waved-through"),
        emitted_row(2, "not-applicable"),
    ];
    let (code, err) = assemble(&dir, &emitted_document(&rows, 3));
    assert_eq!(code, 0, "the assembly stands: {err}");
    let performance = performance(&dir);
    let counts = performance.get("counts").expect("the counts stand");
    assert_eq!(counts.get("caught"), Some(&JsonValue::Int(1)));
    assert_eq!(counts.get("waved-through"), Some(&JsonValue::Int(1)));
    assert_eq!(counts.get("not-applicable"), Some(&JsonValue::Int(1)));
    assert_eq!(counts.get("gate-broken"), Some(&JsonValue::Int(0)));
    assert_eq!(performance.get("scored"), Some(&JsonValue::Int(2)));
}

#[test]
fn test_a_document_missing_its_rows_list_is_still_refused() {
    let dir = scratch("no-rows");
    let document = r#"{"target": "synthetic-target", "target_unchanged": true, "clean_exits": {"unit checks": 0}, "summary": {"unit checks": {"caught": 0, "waved-through": 0, "not-applicable": 0, "gate-broken": 0}}, "unparseable_files": [], "gates": ["unit checks"]}"#;
    let (code, err) = assemble(&dir, document);
    assert_eq!(code, 1, "a document without rows refuses: {err}");
    assert!(
        err.contains("must carry a gates list and a rows list"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_document_missing_its_gates_list_is_still_refused() {
    let dir = scratch("no-gates");
    let rows = caught_rows(2);
    let document = format!(
        r#"{{"target": "synthetic-target", "target_unchanged": true, "clean_exits": {{"unit checks": 0}}, "summary": {{"unit checks": {{"caught": 2, "waved-through": 0, "not-applicable": 0, "gate-broken": 0}}}}, "unparseable_files": [], "rows": [{}]}}"#,
        rows.join(", ")
    );
    let (code, err) = assemble(&dir, &document);
    assert_eq!(code, 1, "a document without gates refuses: {err}");
    assert!(
        err.contains("must carry a gates list and a rows list"),
        "the refusal keeps its standing words: {err}"
    );
}

#[test]
fn test_a_pack_assembled_from_an_emitted_shape_document_validates() {
    let dir = scratch("validates");
    let (code, err) = assemble(&dir, &emitted_document(&caught_rows(30), 30));
    assert_eq!(code, 0, "the assembly stands: {err}");
    let (code, err) = run(
        &dir,
        &[
            "validate",
            "--catchledger",
            "cl-pack.json",
            "--canaryfield",
            "canary.json",
            "--pack",
            "packs/001-unit-checks.json",
        ],
    );
    assert_eq!(code, 0, "the pack validates: {err}");
}

#[test]
fn test_an_emitted_shape_document_carrying_a_timed_out_result_is_read() {
    let dir = scratch("timed-out");
    let mut rows = caught_rows(29);
    rows.push(emitted_row(29, "timed-out"));
    let (code, err) = assemble(&dir, &emitted_document(&rows, 29));
    assert_eq!(code, 0, "the assembly stands: {err}");
    let performance = performance(&dir);
    assert_eq!(performance.get("scored"), Some(&JsonValue::Int(30)));
    let counts = performance.get("counts").expect("the counts stand");
    assert_eq!(counts.get("caught"), Some(&JsonValue::Int(29)));
    assert!(counts.get("timed-out").is_none(), "no fifth count stands");
    let results = performance
        .get("results")
        .and_then(|value| value.as_list())
        .expect("the results stand");
    assert!(results
        .iter()
        .any(|row| { row.get("result").and_then(|value| value.as_text()) == Some("timed-out") }));
}

#[test]
fn test_a_document_carrying_a_further_top_level_field_is_read() {
    let dir = scratch("further-field");
    let rows: Vec<String> = (0..2)
        .map(|i| {
            format!(
                r#"{{"seed": "seed-{i:02}", "name": "synthetic defect {i}", "defect_class": "synthetic class", "planted_files": ["src/synthetic.py"], "gates": {{"unit checks": "caught"}}}}"#
            )
        })
        .collect();
    let document = format!(
        r#"{{"gates": ["unit checks"], "rows": [{}], "publisher_note": "a field this slice does not name"}}"#,
        rows.join(", ")
    );
    let (code, err) = assemble(&dir, &document);
    assert_eq!(code, 0, "the assembly stands: {err}");
    let performance = performance(&dir);
    assert_eq!(performance.get("scored"), Some(&JsonValue::Int(2)));
}
