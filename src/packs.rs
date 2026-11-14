//! Evidence pack assembly, floor accounting, and pack validation,
//! the counterpart of the packs module, under obligation PORT-4.

use crate::dates;
use crate::errors::{Result, TameshiError};
use crate::ingest;
use crate::jsonvalue::JsonValue;
use crate::names::slugify_check;

pub const PACK_FORMAT: &str = "tameshi-pack/1";
pub const PACK_FORMAT_2: &str = "tameshi-pack/2";
pub const DECISION_FORMAT: &str = "tameshi-decision/1";

/// The recommendation words a decision document may carry
/// under DC-1, the ruling set of the generator with the
/// kill word the decision record admits.
pub const RECOMMENDATION_WORDS: [&str; 4] = ["keep", "demote", "quarantine", "kill"];

/// The fields of a decision document under DC-1.
const DECISION_FIELDS: [&str; 6] = [
    "format",
    "check",
    "recommendation",
    "rationale",
    "model",
    "run",
];

/// The fields of the recommendation record a pack carries
/// under DC-2, the decision document's fields with the
/// check standing in the pack itself.
pub const RECOMMENDATION_FIELDS: [&str; 4] = ["recommendation", "rationale", "model", "run"];

/// Read and validate one decision document under DC-1,
/// handing back the check it names and the recommendation
/// record the pack carries for it.
pub fn read_decision(document: &JsonValue) -> Result<(String, JsonValue)> {
    let fields = match document.as_object() {
        Some(fields) => fields,
        None => {
            return Err(TameshiError::refusal(
                "the decision document has the wrong fields (missing check, format, model, rationale, recommendation, run)",
            ))
        }
    };
    let standing: Vec<&str> = fields.iter().map(|(name, _)| name.as_str()).collect();
    let missing: Vec<&str> = DECISION_FIELDS
        .iter()
        .copied()
        .filter(|field| !standing.contains(field))
        .collect();
    let unexpected: Vec<&str> = standing
        .iter()
        .copied()
        .filter(|name| !DECISION_FIELDS.contains(name))
        .collect();
    if !missing.is_empty() || !unexpected.is_empty() {
        let mut details: Vec<String> = Vec::new();
        if !missing.is_empty() {
            details.push(format!("missing {}", missing.join(", ")));
        }
        if !unexpected.is_empty() {
            details.push(format!("unexpected {}", unexpected.join(", ")));
        }
        return Err(TameshiError::refusal(format!(
            "the decision document has the wrong fields ({})",
            details.join("; ")
        )));
    }
    if document.get("format").and_then(|value| value.as_text()) != Some(DECISION_FORMAT) {
        return Err(TameshiError::refusal(
            "the decision document carries the wrong format name",
        ));
    }
    let check = ingest::require_name(
        document.get("check").unwrap_or(&JsonValue::Null),
        "decision check name",
    )?
    .to_string();
    let recommendation = ingest::require_text(
        document.get("recommendation").unwrap_or(&JsonValue::Null),
        "decision recommendation",
    )?;
    if !RECOMMENDATION_WORDS.contains(&recommendation) {
        return Err(TameshiError::refusal(format!(
            "the decision document carries an unknown recommendation ({recommendation})"
        )));
    }
    let rationale = ingest::require_text(
        document.get("rationale").unwrap_or(&JsonValue::Null),
        "decision rationale",
    )?
    .to_string();
    let model = ingest::require_text(
        document.get("model").unwrap_or(&JsonValue::Null),
        "decision model",
    )?
    .to_string();
    let run = ingest::require_text(
        document.get("run").unwrap_or(&JsonValue::Null),
        "decision run",
    )?
    .to_string();
    let record = JsonValue::Object(vec![
        (
            "recommendation".to_string(),
            JsonValue::Text(recommendation.to_string()),
        ),
        ("rationale".to_string(), JsonValue::Text(rationale)),
        ("model".to_string(), JsonValue::Text(model)),
        ("run".to_string(), JsonValue::Text(run)),
    ]);
    Ok((check, record))
}
pub const REQUIRED_CANARY_DEFECTS: i64 = 30;
pub const REQUIRED_CI_HISTORY_DAYS: i64 = 14;

const PERFORMANCE_FIELDS: [&str; 5] = ["gate", "counts", "scored", "catch_rate", "results"];
const PERFORMANCE_ROW_FIELDS: [&str; 5] =
    ["seed", "name", "defect_class", "result", "planted_files"];

