//! The document reader of obligation PORT-3.
//!
//! A document is read under the label the command line gives it, and
//! the refusals carry that label. A document that is not valid JSON
//! is refused as such, and a document in which any object at any
//! depth carries the same key twice is refused with the wording
//! obligation TM2-2 states, naming the label and the repeated key.

use std::path::Path;

use crate::errors::{Result, TameshiError};
use crate::jsonvalue::{self, JsonValue, ParseError};

/// Read a document text under its label.
pub fn read_json_document(text: &str, label: &str) -> Result<JsonValue> {
    match jsonvalue::parse(text) {
        Ok(value) => Ok(value),
        Err(ParseError::DuplicateKey(key)) => Err(TameshiError::refusal(format!(
            "the {label} carries the key {key} more than once"
        ))),
        Err(ParseError::Invalid) => Err(TameshiError::refusal(format!(
            "the {label} is not valid JSON"
        ))),
    }
}

/// Read a document's raw bytes from a local file under its
/// label, with the refusal the command line gives a file it
/// cannot read.
pub fn read_file_bytes(path: &Path, label: &str) -> Result<Vec<u8>> {
    std::fs::read(path).map_err(|_| TameshiError::refusal(format!("the {label} could not be read")))
}

/// Read a document from a local file under its label, with the
/// refusal the command line gives a file it cannot read.
pub fn read_json_file(path: &Path, label: &str) -> Result<JsonValue> {
    match std::fs::read(path) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(text) => read_json_document(&text, label),
            Err(_) => Err(TameshiError::refusal(format!(
                "the {label} is not valid JSON"
            ))),
        },
        Err(_) => Err(TameshiError::refusal(format!(
            "the {label} could not be read"
        ))),
    }
}
