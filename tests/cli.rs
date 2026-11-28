//! The recorded command line battery of obligation PORT3-4.
//!
//! The recordings were made by running the landed Python command
//! line as a black box over invocations drawn from its own suite
//! and from the argument edges the contract probe recorded. Each
//! case stands in a scratch directory with its input files, and its
//! steps run there in order. The comparison is at content level,
//! the exit code and the standard error text and the written files
//! byte for byte. For a step the Python argument reader refused,
//! the recording carries no standard error text, because the
//! contract states the argument reader texts stand outside the
//! port, and the step instead asserts the binary writes its own
//! usage line and nothing to standard output.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};

fn recordings() -> Vec<JsonValue> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/conformance/cli_cases.json"
    );
    let text = std::fs::read_to_string(path).expect("the battery recordings stand in the tree");
    let document = parse(&text).expect("the battery recordings parse");
    document
        .get("cases")
        .and_then(|cases| cases.as_list())
        .expect("the battery recordings carry a case list")
        .to_vec()
}

fn text_of(value: &JsonValue) -> &str {
    value.as_text().unwrap_or("")
}

fn case_dir(index: usize, name: &str) -> PathBuf {
    let slug: String = name
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect();
    std::env::temp_dir()
        .join(format!("tameshi-cli-battery-{}", std::process::id()))
        .join(format!("case-{index:03}-{slug}"))
}

/// A scratch directory that removes itself when it leaves scope,
/// so a panicking assertion leaves no state behind and concurrent
/// test processes never share a path.
struct ScratchDir(PathBuf);

impl ScratchDir {
    fn hold(path: &Path) -> ScratchDir {
        ScratchDir(path.to_path_buf())
    }
}

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn collect_files(root: &Path, dir: &Path, into: &mut BTreeMap<String, Vec<u8>>) {
    let entries = std::fs::read_dir(dir).expect("a case directory reads");
    for entry in entries {
        let path = entry.expect("a case directory entry reads").path();
        if path.is_dir() {
            collect_files(root, &path, into);
        } else {
            let rel = path
                .strip_prefix(root)
                .expect("a written file stands under the case directory")
                .to_string_lossy()
                .replace('\\', "/");
            let bytes = std::fs::read(&path).expect("a written file reads");
            into.insert(rel, bytes);
        }
    }
}

fn run_case(index: usize, case: &JsonValue) {
    let name = text_of(case.get("name").expect("each case carries a name")).to_string();
    let dir = case_dir(index, &name);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).expect("a stale case directory clears");
    }
    std::fs::create_dir_all(&dir).expect("a case directory stands");
    let _scratch = ScratchDir::hold(&dir);
    let inputs = case
        .get("inputs")
        .and_then(|value| value.as_object())
        .expect("each case carries its inputs")
        .clone();
    let input_names: Vec<&str> = inputs.iter().map(|(rel, _)| rel.as_str()).collect();
    for (rel, text) in &inputs {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("an input directory stands");
        }
        std::fs::write(&path, text_of(text).as_bytes()).expect("an input file is written");
    }
    let dir_text = dir
        .to_str()
        .expect("a case directory path is text")
        .to_string();
    let steps = case
        .get("steps")
        .and_then(|value| value.as_list())
        .expect("each case carries its steps");
    for step in steps {
        let argv: Vec<String> = step
            .get("argv")
            .and_then(|value| value.as_list())
            .expect("each step carries its arguments")
            .iter()
            .map(|part| text_of(part).replace("{DIR}", &dir_text))
            .collect();
        let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
            .args(&argv)
            .current_dir(&dir)
            .output()
            .expect("the binary runs");
        let expected_exit = match step.get("exit") {
            Some(JsonValue::Int(code)) => *code,
            _ => panic!("each step carries its exit code"),
        };
        assert_eq!(
            output.status.code(),
            Some(expected_exit as i32),
            "case {name} step {argv:?} exits with a different code"
        );
        match step.get("stderr") {
            Some(JsonValue::Text(expected)) => {
                let stderr =
                    String::from_utf8(output.stderr.clone()).expect("standard error is text");
                assert_eq!(
                    stderr, *expected,
                    "case {name} step {argv:?} speaks a different standard error"
                );
            }
            _ => {
                assert!(
                    !output.stderr.is_empty(),
                    "case {name} step {argv:?} writes its usage line to standard error"
                );
            }
        }
        let stdout_empty = match step.get("stdout_empty") {
            Some(JsonValue::Bool(standing)) => *standing,
            _ => panic!("each step carries its standard output standing"),
        };
        if stdout_empty {
            assert!(
                output.stdout.is_empty(),
                "case {name} step {argv:?} writes to standard output"
            );
        } else {
            assert!(
                !output.stdout.is_empty(),
                "case {name} step {argv:?} answers the help flag on standard output"
            );
        }
    }
    let mut written: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    collect_files(&dir, &dir, &mut written);
    for rel in &input_names {
        written.remove(*rel);
    }
    let mut expected: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    if let Some(outputs) = case.get("outputs").and_then(|value| value.as_object()) {
        for (rel, text) in outputs {
            expected.insert(rel.clone(), text_of(text).as_bytes().to_vec());
        }
    }
    assert_eq!(
        written, expected,
        "case {name} writes different files than the record"
    );
    if let Some(absent) = case.get("absent").and_then(|value| value.as_list()) {
        for rel in absent {
            assert!(
                !dir.join(text_of(rel)).exists(),
                "case {name} leaves {} standing where the record leaves nothing",
                text_of(rel)
            );
        }
    }
    std::fs::remove_dir_all(&dir).expect("a case directory clears");
}

