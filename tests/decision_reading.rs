//! The pins of slice 14, the decision document field
//! reading gathering, driving the binary over synthetic
//! decision documents the suite writes itself, each
//! tampered document otherwise sound, its field standing
//! forged in one cell alone.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::parse;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-decision-reading-{}-{name}",
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

/// The sound decision document, standing in exactly the
/// six fields with sound values throughout.
fn decision() -> String {
    r#"{"format": "tameshi-decision/1", "check": "unit checks", "recommendation": "keep", "rationale": "synthetic grounds", "model": "jev-1.13.0", "run": "synthetic-run"}"#
        .to_string()
}

fn fixture(dir: &Path) {
    write(dir, "cl-pack.json", &cl_pack());
    write(dir, "canary.json", &canary());
}

/// Assemble over the decision document text the caller
/// supplies, handing back the exit standing and the words
/// the run spoke.
fn assemble_with_decision_text(dir: &Path, text: &str) -> (i32, String) {
    fixture(dir);
    write(dir, "decision.json", text);
    run(
        dir,
        &[
            "assemble",
            "--catchledger",
            "cl-pack.json",
            "--canaryfield",
            "canary.json",
            "--decision",
            "decision.json",
            "--packs-dir",
            "packs",
        ],
    )
}

#[test]
fn test_a_decision_document_standing_as_a_list_is_refused_as_not_an_object() {
    let dir = scratch("list");
    let (code, err) = assemble_with_decision_text(&dir, r#"["unit checks"]"#);
    assert_eq!(
        code, 1,
        "a decision document standing as a list refuses: {err}"
    );
    assert!(
        err.contains("the decision document must be an object"),
        "the refusal speaks the shared object sentence: {err}"
    );
}

#[test]
fn test_a_decision_carrying_two_unexpected_fields_is_refused_with_the_names_sorted() {
    let dir = scratch("two-unexpected");
    let doc = decision().replace(
        "\"run\": \"synthetic-run\"}",
        "\"run\": \"synthetic-run\", \"zeal\": \"high\", \"confidence\": \"high\"}",
    );
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision carrying two unexpected fields refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (unexpected confidence, zeal)"),
        "the refusal names the unexpected fields in sorted order: {err}"
    );
}

#[test]
fn test_a_decision_missing_two_fields_is_refused_with_the_names_sorted() {
    let dir = scratch("missing-two");
    let doc = r#"{"recommendation": "keep", "rationale": "synthetic grounds", "model": "jev-1.13.0", "run": "synthetic-run"}"#;
    let (code, err) = assemble_with_decision_text(&dir, doc);
    assert_eq!(code, 1, "a decision missing two fields refuses: {err}");
    assert!(
        err.contains("the decision document has the wrong fields (missing check, format)"),
        "the refusal names the missing fields in sorted order: {err}"
    );
}

#[test]
fn test_a_sound_decision_document_assembles_and_its_pack_validates() {
    let dir = scratch("sound");
    let (code, err) = assemble_with_decision_text(&dir, &decision());
    assert_eq!(code, 0, "the sound decision document assembles: {err}");
    let text =
        std::fs::read_to_string(dir.join("packs/001-unit-checks.json")).expect("the pack reads");
    let pack = parse(&text).expect("the pack parses");
    assert_eq!(
        pack.get("format").and_then(|value| value.as_text()),
        Some("tameshi-pack/2")
    );
    let record = pack
        .get("recommendation")
        .expect("the recommendation stands in the pack");
    assert_eq!(
        record
            .get("recommendation")
            .and_then(|value| value.as_text()),
        Some("keep")
    );
    assert_eq!(
        record.get("model").and_then(|value| value.as_text()),
        Some("jev-1.13.0")
    );
    assert_eq!(
        record.get("run").and_then(|value| value.as_text()),
        Some("synthetic-run")
    );
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
            "--decision",
            "decision.json",
        ],
    );
    assert_eq!(code, 0, "the pack validates against its inputs: {err}");
}

#[test]
fn test_a_decision_carrying_one_unexpected_field_is_refused_naming_it() {
    let dir = scratch("one-unexpected");
    let doc = decision().replace(
        "\"run\": \"synthetic-run\"}",
        "\"run\": \"synthetic-run\", \"confidence\": \"high\"}",
    );
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision carrying one unexpected field refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (unexpected confidence)"),
        "the refusal names the one unexpected field: {err}"
    );
}

#[test]
fn test_a_decision_missing_one_field_is_refused_naming_it() {
    let dir = scratch("missing-one");
    let doc = decision().replace(", \"run\": \"synthetic-run\"}", "}");
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(code, 1, "a decision missing one field refuses: {err}");
    assert!(
        err.contains("the decision document has the wrong fields (missing run)"),
        "the refusal names the one missing field: {err}"
    );
}

#[test]
fn test_a_decision_with_an_unknown_recommendation_is_refused_in_its_standing_words() {
    let dir = scratch("unknown-recommendation");
    let doc = decision().replace("\"keep\"", "\"shred\"");
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision with an unknown recommendation refuses: {err}"
    );
    assert!(
        err.contains("the decision document carries an unknown recommendation (shred)"),
        "the refusal keeps the standing recommendation wording: {err}"
    );
}

#[test]
fn test_a_decision_missing_one_field_and_carrying_one_unexpected_field_is_refused_naming_both_parts(
) {
    let dir = scratch("combined-plain");
    let doc = decision().replace("\"run\": \"synthetic-run\"}", "\"zeal\": \"high\"}");
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision missing one field and carrying one unexpected field refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (missing run; unexpected zeal)"),
        "the refusal names both parts in the one sentence: {err}"
    );
}

#[test]
fn test_a_decision_missing_two_fields_and_carrying_two_unexpected_fields_is_refused_with_each_part_sorted(
) {
    let dir = scratch("combined-sorted");
    let doc = r#"{"recommendation": "keep", "rationale": "synthetic grounds", "model": "jev-1.13.0", "run": "synthetic-run", "zeal": "high", "confidence": "high"}"#;
    let (code, err) = assemble_with_decision_text(&dir, doc);
    assert_eq!(
        code, 1,
        "a decision missing two fields and carrying two unexpected fields refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (missing check, format; unexpected confidence, zeal)"),
        "the refusal names each part with its names in sorted order: {err}"
    );
}
