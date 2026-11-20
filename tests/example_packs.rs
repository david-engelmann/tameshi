//! The pins of slice 15, the example pack rebuild,
//! driving the binary over the shipped example files
//! standing in the tree, each pin reading its files
//! from the examples tree as they stand and asking
//! the door it names for the standing EX states.

use std::path::PathBuf;
use std::process::Command;

/// The path of one shipped example file, standing in
/// the examples tree of the tree this suite runs in.
fn example(rel: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(rel)
        .to_str()
        .expect("the example path stands as text")
        .to_string()
}

fn run(args: &[&str]) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
        .args(args)
        .output()
        .expect("the binary runs");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

/// Ask the validate door over one shipped pack against
/// the shipped inputs its sources describe, the ledger
/// and the canary document and the blindness document
/// with no decision document.
fn validate_shipped_pack(pack: &str) -> (i32, String) {
    let pack = example(pack);
    let ledger = example("synthetic-catchledger-ledger.json");
    let canary = example("synthetic-canaryfield-results.json");
    let blindness = example("synthetic-blindgate-evidence.json");
    run(&[
        "validate",
        "--pack",
        &pack,
        "--catchledger",
        &ledger,
        "--canaryfield",
        &canary,
        "--blindgate",
        &blindness,
    ])
}

#[test]
fn test_the_shipped_style_checks_pack_validates_against_its_shipped_inputs() {
    let (code, err) = validate_shipped_pack("packs/001-style-checks.json");
    assert_eq!(
        code, 0,
        "the shipped style checks pack validates against its shipped inputs: {err}"
    );
}

#[test]
fn test_the_shipped_unit_checks_pack_validates_against_its_shipped_inputs() {
    let (code, err) = validate_shipped_pack("packs/002-unit-checks.json");
    assert_eq!(
        code, 0,
        "the shipped unit checks pack validates against its shipped inputs: {err}"
    );
}

#[test]
fn test_the_shipped_ruling_example_validates_against_the_shipped_unit_checks_pack() {
    let pack = example("packs/002-unit-checks.json");
    let ruling = example("ruling-unit-checks.json");
    let body = example("ruling-unit-checks.md");
    let (code, err) = run(&[
        "validate-ruling",
        "--pack",
        &pack,
        "--ruling",
        &ruling,
        "--body",
        &body,
    ]);
    assert_eq!(
        code, 0,
        "the shipped ruling example validates against the shipped unit checks pack: {err}"
    );
}
