//! The pins of slice 29, the canary performance sentence
//! standings, driving the library reading directly over
//! synthetic performances the suite builds itself.

use tameshi::errors::{ErrorKind, TameshiError};
use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::packs::{canary_performance, validate_canary_performance};

const RESULT_WORD_SENTENCE: &str = "a canary performance result word is not a known result word";
const PLANTED_FILES_SENTENCE: &str =
    "a canary performance result row carries malformed planted files";
const ROW_FIELDS_SENTENCE: &str = "a canary performance result row has the wrong fields";
const COUNTS_COVER_SENTENCE: &str =
    "the canary performance counts must cover the four result words";
const COUNTS_DISAGREE_SENTENCE: &str =
    "the canary performance counts disagree with their result rows";

/// The counts covering the four result words at zero, the
/// counts standing under which the row tests stand reached.
const COVERING_COUNTS: &str =
    "{\"caught\": 0, \"waved-through\": 0, \"not-applicable\": 0, \"gate-broken\": 0}";

/// A canary performance carrying one result row, its
/// counts and its row its result and its planted files
/// standing as the fragments the caller passes, each a
/// fragment of JSON text.
fn performance_with(counts: &str, result: &str, planted_files: &str) -> JsonValue {
    parse(&format!(
        "{{\"gate\": \"style\", \"counts\": {counts}, \"scored\": 1, \
         \"catch_rate\": 1.0, \"results\": [{{\"seed\": \"seed-1\", \"name\": \"row one\", \
         \"defect_class\": \"style\", \"result\": {result}, \"planted_files\": {planted_files}}}]}}"
    ))
    .expect("the performance parses")
}

/// A canary performance carrying one result row that
/// bears no planted files field, its counts standing as
/// the fragment the caller passes.
fn performance_without_planted_files(counts: &str) -> JsonValue {
    parse(&format!(
        "{{\"gate\": \"style\", \"counts\": {counts}, \"scored\": 1, \
         \"catch_rate\": 1.0, \"results\": [{{\"seed\": \"seed-1\", \"name\": \"row one\", \
         \"defect_class\": \"style\", \"result\": \"caught\"}}]}}"
    ))
    .expect("the performance parses")
}

/// A normalized canaryfield results document carrying
/// one result row standing sound, its counts standing
/// tallied from the row it carries.
fn normalized_document() -> JsonValue {
    parse(
        "{\"source_format\": \"canaryfield-catch-table\", \"gate\": \"style\", \
         \"counts\": {\"caught\": 1, \"waved-through\": 0, \"not-applicable\": 0, \
         \"gate-broken\": 0}, \"results\": [{\"seed\": \"seed-1\", \"name\": \"row one\", \
         \"defect_class\": \"style\", \"result\": \"caught\", \
         \"planted_files\": [\"src/a.rs\"]}]}",
    )
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

fn assert_read<T>(outcome: Result<T, TameshiError>, standing: &str) {
    if let Err(error) = outcome {
        panic!(
            "the reading refused {standing} in the words {}",
            error.message
        );
    }
}

#[test]
fn test_a_result_row_carrying_an_unknown_result_word_is_refused_by_the_performance_validation_with_the_result_word_sentence(
) {
    let performance = performance_with(COVERING_COUNTS, "\"vanished\"", "[\"src/a.rs\"]");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, RESULT_WORD_SENTENCE);
}

#[test]
fn test_a_result_row_carrying_planted_files_standing_as_a_bare_text_or_as_null_is_refused_by_the_performance_validation_with_the_planted_files_sentence_and_a_row_carrying_no_planted_files_field_is_refused_with_the_row_fields_sentence_and_a_null_performance_stands_read(
) {
    let performance = performance_with(COVERING_COUNTS, "\"caught\"", "\"src/a.rs\"");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, PLANTED_FILES_SENTENCE);
    let performance = performance_with(COVERING_COUNTS, "\"caught\"", "null");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, PLANTED_FILES_SENTENCE);
    let performance = performance_without_planted_files(COVERING_COUNTS);
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, ROW_FIELDS_SENTENCE);
    assert_read(
        validate_canary_performance(&JsonValue::Null),
        "a performance standing null",
    );
}

#[test]
fn test_the_counts_cover_sentence_stands_ahead_of_the_result_word_sentence_and_the_counts_disagree_sentence_stands_behind_the_row_tests(
) {
    let performance = performance_with(
        "{\"caught\": 0, \"waved-through\": 0, \"not-applicable\": 0}",
        "\"vanished\"",
        "[\"src/a.rs\"]",
    );
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, COUNTS_COVER_SENTENCE);
    let performance = performance_with(
        "{\"caught\": 2, \"waved-through\": 0, \"not-applicable\": 0, \"gate-broken\": 0}",
        "\"caught\"",
        "[\"src/a.rs\"]",
    );
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, COUNTS_DISAGREE_SENTENCE);
}

#[test]
fn test_a_result_word_standing_as_a_whole_number_and_planted_files_holding_a_whole_number_meet_the_same_sentences_at_their_seats(
) {
    let performance = performance_with(COVERING_COUNTS, "7", "[\"src/a.rs\"]");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, RESULT_WORD_SENTENCE);
    let performance = performance_with(COVERING_COUNTS, "\"caught\"", "[\"src/a.rs\", 7]");
    let error = refused(validate_canary_performance(&performance));
    assert_sentence(&error, PLANTED_FILES_SENTENCE);
}

#[test]
fn test_a_performance_built_by_the_builder_from_a_sound_normalized_document_stands_read_whole_by_the_performance_validation(
) {
    let document = normalized_document();
    let built = canary_performance(&document)
        .expect("the builder reads the sound document")
        .expect("the builder builds a performance from a sound document");
    assert_read(
        validate_canary_performance(&built),
        "the performance the builder built",
    );
}
