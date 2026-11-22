//! The pins of slice 32, the timestamp sentence standings,
//! driving the library reading directly over synthetic
//! values the suite builds itself.

use tameshi::errors::{ErrorKind, TameshiError};
use tameshi::ingest::{parse_timestamp, validate_catch_record, validate_history_window};
use tameshi::jsonvalue::{parse, JsonValue};

/// The three labels the timestamp reading speaks under,
/// the catch record label and the two history window
/// labels, standing in the order the doors call them.
const LABELS: [&str; 3] = [
    "catch record started at timestamp",
    "history window first run start",
    "history window last run update",
];

const BLOCK_RUN_ID_SENTENCE: &str = "the catch record block run identifier must be nonempty text";
const OUTCOME_SENTENCE: &str = "a catch record outcome is not a known outcome word";
const ENTRY_FIELDS_SENTENCE: &str =
    "the catch record entry has the wrong fields (missing started_at)";
const WINDOW_FIELDS_SENTENCE: &str =
    "the history window must carry a first run start and a last run update";
const STAND_OR_FALL_SENTENCE: &str = "the history window timestamps must stand or fall together";
const PRECEDES_SENTENCE: &str = "the history window last run update precedes its first run start";

/// The ISO sentence standing over the label the caller
/// passes, the sentence both ISO seats speak.
fn iso_sentence(label: &str) -> String {
    format!("the {label} must be an ISO 8601 timestamp")
}

/// The offset sentence standing over the label the caller
/// passes, the sentence the offset seat speaks.
fn offset_sentence(label: &str) -> String {
    format!("the {label} must carry a timezone offset")
}

/// A catch record entry standing sound in every field
/// but the three the caller passes, each a fragment of
/// JSON text.
fn catch_entry(started_at: &str, block_run_id: &str, outcome: &str) -> JsonValue {
    parse(&format!(
        "{{\"block_run_id\": {block_run_id}, \"blocked_commit\": \"abc123\", \
         \"started_at\": {started_at}, \"outcome\": {outcome}, \
         \"flagged_paths\": [\"src/lib.rs\"], \"evidence\": {{\"type\": \"log\", \
         \"fix_commits\": [], \"validating_run_id\": \"run-2\", \"reason\": \"because\"}}}}"
    ))
    .expect("the catch entry parses")
}

/// A catch record entry carrying no started at field,
/// standing sound in every field it carries.
fn catch_entry_without_started_at() -> JsonValue {
    parse(
        "{\"block_run_id\": \"run-1\", \"blocked_commit\": \"abc123\", \
         \"outcome\": \"catch\", \"flagged_paths\": [\"src/lib.rs\"], \
         \"evidence\": {\"type\": \"log\", \"fix_commits\": [], \
         \"validating_run_id\": \"run-2\", \"reason\": \"because\"}}",
    )
    .expect("the catch entry parses")
}

/// A history window carrying its two timestamps standing
/// as the fragments the caller passes.
fn history_window(first: &str, last: &str) -> JsonValue {
    parse(&format!(
        "{{\"first_run_started_at\": {first}, \"last_run_updated_at\": {last}}}"
    ))
    .expect("the history window parses")
}

