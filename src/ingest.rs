//! The validation and normalization readings the pack assembly and
//! the ruling generation stand on, each the counterpart of the
//! reading of the same name in the ingest module.
//!
//! The reading of catchledger documents whole, the pack and ledger
//! validation and the ledger derivation under the read entry of that
//! module stands after the substrate under the obligations of PORT2.
//! What stands first here is the substrate the ported modules call. A
//! reading refuses exactly where its counterpart refuses and with the
//! same wording.

use crate::dates;
use crate::errors::{Result, TameshiError};
use crate::jsonvalue::JsonValue;

pub const CANARY_SOURCE_FORMAT: &str = "canaryfield-catch-table";
pub const BLINDNESS_FORMAT: &str = "blindgate-evidence/1";
pub const HISTORY_WINDOW_FIELD: &str = "history_window";

pub const RESULT_WORDS: [&str; 4] = ["caught", "waved-through", "not-applicable", "gate-broken"];

/// The result words a canaryfield document may carry, the four the
/// counts stand over and the timed-out word canaryfield's timeout
/// slice added to its result documents, which is counted under
/// none of the four and counts as scored.
pub const KNOWN_RESULT_WORDS: [&str; 5] = [
    "caught",
    "waved-through",
    "not-applicable",
    "gate-broken",
    "timed-out",
];
pub const OUTCOME_WORDS: [&str; 3] = ["catch", "false_block", "unattributed"];

const SCORECARD_FIELDS: [&str; 13] = [
    "fires",
    "blocks",
    "judged_commits",
    "flaky_commits",
    "flake_rate",
    "runtime_seconds",
    "runtime_minutes",
    "catches",
    "false_blocks",
    "unattributed_blocks",
    "false_block_rate",
    "total_cost_usd",
    "cost_per_catch_usd",
];
const CATCH_RECORD_FIELDS: [&str; 6] = [
    "block_run_id",
    "blocked_commit",
    "started_at",
    "outcome",
    "flagged_paths",
    "evidence",
];
const CATCH_EVIDENCE_FIELDS: [&str; 4] = ["type", "fix_commits", "validating_run_id", "reason"];
const CANARY_RESULT_FIELDS: [&str; 4] = ["source_format", "gate", "counts", "results"];
const CANARY_ROW_FIELDS: [&str; 5] = ["seed", "name", "defect_class", "result", "planted_files"];
const BLINDNESS_TRACE_FIELDS: [&str; 4] = ["check", "surface", "observation", "detail"];
const BLINDNESS_EVIDENCE_FIELDS: [&str; 2] = ["format", "traces"];
const WINDOW_FIELDS: [&str; 2] = ["first_run_started_at", "last_run_updated_at"];

/// Whether a character is whitespace to the Python strip reading,
/// whose set stands a little wider than the Unicode white space set.
fn py_is_space(ch: char) -> bool {
    ch.is_whitespace() || matches!(ch, '\u{001C}'..='\u{001F}' | '\u{0085}')
}

/// A text with the Python stripped edges removed.
pub fn py_trim(text: &str) -> &str {
    text.trim_matches(py_is_space)
}

/// The object standing in a value, refused in the wording the field
/// readings give a value that is not an object.
fn object_fields<'a>(value: &'a JsonValue, label: &str) -> Result<&'a Vec<(String, JsonValue)>> {
    match value.as_object() {
        Some(object) => Ok(object),
        None => Err(TameshiError::refusal(format!(
            "the {label} must be an object"
        ))),
    }
}

