//! The pins of slice 34, the canary gates sentence
//! standings, driving the library document reading
//! directly over synthetic documents the suite builds
//! itself.

use tameshi::errors::{ErrorKind, TameshiError};
use tameshi::ingest::validate_canary_document;
use tameshi::jsonvalue::{parse, JsonValue};

const GATES_SENTENCE: &str = "the canaryfield gates must be a list of distinct nonempty names";
const FIELDS_SENTENCE: &str = "the canaryfield document must carry a gates list and a rows list";
const ROWS_SENTENCE: &str = "the canaryfield rows must be a list";
const NEWLINE_SENTENCE: &str = "the canaryfield gate name must not carry a newline";
const EACH_GATE_SENTENCE: &str = "a canaryfield row must carry a result for each gate";

/// A direct canaryfield document, its gates and its rows
/// standing as the fragments the caller passes, each a
/// fragment of JSON text.
fn document_carrying(gates: &str, rows: &str) -> JsonValue {
    parse(&format!("{{\"gates\": {gates}, \"rows\": {rows}}}"))
        .expect("the canaryfield document parses")
}

/// A document row over the style gate in its sound form.
fn sound_row() -> String {
    "{\"seed\": \"seed-1\", \"name\": \"row one\", \"defect_class\": \"style\", \
     \"gates\": {\"style\": \"caught\"}}"
        .to_string()
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
fn test_the_gates_sentence_stands_at_the_list_branch_over_an_empty_list_and_a_member_standing_as_a_whole_number_and_a_blank_text(
) {
    let document = document_carrying("[]", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("[\"style\", 7]", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("[\"style\", \"  \"]", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("[null]", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("[\"style\", true]", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
}

#[test]
fn test_the_gates_sentence_stands_at_the_list_branch_over_duplicate_names_in_and_out_of_order() {
    let document = document_carrying("[\"style\", \"style\"]", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("[\"lint\", \"style\", \"lint\"]", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
}

#[test]
fn test_the_gates_sentence_stands_at_the_non_list_branch_over_a_bare_text_and_an_object_and_a_whole_number_and_null(
) {
    let document = document_carrying("\"style\"", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("{\"style\": true}", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("7", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("null", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("true", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
}

#[test]
fn test_the_fields_sentence_stands_ahead_of_the_gates_sentence_and_the_gates_sentence_stands_ahead_of_the_rows_sentence_and_the_row_tests(
) {
    let document = parse("{\"gates\": [\"style\", \"style\"]}").expect("the document parses");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, FIELDS_SENTENCE);
    let document = document_carrying("[\"style\", \"style\"]", "\"rows\"");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("[]", "[7]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("[\"style\"]", "\"rows\"");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, ROWS_SENTENCE);
}

#[test]
fn test_a_gate_name_carrying_a_newline_meets_the_newline_sentence_behind_the_gates_test_and_a_sound_document_stands_read(
) {
    let document = document_carrying(
        "[\"a\\nb\"]",
        "[{\"seed\": \"seed-1\", \"name\": \"row one\", \"defect_class\": \"style\", \
         \"gates\": {\"a\\nb\": \"caught\"}}]",
    );
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, NEWLINE_SENTENCE);
    let document = document_carrying("[\"a\\nb\", \"a\\nb\"]", "[]");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, GATES_SENTENCE);
    let document = document_carrying("[\"a\\nb\"]", "\"rows\"");
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, NEWLINE_SENTENCE);
    let document = document_carrying(
        "[\"a\\nb\"]",
        "[{\"seed\": \"seed-1\", \"name\": \"row one\", \"defect_class\": \"style\", \
         \"gates\": {\"a\\nb\": \"vanished\"}}]",
    );
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, NEWLINE_SENTENCE);
    let document = document_carrying(
        "[\"a\\rb\"]",
        "[{\"seed\": \"seed-1\", \"name\": \"row one\", \"defect_class\": \"style\", \
         \"gates\": {\"a\\rb\": \"caught\"}}]",
    );
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, NEWLINE_SENTENCE);
    let rows = format!("[{}]", sound_row());
    let document = document_carrying("[\"style\"]", &rows);
    assert_read(validate_canary_document(&document), "a sound document");
}

#[test]
fn test_gate_names_stand_as_written_untrimmed_and_a_row_keyed_by_the_trimmed_name_meets_the_each_gate_sentence(
) {
    let document = document_carrying(
        "[\"style\", \" style\"]",
        "[{\"seed\": \"seed-1\", \"name\": \"row one\", \"defect_class\": \"style\", \
         \"gates\": {\"style\": \"caught\", \" style\": \"caught\"}}]",
    );
    assert_read(
        validate_canary_document(&document),
        "a document whose gates stand untrimmed",
    );
    let document = document_carrying(
        "[\"Style\", \"style\"]",
        "[{\"seed\": \"seed-1\", \"name\": \"row one\", \"defect_class\": \"style\", \
         \"gates\": {\"Style\": \"caught\", \"style\": \"caught\"}}]",
    );
    assert_read(
        validate_canary_document(&document),
        "a document whose gates part by case alone",
    );
    let document = document_carrying(
        "[\" style\"]",
        "[{\"seed\": \"seed-1\", \"name\": \"row one\", \"defect_class\": \"style\", \
         \"gates\": {\"style\": \"caught\"}}]",
    );
    let error = refused(validate_canary_document(&document));
    assert_sentence(&error, EACH_GATE_SENTENCE);
}
