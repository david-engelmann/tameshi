//! The pins of slice 33, the canary counts sentence
//! standings, driving the two library readings directly
//! over synthetic standings the suite builds itself.

use tameshi::errors::{ErrorKind, TameshiError};
use tameshi::ingest::validate_normalized_canary;
use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::packs::validate_canary_performance;

const PERF_COVER_SENTENCE: &str = "the canary performance counts must cover the four result words";
const PERF_NONNEGATIVE_SENTENCE: &str =
    "the canary performance counts must be nonnegative whole numbers";
const PERF_DISAGREE_SENTENCE: &str =
    "the canary performance counts disagree with their result rows";
const NORM_COVER_SENTENCE: &str = "the canaryfield counts must cover the four result words";
const NORM_NONNEGATIVE_SENTENCE: &str = "the canaryfield counts must be nonnegative whole numbers";
const NORM_DISAGREE_SENTENCE: &str = "the canaryfield counts disagree with their result rows";
const GATE_SENTENCE: &str = "the canary performance needs a gate name";
const NORM_PLANTED_SENTENCE: &str = "a canaryfield result row carries malformed planted files";

fn performance_carrying(counts: &str, result: &str, planted_files: &str) -> JsonValue {
    parse(&format!(
        "{{\"gate\": \"style\", \"counts\": {counts}, \"scored\": 1, \
         \"catch_rate\": 1.0, \"results\": [{{\"seed\": \"seed-1\", \"name\": \"row one\", \
         \"defect_class\": \"style\", \"result\": {result}, \"planted_files\": {planted_files}}}]}}"
    ))
    .expect("the performance parses")
}

fn performance_with_gate(gate: &str, counts: &str) -> JsonValue {
    parse(&format!(
        "{{\"gate\": {gate}, \"counts\": {counts}, \"scored\": 1, \
         \"catch_rate\": 1.0, \"results\": [{{\"seed\": \"seed-1\", \"name\": \"row one\", \
         \"defect_class\": \"style\", \"result\": \"caught\", \"planted_files\": [\"src/a.rs\"]}}]}}"
    ))
    .expect("the performance parses")
}

fn normalized_carrying(counts: &str, result: &str, planted_files: &str) -> JsonValue {
    parse(&format!(
        "{{\"source_format\": \"canaryfield-catch-table\", \"gate\": \"style\", \
         \"counts\": {counts}, \"results\": [{{\"seed\": \"seed-1\", \"name\": \"row one\", \
         \"defect_class\": \"style\", \"result\": {result}, \"planted_files\": {planted_files}}}]}}"
    ))
    .expect("the normalized document parses")
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

#[test]
fn test_the_counts_cover_sentence_stands_at_both_readings() {
    let performance = performance_carrying(
        "{\"caught\": 0, \"waved-through\": 0, \"not-applicable\": 0}",
        "\"vanished\"",
        "[\"src/a.rs\"]",
    );
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, PERF_COVER_SENTENCE);
    let document = normalized_carrying(
        "{\"caught\": 0, \"waved-through\": 0, \"not-applicable\": 0}",
        "\"vanished\"",
        "[\"src/a.rs\"]",
    );
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, NORM_COVER_SENTENCE);
}

#[test]
fn test_the_counts_nonnegative_sentence_stands_at_both_readings() {
    let counts =
        "{\"caught\": -1, \"waved-through\": 0, \"not-applicable\": 0, \"gate-broken\": 0}";
    let performance = performance_carrying(counts, "\"vanished\"", "[\"src/a.rs\"]");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, PERF_NONNEGATIVE_SENTENCE);
    let document = normalized_carrying(counts, "\"vanished\"", "[\"src/a.rs\"]");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, NORM_NONNEGATIVE_SENTENCE);
}

#[test]
fn test_the_counts_disagree_sentence_stands_at_both_readings() {
    let counts = "{\"caught\": 2, \"waved-through\": 0, \"not-applicable\": 0, \"gate-broken\": 0}";
    let performance = performance_carrying(counts, "\"caught\"", "[\"src/a.rs\"]");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, PERF_DISAGREE_SENTENCE);
    let document = normalized_carrying(counts, "\"caught\"", "[\"src/a.rs\"]");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, NORM_DISAGREE_SENTENCE);
}

#[test]
fn test_the_cover_test_stands_ahead_of_the_nonnegative_test_in_both_readings_and_the_disagree_test_stands_behind_the_row_tests_in_the_normalized_reading(
) {
    let counts = "{\"caught\": -1, \"waved-through\": 0, \"not-applicable\": 0}";
    let performance = performance_carrying(counts, "\"vanished\"", "[\"src/a.rs\"]");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, PERF_COVER_SENTENCE);
    let document = normalized_carrying(counts, "\"vanished\"", "[\"src/a.rs\"]");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, NORM_COVER_SENTENCE);
    let counts = "{\"caught\": 2, \"waved-through\": 0, \"not-applicable\": 0, \"gate-broken\": 0}";
    let document = normalized_carrying(counts, "\"caught\"", "\"src/a.rs\"");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, NORM_PLANTED_SENTENCE);
}

#[test]
fn test_counts_standing_as_a_bare_text_and_a_count_standing_as_a_decimal_meet_the_counts_sentences_at_both_readings(
) {
    let performance = performance_carrying("\"tally\"", "\"caught\"", "[\"src/a.rs\"]");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, PERF_COVER_SENTENCE);
    let document = normalized_carrying("\"tally\"", "\"caught\"", "[\"src/a.rs\"]");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, NORM_COVER_SENTENCE);
    let counts =
        "{\"caught\": 1.0, \"waved-through\": 0, \"not-applicable\": 0, \"gate-broken\": 0}";
    let performance = performance_carrying(counts, "\"caught\"", "[\"src/a.rs\"]");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, PERF_NONNEGATIVE_SENTENCE);
    let document = normalized_carrying(counts, "\"caught\"", "[\"src/a.rs\"]");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, NORM_NONNEGATIVE_SENTENCE);
}

#[test]
fn test_the_gate_sentence_stands_ahead_of_the_counts_cover_sentence() {
    let counts = "{\"caught\": 0, \"waved-through\": 0, \"not-applicable\": 0}";
    let performance = performance_with_gate("\"\"", counts);
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, GATE_SENTENCE);
    let performance = performance_with_gate("7", counts);
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, GATE_SENTENCE);
}