/// The refusal the field readings give a wrong field set, the
/// missing names and the unexpected names each in sorted order.
fn wrong_fields(label: &str, mut missing: Vec<&str>, mut unexpected: Vec<&str>) -> TameshiError {
    missing.sort();
    unexpected.sort();
    let mut details: Vec<String> = Vec::new();
    if !missing.is_empty() {
        details.push(format!("missing {}", missing.join(", ")));
    }
    if !unexpected.is_empty() {
        let printed: Vec<&str> = unexpected
            .iter()
            .map(|name| if name.is_empty() { "empty name" } else { name })
            .collect();
        details.push(format!("unexpected {}", printed.join(", ")));
    }
    TameshiError::refusal(format!(
        "the {label} has the wrong fields ({})",
        details.join("; ")
    ))
}

pub fn require_exact_fields(value: &JsonValue, fields: &[&str], label: &str) -> Result<()> {
    let object = object_fields(value, label)?;
    let standing: Vec<&str> = object.iter().map(|(name, _)| name.as_str()).collect();
    let missing: Vec<&str> = fields
        .iter()
        .copied()
        .filter(|field| !standing.contains(field))
        .collect();
    let unexpected: Vec<&str> = standing
        .iter()
        .copied()
        .filter(|name| !fields.contains(name))
        .collect();
    if !missing.is_empty() || !unexpected.is_empty() {
        return Err(wrong_fields(label, missing, unexpected));
    }
    Ok(())
}

pub fn require_text<'a>(value: &'a JsonValue, label: &str) -> Result<&'a str> {
    match value.as_text() {
        Some(text) if !py_trim(text).is_empty() => Ok(text),
        _ => Err(TameshiError::refusal(format!(
            "the {label} must be nonempty text"
        ))),
    }
}

pub fn require_name<'a>(value: &'a JsonValue, label: &str) -> Result<&'a str> {
    let text = require_text(value, label)?;
    if text.contains('\n') || text.contains('\r') {
        return Err(TameshiError::refusal(format!(
            "the {label} must not carry a newline"
        )));
    }
    Ok(text)
}

/// Parse an ISO 8601 timestamp that carries a timezone offset, and
/// hand back the instant in microseconds on the UTC clock. A
/// trailing capital Z is first rewritten as the zero offset, as the
/// Python reading rewrites it before its own reading.
pub fn parse_timestamp(value: &JsonValue, label: &str) -> Result<i128> {
    let trimmed = match value.as_text() {
        Some(text) if !py_trim(text).is_empty() => py_trim(text),
        _ => {
            return Err(TameshiError::refusal(format!(
                "the {label} must be an ISO 8601 timestamp"
            )))
        }
    };
    let replaced;
    let text = if let Some(before) = trimmed.strip_suffix('Z') {
        replaced = format!("{before}+00:00");
        &replaced
    } else {
        trimmed
    };
    match dates::parse_timestamp_instant(text) {
        Some((_, false)) => Err(TameshiError::refusal(format!(
            "the {label} must carry a timezone offset"
        ))),
        Some((instant, true)) => Ok(instant),
        None => Err(TameshiError::refusal(format!(
            "the {label} must be an ISO 8601 timestamp"
        ))),
    }
}

pub fn validate_scorecard(scorecard: &JsonValue) -> Result<()> {
    require_exact_fields(scorecard, &SCORECARD_FIELDS, "catchledger scorecard")
}

pub fn validate_catch_record(catch_record: &JsonValue) -> Result<()> {
    let entries = match catch_record.as_list() {
        Some(entries) => entries,
        None => return Err(TameshiError::refusal("the catch record must be a list")),
    };
    for entry in entries {
        require_exact_fields(entry, &CATCH_RECORD_FIELDS, "catch record entry")?;
        require_text(
            entry.get("block_run_id").unwrap_or(&JsonValue::Null),
            "catch record block run identifier",
        )?;
        require_text(
            entry.get("blocked_commit").unwrap_or(&JsonValue::Null),
            "catch record blocked commit",
        )?;
        parse_timestamp(
            entry.get("started_at").unwrap_or(&JsonValue::Null),
            "catch record started at timestamp",
        )?;
        let outcome = entry.get("outcome").and_then(|value| value.as_text());
        if !matches!(outcome, Some(word) if OUTCOME_WORDS.contains(&word)) {
            return Err(TameshiError::refusal(
                "a catch record outcome is not a known outcome word",
            ));
        }
        let flagged = entry.get("flagged_paths").and_then(|value| value.as_list());
        match flagged {
            Some(paths) if paths.iter().all(|path| path.as_text().is_some()) => {}
            _ => {
                return Err(TameshiError::refusal(
                    "the catch record flagged paths must be a list of text",
                ))
            }
        }
        require_exact_fields(
            entry.get("evidence").unwrap_or(&JsonValue::Null),
            &CATCH_EVIDENCE_FIELDS,
            "catch evidence",
        )?;
    }
    Ok(())
}

