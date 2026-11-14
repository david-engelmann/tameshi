//! The pins of slice 3, the decision record and the kill
//! ruling, driving the binary over synthetic documents the
//! suite writes itself.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tameshi-decision-{}-{name}", std::process::id()));
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

fn decision(check: &str, recommendation: &str) -> String {
    format!(
        r#"{{"format": "tameshi-decision/1", "check": "{check}", "recommendation": "{recommendation}", "rationale": "synthetic grounds", "model": "jev-1.13.0", "run": "synthetic-run"}}"#
    )
}

fn fixture(dir: &Path, days: std::ops::RangeInclusive<i32>) {
    write(dir, "cl-pack.json", &cl_pack(days));
    write(dir, "canary.json", &canary());
}

fn assemble_with_decision(dir: &Path, recommendation: &str) -> PathBuf {
    fixture(dir, 1..=14);
    write(
        dir,
        "decision.json",
        &decision("unit checks", recommendation),
    );
    let (code, err) = run(
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
    );
    assert_eq!(code, 0, "the assembly stands: {err}");
    dir.join("packs/001-unit-checks.json")
}

fn pack_document(path: &Path) -> JsonValue {
    let text = std::fs::read_to_string(path).expect("the pack reads");
    parse(&text).expect("the pack parses")
}

#[test]
fn test_a_decision_with_a_wrong_format_name_is_refused() {
    let dir = scratch("wrong-format");
    fixture(&dir, 1..=14);
    let doc = decision("unit checks", "kill").replace("tameshi-decision/1", "tameshi-decision/9");
    write(&dir, "decision.json", &doc);
    let (code, err) = run(
        &dir,
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
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("the decision document carries the wrong format name"),
        "{err}"
    );
}