/// The catch rate as the ratio of caught to scored, rounded to six
/// places with halves rounded away from zero, standing null where
/// nothing is scored.
fn ratio(caught: i64, scored: i64) -> JsonValue {
    if scored == 0 {
        return JsonValue::Null;
    }
    // Six places of a ratio of whole numbers, in exact arithmetic.
    let numerator = caught as i128 * 2_000_000 + scored as i128;
    let denominator = scored as i128 * 2;
    let micro = numerator.div_euclid(denominator);
    JsonValue::Float(micro as f64 / 1_000_000.0)
}

fn derived_scored(counts: &JsonValue) -> i64 {
    let mut total: i64 = 0;
    for word in ["caught", "waved-through", "gate-broken"] {
        if let Some(JsonValue::Int(held)) = counts.get(word) {
            total += held;
        }
    }
    total
}

fn counts_cover_words(counts: &JsonValue) -> bool {
    match counts.as_object() {
        Some(fields) => {
            fields.len() == ingest::RESULT_WORDS.len()
                && ingest::RESULT_WORDS
                    .iter()
                    .all(|word| fields.iter().any(|(name, _)| name == word))
        }
        None => false,
    }
}

/// Validate a pack canary performance against its own row evidence.
/// A null performance stands as no performance and validates.
pub fn validate_canary_performance(performance: &JsonValue) -> Result<()> {
    if matches!(performance, JsonValue::Null) {
        return Ok(());
    }
    let fields_ok = match performance.as_object() {
        Some(fields) => {
            fields.len() == PERFORMANCE_FIELDS.len()
                && PERFORMANCE_FIELDS
                    .iter()
                    .all(|field| fields.iter().any(|(name, _)| name == field))
        }
        None => false,
    };
    if !fields_ok {
        return Err(TameshiError::refusal(
            "the canary performance has the wrong fields",
        ));
    }
    let gate_ok = match performance.get("gate").and_then(|item| item.as_text()) {
        Some(gate) => {
            !ingest::py_trim(gate).is_empty() && !gate.contains('\n') && !gate.contains('\r')
        }
        None => false,
    };
    if !gate_ok {
        return Err(TameshiError::refusal(
            "the canary performance needs a gate name",
        ));
    }
    let counts = performance.get("counts").unwrap_or(&JsonValue::Null);
    if !counts_cover_words(counts) {
        return Err(TameshiError::refusal(
            "the canary performance counts must cover the four result words",
        ));
    }
    let counts_ok = match counts.as_object() {
        Some(fields) => fields.iter().all(|(_, value)| value.is_nonnegative_whole()),
        None => false,
    };
    if !counts_ok {
        return Err(TameshiError::refusal(
            "the canary performance counts must be nonnegative whole numbers",
        ));
    }
    let results = match performance.get("results").and_then(|item| item.as_list()) {
        Some(results) => results,
        None => {
            return Err(TameshiError::refusal(
                "the canary performance results must be a list",
            ))
        }
    };
    for row in results {
        let row_ok = match row.as_object() {
            Some(fields) => {
                fields.len() == PERFORMANCE_ROW_FIELDS.len()
                    && PERFORMANCE_ROW_FIELDS
                        .iter()
                        .all(|field| fields.iter().any(|(name, _)| name == field))
            }
            None => false,
        };
        if !row_ok {
            return Err(TameshiError::refusal(
                "a canary performance result row has the wrong fields",
            ));
        }
        for field in ["seed", "name", "defect_class"] {
            let present = row
                .get(field)
                .and_then(|item| item.as_text())
                .map(|text| !ingest::py_trim(text).is_empty())
                .unwrap_or(false);
            if !present {
                return Err(TameshiError::refusal(format!(
                    "a canary performance result row needs a nonempty {field}"
                )));
            }
        }
        let result = row.get("result").and_then(|item| item.as_text());
        if !matches!(result, Some(word) if ingest::RESULT_WORDS.contains(&word)) {
            return Err(TameshiError::refusal(
                "a canary performance result word is not a known result word",
            ));
        }
        let planted_ok = match row.get("planted_files").and_then(|item| item.as_list()) {
            Some(files) => files.iter().all(|file| file.as_text().is_some()),
            None => false,
        };
        if !planted_ok {
            return Err(TameshiError::refusal(
                "a canary performance result row carries malformed planted files",
            ));
        }
    }
    if counts != &ingest::tally_canary_results(results) {
        return Err(TameshiError::refusal(
            "the canary performance counts disagree with their result rows",
        ));
    }
    let scored = derived_scored(counts);
    let scored_ok = match performance.get("scored") {
        Some(value) => value.whole_digits() == Some(scored.to_string()),
        None => false,
    };
    if !scored_ok {
        return Err(TameshiError::refusal(
            "the canary performance scored count disagrees with its rows",
        ));
    }
    let caught = match counts.get("caught") {
        Some(JsonValue::Int(held)) => *held,
        _ => 0,
    };
    if performance.get("catch_rate").unwrap_or(&JsonValue::Null) != &ratio(caught, scored) {
        return Err(TameshiError::refusal(
            "the canary performance catch rate disagrees with its rows",
        ));
    }
    Ok(())
}

