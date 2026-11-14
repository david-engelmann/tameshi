//! The tameshi command line over the Rust library.
//!
//! This shell is the closing slice of the port under obligations
//! PORT3-1 to PORT3-3. It carries the three doors of the Python
//! command line, assemble and rule and validate, with the argument
//! reading the contract probe recorded. A reading the argument
//! reader refuses exits with code 2 and the short usage line of the
//! door on standard error, and a refusal of the reader or the
//! library exits with code 1 and the refusal wording on standard
//! error, and a door that succeeds exits with code 0 in silence.
//! The arguments stand as the operating system hands them over,
//! their bytes whole, so a file name carrying a byte that is not
//! UTF-8 names the file it names, and a word carrying such a byte
//! is refused in the wording the library gives that word, with the
//! word rendered the way the Python renders it on standard error.

use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Write;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::process::ExitCode;

use tameshi::ingest;
use tameshi::jsonvalue::JsonValue;
use tameshi::packs;
use tameshi::reader;
use tameshi::render;
use tameshi::rulings;
use tameshi::TameshiError;

const TOP_USAGE: &str = "usage: tameshi [-h] {assemble,rule,validate} ...";

const TOP_HELP: &str = "usage: tameshi [-h] {assemble,rule,validate} ...

tameshi, the evidence pack tool for checks.

commands:
  assemble  assemble one evidence pack per check from local evidence files
  rule      generate a ruling record and pull request body from an evidence pack
  validate  validate an evidence pack against the pack rebuilt from its inputs
";

const ASSEMBLE_USAGE: &str = "usage: tameshi assemble --catchledger PATH [--canaryfield PATH] [--blindgate PATH] [--decision PATH] --packs-dir DIR";
const RULE_USAGE: &str =
    "usage: tameshi rule --pack PATH --ruling WORD [--expires DATE] [--today DATE] --out DIR";
const VALIDATE_USAGE: &str = "usage: tameshi validate --pack PATH --catchledger PATH [--canaryfield PATH] [--blindgate PATH] [--decision PATH]";

const ASSEMBLE_HELP: &str = "usage: tameshi assemble --catchledger PATH [--canaryfield PATH] [--blindgate PATH] [--decision PATH] --packs-dir DIR

assemble one evidence pack per check from local evidence files
";
const RULE_HELP: &str =
    "usage: tameshi rule --pack PATH --ruling WORD [--expires DATE] [--today DATE] --out DIR

generate a ruling record and pull request body from an evidence pack
";
const VALIDATE_HELP: &str =
    "usage: tameshi validate --pack PATH --catchledger PATH [--canaryfield PATH] [--blindgate PATH] [--decision PATH]

validate an evidence pack against the pack rebuilt from its inputs
";

struct FlagSpec {
    name: &'static str,
    required: bool,
}

const ASSEMBLE_FLAGS: &[FlagSpec] = &[
    FlagSpec {
        name: "catchledger",
        required: true,
    },
    FlagSpec {
        name: "canaryfield",
        required: false,
    },
    FlagSpec {
        name: "blindgate",
        required: false,
    },
    FlagSpec {
        name: "decision",
        required: false,
    },
    FlagSpec {
        name: "packs-dir",
        required: true,
    },
];

const RULE_FLAGS: &[FlagSpec] = &[
    FlagSpec {
        name: "pack",
        required: true,
    },
    FlagSpec {
        name: "ruling",
        required: true,
    },
    FlagSpec {
        name: "expires",
        required: false,
    },
    FlagSpec {
        name: "today",
        required: false,
    },
    FlagSpec {
        name: "out",
        required: true,
    },
];

const VALIDATE_FLAGS: &[FlagSpec] = &[
    FlagSpec {
        name: "pack",
        required: true,
    },
    FlagSpec {
        name: "catchledger",
        required: true,
    },
    FlagSpec {
        name: "canaryfield",
        required: false,
    },
    FlagSpec {
        name: "blindgate",
        required: false,
    },
    FlagSpec {
        name: "decision",
        required: false,
    },
];

/// The values of one door reading, each standing as the operating
/// system handed it over.
type Values = HashMap<&'static str, OsString>;

