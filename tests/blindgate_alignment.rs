//! The pins of slice 8, the settlement of the blindgate
//! alignment, over synthetic documents the suite writes
//! itself. The blindness document stands in the exact shape
//! blindgate's emitter publishes under its PORT4-7, one trace
//! per check result per claimed surface entry, so one check's
//! traces stand in separated groups across the document
//! where the check stands in more than one result.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::rulings::generate_ruling;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-blindgate-alignment-{}-{name}",
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

fn catch_record() -> String {
    let entries: Vec<String> = (1..=14)
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

fn ledger() -> String {
    let entry = |name: &str, file: &str| {
        format!(
            r#"{{"name": "{name}", "scorecard": {SCORECARD}, "catch_record": {}, "canaryfield_results": null, "evidence_pack_file": "{file}"}}"#,
            catch_record()
        )
    };
    format!(
        r#"{{"format": "catchledger-ledger/1", "repo": "synthetic-repo", "pricing": {{"currency": "USD", "cost_per_minute": "0.008"}}, "source": {{"ci_history_sha256": null, "git_history_sha256": null, "canaryfield_results_sha256": null}}, "checks": [{}, {}], "totals": {{}}}}"#,
        entry("unit tests", "unit-tests.json"),
        entry("docs check", "docs-check.json")
    )
}

fn canary() -> String {
    let rows: Vec<String> = (0..30)
        .map(|i| {
            format!(
                r#"{{"seed": "seed-{i:02}", "name": "synthetic defect {i}", "defect_class": "synthetic class", "planted_files": ["src/synthetic.py"], "gates": {{"unit tests": "caught", "docs check": "caught"}}}}"#
            )
        })
        .collect();
    format!(
        r#"{{"gates": ["unit tests", "docs check"], "rows": [{}]}}"#,
        rows.join(", ")
    )
}

/// The blindness document in the published emitter's shape.
/// The unit tests traces stand in two separated groups, as
/// they stand where the check stands in two results, with a
/// docs check trace between them.
fn blindgate_document() -> String {
    r#"{"format": "blindgate-evidence/1", "traces": [
  {"check": "unit tests", "surface": "src/**", "observation": "read", "detail": "own trace file trace-unit.log"},
  {"check": "docs check", "surface": "docs/*.md", "observation": "not read", "detail": "not recorded"},
  {"check": "unit tests", "surface": "tests/**", "observation": "read", "detail": "own trace file trace-unit.log"}
]}"#
    .to_string()
}

/// Assemble the packs over the ledger, the canary document,
/// and the published shape blindness document, and return
/// the packs directory.
fn assemble(dir: &Path) -> PathBuf {
    write(dir, "ledger.json", &ledger());
    write(dir, "canary.json", &canary());
    write(dir, "blindgate.json", &blindgate_document());
    let packs = dir.join("packs");
    let (code, err) = run(
        dir,
        &[
            "assemble",
            "--catchledger",
            "ledger.json",
            "--canaryfield",
            "canary.json",
            "--blindgate",
            "blindgate.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 0, "the assemble succeeds: {err}");
    packs
}

fn pack_for(dir: &Path, packs: &Path, check: &str) -> JsonValue {
    for entry in std::fs::read_dir(packs).expect("the packs stand") {
        let path = entry.expect("the entry stands").path();
        let doc = parse(&std::fs::read_to_string(&path).expect("the pack reads"))
            .expect("the pack parses");
        if doc.get("check").and_then(|v| v.as_text()) == Some(check) {
            return doc;
        }
    }
    panic!("no pack stands for {check} in {}", dir.display());
}

fn traces_of(pack: &JsonValue) -> Vec<(String, String, String)> {
    pack.get("blindness_traces")
        .and_then(|v| v.as_list())
        .expect("the traces stand")
        .iter()
        .map(|trace| {
            (
                trace
                    .get("surface")
                    .and_then(|v| v.as_text())
                    .unwrap_or("")
                    .to_string(),
                trace
                    .get("observation")
                    .and_then(|v| v.as_text())
                    .unwrap_or("")
                    .to_string(),
                trace
                    .get("detail")
                    .and_then(|v| v.as_text())
                    .unwrap_or("")
                    .to_string(),
            )
        })
        .collect()
}

#[test]
fn test_a_blindgate_published_shape_document_assembles_with_traces_carried_in_order() {
    let dir = scratch("assembles");
    let packs = assemble(&dir);
    let unit = pack_for(&dir, &packs, "unit tests");
    assert_eq!(traces_of(&unit).len(), 2);
    let docs = pack_for(&dir, &packs, "docs check");
    assert_eq!(traces_of(&docs).len(), 1);
    assert_eq!(
        unit.get("sources")
            .and_then(|s| s.get("blindgate_unmatched_trace_count")),
        Some(&JsonValue::Int(0))
    );
}

#[test]
fn test_a_pack_from_a_published_shape_document_validates_against_its_inputs() {
    let dir = scratch("validates");
    let packs = assemble(&dir);
    let mut pack_path = None;
    for entry in std::fs::read_dir(&packs).expect("the packs stand") {
        let path = entry.expect("the entry stands").path();
        let doc = parse(&std::fs::read_to_string(&path).expect("the pack reads"))
            .expect("the pack parses");
        if doc.get("check").and_then(|v| v.as_text()) == Some("unit tests") {
            pack_path = Some(path);
        }
    }
    let pack_path = pack_path.expect("the unit tests pack stands");
    let (code, err) = run(
        &dir,
        &[
            "validate",
            "--pack",
            pack_path.to_str().expect("the path spells"),
            "--catchledger",
            "ledger.json",
            "--canaryfield",
            "canary.json",
            "--blindgate",
            "blindgate.json",
        ],
    );
    assert_eq!(code, 0, "the pack validates: {err}");
}

#[test]
fn test_traces_from_repeated_results_for_one_check_stand_in_document_order() {
    let dir = scratch("order");
    let packs = assemble(&dir);
    let unit = pack_for(&dir, &packs, "unit tests");
    let surfaces: Vec<String> = traces_of(&unit)
        .into_iter()
        .map(|(surface, _, _)| surface)
        .collect();
    assert_eq!(surfaces, vec!["src/**".to_string(), "tests/**".to_string()]);
}

#[test]
fn test_a_not_read_observation_and_a_not_recorded_detail_are_carried_whole() {
    let dir = scratch("carried-whole");
    let packs = assemble(&dir);
    let docs = pack_for(&dir, &packs, "docs check");
    assert_eq!(
        traces_of(&docs),
        vec![(
            "docs/*.md".to_string(),
            "not read".to_string(),
            "not recorded".to_string()
        )]
    );
}

#[test]
fn test_a_ruling_body_counts_the_traces_of_a_published_shape_pack() {
    let dir = scratch("body-count");
    let packs = assemble(&dir);
    let unit = pack_for(&dir, &packs, "unit tests");
    let (_record, body) = generate_ruling(&unit, "keep", None, None).expect("the ruling generates");
    assert!(
        body.contains("| Blindness traces | 2 |"),
        "the body counts the two traces: {body}"
    );
}