/// Tally normalized canaryfield rows over the four result words.
pub fn tally_canary_results(results: &[JsonValue]) -> JsonValue {
    let mut counts: Vec<(String, JsonValue)> = RESULT_WORDS
        .iter()
        .map(|word| (word.to_string(), JsonValue::Int(0)))
        .collect();
    for row in results {
        if let Some(word) = row.get("result").and_then(|value| value.as_text()) {
            if let Some((_, count)) = counts.iter_mut().find(|(name, _)| name == word) {
                if let JsonValue::Int(held) = count {
                    *held += 1;
                }
            }
        }
    }
    JsonValue::Object(counts)
}

fn counts_cover_result_words(counts: &JsonValue) -> bool {
    match counts.as_object() {
        Some(fields) => {
            fields.len() == RESULT_WORDS.len()
                && RESULT_WORDS
                    .iter()
                    .all(|word| fields.iter().any(|(name, _)| name == word))
        }
        None => false,
    }
}

fn counts_are_nonnegative_wholes(counts: &JsonValue) -> bool {
    match counts.as_object() {
        Some(fields) => fields.iter().all(|(_, value)| value.is_nonnegative_whole()),
        None => false,
    }
}

fn is_list_of_text(value: &JsonValue) -> bool {
    match value.as_list() {
        Some(items) => items.iter().all(|item| item.as_text().is_some()),
        None => false,
    }
}

pub fn validate_normalized_canary(value: &JsonValue) -> Result<()> {
    if matches!(value, JsonValue::Null) {
        return Ok(());
    }
    require_exact_fields(value, &CANARY_RESULT_FIELDS, "canaryfield results")?;
    if value.get("source_format").and_then(|item| item.as_text()) != Some(CANARY_SOURCE_FORMAT) {
        return Err(TameshiError::refusal(
            "the canaryfield results carry an unknown source format",
        ));
    }
    require_name(
        value.get("gate").unwrap_or(&JsonValue::Null),
        "canaryfield gate name",
    )?;
    let counts = value.get("counts").unwrap_or(&JsonValue::Null);
    if !counts_cover_result_words(counts) {
        return Err(TameshiError::refusal(
            "the canaryfield counts must cover the four result words",
        ));
    }
    if !counts_are_nonnegative_wholes(counts) {
        return Err(TameshiError::refusal(
            "the canaryfield counts must be nonnegative whole numbers",
        ));
    }
    let results = match value.get("results").and_then(|item| item.as_list()) {
        Some(results) => results,
        None => {
            return Err(TameshiError::refusal(
                "the canaryfield results rows must be a list",
            ))
        }
    };
    for row in results {
        require_exact_fields(row, &CANARY_ROW_FIELDS, "canaryfield result row")?;
        for field in ["seed", "name", "defect_class"] {
            require_text(
                row.get(field).unwrap_or(&JsonValue::Null),
                &format!("canaryfield result row {field}"),
            )?;
        }
        let result = row.get("result").and_then(|item| item.as_text());
        if !matches!(result, Some(word) if KNOWN_RESULT_WORDS.contains(&word)) {
            return Err(TameshiError::refusal(
                "a canaryfield result word is not a known result word",
            ));
        }
        if !is_list_of_text(row.get("planted_files").unwrap_or(&JsonValue::Null)) {
            return Err(TameshiError::refusal(
                "a canaryfield result row carries malformed planted files",
            ));
        }
    }
    if counts != &tally_canary_results(results) {
        return Err(TameshiError::refusal(
            "the canaryfield counts disagree with their result rows",
        ));
    }
    Ok(())
}