fn token_bytes(token: &OsString) -> &[u8] {
    token.as_os_str().as_bytes()
}

/// The negative number shapes the Python argument reader does not
/// treat as option strings, a minus then digits with an optional
/// fractional part.
fn is_negative_number(token: &[u8]) -> bool {
    if token.first() != Some(&b'-') {
        return false;
    }
    let body = &token[1..];
    if body.is_empty() {
        return false;
    }
    if body.iter().all(|byte| byte.is_ascii_digit()) {
        return true;
    }
    match body.iter().position(|byte| *byte == b'.') {
        Some(dot) => {
            let (whole, fraction) = (&body[..dot], &body[dot + 1..]);
            whole.iter().all(|byte| byte.is_ascii_digit())
                && !fraction.is_empty()
                && fraction.iter().all(|byte| byte.is_ascii_digit())
        }
        None => false,
    }
}

/// Whether a token standing after a flag reads as a further option
/// string to the argument reader, in which case the flag stands
/// without its value.
fn looks_like_option(token: &[u8]) -> bool {
    token.len() > 1 && token[0] == b'-' && !is_negative_number(token)
}

/// Whether a single dash token is the help flag standing first in
/// a cluster of short flags. The Python argument reader expands a
/// cluster letter by letter, so the help flag fires where it
/// stands first, ahead of any letter it does not know, and a
/// cluster that carries an equals sign after the help letter is
/// the help flag carrying an explicit value, which that reader
/// refuses rather than answers.
fn is_help_cluster(token: &[u8]) -> bool {
    token.len() > 1
        && token[0] == b'-'
        && token[1] != b'-'
        && !is_negative_number(token)
        && token[1] == b'h'
        && (token.len() == 2 || token[2] != b'=')
}

enum LongMatch {
    Flag(usize),
    Help,
    Ambiguous,
    Unknown,
}

/// Match a long flag name against the flags of a door and the help
/// flag, exactly first and then by a prefix that names one alone.
fn match_long(name: &[u8], specs: &[FlagSpec]) -> LongMatch {
    if name.is_empty() {
        return LongMatch::Unknown;
    }
    if name == b"help" {
        return LongMatch::Help;
    }
    for (index, spec) in specs.iter().enumerate() {
        if spec.name.as_bytes() == name {
            return LongMatch::Flag(index);
        }
    }
    let mut found: Option<LongMatch> = None;
    let mut ambiguous = false;
    if b"help".starts_with(name) {
        found = Some(LongMatch::Help);
    }
    for (index, spec) in specs.iter().enumerate() {
        if spec.name.as_bytes().starts_with(name) {
            if found.is_some() {
                ambiguous = true;
            }
            found = Some(LongMatch::Flag(index));
        }
    }
    if ambiguous {
        LongMatch::Ambiguous
    } else {
        found.unwrap_or(LongMatch::Unknown)
    }
}

enum Scan {
    Help,
    UsageError,
    Parsed(Values),
}

