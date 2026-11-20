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

/// Assemble over the catchledger pack text the caller
/// supplies beside the sound canary document and the
/// sound decision document, handing back the exit
/// standing and the words the run spoke.
fn assemble_with_pack_text(dir: &Path, text: &str) -> (i32, String) {
    write(dir, "cl-pack.json", text);
    write(dir, "canary.json", &canary());
    write(dir, "decision.json", &decision());
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
fn test_a_decision_carrying_a_further_field_under_the_empty_name_is_refused_with_the_placeholder_printed_in_its_place(
) {
    let dir = scratch("empty-name-alone");
    let doc = decision().replace(
        "\"run\": \"synthetic-run\"}",
        "\"run\": \"synthetic-run\", \"\": \"high\"}",
    );
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision carrying a further field under the empty name refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (unexpected empty name)"),
        "the refusal prints the placeholder in the place of the empty name: {err}"
    );
}

#[test]
fn test_a_decision_missing_one_field_and_carrying_the_empty_name_beside_a_named_field_is_refused_with_the_placeholder_at_the_head_of_the_unexpected_part(
) {
    let dir = scratch("empty-name-combined");
    let doc = decision().replace(
        "\"run\": \"synthetic-run\"}",
        "\"\": \"high\", \"confidence\": \"high\"}",
    );
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision missing one field and carrying the empty name beside a named field refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (missing run; unexpected empty name, confidence)"),
        "the refusal names the missing field and carries the placeholder at the head of the unexpected part: {err}"
    );
}

#[test]
fn test_a_catchledger_pack_carrying_a_field_under_the_empty_name_is_refused_with_the_placeholder_printed_in_its_place(
) {
    let dir = scratch("empty-name-window");
    let pack = cl_pack().replace(
        "\"format\": \"catchledger-pack/1\"",
        "\"\": \"high\", \"format\": \"catchledger-pack/1\"",
    );
    let (code, err) = assemble_with_pack_text(&dir, &pack);
    assert_eq!(
        code, 1,
        "a catchledger pack carrying a field under the empty name refuses: {err}"
    );
    assert!(
        err.contains("the catchledger pack has the wrong fields (unexpected empty name)"),
        "the refusal prints the placeholder in the place of the empty name: {err}"
    );
}

#[test]
fn test_a_decision_carrying_a_further_field_under_a_name_carrying_a_space_is_refused_with_the_name_printed_as_it_stands(
) {
    let dir = scratch("space-name-fence");
    let doc = decision().replace(
        "\"run\": \"synthetic-run\"}",
        "\"run\": \"synthetic-run\", \"publisher note\": \"high\"}",
    );
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision carrying a further field under a name carrying a space refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (unexpected publisher note)"),
        "the refusal prints the name as it stands: {err}"
    );
}

#[test]
fn test_a_decision_carrying_a_further_field_under_a_name_carrying_the_list_separator_is_refused_with_the_name_printed_across_it(
) {
    let dir = scratch("list-separator-alone");
    let doc = decision().replace(
        "\"run\": \"synthetic-run\"}",
        "\"run\": \"synthetic-run\", \"zeal, confidence\": \"high\"}",
    );
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision carrying a further field under a name carrying the list separator refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (unexpected zeal, confidence)"),
        "the refusal prints the name across the list separator it carries: {err}"
    );
}

#[test]
fn test_a_decision_carrying_a_further_field_under_a_name_carrying_the_part_separator_is_refused_with_the_name_printed_across_it(
) {
    let dir = scratch("part-separator-alone");
    let doc = decision().replace(
        "\"run\": \"synthetic-run\"}",
        "\"run\": \"synthetic-run\", \"a; b\": \"high\"}",
    );
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision carrying a further field under a name carrying the part separator refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (unexpected a; b)"),
        "the refusal prints the name across the part separator it carries: {err}"
    );
}

#[test]
fn test_a_decision_missing_one_field_and_carrying_a_further_field_under_a_name_carrying_the_part_separator_is_refused_with_the_name_printed_whole_in_the_unexpected_part(
) {
    let dir = scratch("part-separator-combined");
    let doc = decision().replace("\"run\": \"synthetic-run\"}", "\"a; b\": \"high\"}");
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision missing one field and carrying a further field under a name carrying the part separator refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (missing run; unexpected a; b)"),
        "the refusal names the missing field and prints the name whole across the part separator in the unexpected part: {err}"
    );
}

#[test]
fn test_a_decision_carrying_a_further_field_under_a_name_carrying_the_part_separator_before_the_words_of_a_missing_field_is_refused_with_the_unexpected_part_alone_reading_as_two_parts(
) {
    let dir = scratch("fake-missing-part");
    let doc = decision().replace(
        "\"run\": \"synthetic-run\"}",
        "\"run\": \"synthetic-run\", \"x; missing run\": \"high\"}",
    );
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision carrying a further field under a name carrying the part separator before the words of a missing field refuses: {err}"
    );
    assert!(
        err.contains("the decision document has the wrong fields (unexpected x; missing run)"),
        "the refusal carries the unexpected part alone reading as two parts: {err}"
    );
}

#[test]
fn test_a_decision_carrying_two_further_fields_one_under_a_name_carrying_the_list_separator_is_refused_with_the_two_fields_reading_as_three_names(
) {
    let dir = scratch("list-separator-beside-name");
    let doc = decision().replace(
        "\"run\": \"synthetic-run\"}",
        "\"run\": \"synthetic-run\", \"zeal\": \"high\", \"zeal, confidence\": \"high\"}",
    );
    let (code, err) = assemble_with_decision_text(&dir, &doc);
    assert_eq!(
        code, 1,
        "a decision carrying two further fields, one under a name carrying the list separator, refuses: {err}"
    );
    assert!(
        err.contains(
            "the decision document has the wrong fields (unexpected zeal, zeal, confidence)"
        ),
        "the refusal reads the two fields as three names: {err}"
    );
}

#[test]
fn test_a_catchledger_pack_carrying_a_field_under_a_name_carrying_the_list_separator_is_refused_with_the_name_printed_across_it(
) {
    let dir = scratch("list-separator-window");
    let pack = cl_pack().replace(
        "\"format\": \"catchledger-pack/1\"",
        "\"zeal, confidence\": \"high\", \"format\": \"catchledger-pack/1\"",
    );
    let (code, err) = assemble_with_pack_text(&dir, &pack);
    assert_eq!(
        code, 1,
        "a catchledger pack carrying a field under a name carrying the list separator refuses: {err}"
    );
    assert!(
        err.contains("the catchledger pack has the wrong fields (unexpected zeal, confidence)"),
        "the refusal prints the name across the list separator it carries: {err}"
    );
}