/// One input file of one recorded battery case, standing as the
/// single source of the synthetic documents the named pins below
/// drive the binary over.
fn recorded_input(case_name: &str, input: &str) -> String {
    let cases = recordings();
    let case = cases
        .iter()
        .find(|case| case.get("name").and_then(|value| value.as_text()) == Some(case_name))
        .expect("the named battery case stands in the recordings");
    let inputs = case
        .get("inputs")
        .and_then(|value| value.as_object())
        .expect("the named case carries its inputs");
    inputs
        .iter()
        .find(|(name, _)| name == input)
        .map(|(_, text)| text_of(text).to_string())
        .expect("the named input stands in the case")
}

fn pin_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    if dir.exists() {
        std::fs::remove_dir_all(&dir).expect("a stale pin directory clears");
    }
    std::fs::create_dir_all(&dir).expect("a pin directory stands");
    dir
}

#[test]
fn test_cli_battery_matches_the_python_record() {
    let cases = recordings();
    assert_eq!(cases.len(), 54, "the battery carries 54 cases");
    // Use a fixed array to satisfy `clippy::useless_vec` under `-D warnings`.
    let sections: [(&str, usize); 4] = [
        ("behaviours", 9),
        ("arguments", 31),
        ("refusals", 12),
        ("write_failure", 2),
    ];
    for (section, count) in &sections {
        let standing = cases
            .iter()
            .filter(|case| case.get("section").and_then(|value| value.as_text()) == Some(section))
            .count();
        assert_eq!(
            standing, *count,
            "the battery section {section} carries its recorded count"
        );
    }
    for (index, case) in cases.iter().enumerate() {
        run_case(index, case);
    }
}