/// Validate a history window record, refusing where its counterpart
/// refuses. A null window stands as no window.
pub fn validate_history_window(value: &JsonValue) -> Result<()> {
    if matches!(value, JsonValue::Null) {
        return Ok(());
    }
    let fields_ok = match value.as_object() {
        Some(fields) => {
            fields.len() == WINDOW_FIELDS.len()
                && WINDOW_FIELDS
                    .iter()
                    .all(|field| fields.iter().any(|(name, _)| name == field))
        }
        None => false,
    };
    if !fields_ok {
        return Err(TameshiError::refusal(
            "the history window must carry a first run start and a last run update",
        ));
    }
    let first = value
        .get("first_run_started_at")
        .unwrap_or(&JsonValue::Null);
    let last = value.get("last_run_updated_at").unwrap_or(&JsonValue::Null);
    if matches!(first, JsonValue::Null) != matches!(last, JsonValue::Null) {
        return Err(TameshiError::refusal(
            "the history window timestamps must stand or fall together",
        ));
    }
    if !matches!(first, JsonValue::Null) {
        let first_at = parse_timestamp(first, "history window first run start")?;
        let last_at = parse_timestamp(last, "history window last run update")?;
        if last_at < first_at {
            return Err(TameshiError::refusal(
                "the history window last run update precedes its first run start",
            ));
        }
    }
    Ok(())
}

pub fn validate_canary_document(document: &JsonValue) -> Result<()> {
    let fields_ok = match document.as_object() {
        Some(fields) => {
            fields.iter().any(|(name, _)| name == "gates")
                && fields.iter().any(|(name, _)| name == "rows")
        }
        None => false,
    };
    if !fields_ok {
        return Err(TameshiError::refusal(
            "the canaryfield document must carry a gates list and a rows list",
        ));
    }
    let gates_value = document.get("gates").unwrap_or(&JsonValue::Null);
    let gates: Vec<&str> = match gates_value.as_list() {
        Some(items) => {
            let mut names = Vec::new();
            let mut well_formed = !items.is_empty();
            for item in items {
                match item.as_text() {
                    Some(name) if !py_trim(name).is_empty() => names.push(name),
                    _ => {
                        well_formed = false;
                        break;
                    }
                }
            }
            let mut distinct = names.clone();
            distinct.sort();
            distinct.dedup();
            if !well_formed || distinct.len() != names.len() {
                return Err(TameshiError::refusal(
                    "the canaryfield gates must be a list of distinct nonempty names",
                ));
            }
            names
        }
        None => {
            return Err(TameshiError::refusal(
                "the canaryfield gates must be a list of distinct nonempty names",
            ))
        }
    };
    for gate in &gates {
        require_name(&JsonValue::Text(gate.to_string()), "canaryfield gate name")?;
    }
    let rows = match document.get("rows").and_then(|item| item.as_list()) {
        Some(rows) => rows,
        None => return Err(TameshiError::refusal("the canaryfield rows must be a list")),
    };
    let mut seen_seeds: Vec<&str> = Vec::new();
    for row in rows {
        if !row.is_object() {
            return Err(TameshiError::refusal("a canaryfield row must be an object"));
        }
        for field in ["seed", "name", "defect_class"] {
            let present = row
                .get(field)
                .and_then(|item| item.as_text())
                .map(|text| !py_trim(text).is_empty())
                .unwrap_or(false);
            if !present {
                return Err(TameshiError::refusal(format!(
                    "a canaryfield row needs a nonempty {field}"
                )));
            }
        }
        let seed = row
            .get("seed")
            .and_then(|item| item.as_text())
            .unwrap_or("");
        if seen_seeds.contains(&seed) {
            return Err(TameshiError::refusal(
                "the canaryfield document repeats a seed",
            ));
        }
        seen_seeds.push(seed);
        let row_gates_ok = match row.get("gates") {
            Some(gate_map) => match gate_map.as_object() {
                Some(fields) => {
                    fields.len() == gates.len()
                        && gates
                            .iter()
                            .all(|gate| fields.iter().any(|(name, _)| name == gate))
                }
                None => false,
            },
            None => false,
        };
        if !row_gates_ok {
            return Err(TameshiError::refusal(
                "a canaryfield row must carry a result for each gate",
            ));
        }
        if let Some(gate_map) = row.get("gates").and_then(|item| item.as_object()) {
            for (_, result) in gate_map {
                let word = result.as_text();
                if !matches!(word, Some(word) if KNOWN_RESULT_WORDS.contains(&word)) {
                    return Err(TameshiError::refusal(
                        "a canaryfield result word is not a known result word",
                    ));
                }
            }
        }
        if let Some(planted_files) = row.get("planted_files") {
            if !matches!(planted_files, JsonValue::Null) && !is_list_of_text(planted_files) {
                return Err(TameshiError::refusal(
                    "a canaryfield row carries malformed planted files",
                ));
            }
        }
        if let Some(planted_file) = row.get("planted_file") {
            if !matches!(planted_file, JsonValue::Null) && planted_file.as_text().is_none() {
                return Err(TameshiError::refusal(
                    "a canaryfield row carries a malformed planted file",
                ));
            }
        }
    }
    Ok(())
}

