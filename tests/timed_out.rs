//! The pins of slice 5, the timed out result word, driving
//! the binary over synthetic documents the suite writes
//! itself.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tameshi-timed-out-{}-{name}", std::process::id()));
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

/// A canaryfield document of thirty rows for the one gate.
/// Twenty nine rows read caught and the thirtieth reads the
/// word the caller names, so the document's scored count
/// stands at the floor only when that word counts as scored.
fn canary(last_word: &str) -> String {
    let mut rows: Vec<String> = (0..29)
        .map(|i| {
            format!(
                r#"{{"seed": "seed-{i:02}", "name": "synthetic defect {i}", "defect_class": "synthetic class", "planted_files": ["src/synthetic.py"], "gates": {{"unit checks": "caught"}}}}"#
            )
        })
        .collect();
    rows.push(format!(
        r#"{{"seed": "seed-29", "name": "synthetic defect 29", "defect_class": "synthetic class", "planted_files": ["src/synthetic.py"], "gates": {{"unit checks": "{last_word}"}}}}"#
    ));
    format!(
        r#"{{"gates": ["unit checks"], "rows": [{}]}}"#,
        rows.join(", ")
    )
}

fn assemble(dir: &Path, last_word: &str) -> (i32, String) {
    write(dir, "cl-pack.json", &cl_pack(1..=14));
    write(dir, "canary.json", &canary(last_word));
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
fn test_a_canaryfield_document_with_a_timed_out_result_is_read() {
    let dir = scratch("read");
    let (code, err) = assemble(&dir, "timed-out");
    assert_eq!(code, 0, "the assembly stands: {err}");
    let performance = performance(&dir);
    let results = performance
        .get("results")
        .and_then(|value| value.as_list())
        .expect("the results stand");
    assert_eq!(results.len(), 30);
    assert!(
        results.iter().any(|row| {
            row.get("result").and_then(|value| value.as_text()) == Some("timed-out")
        }),
        "the timed-out word stands in its matched row"
    );
}

#[test]
fn test_a_timed_out_result_counts_as_scored() {
    let dir = scratch("scored");
    let (code, err) = assemble(&dir, "timed-out");
    assert_eq!(code, 0, "the assembly stands: {err}");
    let performance = performance(&dir);
    assert_eq!(
        performance.get("scored"),
        Some(&JsonValue::Int(30)),
        "twenty nine catches and one timed-out row score thirty"
    );
    let pack = pack_document(&dir);
    let floor = pack.get("floor").expect("the floor accounting stands");
    assert_eq!(floor.get("canary_defects"), Some(&JsonValue::Int(30)));
    assert_eq!(pack.get("below_floor"), Some(&JsonValue::Bool(false)));
}

#[test]
fn test_a_timed_out_result_is_counted_under_no_count_word() {
    let dir = scratch("counts");
    let (code, err) = assemble(&dir, "timed-out");
    assert_eq!(code, 0, "the assembly stands: {err}");
    let performance = performance(&dir);
    let counts = performance.get("counts").expect("the counts stand");
    assert_eq!(counts.get("caught"), Some(&JsonValue::Int(29)));
    assert_eq!(counts.get("waved-through"), Some(&JsonValue::Int(0)));
    assert_eq!(counts.get("not-applicable"), Some(&JsonValue::Int(0)));
    assert_eq!(counts.get("gate-broken"), Some(&JsonValue::Int(0)));
    assert!(counts.get("timed-out").is_none(), "no fifth count stands");
}

#[test]
fn test_a_pack_with_timed_out_rows_validates() {
    let dir = scratch("validates");
    let (code, err) = assemble(&dir, "timed-out");
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
fn test_a_ruling_issues_over_a_pack_whose_floor_needs_its_timed_out_rows() {
    let dir = scratch("floor");
    let (code, err) = assemble(&dir, "timed-out");
    assert_eq!(code, 0, "the assembly stands: {err}");
    let (code, err) = run(
        &dir,
        &[
            "rule",
            "--pack",
            "packs/001-unit-checks.json",
            "--ruling",
            "keep",
            "--out",
            "out",
        ],
    );
    assert_eq!(
        code, 0,
        "the ruling issues over the pack at its floor: {err}"
    );
}

#[test]
fn test_an_unknown_result_word_is_still_refused() {
    let dir = scratch("unknown");
    let (code, err) = assemble(&dir, "exploded");
    assert_ne!(code, 0, "an unknown word refuses the document");
    assert!(
        err.contains("not a known result word"),
        "the refusal names the unknown word: {err}"
    );
}