#[test]
fn test_an_unknown_field_in_a_decision_is_refused() {
    let dir = scratch("unknown-field");
    fixture(&dir, 1..=14);
    let doc = decision("unit checks", "kill").replace(
        "\"run\": \"synthetic-run\"}",
        "\"run\": \"synthetic-run\", \"confidence\": \"high\"}",
    );
    write(&dir, "decision.json", &doc);
    let (code, err) = run(
        &dir,
        &[
            "assemble",
            "--catchledger",
            "cl-pack.json",
            "--decision",
            "decision.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("the decision document has the wrong fields (unexpected confidence)"),
        "{err}"
    );
}

#[test]
fn test_a_decision_with_an_unknown_recommendation_is_refused() {
    let dir = scratch("unknown-recommendation");
    fixture(&dir, 1..=14);
    write(&dir, "decision.json", &decision("unit checks", "shred"));
    let (code, err) = run(
        &dir,
        &[
            "assemble",
            "--catchledger",
            "cl-pack.json",
            "--decision",
            "decision.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("the decision document carries an unknown recommendation (shred)"),
        "{err}"
    );
}

#[test]
fn test_a_pack_with_a_decision_stands_as_pack_format_2() {
    let dir = scratch("pack-format-2");
    let pack_path = assemble_with_decision(&dir, "demote");
    let pack = pack_document(&pack_path);
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
        Some("demote")
    );
    assert_eq!(
        record.get("model").and_then(|value| value.as_text()),
        Some("jev-1.13.0")
    );
    assert_eq!(
        record.get("run").and_then(|value| value.as_text()),
        Some("synthetic-run")
    );
}

#[test]
fn test_a_pack_without_a_decision_stands_as_before() {
    let dir = scratch("pack-format-1");
    fixture(&dir, 1..=14);
    let (code, err) = run(
        &dir,
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
    assert_eq!(code, 0, "{err}");
    let pack = pack_document(&dir.join("packs/001-unit-checks.json"));
    assert_eq!(
        pack.get("format").and_then(|value| value.as_text()),
        Some("tameshi-pack/1")
    );
    assert!(pack.get("recommendation").is_none());
}

#[test]
fn test_a_decision_naming_a_check_the_run_does_not_assemble_is_refused() {
    let dir = scratch("decision-other-check");
    fixture(&dir, 1..=14);
    write(&dir, "decision.json", &decision("other checks", "kill"));
    let (code, err) = run(
        &dir,
        &[
            "assemble",
            "--catchledger",
            "cl-pack.json",
            "--decision",
            "decision.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains(
            "the decision document names a check the run does not assemble (other checks)"
        ),
        "{err}"
    );
}

#[test]
fn test_a_decision_document_is_validated_before_any_pack_is_written() {
    let dir = scratch("decision-validated-first");
    fixture(&dir, 1..=14);
    let doc = decision("unit checks", "kill").replace("synthetic grounds", "");
    write(&dir, "decision.json", &doc);
    let (code, err) = run(
        &dir,
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
    );
    assert_eq!(code, 1);
    assert!(err.contains("must be nonempty text"), "{err}");
    assert!(
        !dir.join("packs").exists(),
        "no pack stands where the decision refused"
    );
}

#[test]
fn test_pack_format_2_validation_reads_the_recommendation() {
    let dir = scratch("validate-format-2");
    assemble_with_decision(&dir, "keep");
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
    assert_eq!(code, 0, "the validation stands: {err}");
}

#[test]
fn test_a_pack_format_2_without_a_recommendation_is_refused() {
    let dir = scratch("format-2-no-recommendation");
    let pack_path = assemble_with_decision(&dir, "keep");
    let text = std::fs::read_to_string(&pack_path).expect("the pack reads");
    let mut pack = parse(&text).expect("the pack parses");
    // Remove the recommendation member, leaving the second
    // format name standing alone.
    if let JsonValue::Object(fields) = &mut pack {
        fields.retain(|(name, _)| name != "recommendation");
    }
    write(&dir, "stripped.json", &tameshi::render::render_json(&pack));
    let (code, err) = run(
        &dir,
        &[
            "rule",
            "--pack",
            "stripped.json",
            "--ruling",
            "keep",
            "--out",
            "out",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("the evidence pack carries the format tameshi-pack/2 and no recommendation"),
        "{err}"
    );
}

#[test]
fn test_a_kill_ruling_issues_on_a_pack_recommending_kill() {
    let dir = scratch("kill-issues");
    assemble_with_decision(&dir, "kill");
    let (code, err) = run(
        &dir,
        &[
            "rule",
            "--pack",
            "packs/001-unit-checks.json",
            "--ruling",
            "kill",
            "--out",
            "out",
        ],
    );
    assert_eq!(code, 0, "the kill ruling issues: {err}");
    let record = pack_document(&dir.join("out/ruling-unit-checks.json"));
    assert_eq!(
        record.get("ruling").and_then(|value| value.as_text()),
        Some("kill")
    );
    assert_eq!(
        record
            .get("state_after_ruling")
            .and_then(|value| value.as_text()),
        Some("removed")
    );
    let citation = record
        .get("recommendation")
        .expect("the ruling record cites the recommendation");
    assert_eq!(
        citation
            .get("recommendation")
            .and_then(|value| value.as_text()),
        Some("kill")
    );
    let body =
        std::fs::read_to_string(dir.join("out/ruling-unit-checks.md")).expect("the body reads");
    assert!(body.contains("Decision model recommendation"), "{body}");
}

#[test]
fn test_a_kill_ruling_without_a_recommendation_is_refused() {
    let dir = scratch("kill-no-recommendation");
    fixture(&dir, 1..=14);
    let (code, _) = run(
        &dir,
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
    assert_eq!(code, 0);
    let (code, err) = run(
        &dir,
        &[
            "rule",
            "--pack",
            "packs/001-unit-checks.json",
            "--ruling",
            "kill",
            "--out",
            "out",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("a kill ruling requires a decision record recommending kill in the pack"),
        "{err}"
    );
}

#[test]
fn test_a_kill_ruling_against_another_recommendation_is_refused() {
    let dir = scratch("kill-against-keep");
    assemble_with_decision(&dir, "keep");
    let (code, err) = run(
        &dir,
        &[
            "rule",
            "--pack",
            "packs/001-unit-checks.json",
            "--ruling",
            "kill",
            "--out",
            "out",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("a kill ruling requires a decision record recommending kill in the pack"),
        "{err}"
    );
}

#[test]
fn test_a_kill_ruling_below_the_floor_refuses_on_the_floor() {
    let dir = scratch("kill-below-floor");
    fixture(&dir, 1..=3);
    write(&dir, "decision.json", &decision("unit checks", "kill"));
    let (code, err) = run(
        &dir,
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
    );
    assert_eq!(code, 0, "the below floor pack assembles: {err}");
    let (code, err) = run(
        &dir,
        &[
            "rule",
            "--pack",
            "packs/001-unit-checks.json",
            "--ruling",
            "kill",
            "--out",
            "out",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        !err.contains("a kill ruling requires"),
        "the floor speaks first: {err}"
    );
    assert!(
        err.contains("below the floor") || err.contains("floor"),
        "{err}"
    );
}