/// Return normalized canaryfield evidence for one exact check name,
/// or nothing when the document holds no gate of that name.
pub fn normalize_canary_document(document: &JsonValue, check: &str) -> Result<Option<JsonValue>> {
    validate_canary_document(document)?;
    let gates = document
        .get("gates")
        .and_then(|item| item.as_list())
        .map(|items| items.to_vec())
        .unwrap_or_default();
    let gate_names: Vec<&str> = gates.iter().filter_map(|item| item.as_text()).collect();
    if !gate_names.contains(&check) {
        return Ok(None);
    }
    let mut counts: Vec<(String, JsonValue)> = RESULT_WORDS
        .iter()
        .map(|word| (word.to_string(), JsonValue::Int(0)))
        .collect();
    let mut results: Vec<JsonValue> = Vec::new();
    let rows = document
        .get("rows")
        .and_then(|item| item.as_list())
        .map(|items| items.to_vec())
        .unwrap_or_default();
    for row in &rows {
        let result = row
            .get("gates")
            .and_then(|item| item.get(check))
            .cloned()
            .unwrap_or(JsonValue::Null);
        if let Some(word) = result.as_text() {
            if let Some((_, count)) = counts.iter_mut().find(|(name, _)| name == word) {
                if let JsonValue::Int(held) = count {
                    *held += 1;
                }
            }
        }
        let planted_files = match row.get("planted_files") {
            Some(files @ JsonValue::List(_)) => files.clone(),
            _ => match row.get("planted_file") {
                Some(file @ JsonValue::Text(_)) => JsonValue::List(vec![file.clone()]),
                None | Some(JsonValue::Null) => match row.get("planted_files") {
                    None | Some(JsonValue::Null) => JsonValue::List(Vec::new()),
                    _ => {
                        return Err(TameshiError::refusal(
                            "a canaryfield row carries malformed planted files",
                        ))
                    }
                },
                _ => {
                    return Err(TameshiError::refusal(
                        "a canaryfield row carries malformed planted files",
                    ))
                }
            },
        };
        results.push(JsonValue::Object(vec![
            (
                "seed".to_string(),
                row.get("seed").cloned().unwrap_or(JsonValue::Null),
            ),
            (
                "name".to_string(),
                row.get("name").cloned().unwrap_or(JsonValue::Null),
            ),
            (
                "defect_class".to_string(),
                row.get("defect_class").cloned().unwrap_or(JsonValue::Null),
            ),
            ("result".to_string(), result),
            ("planted_files".to_string(), planted_files),
        ]));
    }
    results.sort_by(|a, b| {
        let left = a.get("seed").and_then(|item| item.as_text()).unwrap_or("");
        let right = b.get("seed").and_then(|item| item.as_text()).unwrap_or("");
        left.cmp(right)
    });
    Ok(Some(JsonValue::Object(vec![
        (
            "source_format".to_string(),
            JsonValue::Text(CANARY_SOURCE_FORMAT.to_string()),
        ),
        ("gate".to_string(), JsonValue::Text(check.to_string())),
        ("counts".to_string(), JsonValue::Object(counts)),
        ("results".to_string(), JsonValue::List(results)),
    ])))
}