/// Build the pack canary performance from normalized canaryfield
/// evidence. A null normalized value builds no performance.
pub fn canary_performance(normalized: &JsonValue) -> Result<Option<JsonValue>> {
    if matches!(normalized, JsonValue::Null) {
        return Ok(None);
    }
    ingest::validate_normalized_canary(normalized)?;
    let results = normalized
        .get("results")
        .and_then(|item| item.as_list())
        .map(|items| items.to_vec())
        .unwrap_or_default();
    let counts = ingest::tally_canary_results(&results);
    let scored = derived_scored(&counts);
    let caught = match counts.get("caught") {
        Some(JsonValue::Int(held)) => *held,
        _ => 0,
    };
    Ok(Some(JsonValue::Object(vec![
        (
            "gate".to_string(),
            normalized.get("gate").cloned().unwrap_or(JsonValue::Null),
        ),
        ("counts".to_string(), counts),
        ("scored".to_string(), JsonValue::Int(scored)),
        ("catch_rate".to_string(), ratio(caught, scored)),
        ("results".to_string(), JsonValue::List(results)),
    ])))
}

/// Count the UTC calendar days one check catch record spans, from the
/// date of the earliest started at timestamp through the date of the
/// latest, each converted to UTC first. An empty record holds zero.
pub fn catch_record_days(catch_record: &JsonValue) -> Result<i64> {
    ingest::validate_catch_record(catch_record)?;
    let entries = catch_record.as_list().unwrap_or(&[]);
    if entries.is_empty() {
        return Ok(0);
    }
    let mut days: Vec<i64> = Vec::new();
    for entry in entries {
        let instant = ingest::parse_timestamp(
            entry.get("started_at").unwrap_or(&JsonValue::Null),
            "catch record started at timestamp",
        )?;
        days.push(dates::utc_day_of_instant(instant));
    }
    let latest = days.iter().max().copied().unwrap_or(0);
    let earliest = days.iter().min().copied().unwrap_or(0);
    Ok(latest - earliest + 1)
}

/// Account one check evidence against the floor on both legs.
pub fn floor_accounting(performance: &JsonValue, catch_record: &JsonValue) -> Result<JsonValue> {
    if !matches!(performance, JsonValue::Null) {
        validate_canary_performance(performance)?;
    }
    let held_defects = if matches!(performance, JsonValue::Null) {
        0
    } else {
        derived_scored(performance.get("counts").unwrap_or(&JsonValue::Null))
    };
    let held_days = catch_record_days(catch_record)?;
    Ok(JsonValue::Object(vec![
        (
            "required_canary_defects".to_string(),
            JsonValue::Int(REQUIRED_CANARY_DEFECTS),
        ),
        (
            "required_ci_history_days".to_string(),
            JsonValue::Int(REQUIRED_CI_HISTORY_DAYS),
        ),
        ("canary_defects".to_string(), JsonValue::Int(held_defects)),
        ("ci_history_days".to_string(), JsonValue::Int(held_days)),
        (
            "canary_shortfall".to_string(),
            JsonValue::Int((REQUIRED_CANARY_DEFECTS - held_defects).max(0)),
        ),
        (
            "ci_history_shortfall".to_string(),
            JsonValue::Int((REQUIRED_CI_HISTORY_DAYS - held_days).max(0)),
        ),
        (
            "meets_floor".to_string(),
            JsonValue::Bool(
                held_defects >= REQUIRED_CANARY_DEFECTS && held_days >= REQUIRED_CI_HISTORY_DAYS,
            ),
        ),
    ]))
}

