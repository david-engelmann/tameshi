//! The pins of slice 30, the reader label sentence
//! standings, driving the reader through the library
//! door over synthetic documents and synthetic files
//! the suite writes itself in its own scratch
//! directories.

use std::path::{Path, PathBuf};

use tameshi::errors::{ErrorKind, TameshiError};
use tameshi::reader::{read_file_bytes, read_json_document, read_json_file};

/// The seven labels the doors give the reader, in the
/// order the doors stand in the binary, the assemble
/// door first and the validate-ruling door last.
const DOOR_LABELS: [&str; 7] = [
    "catchledger evidence",
    "canaryfield results",
    "blindness evidence",
    "decision document",
    "evidence pack",
    "ruling record",
    "ruling body",
];

/// The six labels that reach the file reading, the
/// ruling body label standing apart at the bytes
/// reading alone.
const DOCUMENT_LABELS: [&str; 6] = [
    "catchledger evidence",
    "canaryfield results",
    "blindness evidence",
    "decision document",
    "evidence pack",
    "ruling record",
];

/// The sentence the reader gives a document that is
/// not valid JSON, under the label standing in its
/// label place.
fn not_valid_json(label: &str) -> String {
    format!("the {label} is not valid JSON")
}

/// The sentence the reader gives a file it cannot
/// read, under the label standing in its label place.
fn could_not_be_read(label: &str) -> String {
    format!("the {label} could not be read")
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-reader-labels-{}-{name}",
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

fn write_bytes(dir: &Path, name: &str, bytes: &[u8]) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, bytes).expect("the fixture writes");
    path
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
fn test_an_invalid_document_text_is_refused_under_its_caller_label_with_the_not_valid_json_sentence(
) {
    let error = refused(read_json_document("not a document", "evidence pack"));
    assert_sentence(&error, &not_valid_json("evidence pack"));
}

#[test]
fn test_a_file_carrying_bytes_that_are_not_utf8_is_refused_with_the_not_valid_json_sentence_and_a_file_carrying_invalid_json_as_text_meets_the_same_sentence(
) {
    let dir = scratch("not-valid-json");
    let bytes_path = write_bytes(&dir, "bytes.bin", &[0xff, 0xfe, 0x00]);
    let error = refused(read_json_file(&bytes_path, "evidence pack"));
    assert_sentence(&error, &not_valid_json("evidence pack"));
    let text_path = write(&dir, "text.json", "not a document");
    let error = refused(read_json_file(&text_path, "evidence pack"));
    assert_sentence(&error, &not_valid_json("evidence pack"));
}

#[test]
fn test_a_document_file_that_cannot_be_read_is_refused_with_the_could_not_be_read_sentence() {
    let dir = scratch("could-not-be-read");
    let absent = dir.join("absent.json");
    let error = refused(read_json_file(&absent, "evidence pack"));
    assert_sentence(&error, &could_not_be_read("evidence pack"));
}

#[test]
fn test_a_body_file_that_cannot_be_read_is_refused_with_the_could_not_be_read_sentence_and_a_standing_body_file_reads_whole(
) {
    let dir = scratch("body-bytes");
    let absent = dir.join("absent.txt");
    let error = refused(read_file_bytes(&absent, "ruling body"));
    assert_sentence(&error, &could_not_be_read("ruling body"));
    let path = write(&dir, "body.txt", "the ruling body text\n");
    let bytes = read_file_bytes(&path, "ruling body").expect("the standing body file reads");
    assert_eq!(bytes, b"the ruling body text\n", "the body reads whole");
}

#[test]
fn test_every_door_label_stands_in_the_not_valid_json_sentence_at_the_document_door() {
    for label in DOOR_LABELS {
        let error = refused(read_json_document("not a document", label));
        assert_sentence(&error, &not_valid_json(label));
    }
}

#[test]
fn test_every_document_label_stands_in_the_could_not_be_read_sentence_at_the_file_door() {
    let dir = scratch("door-labels");
    let absent = dir.join("absent.json");
    for label in DOCUMENT_LABELS {
        let error = refused(read_json_file(&absent, label));
        assert_sentence(&error, &could_not_be_read(label));
    }
}
