//! Pins for slice 35, the rulings calendar date template
//! standings. The pins drive the two public ruling doors over
//! synthetic standings the suite builds itself and assert the
//! refusal its kind and its class name and its wording together.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::errors::{ErrorKind, TameshiError};
use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::rulings::{generate_ruling, validate_ruling};

fn text(value: &str) -> JsonValue {
    JsonValue::Text(value.to_string())
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-calendar-date-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch directory stands");
    dir
}

fn write(dir: &Path, name: &str, contents: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, contents).expect("the fixture writes");
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
/// back as a document, in the form the standing ruling pins
/// build their packs.
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
    let pack_text =
        std::fs::read_to_string(dir.join("packs/001-unit-checks.json")).expect("the pack reads");
    parse(&pack_text).expect("the pack parses")
}

fn sound_pack(name: &str) -> JsonValue {
    let dir = scratch(name);
    pack_above_floor(&dir)
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

/// A quarantine record the generator issued over the pack,
/// carrying the supplied expiry in place of its sound one, so
/// every standing ahead of the validation its calendar test
/// stands passed whole.
fn quarantine_record(pack: &JsonValue, expiry: JsonValue) -> JsonValue {
    let (record, _body) = generate_ruling(
        pack,
        "quarantine",
        Some(&text("2027-01-01")),
        Some(&text("2026-01-01")),
    )
    .expect("the sound quarantine generates");
    replace_field(&record, "expiry", expiry)
}

fn refused<T>(outcome: Result<T, TameshiError>) -> TameshiError {
    match outcome {
        Ok(_) => panic!("the reading accepted a standing it refuses"),
        Err(error) => error,
    }
}

fn assert_sentence(error: &TameshiError, sentence: &str) {
    assert_eq!(
        error.kind,
        ErrorKind::Refusal,
        "the refusal kind stands at its seat"
    );
    assert_eq!(
        error.class_name(),
        "TameshiError",
        "the refusal class stands at its seat"
    );
    assert_eq!(
        error.message, sentence,
        "the refusal wording stands at its seat"
    );
}

const EXPIRY_SENTENCE: &str = "the expiry must be an ISO calendar date";
const CURRENT_SENTENCE: &str = "the current date must be an ISO calendar date";

#[test]
fn test_the_expiry_calendar_date_sentence_stands_at_the_generator_over_a_basic_form_and_an_invalid_date_and_a_year_zero(
) {
    let pack = sound_pack("generator-expiry");
    for expiry in ["20260214", "2023-02-29", "0000-06-01", "1900-02-29"] {
        let error = refused(generate_ruling(
            &pack,
            "quarantine",
            Some(&text(expiry)),
            Some(&text("2026-01-01")),
        ));
        assert_sentence(&error, EXPIRY_SENTENCE);
    }
}

#[test]
fn test_the_current_date_calendar_date_sentence_stands_at_the_generator_over_a_year_zero_and_a_malformed_text(
) {
    let pack = sound_pack("generator-current");
    for today in ["0000-01-01", "not a date"] {
        let error = refused(generate_ruling(
            &pack,
            "quarantine",
            Some(&text("2027-01-01")),
            Some(&text(today)),
        ));
        assert_sentence(&error, CURRENT_SENTENCE);
    }
}

#[test]
fn test_the_expiry_calendar_date_sentence_stands_at_the_validation_over_a_basic_form_and_an_invalid_date(
) {
    let pack = sound_pack("validation-expiry");
    for expiry in ["20260214", "2026-04-31"] {
        let record = quarantine_record(&pack, text(expiry));
        let error = refused(validate_ruling(&record, &pack));
        assert_sentence(&error, EXPIRY_SENTENCE);
    }
}

#[test]
fn test_calendar_date_shapes_stand_refused_at_the_generator_over_a_whole_number_and_null_and_a_blank_text_and_a_text_carrying_a_time_part(
) {
    let pack = sound_pack("generator-shapes");
    for expiry in [
        JsonValue::Int(20260214),
        JsonValue::Null,
        text(""),
        text("2026-02-14T00:00:00Z"),
    ] {
        let error = refused(generate_ruling(
            &pack,
            "quarantine",
            Some(&expiry),
            Some(&text("2026-01-01")),
        ));
        assert_sentence(&error, EXPIRY_SENTENCE);
    }
}

#[test]
fn test_the_calendar_date_tests_stand_in_their_order_at_the_generator_and_the_validation() {
    let pack = sound_pack("order");
    let error = refused(generate_ruling(
        &pack,
        "quarantine",
        Some(&text("2023-02-29")),
        Some(&text("0000-01-01")),
    ));
    assert_sentence(&error, EXPIRY_SENTENCE);
    let error = refused(generate_ruling(
        &pack,
        "nonsense",
        Some(&text("2023-02-29")),
        None,
    ));
    assert_eq!(
        error.kind,
        ErrorKind::Refusal,
        "the refusal kind stands at its seat"
    );
    assert!(
        error.message.contains("an unknown ruling was requested"),
        "the ruling word test stands ahead of the calendar tests, wording was {}",
        error.message
    );
}

#[test]
fn test_sound_calendar_dates_stand_read_at_the_generator_and_the_validation_over_a_leap_day_and_the_last_day_of_the_year(
) {
    let pack = sound_pack("sound");
    let (record, _body) = generate_ruling(
        &pack,
        "quarantine",
        Some(&text("2024-02-29")),
        Some(&text("2024-01-01")),
    )
    .expect("a leap day expiry must issue");
    assert!(
        validate_ruling(&record, &pack).expect("a sound record must validate"),
        "a leap day record validates"
    );
    let (record, _body) = generate_ruling(
        &pack,
        "quarantine",
        Some(&text("9999-12-31")),
        Some(&text("2024-01-01")),
    )
    .expect("the last day of the last year must issue");
    assert!(
        validate_ruling(&record, &pack).expect("a sound record must validate"),
        "the last day record validates"
    );
}