/// A history window carrying a third field beside its
/// two timestamps, standing as the fragments the caller
/// passes.
fn history_window_with_extra_field(first: &str, last: &str) -> JsonValue {
    parse(&format!(
        "{{\"first_run_started_at\": {first}, \"last_run_updated_at\": {last}, \
         \"extra\": 1}}"
    ))
    .expect("the history window parses")
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
fn test_a_timestamp_value_standing_as_no_text_or_as_a_blank_text_is_refused_with_the_iso_sentence()
{
    let label = "catch record started at timestamp";
    for value in [
        JsonValue::Int(7),
        JsonValue::Null,
        JsonValue::List(Vec::new()),
        JsonValue::Text("   ".to_string()),
    ] {
        let error = refused(parse_timestamp(&value, label));
        assert_sentence(&error, &iso_sentence(label));
    }
}

#[test]
fn test_a_timestamp_text_the_grammar_refuses_is_refused_with_the_iso_sentence_under_each_caller_label(
) {
    for label in LABELS {
        for text in ["not-a-timestamp", "2026-01-01T00:10:00z"] {
            let error = refused(parse_timestamp(&JsonValue::Text(text.to_string()), label));
            assert_sentence(&error, &iso_sentence(label));
        }
    }
}

#[test]
fn test_a_timestamp_text_carrying_no_offset_is_refused_with_the_offset_sentence_under_each_caller_label(
) {
    for label in LABELS {
        for text in ["2026-01-01T00:10:00", "2026-01-01"] {
            let error = refused(parse_timestamp(&JsonValue::Text(text.to_string()), label));
            assert_sentence(&error, &offset_sentence(label));
        }
    }
}

#[test]
fn test_a_sound_timestamp_reads_at_its_instant_in_each_of_its_forms() {
    let label = "catch record started at timestamp";
    for text in [
        "2026-01-01T00:10:00Z",
        "2026-01-01T02:10:00+02:00",
        "  2026-01-01T00:10:00Z  ",
        "2026-W01-4T00:10:00Z",
        "2026-01-01 00:10:00Z",
    ] {
        let instant = parse_timestamp(&JsonValue::Text(text.to_string()), label)
            .expect("the sound form reads");
        assert_eq!(instant, 1767226200000000, "the form reads at its instant");
    }
    let instant = parse_timestamp(
        &JsonValue::Text("2026-01-01T00:10:00.1234567Z".to_string()),
        label,
    )
    .expect("the fraction form reads");
    assert_eq!(
        instant, 1767226200123456,
        "the fraction past the sixth digit stands discarded"
    );
}

#[test]
fn test_the_timestamp_sentences_stand_in_the_catch_record_reading_order() {
    let sound = "\"2026-01-01T00:10:00Z\"";
    let entry = catch_entry("7", "\"  \"", "\"catch\"");
    let error = refused(validate_catch_record(&JsonValue::List(vec![entry])));
    assert_sentence(&error, BLOCK_RUN_ID_SENTENCE);
    let entry = catch_entry("7", "\"run-1\"", "\"catch\"");
    let error = refused(validate_catch_record(&JsonValue::List(vec![entry])));
    assert_sentence(&error, &iso_sentence(LABELS[0]));
    let entry = catch_entry(sound, "\"run-1\"", "\"unknown-word\"");
    let error = refused(validate_catch_record(&JsonValue::List(vec![entry])));
    assert_sentence(&error, OUTCOME_SENTENCE);
    let entry = catch_entry("7", "\"run-1\"", "\"unknown-word\"");
    let error = refused(validate_catch_record(&JsonValue::List(vec![entry])));
    assert_sentence(&error, &iso_sentence(LABELS[0]));
    let entry = catch_entry_without_started_at();
    let error = refused(validate_catch_record(&JsonValue::List(vec![entry])));
    assert_sentence(&error, ENTRY_FIELDS_SENTENCE);
    let first = catch_entry(sound, "\"run-1\"", "\"catch\"");
    let second = catch_entry("\"2026-01-01\"", "\"run-1\"", "\"catch\"");
    let error = refused(validate_catch_record(&JsonValue::List(vec![first, second])));
    assert_sentence(&error, &offset_sentence(LABELS[0]));
}

#[test]
fn test_the_timestamp_sentences_stand_in_the_history_window_reading_order() {
    let sound = "\"2026-01-01T00:10:00Z\"";
    assert_read(validate_history_window(&JsonValue::Null), "a null window");
    let window = history_window("null", "null");
    assert_read(validate_history_window(&window), "a window standing empty");
    let window = history_window(sound, "null");
    let error = refused(validate_history_window(&window));
    assert_sentence(&error, STAND_OR_FALL_SENTENCE);
    let window = history_window("\"2026-01-01\"", sound);
    let error = refused(validate_history_window(&window));
    assert_sentence(&error, &offset_sentence(LABELS[1]));
    let window = history_window(sound, "\"not-a-timestamp\"");
    let error = refused(validate_history_window(&window));
    assert_sentence(&error, &iso_sentence(LABELS[2]));
    let window = history_window("\"not-a-timestamp\"", "\"also-not-a-timestamp\"");
    let error = refused(validate_history_window(&window));
    assert_sentence(&error, &iso_sentence(LABELS[1]));
    let window = history_window("\"2026-01-02T00:10:00Z\"", sound);
    let error = refused(validate_history_window(&window));
    assert_sentence(&error, PRECEDES_SENTENCE);
    let window = history_window_with_extra_field("\"not-a-timestamp\"", sound);
    let error = refused(validate_history_window(&window));
    assert_sentence(&error, WINDOW_FIELDS_SENTENCE);
}
