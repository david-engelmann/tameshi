//! The pins of slice 27, the pack object sentence standings,
//! driving the library readings directly over synthetic
//! packs the suite builds itself.

use tameshi::errors::{ErrorKind, TameshiError};
use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::packs::{recompute_floor, validate_pack};
use tameshi::rulings::{generate_ruling, validate_ruling};

const OBJECT_SENTENCE: &str = "an evidence pack must be an object";

/// The pack the recorded validate case carries, a list of
/// three texts, standing here as the list pack of the pins.
fn list_pack() -> JsonValue {
    parse(r#"["not", "a", "pack"]"#).expect("the list pack parses")
}

/// A pack standing as a text, the shape cell of the slice.
fn text_pack() -> JsonValue {
    parse("\"not a pack\"").expect("the text pack parses")
}

/// A ruling record carrying its format name and nothing
/// else, standing past the record shape test of the ruling
/// validation so that its pack reading is the seat that
/// speaks.
fn ruling_record() -> JsonValue {
    parse(r#"{"format": "tameshi-ruling/1"}"#).expect("the ruling record parses")
}

fn refused<T>(outcome: Result<T, TameshiError>) -> TameshiError {
    match outcome {
        Ok(_) => panic!("the reading accepted a pack that does not stand as an object"),
        Err(error) => error,
    }
}

fn assert_object_refusal(error: &TameshiError, kind: ErrorKind, class: &str) {
    assert_eq!(error.kind, kind, "the refusal kind stands at its seat");
    assert_eq!(
        error.class_name(),
        class,
        "the refusal class stands at its seat"
    );
    assert_eq!(
        error.message, OBJECT_SENTENCE,
        "the refusal wording stands at its seat"
    );
}

#[test]
fn test_a_pack_standing_as_a_list_is_refused_by_the_pack_validation_in_the_pack_validation_kind_with_the_object_sentence(
) {
    let pack = list_pack();
    let error = refused(validate_pack(&pack, &[], None, None, None));
    assert_object_refusal(&error, ErrorKind::PackValidation, "PackValidationError");
}

#[test]
fn test_a_pack_standing_as_a_list_is_refused_by_the_ruling_generator_in_the_plain_kind_with_the_object_sentence(
) {
    let pack = list_pack();
    let error = refused(generate_ruling(&pack, "keep", None, None));
    assert_object_refusal(&error, ErrorKind::Refusal, "TameshiError");
}

#[test]
fn test_a_pack_standing_as_a_list_is_refused_by_the_ruling_validation_in_the_plain_kind_with_the_object_sentence(
) {
    let record = ruling_record();
    let pack = list_pack();
    let error = refused(validate_ruling(&record, &pack));
    assert_object_refusal(&error, ErrorKind::Refusal, "TameshiError");
}

#[test]
fn test_a_pack_standing_as_a_list_is_refused_by_the_floor_recompute_in_the_plain_kind_with_the_object_sentence(
) {
    let pack = list_pack();
    let error = refused(recompute_floor(&pack));
    assert_object_refusal(&error, ErrorKind::Refusal, "TameshiError");
}

#[test]
fn test_a_pack_standing_as_text_meets_the_object_sentence_at_every_seat() {
    let pack = text_pack();
    let error = refused(validate_pack(&pack, &[], None, None, None));
    assert_object_refusal(&error, ErrorKind::PackValidation, "PackValidationError");
    let error = refused(generate_ruling(&pack, "keep", None, None));
    assert_object_refusal(&error, ErrorKind::Refusal, "TameshiError");
    let record = ruling_record();
    let error = refused(validate_ruling(&record, &pack));
    assert_object_refusal(&error, ErrorKind::Refusal, "TameshiError");
    let error = refused(recompute_floor(&pack));
    assert_object_refusal(&error, ErrorKind::Refusal, "TameshiError");
}
