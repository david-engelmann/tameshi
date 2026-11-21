//! The pins of slice 26, the duplicate check name refusal,
//! driving the binary over synthetic ledgers the suite
//! writes itself.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tameshi-ledger-{}-{name}", std::process::id()));
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

const SCORECARD_FIRES_41: &str = r#"{"fires": 41, "blocks": 6, "judged_commits": 20, "flaky_commits": 1, "flake_rate": 0.05, "runtime_seconds": 900, "runtime_minutes": 15.0, "catches": 3, "false_blocks": 1, "unattributed_blocks": 2, "false_block_rate": 0.1667, "total_cost_usd": 0.12, "cost_per_catch_usd": 0.04}"#;

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

fn entry(name: &str, scorecard: &str, file: &str) -> String {
    format!(
        r#"{{"name": "{name}", "scorecard": {scorecard}, "catch_record": {}, "canaryfield_results": null, "evidence_pack_file": "{file}"}}"#,
        catch_record(1..=14)
    )
}

fn ledger(entries: &[String]) -> String {
    format!(
        r#"{{"format": "catchledger-ledger/1", "repo": "synthetic-repo", "pricing": {{"currency": "USD", "cost_per_minute": "0.008"}}, "source": {{"ci_history_sha256": null, "git_history_sha256": null, "canaryfield_results_sha256": null}}, "checks": [{}], "totals": {{}}}}"#,
        entries.join(", ")
    )
}

fn duplicate_ledger() -> String {
    ledger(&[
        entry("unit checks", SCORECARD, "001-unit-checks.json"),
        entry("unit checks", SCORECARD_FIRES_41, "002-unit-checks.json"),
    ])
}

fn pack_document(path: &Path) -> JsonValue {
    let text = std::fs::read_to_string(path).expect("the pack reads");
    parse(&text).expect("the pack parses")
}

#[test]
fn test_a_ledger_whose_entries_name_the_same_check_twice_is_refused_at_the_assemble_door_and_no_pack_stands_written(
) {
    let dir = scratch("duplicate-at-assemble");
    write(&dir, "ledger.json", &duplicate_ledger());
    let (code, err) = run(
        &dir,
        &[
            "assemble",
            "--catchledger",
            "ledger.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("the catchledger ledger names a check more than once (unit checks)"),
        "{err}"
    );
    assert!(
        !dir.join("packs").exists(),
        "no pack stands where the ledger refused"
    );
}

#[test]
fn test_a_ledger_whose_entries_name_the_same_check_twice_is_refused_at_the_validate_door_through_its_rebuild(
) {
    let dir = scratch("duplicate-at-validate");
    let single = ledger(&[entry("unit checks", SCORECARD, "001-unit-checks.json")]);
    write(&dir, "ledger-one.json", &single);
    let (code, err) = run(
        &dir,
        &[
            "assemble",
            "--catchledger",
            "ledger-one.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 0, "the single entry ledger assembles: {err}");
    write(&dir, "ledger-two.json", &duplicate_ledger());
    let (code, err) = run(
        &dir,
        &[
            "validate",
            "--pack",
            "packs/001-unit-checks.json",
            "--catchledger",
            "ledger-two.json",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("the catchledger ledger names a check more than once (unit checks)"),
        "{err}"
    );
}

#[test]
fn test_a_ledger_whose_entries_name_distinct_checks_assembles_both_packs_in_the_sorted_order() {
    let dir = scratch("distinct-names-assemble");
    let doc = ledger(&[
        entry("unit checks", SCORECARD, "002-unit-checks.json"),
        entry("style checks", SCORECARD_FIRES_41, "001-style-checks.json"),
    ]);
    write(&dir, "ledger.json", &doc);
    let (code, err) = run(
        &dir,
        &[
            "assemble",
            "--catchledger",
            "ledger.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 0, "the distinct ledger assembles: {err}");
    let first = pack_document(&dir.join("packs/001-style-checks.json"));
    assert_eq!(
        first.get("check").and_then(|value| value.as_text()),
        Some("style checks")
    );
    let second = pack_document(&dir.join("packs/002-unit-checks.json"));
    assert_eq!(
        second.get("check").and_then(|value| value.as_text()),
        Some("unit checks")
    );
}

#[test]
fn test_a_pack_document_stands_unaffected_by_the_ledger_repetition_refusal() {
    let dir = scratch("pack-document-unaffected");
    let doc = format!(
        r#"{{"format": "catchledger-pack/1", "repo": "synthetic-repo", "check": "unit checks", "scorecard": {SCORECARD}, "catch_record": {}, "canaryfield_results": null, "false_block_rate": 0.1667, "pricing": {{"currency": "USD", "cost_per_minute": "0.008"}}, "source": {{"ci_history_sha256": null, "git_history_sha256": null, "canaryfield_results_sha256": null}}}}"#,
        catch_record(1..=14)
    );
    write(&dir, "cl-pack.json", &doc);
    let (code, err) = run(
        &dir,
        &[
            "assemble",
            "--catchledger",
            "cl-pack.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 0, "the pack document assembles: {err}");
    let pack = pack_document(&dir.join("packs/001-unit-checks.json"));
    assert_eq!(
        pack.get("check").and_then(|value| value.as_text()),
        Some("unit checks")
    );
}

#[test]
fn test_a_check_repeated_at_a_distance_in_a_ledger_is_the_name_the_refusal_echoes() {
    let dir = scratch("repeated-at-a-distance");
    let doc = ledger(&[
        entry("unit checks", SCORECARD, "001-unit-checks.json"),
        entry("style checks", SCORECARD, "002-style-checks.json"),
        entry("unit checks", SCORECARD_FIRES_41, "003-unit-checks.json"),
    ]);
    write(&dir, "ledger.json", &doc);
    let (code, err) = run(
        &dir,
        &[
            "assemble",
            "--catchledger",
            "ledger.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("the catchledger ledger names a check more than once (unit checks)"),
        "{err}"
    );
    assert!(
        !err.contains("more than once (style checks)"),
        "the first repeated name stands echoed: {err}"
    );
}

#[test]
fn test_a_repeated_entry_standing_unsound_in_its_own_fields_meets_its_own_reading_ahead_of_the_repetition_refusal(
) {
    let dir = scratch("repeated-entry-unsound");
    let broken = SCORECARD.replace("\"fires\": 40, ", "");
    let doc = ledger(&[
        entry("unit checks", SCORECARD, "001-unit-checks.json"),
        entry("unit checks", &broken, "002-unit-checks.json"),
    ]);
    write(&dir, "ledger.json", &doc);
    let (code, err) = run(
        &dir,
        &[
            "assemble",
            "--catchledger",
            "ledger.json",
            "--packs-dir",
            "packs",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        err.contains("the catchledger scorecard has the wrong fields (missing fires)"),
        "{err}"
    );
    assert!(
        !err.contains("names a check more than once"),
        "the entry its own reading stands ahead of the repetition refusal: {err}"
    );
}