/// Read one blindness trace entry under the discipline the
/// blindness document reading demands, the exact four fields
/// and the check standing as a name and the other three
/// fields standing as nonempty text, and hand back the check
/// name the entry carries. The ruling doors read a pack's
/// traces through this same reading, so the discipline
/// stands in one place.
pub fn validate_blindness_trace(trace: &JsonValue) -> Result<String> {
    require_exact_fields(trace, &BLINDNESS_TRACE_FIELDS, "blindness trace")?;
    let check = require_name(
        trace.get("check").unwrap_or(&JsonValue::Null),
        "blindness trace check",
    )?
    .to_string();
    for field in ["surface", "observation", "detail"] {
        require_text(
            trace.get(field).unwrap_or(&JsonValue::Null),
            &format!("blindness trace {field}"),
        )?;
    }
    Ok(check)
}

/// Return the blindness traces a document carries, grouped by check
/// name in the order the checks first stand.
pub fn read_blindness_traces(document: &JsonValue) -> Result<Vec<(String, Vec<JsonValue>)>> {
    require_exact_fields(document, &BLINDNESS_EVIDENCE_FIELDS, "blindness evidence")?;
    if document.get("format").and_then(|item| item.as_text()) != Some(BLINDNESS_FORMAT) {
        return Err(TameshiError::refusal(
            "the blindness evidence carries the wrong format name",
        ));
    }
    let traces = match document.get("traces").and_then(|item| item.as_list()) {
        Some(traces) => traces,
        None => return Err(TameshiError::refusal("the blindness traces must be a list")),
    };
    let mut grouped: Vec<(String, Vec<JsonValue>)> = Vec::new();
    for trace in traces {
        let check = validate_blindness_trace(trace)?;
        match grouped.iter_mut().find(|(name, _)| name == &check) {
            Some((_, held)) => held.push(trace.clone()),
            None => grouped.push((check, vec![trace.clone()])),
        }
    }
    Ok(grouped)
}

pub const CATCHLEDGER_PACK_FORMAT: &str = "catchledger-pack/1";
pub const CATCHLEDGER_LEDGER_FORMAT: &str = "catchledger-ledger/1";

const CATCHLEDGER_PACK_FIELDS: [&str; 9] = [
    "format",
    "repo",
    "check",
    "scorecard",
    "catch_record",
    "canaryfield_results",
    "false_block_rate",
    "pricing",
    "source",
];
const CATCHLEDGER_LEDGER_FIELDS: [&str; 6] =
    ["format", "repo", "pricing", "source", "checks", "totals"];
const CATCHLEDGER_LEDGER_ENTRY_FIELDS: [&str; 5] = [
    "name",
    "scorecard",
    "catch_record",
    "canaryfield_results",
    "evidence_pack_file",
];