fn assemble_one(
    catchledger_pack: &JsonValue,
    canary_document: Option<&JsonValue>,
    traces_by_check: Option<&Vec<(String, Vec<JsonValue>)>>,
    blindness_supplied: bool,
    unmatched_trace_count: i64,
    recommendation: Option<&JsonValue>,
) -> Result<JsonValue> {
    let check = catchledger_pack
        .get("check")
        .and_then(|item| item.as_text())
        .unwrap_or("")
        .to_string();
    let (origin, performance) = match canary_document {
        Some(document) => {
            let normalized = ingest::normalize_canary_document(document, &check)?;
            let performance = match normalized {
                Some(value) => canary_performance(&value)?,
                None => None,
            };
            ("direct", performance)
        }
        None => {
            let embedded = catchledger_pack
                .get("canaryfield_results")
                .cloned()
                .unwrap_or(JsonValue::Null);
            if matches!(embedded, JsonValue::Null) {
                ("none", None)
            } else {
                let performance = canary_performance(&embedded)?;
                ("embedded", performance)
            }
        }
    };
    let traces: Vec<JsonValue> = match traces_by_check {
        None => Vec::new(),
        Some(grouped) => grouped
            .iter()
            .find(|(name, _)| name == &check)
            .map(|(_, held)| held.clone())
            .unwrap_or_default(),
    };
    let window = catchledger_pack
        .get(ingest::HISTORY_WINDOW_FIELD)
        .cloned()
        .unwrap_or(JsonValue::Null);
    let catch_record = catchledger_pack
        .get("catch_record")
        .cloned()
        .unwrap_or(JsonValue::Null);
    let performance_value = performance.clone().unwrap_or(JsonValue::Null);
    let accounting = floor_accounting(&performance_value, &catch_record)?;
    let meets_floor = matches!(accounting.get("meets_floor"), Some(JsonValue::Bool(true)));
    let scorecard = catchledger_pack
        .get("scorecard")
        .cloned()
        .unwrap_or(JsonValue::Null);
    let mut pack_fields: Vec<(String, JsonValue)> = vec![
        (
            "format".to_string(),
            JsonValue::Text(
                if recommendation.is_some() {
                    PACK_FORMAT_2
                } else {
                    PACK_FORMAT
                }
                .to_string(),
            ),
        ),
        (
            "repo".to_string(),
            catchledger_pack
                .get("repo")
                .cloned()
                .unwrap_or(JsonValue::Null),
        ),
        ("check".to_string(), JsonValue::Text(check)),
        ("scorecard".to_string(), scorecard.clone()),
        ("catch_record".to_string(), catch_record),
        ("history_window".to_string(), window),
        (
            "false_block_rate".to_string(),
            catchledger_pack
                .get("false_block_rate")
                .cloned()
                .unwrap_or(JsonValue::Null),
        ),
        (
            "cost_per_catch_usd".to_string(),
            scorecard
                .get("cost_per_catch_usd")
                .cloned()
                .unwrap_or(JsonValue::Null),
        ),
        ("canary_performance".to_string(), performance_value),
        ("blindness_traces".to_string(), JsonValue::List(traces)),
        ("floor".to_string(), accounting),
        ("below_floor".to_string(), JsonValue::Bool(!meets_floor)),
        (
            "sources".to_string(),
            JsonValue::Object(vec![
                (
                    "catchledger_source".to_string(),
                    catchledger_pack
                        .get("source")
                        .cloned()
                        .unwrap_or(JsonValue::Null),
                ),
                (
                    "canary_origin".to_string(),
                    JsonValue::Text(origin.to_string()),
                ),
                (
                    "blindgate_evidence_supplied".to_string(),
                    JsonValue::Bool(blindness_supplied),
                ),
                (
                    "blindgate_unmatched_trace_count".to_string(),
                    JsonValue::Int(unmatched_trace_count),
                ),
            ]),
        ),
    ];
    if let Some(record) = recommendation {
        pack_fields.push(("recommendation".to_string(), record.clone()));
    }
    Ok(JsonValue::Object(pack_fields))
}

fn pack_filename(index: usize, check: &str) -> String {
    format!("{:03}-{}.json", index, slugify_check(check))
}