/// Read the arguments of one door the way the Python argument reader
/// reads them. An ambiguous prefix and a flag standing without its
/// value refuse the reading at the point they stand. A word or flag
/// the door does not take is set aside and refuses the reading once
/// the scan ends, so a help flag standing after one still answers.
/// The separator word ends the option reading and every word after
/// it is set aside the same way. A flag given more than once keeps
/// its last value. A single dash token reads as a cluster of short
/// flags, the help flag firing where it stands first in one.
fn scan(specs: &[FlagSpec], args: &[OsString]) -> Scan {
    let mut values: Values = HashMap::new();
    let mut refused_at_end = false;
    let mut after_separator = false;
    let mut position = 0;
    while position < args.len() {
        let token = &args[position];
        let tok = token_bytes(token);
        if after_separator {
            refused_at_end = true;
            position += 1;
            continue;
        }
        if tok == b"--" {
            after_separator = true;
            refused_at_end = true;
            position += 1;
            continue;
        }
        if tok == b"-h" || is_help_cluster(tok) {
            return Scan::Help;
        }
        if let Some(long) = tok.strip_prefix(b"--") {
            let (name, inline) = match long.iter().position(|byte| *byte == b'=') {
                Some(equal) => (
                    &long[..equal],
                    Some(OsString::from_vec(long[equal + 1..].to_vec())),
                ),
                None => (long, None),
            };
            match match_long(name, specs) {
                LongMatch::Help => {
                    if inline.is_some() {
                        return Scan::UsageError;
                    }
                    return Scan::Help;
                }
                LongMatch::Ambiguous => return Scan::UsageError,
                LongMatch::Unknown => {
                    refused_at_end = true;
                    position += 1;
                }
                LongMatch::Flag(index) => {
                    let spec = &specs[index];
                    match inline {
                        Some(value) => {
                            values.insert(spec.name, value);
                            position += 1;
                        }
                        None => match args.get(position + 1) {
                            Some(value) if !looks_like_option(token_bytes(value)) => {
                                values.insert(spec.name, value.clone());
                                position += 2;
                            }
                            _ => return Scan::UsageError,
                        },
                    }
                }
            }
            continue;
        }
        refused_at_end = true;
        position += 1;
    }
    if refused_at_end {
        return Scan::UsageError;
    }
    for spec in specs {
        if spec.required && !values.contains_key(spec.name) {
            return Scan::UsageError;
        }
    }
    Scan::Parsed(values)
}

/// Whether a token standing before the subcommand word is the help
/// flag, in full or in a prefix that names it alone or standing
/// first in a cluster of short flags.
fn is_top_help(token: &[u8]) -> Option<bool> {
    if token == b"-h" || is_help_cluster(token) {
        return Some(true);
    }
    let long = token.strip_prefix(b"--")?;
    let (name, inline) = match long.iter().position(|byte| *byte == b'=') {
        Some(equal) => (&long[..equal], true),
        None => (long, false),
    };
    if name == b"help" || (!name.is_empty() && b"help".starts_with(name)) {
        return Some(!inline);
    }
    None
}

fn need<'a>(values: &'a Values, name: &str) -> &'a OsString {
    values
        .get(name)
        .expect("a required flag stands once the reading accepts")
}

fn path_of(values: &Values, name: &str) -> PathBuf {
    PathBuf::from(need(values, name))
}

/// The text of a word the library reads as a calendar date. Such a
/// word is never echoed and never names a file, and a word carrying
/// a byte that is not UTF-8 reads here with the replacement
/// character standing in it, which no date the library accepts can
/// contain, so the library refuses the word with its own wording
/// at the point the Python refuses the same bytes, while a word
/// that stands whole as UTF-8 passes through unchanged.
fn word_text(value: &OsString) -> String {
    value.to_string_lossy().into_owned()
}

/// Decode the first character a byte string carries as UTF-8, or
/// refuse the shape, the strict reading the Python gives a word it
/// takes from its argument vector.
fn decode_scalar(bytes: &[u8]) -> Option<(char, usize)> {
    let first = *bytes.first()?;
    if first < 0x80 {
        return Some((first as char, 1));
    }
    let (width, minimum, mut value) = match first {
        0xc2..=0xdf => (2, 0x80, u32::from(first & 0x1f)),
        0xe0..=0xef => (3, 0x800, u32::from(first & 0x0f)),
        0xf0..=0xf4 => (4, 0x10000, u32::from(first & 0x07)),
        _ => return None,
    };
    if bytes.len() < width {
        return None;
    }
    for continuation in &bytes[1..width] {
        if !(0x80..=0xbf).contains(continuation) {
            return None;
        }
        value = (value << 6) | u32::from(continuation & 0x3f);
    }
    if value < minimum || value > 0x10ffff || (0xd800..=0xdfff).contains(&value) {
        return None;
    }
    char::from_u32(value).map(|ch| (ch, width))
}