/// The field reading of the catchledger documents, exact in the
/// required set with the history window standing optional beside it,
/// as the field reading of the ingest module reads a pack or a
/// ledger.
fn require_fields_with_optional_window(
    value: &JsonValue,
    fields: &[&str],
    label: &str,
) -> Result<()> {
    let object = object_fields(value, label)?;
    let standing: Vec<&str> = object.iter().map(|(name, _)| name.as_str()).collect();
    let missing: Vec<&str> = fields
        .iter()
        .copied()
        .filter(|field| !standing.contains(field))
        .collect();
    let unexpected: Vec<&str> = standing
        .iter()
        .copied()
        .filter(|name| !fields.contains(name) && *name != HISTORY_WINDOW_FIELD)
        .collect();
    if !missing.is_empty() || !unexpected.is_empty() {
        return Err(wrong_fields(label, missing, unexpected));
    }
    Ok(())
}

/// The history window standing in a document, the value it stands as
/// where it stands and null where it does not.
fn standing_window(document: &JsonValue) -> JsonValue {
    document
        .get(HISTORY_WINDOW_FIELD)
        .cloned()
        .unwrap_or(JsonValue::Null)
}

/// Write a field into an object value, standing in place of the
/// value it carried or standing last where it did not stand.
fn set_field(value: &mut JsonValue, name: &str, field_value: JsonValue) {
    if let JsonValue::Object(fields) = value {
        match fields.iter_mut().find(|(key, _)| key == name) {
            Some((_, slot)) => *slot = field_value,
            None => fields.push((name.to_string(), field_value)),
        }
    }
}

/// The pack validation of the ingest module, reading one catchledger
/// pack document and handing it back with its history window
/// standing in it.
fn validate_catchledger_pack(pack: &JsonValue) -> Result<JsonValue> {
    require_fields_with_optional_window(pack, &CATCHLEDGER_PACK_FIELDS, "catchledger pack")?;
    if pack.get("format").and_then(|value| value.as_text()) != Some(CATCHLEDGER_PACK_FORMAT) {
        return Err(TameshiError::refusal(
            "the catchledger pack carries the wrong format name",
        ));
    }
    require_name(
        pack.get("repo").unwrap_or(&JsonValue::Null),
        "catchledger repository label",
    )?;
    require_name(
        pack.get("check").unwrap_or(&JsonValue::Null),
        "catchledger check name",
    )?;
    let scorecard = pack.get("scorecard").unwrap_or(&JsonValue::Null);
    validate_scorecard(scorecard)?;
    if pack.get("false_block_rate").unwrap_or(&JsonValue::Null)
        != scorecard
            .get("false_block_rate")
            .unwrap_or(&JsonValue::Null)
    {
        return Err(TameshiError::refusal(
            "the catchledger pack false block rate disagrees with its scorecard",
        ));
    }
    validate_catch_record(pack.get("catch_record").unwrap_or(&JsonValue::Null))?;
    validate_normalized_canary(pack.get("canaryfield_results").unwrap_or(&JsonValue::Null))?;
    if !pack
        .get("pricing")
        .map(|value| value.is_object())
        .unwrap_or(false)
        || !pack
            .get("source")
            .map(|value| value.is_object())
            .unwrap_or(false)
    {
        return Err(TameshiError::refusal(
            "the catchledger pack pricing and source must be objects",
        ));
    }
    let window = standing_window(pack);
    validate_history_window(&window)?;
    let mut normalized = pack.clone();
    set_field(&mut normalized, HISTORY_WINDOW_FIELD, window);
    Ok(normalized)
}

