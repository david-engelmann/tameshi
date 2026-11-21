//! The pins of slice 28, the canary readings sentence
//! standings, driving the library readings directly over
//! synthetic documents the suite builds itself.

use tameshi::errors::{ErrorKind, TameshiError};
use tameshi::ingest::{
    normalize_canary_document, validate_canary_document, validate_normalized_canary,
};
use tameshi::jsonvalue::{parse, JsonValue};

const RESULT_WORD_SENTENCE: &str = "a canaryfield result word is not a known result word";
const RESULT_ROW_SENTENCE: &str = "a canaryfield result row carries malformed planted files";
const ROW_SENTENCE: &str = "a canaryfield row carries malformed planted files";
const SINGULAR_SENTENCE: &str = "a canaryfield row carries a malformed planted file";

/// A normalized canaryfield results document carrying one
/// result row, the row its result and its planted files
/// standing as the fragments the caller passes, each a
/// fragment of JSON text.
fn normalized_document(result: &str, planted_files: &str) -> JsonValue {
    parse(&format!(
        "{{\"source_format\": \"canaryfield-catch-table\", \"gate\": \"style\", \
         \"counts\": {{\"caught\": 0, \"waved-through\": 0, \"not-applicable\": 0, \
         \"gate-broken\": 0}}, \"results\": [{{\"seed\": \"seed-1\", \"name\": \"row one\", \
         \"defect_class\": \"style\", \"result\": {result}, \"planted_files\": {planted_files}}}]}}"
    ))
    .expect("the normalized document parses")
}

/// A direct canaryfield document carrying the one gate
/// style and one row, the row standing as the fragment
/// the caller passes.
fn canary_document(row: &str) -> JsonValue {
    parse(&format!("{{\"gates\": [\"style\"], \"rows\": [{row}]}}"))
        .expect("the canaryfield document parses")
}

/// A document row over the style gate, its gates map and
/// its planted standings standing as the fragments the
/// caller passes, each a fragment of JSON text, the
/// planted fragment carrying its own leading comma or
/// standing empty.
fn document_row(gates: &str, planted: &str) -> String {
    format!(
        "{{\"seed\": \"seed-1\", \"name\": \"row one\", \"defect_class\": \"style\", \
         \"gates\": {gates}{planted}}}"
    )
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
fn test_a_result_row_carrying_an_unknown_result_word_is_refused_by_the_normalized_reading_with_the_result_word_sentence(
) {
    let document = normalized_document("\"vanished\"", "[\"src/a.rs\"]");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, RESULT_WORD_SENTENCE);
}

#[test]
fn test_a_row_carrying_an_unknown_result_word_for_a_gate_is_refused_by_the_document_reading_with_the_result_word_sentence(
) {
    let document = canary_document(&document_row("{\"style\": \"vanished\"}", ""));
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, RESULT_WORD_SENTENCE);
}

#[test]
fn test_a_result_row_carrying_planted_files_standing_as_a_bare_text_is_refused_by_the_normalized_reading_with_the_result_row_sentence(
) {
    let document = normalized_document("\"caught\"", "\"src/a.rs\"");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, RESULT_ROW_SENTENCE);
}

#[test]
fn test_a_row_carrying_planted_files_as_a_bare_text_is_refused_by_the_document_reading_with_the_row_sentence_and_the_absent_and_null_rows_stand_read(
) {
    let document = canary_document(&document_row(
        "{\"style\": \"caught\"}",
        ", \"planted_files\": \"src/a.rs\"",
    ));
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, ROW_SENTENCE);
    let document = canary_document(&document_row("{\"style\": \"caught\"}", ""));
    assert_read(
        validate_canary_document(&document),
        "a row carrying no planted files",
    );
    let document = canary_document(&document_row(
        "{\"style\": \"caught\"}",
        ", \"planted_files\": null",
    ));
    assert_read(
        validate_canary_document(&document),
        "a row carrying its planted files as null",
    );
}

#[test]
fn test_the_normalization_refuses_a_planted_file_standing_as_a_whole_number_in_the_singular_sentence_and_planted_files_standing_as_a_bare_text_in_the_row_sentence(
) {
    let document = canary_document(&document_row(
        "{\"style\": \"caught\"}",
        ", \"planted_file\": 7",
    ));
    let error = refused(normalize_canary_document(&document, "style"));
    assert_sentence(&error, SINGULAR_SENTENCE);
    let document = canary_document(&document_row(
        "{\"style\": \"caught\"}",
        ", \"planted_files\": \"src/a.rs\"",
    ));
    let error = refused(normalize_canary_document(&document, "style"));
    assert_sentence(&error, ROW_SENTENCE);
}

#[test]
fn test_a_result_word_standing_as_a_whole_number_and_planted_files_holding_a_whole_number_meet_the_same_sentences_at_every_seat(
) {
    let document = normalized_document("7", "[\"src/a.rs\"]");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, RESULT_WORD_SENTENCE);
    let document = canary_document(&document_row("{\"style\": 7}", ""));
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, RESULT_WORD_SENTENCE);
    let document = normalized_document("\"caught\"", "[\"src/a.rs\", 7]");
    let error = refused(validate_normalized_canary(&document));
    assert_sentence(&error, RESULT_ROW_SENTENCE);
    let document = canary_document(&document_row(
        "{\"style\": \"caught\"}",
        ", \"planted_files\": [\"src/a.rs\", 7]",
    ));
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, ROW_SENTENCE);
}