/// Render a word the way the Python writes it on standard error.
/// The Python takes its argument words with the bytes that name no
/// character standing as escapes in their place and writes its
/// error stream with each such escape spelled out as the escape
/// text itself, so a byte renders here as the same spelled escape
/// and every whole character renders as itself.
fn escape_word(bytes: &[u8]) -> String {
    let mut rendered = String::new();
    let mut at = 0;
    while at < bytes.len() {
        match decode_scalar(&bytes[at..]) {
            Some((ch, width)) => {
                rendered.push(ch);
                at += width;
            }
            None => {
                rendered.push_str(&format!("\\u{:04x}", 0xdc00 + u32::from(bytes[at])));
                at += 1;
            }
        }
    }
    rendered
}

/// The refusal the library gives the kill word, standing in the
/// rulings module. A ruling word whose bytes are not UTF-8 reaches
/// this refusal only past a pack that validates, which the probe
/// below establishes.
const KILL_REFUSAL: &str = "a kill ruling requires a decision record recommending kill in the pack";

/// Refuse a ruling word whose bytes are not UTF-8. The library
/// reaches its ruling word only once it has validated the pack,
/// so that validation runs first here through a call on the kill
/// word, whose own refusal is the first refusal past a sound pack
/// and whose standing therefore proves the pack sound, and past a
/// sound pack the refusal is the wording the library gives an
/// unknown ruling, with the word rendered as the Python renders
/// it.
fn refuse_unreadable_ruling(pack: &JsonValue, ruling: &OsString) -> TameshiError {
    match rulings::generate_ruling(pack, "kill", None, None) {
        Err(error) if error.message != KILL_REFUSAL => error,
        _ => TameshiError::refusal(format!(
            "an unknown ruling was requested ({})",
            escape_word(token_bytes(ruling))
        )),
    }
}

/// Write one output file the way the Python write helper does,
/// creating the directory path where it does not stand, with the
/// one refusal wording every write failure carries.
fn write_text(path: &std::path::Path, text: &str) -> Result<(), TameshiError> {
    let outcome = (|| -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, text)
    })();
    match outcome {
        Ok(()) => Ok(()),
        Err(_) => Err(TameshiError::refusal("an output could not be written")),
    }
}

/// Read the two optional evidence documents both reading doors
/// take, the canaryfield results and the blindness evidence, under
/// the labels the command line gives them.
/// Read the decision document a run carries where the
/// decision flag stands, validating it under DC-1 before
/// any pack is written or compared.
fn read_decision_document(values: &Values) -> Result<Option<(String, JsonValue)>, TameshiError> {
    match values.get("decision") {
        Some(path) => {
            let document = reader::read_json_file(&PathBuf::from(path), "decision document")?;
            Ok(Some(packs::read_decision(&document)?))
        }
        None => Ok(None),
    }
}

fn read_optional_documents(
    values: &Values,
) -> Result<(Option<JsonValue>, Option<JsonValue>), TameshiError> {
    let canary = match values.get("canaryfield") {
        Some(path) => Some(reader::read_json_file(
            &PathBuf::from(path),
            "canaryfield results",
        )?),
        None => None,
    };
    let blindness = match values.get("blindgate") {
        Some(path) => Some(reader::read_json_file(
            &PathBuf::from(path),
            "blindness evidence",
        )?),
        None => None,
    };
    Ok((canary, blindness))
}

fn door_assemble(values: &Values) -> Result<(), TameshiError> {
    let document = reader::read_json_file(&path_of(values, "catchledger"), "catchledger evidence")?;
    let cl_packs = ingest::read_catchledger_packs(&document)?;
    let (canary, blindness) = read_optional_documents(values)?;
    let decision = read_decision_document(values)?;
    let packs = packs::assemble_packs(
        &cl_packs,
        canary.as_ref(),
        blindness.as_ref(),
        decision.as_ref(),
    )?;
    let packs_dir = path_of(values, "packs-dir");
    for (filename, pack) in &packs {
        write_text(&packs_dir.join(filename), &render::render_json(pack))?;
    }
    Ok(())
}