/// The derivation of one pack from one ledger entry, as the ingest
/// module derives it, the pack carrying the ledger repository label
/// and pricing and source and history window as written.
fn derive_pack_from_entry(
    entry: &JsonValue,
    ledger: &JsonValue,
    window: &JsonValue,
) -> Result<JsonValue> {
    require_exact_fields(
        entry,
        &CATCHLEDGER_LEDGER_ENTRY_FIELDS,
        "catchledger ledger entry",
    )?;
    require_name(
        entry.get("name").unwrap_or(&JsonValue::Null),
        "catchledger check name",
    )?;
    let scorecard = entry.get("scorecard").unwrap_or(&JsonValue::Null);
    validate_scorecard(scorecard)?;
    validate_catch_record(entry.get("catch_record").unwrap_or(&JsonValue::Null))?;
    validate_normalized_canary(entry.get("canaryfield_results").unwrap_or(&JsonValue::Null))?;
    Ok(JsonValue::Object(vec![
        (
            "format".to_string(),
            JsonValue::Text(CATCHLEDGER_PACK_FORMAT.to_string()),
        ),
        (
            "repo".to_string(),
            ledger.get("repo").cloned().unwrap_or(JsonValue::Null),
        ),
        (
            "check".to_string(),
            entry.get("name").cloned().unwrap_or(JsonValue::Null),
        ),
        ("scorecard".to_string(), scorecard.clone()),
        (
            "catch_record".to_string(),
            entry
                .get("catch_record")
                .cloned()
                .unwrap_or(JsonValue::Null),
        ),
        (
            "canaryfield_results".to_string(),
            entry
                .get("canaryfield_results")
                .cloned()
                .unwrap_or(JsonValue::Null),
        ),
        (
            "false_block_rate".to_string(),
            scorecard
                .get("false_block_rate")
                .cloned()
                .unwrap_or(JsonValue::Null),
        ),
        (
            "pricing".to_string(),
            ledger.get("pricing").cloned().unwrap_or(JsonValue::Null),
        ),
        (
            "source".to_string(),
            ledger.get("source").cloned().unwrap_or(JsonValue::Null),
        ),
        (HISTORY_WINDOW_FIELD.to_string(), window.clone()),
    ]))
}

/// Read one catchledger document, a pack or a ledger, into the
/// catchledger packs it carries, as the read entry of the ingest
/// module reads one. A pack reads as the one pack it carries, and a
/// ledger reads as the packs derived from its entries in their
/// standing order.
pub fn read_catchledger_packs(document: &JsonValue) -> Result<Vec<JsonValue>> {
    if !document.is_object() {
        return Err(TameshiError::refusal(
            "the catchledger evidence must be an object",
        ));
    }
    match document.get("format").and_then(|value| value.as_text()) {
        Some(CATCHLEDGER_PACK_FORMAT) => Ok(vec![validate_catchledger_pack(document)?]),
        Some(CATCHLEDGER_LEDGER_FORMAT) => {
            require_fields_with_optional_window(
                document,
                &CATCHLEDGER_LEDGER_FIELDS,
                "catchledger ledger",
            )?;
            require_name(
                document.get("repo").unwrap_or(&JsonValue::Null),
                "catchledger repository label",
            )?;
            let checks = match document.get("checks").and_then(|value| value.as_list()) {
                Some(checks) => checks,
                None => {
                    return Err(TameshiError::refusal(
                        "the catchledger ledger checks must be a list",
                    ))
                }
            };
            let window = standing_window(document);
            validate_history_window(&window)?;
            let mut packs = Vec::new();
            let mut seen_checks: Vec<&str> = Vec::new();
            for entry in checks {
                let pack = derive_pack_from_entry(entry, document, &window)?;
                let check = entry
                    .get("name")
                    .and_then(|value| value.as_text())
                    .unwrap_or("");
                if seen_checks.contains(&check) {
                    return Err(TameshiError::refusal(format!(
                        "the catchledger ledger names a check more than once ({check})"
                    )));
                }
                seen_checks.push(check);
                packs.push(pack);
            }
            Ok(packs)
        }
        _ => Err(TameshiError::refusal(
            "the catchledger evidence carries an unknown format name",
        )),
    }
}