/// The argument bytes pin of the Round 4 fix pass. The shell takes
/// its arguments as the operating system hands them over, so a
/// catchledger document whose file name carries a byte that is
/// not UTF-8 assembles whole, and a ruling word carrying such a
/// byte is refused with the library wording for an unknown ruling,
/// the word rendered in the escape the Python writes on standard
/// error, and nothing written.
#[cfg(unix)]
#[test]
fn test_a_file_name_carrying_a_non_utf8_byte_is_read() {
    use std::ffi::OsString;
    use std::os::unix::ffi::{OsStrExt, OsStringExt};

    let dir = pin_dir("tameshi-cli-nonutf8");
    let _scratch = ScratchDir::hold(&dir);
    let ledger = recorded_input("deterministic pack outputs", "ledger.json");
    let mut bad_name = dir.as_os_str().as_bytes().to_vec();
    bad_name.extend_from_slice(b"/led\xffger.json");
    let bad_path = PathBuf::from(OsString::from_vec(bad_name));
    let packs_dir = dir.join("packs");
    match std::fs::write(&bad_path, ledger.as_bytes()) {
        Ok(()) => {
            let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
                .arg("assemble")
                .arg("--catchledger")
                .arg(bad_path.as_os_str())
                .arg("--packs-dir")
                .arg(&packs_dir)
                .output()
                .expect("the binary runs");
            assert_eq!(
                output.status.code(),
                Some(0),
                "a ledger named with a raw byte assembles"
            );
            assert!(packs_dir.join("001-style-checks.json").exists());
            assert!(packs_dir.join("002-unit-checks.json").exists());
        }
        // macOS APFS (`EILSEQ` 92) and Seatbelt (`EPERM` 1) refuse non-UTF-8 directory entries;
        // stage a `U+FFFD` sibling decoy to prove raw `0xff` bytes reach the OS unmodified.
        Err(err)
            if cfg!(target_os = "macos") && matches!(err.raw_os_error(), Some(92) | Some(1)) =>
        {
            std::fs::write(dir.join("led\u{FFFD}ger.json"), ledger.as_bytes())
                .expect("the replacement-character decoy stands");
            let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
                .arg("assemble")
                .arg("--catchledger")
                .arg(bad_path.as_os_str())
                .arg("--packs-dir")
                .arg(&packs_dir)
                .output()
                .expect("the binary runs");
            assert_eq!(
                output.status.code(),
                Some(1),
                "raw non-UTF-8 bytes reach the OS without replacement-character fallback"
            );
            assert_eq!(
                output.stderr,
                b"the catchledger evidence could not be read\n"
            );
            assert!(!packs_dir.exists());
        }
        Err(err) => panic!("the ledger stands under its byte name: {err}"),
    }

    let pack = recorded_input("deterministic ruling outputs", "pack.json");
    let pack_path = dir.join("pack.json");
    std::fs::write(&pack_path, pack.as_bytes()).expect("the pack stands");
    let out_dir = dir.join("out");
    let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
        .arg("rule")
        .arg("--pack")
        .arg(&pack_path)
        .arg("--ruling")
        .arg(OsString::from_vec(b"ke\xffep".to_vec()))
        .arg("--out")
        .arg(&out_dir)
        .output()
        .expect("the binary runs");
    assert_eq!(
        output.status.code(),
        Some(1),
        "a ruling word carrying a raw byte refuses"
    );
    assert_eq!(
        output.stderr,
        b"an unknown ruling was requested (ke\\udcffep)\n".to_vec(),
        "the refusal renders the word in the escape the Python writes"
    );
    assert!(!out_dir.exists(), "the refused ruling writes nothing");
    std::fs::remove_dir_all(&dir).expect("a pin directory clears");
}

/// The nesting band pin of the Round 4 fix pass. A value nested
/// inside the pricing object of a catchledger pack at 4500 lists
/// deep completes under the Python through the assemble door, and
/// the port refuses the same document whole at its stated reader
/// bound of 4000, in the reader wording, writing nothing.
#[test]
fn test_a_document_nested_past_the_reader_bound_is_refused_by_the_command_line() {
    let dir = pin_dir("tameshi-cli-nesting");
    let _scratch = ScratchDir::hold(&dir);
    let ledger = recorded_input("deterministic pack outputs", "ledger.json");
    let nest = format!("{}{}", "[".repeat(4500), "]".repeat(4500));
    let deep = ledger.replacen(
        "\"pricing\": {",
        &format!("\"pricing\": {{\n    \"deep\": {nest},"),
        1,
    );
    assert_ne!(
        deep, ledger,
        "the pricing object stands in the recorded ledger"
    );
    let deep_path = dir.join("deep.json");
    std::fs::write(&deep_path, deep.as_bytes()).expect("the deep document stands");
    let packs_dir = dir.join("packs");
    let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
        .arg("assemble")
        .arg("--catchledger")
        .arg(&deep_path)
        .arg("--packs-dir")
        .arg(&packs_dir)
        .output()
        .expect("the binary runs");
    assert_eq!(
        output.status.code(),
        Some(1),
        "a document nested past the reader bound refuses"
    );
    let stderr = String::from_utf8(output.stderr).expect("standard error is text");
    assert_eq!(stderr, "the catchledger evidence is not valid JSON\n");
    assert!(!packs_dir.exists(), "the refused document writes no packs");
    std::fs::remove_dir_all(&dir).expect("a pin directory clears");
}