fn door_rule(values: &Values) -> Result<(), TameshiError> {
    let pack = reader::read_json_file(&path_of(values, "pack"), "evidence pack")?;
    let expiry = values
        .get("expires")
        .map(|text| JsonValue::Text(word_text(text)));
    let today = values
        .get("today")
        .map(|text| JsonValue::Text(word_text(text)));
    let ruling = need(values, "ruling");
    let (record, body) = match ruling.to_str() {
        Some(word) => rulings::generate_ruling(&pack, word, expiry.as_ref(), today.as_ref())?,
        None => return Err(refuse_unreadable_ruling(&pack, ruling)),
    };
    let check = pack
        .get("check")
        .and_then(|item| item.as_text())
        .unwrap_or("");
    let slug = rulings::ruling_slug(check);
    let out = path_of(values, "out");
    write_text(
        &out.join(format!("ruling-{slug}.json")),
        &render::render_json(&record),
    )?;
    write_text(&out.join(format!("ruling-{slug}.md")), &body)?;
    Ok(())
}

fn door_validate(values: &Values) -> Result<(), TameshiError> {
    let pack = reader::read_json_file(&path_of(values, "pack"), "evidence pack")?;
    let document = reader::read_json_file(&path_of(values, "catchledger"), "catchledger evidence")?;
    let cl_packs = ingest::read_catchledger_packs(&document)?;
    let (canary, blindness) = read_optional_documents(values)?;
    let decision = read_decision_document(values)?;
    packs::validate_pack(
        &pack,
        &cl_packs,
        canary.as_ref(),
        blindness.as_ref(),
        decision.as_ref(),
    )?;
    Ok(())
}

fn print_stdout(text: &str) {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let _ = handle.write_all(text.as_bytes());
    let _ = handle.flush();
}

fn print_stderr(bytes: &[u8]) {
    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    let _ = handle.write_all(bytes);
    let _ = handle.flush();
}

fn usage_refusal(usage: &str) -> u8 {
    print_stderr(usage.as_bytes());
    print_stderr(b"\n");
    2
}

fn run_door(
    specs: &[FlagSpec],
    args: &[OsString],
    usage: &str,
    help: &str,
    poisoned: bool,
    door: fn(&Values) -> Result<(), TameshiError>,
) -> u8 {
    match scan(specs, args) {
        Scan::Help => {
            print_stdout(help);
            0
        }
        Scan::UsageError => usage_refusal(usage),
        Scan::Parsed(values) => {
            if poisoned {
                return usage_refusal(TOP_USAGE);
            }
            match door(&values) {
                Ok(()) => 0,
                Err(error) => {
                    print_stderr(error.message.as_bytes());
                    print_stderr(b"\n");
                    1
                }
            }
        }
    }
}

fn run(args: &[OsString]) -> u8 {
    let mut position = 0;
    let mut poisoned = false;
    loop {
        let token = match args.get(position) {
            Some(token) => token,
            None => return usage_refusal(TOP_USAGE),
        };
        let tok = token_bytes(token);
        if let Some(answers) = is_top_help(tok) {
            if answers {
                print_stdout(TOP_HELP);
                return 0;
            }
            return usage_refusal(TOP_USAGE);
        }
        if tok == b"assemble" {
            return run_door(
                ASSEMBLE_FLAGS,
                &args[position + 1..],
                ASSEMBLE_USAGE,
                ASSEMBLE_HELP,
                poisoned,
                door_assemble,
            );
        }
        if tok == b"rule" {
            return run_door(
                RULE_FLAGS,
                &args[position + 1..],
                RULE_USAGE,
                RULE_HELP,
                poisoned,
                door_rule,
            );
        }
        if tok == b"validate" {
            return run_door(
                VALIDATE_FLAGS,
                &args[position + 1..],
                VALIDATE_USAGE,
                VALIDATE_HELP,
                poisoned,
                door_validate,
            );
        }
        if tok == b"--" || tok.first() != Some(&b'-') || tok == b"-" || is_negative_number(tok) {
            // A word standing where the subcommand stands that names
            // none of the three refuses the reading where it stands.
            return usage_refusal(TOP_USAGE);
        }
        // A flag the top reader does not carry is set aside and
        // refuses the reading once a subcommand reading accepts, so
        // a help flag standing after one still answers.
        poisoned = true;
        position += 1;
    }
}

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    ExitCode::from(run(&args))
}
