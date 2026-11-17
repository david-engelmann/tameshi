//! Diagnostic for slice 9 probe B, the trimmed publisher
//! document. Not a contract pin. The banked trimmed copy of
//! the publisher's committed example, carrying only its
//! gates and its rows, stands in tests/probe_b beside the
//! banked synthetic ledger pack naming its gate. The probe
//! stands discharged where this test passes, the trimmed
//! document assembling cleanly through the assemble door
//! and the pack for its gate standing written and readable.

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::parse;

const TRIMMED: &str = include_str!("probe_b/trimmed.json");
const LEDGER: &str = include_str!("probe_b/ledger.json");

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tameshi-probe-b-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch directory stands");
    dir
}

fn write(dir: &Path, name: &str, text: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the fixture writes");
    path
}

#[test]
fn test_probe_b_the_trimmed_document_assembles_cleanly() {
    let dir = scratch("assemble");
    write(&dir, "cl-pack.json", LEDGER);
    write(&dir, "canary.json", TRIMMED);
    let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
        .args([
            "assemble",
            "--catchledger",
            "cl-pack.json",
            "--canaryfield",
            "canary.json",
            "--packs-dir",
            "packs",
        ])
        .current_dir(&dir)
        .output()
        .expect("the binary runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "the assemble door accepts the trimmed document, stderr {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = std::fs::read_to_string(dir.join("packs/001-tests.json"))
        .expect("the pack for the gate stands written");
    let pack = parse(&text).expect("the pack parses");
    assert!(
        pack.get("canary_performance").is_some(),
        "the pack carries the canary performance"
    );
}