/// Assemble one evidence pack per check, keyed by deterministic file
/// name, from catchledger packs already read.
pub fn assemble_packs(
    catchledger_packs: &[JsonValue],
    canary_document: Option<&JsonValue>,
    blindness_document: Option<&JsonValue>,
    decision: Option<&(String, JsonValue)>,
) -> Result<Vec<(String, JsonValue)>> {
    let mut traces_by_check: Option<Vec<(String, Vec<JsonValue>)>> = None;
    let mut unmatched_trace_count: i64 = 0;
    if let Some(document) = blindness_document {
        let grouped = ingest::read_blindness_traces(document)?;
        let known: Vec<&str> = catchledger_packs
            .iter()
            .filter_map(|pack| pack.get("check").and_then(|item| item.as_text()))
            .collect();
        unmatched_trace_count = grouped
            .iter()
            .filter(|(name, _)| !known.contains(&name.as_str()))
            .map(|(_, held)| held.len() as i64)
            .sum();
        traces_by_check = Some(grouped);
    }
    let mut ordered: Vec<&JsonValue> = catchledger_packs.iter().collect();
    ordered.sort_by(|a, b| {
        let left = a.get("check").and_then(|item| item.as_text()).unwrap_or("");
        let right = b.get("check").and_then(|item| item.as_text()).unwrap_or("");
        left.cmp(right)
    });
    if let Some((named, _)) = decision {
        let known = ordered
            .iter()
            .any(|pack| pack.get("check").and_then(|item| item.as_text()) == Some(named.as_str()));
        if !known {
            return Err(TameshiError::refusal(format!(
                "the decision document names a check the run does not assemble ({named})"
            )));
        }
    }
    let mut packs = Vec::new();
    for (position, catchledger_pack) in ordered.iter().enumerate() {
        let check_name = catchledger_pack
            .get("check")
            .and_then(|item| item.as_text())
            .unwrap_or("");
        let recommendation = match decision {
            Some((named, record)) if named == check_name => Some(record),
            _ => None,
        };
        let pack = assemble_one(
            catchledger_pack,
            canary_document,
            traces_by_check.as_ref(),
            blindness_document.is_some(),
            unmatched_trace_count,
            recommendation,
        )?;
        let check = pack
            .get("check")
            .and_then(|item| item.as_text())
            .unwrap_or("")
            .to_string();
        packs.push((pack_filename(position + 1, &check), pack));
    }
    Ok(packs)
}

/// Return true when a pack exactly matches the pack rebuilt from its
/// inputs, refusing as a pack validation error where it does not.
pub fn validate_pack(
    pack: &JsonValue,
    catchledger_packs: &[JsonValue],
    canary_document: Option<&JsonValue>,
    blindness_document: Option<&JsonValue>,
    decision: Option<&(String, JsonValue)>,
) -> Result<bool> {
    if !pack.is_object() {
        return Err(TameshiError::pack_validation(
            "an evidence pack must be an object",
        ));
    }
    let check_name = match pack.get("check").and_then(|item| item.as_text()) {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => {
            return Err(TameshiError::pack_validation(
                "an evidence pack needs a check name",
            ))
        }
    };
    let rebuilt = assemble_packs(
        catchledger_packs,
        canary_document,
        blindness_document,
        decision,
    )?;
    let mut expected: Option<&JsonValue> = None;
    for (_, candidate) in &rebuilt {
        if candidate.get("check").and_then(|item| item.as_text()) == Some(check_name.as_str()) {
            expected = Some(candidate);
            break;
        }
    }
    let expected = match expected {
        Some(expected) => expected,
        None => {
            return Err(TameshiError::pack_validation(
                "the evidence pack names a check outside its inputs",
            ))
        }
    };
    if pack != expected {
        return Err(TameshiError::pack_validation(
            "the evidence pack disagrees with its rebuilt pack",
        ));
    }
    Ok(true)
}

/// Recompute a pack floor accounting from the pack own evidence.
pub fn recompute_floor(pack: &JsonValue) -> Result<JsonValue> {
    if !pack.is_object() {
        return Err(TameshiError::refusal("an evidence pack must be an object"));
    }
    let performance = pack
        .get("canary_performance")
        .cloned()
        .unwrap_or(JsonValue::Null);
    let catch_record = pack.get("catch_record").cloned().unwrap_or(JsonValue::Null);
    floor_accounting(&performance, &catch_record)
}
