//! The ruling generator, the counterpart of the rulings module,
//! under obligation PORT-5. It writes nothing itself and deletes
//! nothing. It hands back the ruling record and the rendered body.

use crate::dates;
use crate::errors::{Result, TameshiError};
use crate::ingest;
use crate::jsonvalue::JsonValue;
use crate::names::slugify_check;
use crate::packs;
use crate::render;

pub const RULING_FORMAT: &str = "tameshi-ruling/1";

const PACK_FIELDS: [&str; 13] = [
    "format",
    "repo",
    "check",
    "scorecard",
    "catch_record",
    "history_window",
    "false_block_rate",
    "cost_per_catch_usd",
    "canary_performance",
    "blindness_traces",
    "floor",
    "below_floor",
    "sources",
];

/// The slug a check name gives its ruling files.
pub fn ruling_slug(check: &str) -> String {
    slugify_check(check)
}

fn validate_pack_name(value: &JsonValue, label: &str) -> Result<()> {
    let text = match value.as_text() {
        Some(text) if !ingest::py_trim(text).is_empty() => text,
        _ => {
            return Err(TameshiError::refusal(format!(
                "the evidence pack needs a {label}"
            )))
        }
    };
    if text.contains('\n') || text.contains('\r') {
        return Err(TameshiError::refusal(format!(
            "the evidence pack {label} must not carry a newline"
        )));
    }
    Ok(())
}

/// Read the pack's blindness traces under the discipline
/// the assemble reading demands of a trace, every entry
/// standing in the four field shape with its fields as
/// nonempty text and its check naming the pack's own check.
fn validate_blindness_traces(pack: &JsonValue) -> Result<()> {
    let traces = match pack.get("blindness_traces").and_then(|item| item.as_list()) {
        Some(traces) => traces,
        None => {
            return Err(TameshiError::refusal(
                "the evidence pack blindness traces must be a list",
            ))
        }
    };
    let check = pack
        .get("check")
        .and_then(|item| item.as_text())
        .unwrap_or("");
    for trace in traces {
        let name = ingest::validate_blindness_trace(trace)?;
        if name != check {
            return Err(TameshiError::refusal(
                "the evidence pack blindness traces must name the pack's own check",
            ));
        }
    }
    Ok(())
}

fn validate_pack(pack: &JsonValue) -> Result<()> {
    if !pack.is_object() {
        return Err(TameshiError::refusal("an evidence pack must be an object"));
    }
    let format = pack.get("format").and_then(|item| item.as_text());
    let fields: Vec<&str> = match pack.as_object() {
        Some(fields) => fields.iter().map(|(name, _)| name.as_str()).collect(),
        None => Vec::new(),
    };
    if format == Some(packs::PACK_FORMAT_2) {
        // The second pack standing of DC-3. The pack carries
        // the fields of the first standing and the
        // recommendation, and the recommendation record
        // carries exactly its four fields.
        if !fields.contains(&"recommendation") {
            return Err(TameshiError::refusal(
                "the evidence pack carries the format tameshi-pack/2 and no recommendation",
            ));
        }
        let fields_ok = fields.len() == PACK_FIELDS.len() + 1
            && PACK_FIELDS.iter().all(|field| fields.contains(field));
        if !fields_ok {
            return Err(TameshiError::refusal(
                "the evidence pack has the wrong fields",
            ));
        }
        let record = pack
            .get("recommendation")
            .cloned()
            .unwrap_or(JsonValue::Null);
        let record_fields: Vec<&str> = match record.as_object() {
            Some(held) => held.iter().map(|(name, _)| name.as_str()).collect(),
            None => Vec::new(),
        };
        let record_ok = record_fields.len() == packs::RECOMMENDATION_FIELDS.len()
            && packs::RECOMMENDATION_FIELDS
                .iter()
                .all(|field| record_fields.contains(field));
        if !record_ok {
            return Err(TameshiError::refusal(
                "the evidence pack has the wrong fields",
            ));
        }
    } else {
        let fields_ok = fields.len() == PACK_FIELDS.len()
            && PACK_FIELDS.iter().all(|field| fields.contains(field));
        if !fields_ok {
            return Err(TameshiError::refusal(
                "the evidence pack has the wrong fields",
            ));
        }
        if format != Some(packs::PACK_FORMAT) {
            return Err(TameshiError::refusal(
                "the evidence pack carries the wrong format name",
            ));
        }
    }
    validate_pack_name(
        pack.get("repo").unwrap_or(&JsonValue::Null),
        "repository label",
    )?;
    validate_pack_name(pack.get("check").unwrap_or(&JsonValue::Null), "check name")?;
    ingest::validate_scorecard(pack.get("scorecard").unwrap_or(&JsonValue::Null))?;
    ingest::validate_catch_record(pack.get("catch_record").unwrap_or(&JsonValue::Null))?;
    ingest::validate_history_window(pack.get("history_window").unwrap_or(&JsonValue::Null))?;
    let scorecard = pack.get("scorecard").unwrap_or(&JsonValue::Null);
    if pack.get("false_block_rate").unwrap_or(&JsonValue::Null)
        != scorecard
            .get("false_block_rate")
            .unwrap_or(&JsonValue::Null)
    {
        return Err(TameshiError::refusal(
            "the evidence pack false block rate disagrees with its scorecard",
        ));
    }
    if pack.get("cost_per_catch_usd").unwrap_or(&JsonValue::Null)
        != scorecard
            .get("cost_per_catch_usd")
            .unwrap_or(&JsonValue::Null)
    {
        return Err(TameshiError::refusal(
            "the evidence pack cost per catch disagrees with its scorecard",
        ));
    }
    packs::validate_canary_performance(pack.get("canary_performance").unwrap_or(&JsonValue::Null))?;
    validate_blindness_traces(pack)?;
    if !pack
        .get("floor")
        .map(|item| item.is_object())
        .unwrap_or(false)
    {
        return Err(TameshiError::refusal(
            "the evidence pack floor accounting must be an object",
        ));
    }
    if !matches!(pack.get("below_floor"), Some(JsonValue::Bool(_))) {
        return Err(TameshiError::refusal(
            "the evidence pack below floor mark must be a boolean",
        ));
    }
    if !pack
        .get("sources")
        .map(|item| item.is_object())
        .unwrap_or(false)
    {
        return Err(TameshiError::refusal(
            "the evidence pack sources record must be an object",
        ));
    }
    Ok(())
}

fn field_int(accounting: &JsonValue, field: &str) -> i64 {
    match accounting.get(field) {
        Some(JsonValue::Int(held)) => *held,
        _ => 0,
    }
}

fn floor_refusal(check: &str, accounting: &JsonValue) -> TameshiError {
    let mut legs: Vec<String> = Vec::new();
    if field_int(accounting, "canary_shortfall") > 0 {
        legs.push(format!(
            "the canary evidence leg holds {} of {} required scored planted defects and falls short by {}",
            field_int(accounting, "canary_defects"),
            field_int(accounting, "required_canary_defects"),
            field_int(accounting, "canary_shortfall")
        ));
    }
    if field_int(accounting, "ci_history_shortfall") > 0 {
        legs.push(format!(
            "the CI history leg holds {} of {} required days and falls short by {}",
            field_int(accounting, "ci_history_days"),
            field_int(accounting, "required_ci_history_days"),
            field_int(accounting, "ci_history_shortfall")
        ));
    }
    TameshiError::floor_refusal(format!(
        "ruling refused for check {check} because the evidence is below the floor. {}.",
        legs.join(" And ")
    ))
}

fn number_text(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "none".to_string(),
        JsonValue::Float(decimal) => render::repr_float_text(*decimal),
        JsonValue::Int(whole) => whole.to_string(),
        JsonValue::BigInt(digits) => digits.clone(),
        JsonValue::Bool(word) => {
            if *word {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        JsonValue::Text(text) => text.clone(),
        JsonValue::List(_) | JsonValue::Object(_) => render::repr_value(value),
    }
}

fn getv<'a>(value: &'a JsonValue, field: &str) -> &'a JsonValue {
    value.get(field).unwrap_or(&JsonValue::Null)
}

/// Render the ruling pull request body. Every number is read from
/// the pack.
pub fn render_body(pack: &JsonValue, record: &JsonValue) -> String {
    let scorecard = getv(pack, "scorecard");
    let performance = getv(pack, "canary_performance");
    let accounting = getv(pack, "floor");
    let performance_stands = !matches!(performance, JsonValue::Null);
    let counts = getv(performance, "counts");
    let mut rows: Vec<(&str, String)> = vec![
        (
            "Repository",
            getv(pack, "repo").as_text().unwrap_or("").to_string(),
        ),
        (
            "Check",
            getv(pack, "check").as_text().unwrap_or("").to_string(),
        ),
        (
            "Ruling",
            getv(record, "ruling").as_text().unwrap_or("").to_string(),
        ),
        (
            "State after ruling",
            getv(record, "state_after_ruling")
                .as_text()
                .unwrap_or("")
                .to_string(),
        ),
    ];
    if !matches!(getv(record, "expiry"), JsonValue::Null) {
        rows.push((
            "Expiry",
            getv(record, "expiry").as_text().unwrap_or("").to_string(),
        ));
    }
    let recommendation = getv(pack, "recommendation");
    if !matches!(recommendation, JsonValue::Null) {
        // The citation rows of DC-5, standing where a pack
        // carries a recommendation and nowhere else.
        rows.push((
            "Decision model recommendation",
            getv(recommendation, "recommendation")
                .as_text()
                .unwrap_or("")
                .to_string(),
        ));
        rows.push((
            "Decision model",
            getv(recommendation, "model")
                .as_text()
                .unwrap_or("")
                .to_string(),
        ));
        rows.push((
            "Decision rationale",
            getv(recommendation, "rationale")
                .as_text()
                .unwrap_or("")
                .to_string(),
        ));
    }
    let perf_number = |field: &str| -> String {
        if performance_stands {
            number_text(getv(performance, field))
        } else {
            "none".to_string()
        }
    };
    let count_number = |field: &str| -> String {
        if performance_stands {
            number_text(getv(counts, field))
        } else {
            "none".to_string()
        }
    };
    rows.extend([
        ("Fires", number_text(getv(scorecard, "fires"))),
        ("Blocks", number_text(getv(scorecard, "blocks"))),
        ("Catches", number_text(getv(scorecard, "catches"))),
        ("False blocks", number_text(getv(scorecard, "false_blocks"))),
        (
            "Unattributed blocks",
            number_text(getv(scorecard, "unattributed_blocks")),
        ),
        (
            "False block rate",
            number_text(getv(pack, "false_block_rate")),
        ),
        (
            "Runtime seconds",
            number_text(getv(scorecard, "runtime_seconds")),
        ),
        (
            "Total cost USD",
            number_text(getv(scorecard, "total_cost_usd")),
        ),
        (
            "Cost per catch USD",
            number_text(getv(pack, "cost_per_catch_usd")),
        ),
        ("Canary scored defects", perf_number("scored")),
        ("Canary caught", count_number("caught")),
        ("Canary waved through", count_number("waved-through")),
        ("Canary gate broken", count_number("gate-broken")),
        ("Canary not applicable", count_number("not-applicable")),
        ("Canary catch rate", perf_number("catch_rate")),
        (
            "Canary defects held",
            number_text(getv(accounting, "canary_defects")),
        ),
        (
            "Canary defects required",
            number_text(getv(accounting, "required_canary_defects")),
        ),
        (
            "CI history days held",
            number_text(getv(accounting, "ci_history_days")),
        ),
        (
            "CI history days required",
            number_text(getv(accounting, "required_ci_history_days")),
        ),
        (
            "Blindness traces",
            match getv(pack, "blindness_traces").as_list() {
                Some(traces) => traces.len().to_string(),
                None => "0".to_string(),
            },
        ),
    ]);
    let mut lines: Vec<String> = vec![
        format!(
            "# Ruling for {}",
            getv(pack, "check").as_text().unwrap_or("")
        ),
        String::new(),
        "The evidence pack for this check has been assembled and stands at or above the evidence floor on both legs. The ruling below is issued from that pack and every number in the receipts is read from it.".to_string(),
        String::new(),
        "This ruling is unsigned. A human signs it outside the tool before the pull request is submitted, and nothing is deleted by this ruling.".to_string(),
        String::new(),
        "| Evidence | Value |".to_string(),
        "| --- | --- |".to_string(),
    ];
    for (label, value) in &rows {
        lines.push(format!("| {label} | {value} |"));
    }
    lines.push(String::new());
    let mut body = lines.join("\n");
    body.push('\n');
    body
}

fn calendar_date(value: &JsonValue, label: &str) -> Result<i64> {
    let parsed = value
        .as_text()
        .and_then(|text| dates::parse_calendar_date(text));
    match parsed {
        Some(day) => Ok(day),
        None => Err(TameshiError::refusal(format!(
            "the {label} must be an ISO calendar date"
        ))),
    }
}

fn current_date(today: Option<&JsonValue>) -> Result<i64> {
    match today {
        None => Ok(dates::today_day_number()),
        Some(value) => calendar_date(value, "current date"),
    }
}

fn ruling_words(ruling: &str) -> Option<(&'static str, &'static str)> {
    match ruling {
        "keep" => Some(("keep", "required")),
        "demote" => Some(("demote-to-advisory", "advisory")),
        "quarantine" => Some(("quarantine", "quarantined")),
        "kill" => Some(("kill", "removed")),
        _ => None,
    }
}

/// Generate the ruling record and body for a pack, or refuse. The
/// generator recomputes the floor accounting from the pack own
/// evidence before honouring the recorded accounting, refuses below
/// the floor, and emits an unsigned record only.
pub fn generate_ruling(
    pack: &JsonValue,
    ruling: &str,
    expiry: Option<&JsonValue>,
    today: Option<&JsonValue>,
) -> Result<(JsonValue, String)> {
    validate_pack(pack)?;
    let (verdict, state) = match ruling_words(ruling) {
        Some(words) => words,
        None => {
            return Err(TameshiError::refusal(format!(
                "an unknown ruling was requested ({ruling})"
            )))
        }
    };
    if ruling == "quarantine" {
        let expiry_value = match expiry {
            None => {
                return Err(TameshiError::refusal(
                    "a quarantine ruling requires an expiry date",
                ))
            }
            Some(value) => value,
        };
        let expiry_day = calendar_date(expiry_value, "expiry")?;
        let today_day = current_date(today)?;
        if expiry_day < today_day {
            return Err(TameshiError::refusal(
                "a quarantine expiry must not precede the current date",
            ));
        }
    } else if expiry.is_some() {
        return Err(TameshiError::refusal(
            "an expiry date stands only on a quarantine ruling",
        ));
    }
    let performance = pack
        .get("canary_performance")
        .cloned()
        .unwrap_or(JsonValue::Null);
    let catch_record = pack.get("catch_record").cloned().unwrap_or(JsonValue::Null);
    let recomputed = packs::floor_accounting(&performance, &catch_record)?;
    if pack.get("floor").unwrap_or(&JsonValue::Null) != &recomputed {
        return Err(TameshiError::refusal(
            "the pack floor accounting disagrees with the accounting recomputed from the pack evidence",
        ));
    }
    let meets_floor = matches!(recomputed.get("meets_floor"), Some(JsonValue::Bool(true)));
    if !meets_floor {
        let check = pack
            .get("check")
            .and_then(|item| item.as_text())
            .unwrap_or("");
        return Err(floor_refusal(check, &recomputed));
    }
    if ruling == "kill" {
        // The kill gate of DC-4, read after the floor. A
        // kill issues only on a pack whose recommendation
        // record recommends it.
        let recommended = pack
            .get("recommendation")
            .and_then(|record| record.get("recommendation"))
            .and_then(|word| word.as_text())
            == Some("kill");
        if !recommended {
            return Err(TameshiError::refusal(
                "a kill ruling requires a decision record recommending kill in the pack",
            ));
        }
    }
    let mut record_fields: Vec<(String, JsonValue)> = vec![
        (
            "format".to_string(),
            JsonValue::Text(RULING_FORMAT.to_string()),
        ),
        (
            "repo".to_string(),
            pack.get("repo").cloned().unwrap_or(JsonValue::Null),
        ),
        (
            "check".to_string(),
            pack.get("check").cloned().unwrap_or(JsonValue::Null),
        ),
        ("ruling".to_string(), JsonValue::Text(verdict.to_string())),
        (
            "state_after_ruling".to_string(),
            JsonValue::Text(state.to_string()),
        ),
        (
            "expiry".to_string(),
            expiry.cloned().unwrap_or(JsonValue::Null),
        ),
        (
            "floor".to_string(),
            pack.get("floor").cloned().unwrap_or(JsonValue::Null),
        ),
        (
            "signature".to_string(),
            JsonValue::Object(vec![
                (
                    "status".to_string(),
                    JsonValue::Text("unsigned".to_string()),
                ),
                ("signed_by".to_string(), JsonValue::Null),
                ("signed_at".to_string(), JsonValue::Null),
            ]),
        ),
    ];
    if let Some(recommendation) = pack.get("recommendation") {
        // The citation of DC-5. The recommendation stands
        // in the ruling record beside the floor, ahead of
        // the signature.
        let signature = record_fields.pop();
        record_fields.push(("recommendation".to_string(), recommendation.clone()));
        if let Some(signature) = signature {
            record_fields.push(signature);
        }
    }
    let record = JsonValue::Object(record_fields);
    let body = render_body(pack, &record);
    Ok((record, body))
}

const RULING_DISAGREEMENT: &str = "the ruling record disagrees with the pack it was ruled from";

fn ruling_disagreement() -> TameshiError {
    TameshiError::pack_validation(RULING_DISAGREEMENT)
}

/// Check a ruling record's signature block stands in a shape
/// the validation honours. The generator emits unsigned
/// records only and a human signs outside the tool, so the
/// signer and the signing time are the human's own values and
/// are compared in neither direction, and the block is read
/// for its shape and its consistency alone.
fn validate_signature_block(record: &JsonValue) -> Result<()> {
    let signature = match record.get("signature") {
        Some(value) => value,
        None => return Err(ruling_disagreement()),
    };
    let pairs = match signature.as_object() {
        Some(pairs) if pairs.len() == 3 => pairs,
        _ => return Err(ruling_disagreement()),
    };
    let mut status: Option<&str> = None;
    let mut signed_by: Option<&JsonValue> = None;
    let mut signed_at: Option<&JsonValue> = None;
    for (key, value) in pairs {
        match key.as_str() {
            "status" => status = value.as_text(),
            "signed_by" => signed_by = Some(value),
            "signed_at" => signed_at = Some(value),
            _ => return Err(ruling_disagreement()),
        }
    }
    let nonempty_text = |value: Option<&JsonValue>| -> bool {
        matches!(value, Some(JsonValue::Text(text)) if !text.is_empty())
    };
    match status {
        Some("unsigned") => {
            if signed_by != Some(&JsonValue::Null) || signed_at != Some(&JsonValue::Null) {
                return Err(ruling_disagreement());
            }
        }
        Some("signed") => {
            if !nonempty_text(signed_by) || !nonempty_text(signed_at) {
                return Err(ruling_disagreement());
            }
        }
        _ => return Err(ruling_disagreement()),
    }
    Ok(())
}

/// The verdict and state pairs the generator writes, read
/// back as the consistency a supplied record must stand in.
fn verdict_state(verdict: &str) -> Option<&'static str> {
    match verdict {
        "keep" => Some("required"),
        "demote-to-advisory" => Some("advisory"),
        "quarantine" => Some("quarantined"),
        "kill" => Some("removed"),
        _ => None,
    }
}

/// Validate a ruling record against the pack it was ruled
/// from. The record is checked for the standing the generator
/// gives it, field by field against the pack it cites. Its
/// repository and check stand as the pack's, its floor stands
/// as the pack's floor copied whole and that floor must be
/// the accounting the pack's own evidence recomputes and
/// must meet the floor, its verdict and state stand as a
/// pair the generator writes, its expiry stands only on a
/// quarantine and there as a calendar date, a kill verdict
/// stands only where the pack recommends kill, and its
/// recommendation citation stands as the pack's record
/// exactly where the pack carries one and nowhere else. The
/// ruling word and the expiry are the record's own carriers,
/// so the validation proves the record is a ruling the
/// generator could have issued from this pack under the
/// word and the expiry it carries, and no more than that.
pub fn validate_ruling(record: &JsonValue, pack: &JsonValue) -> Result<bool> {
    if !record.is_object()
        || record.get("format").and_then(|item| item.as_text()) != Some(RULING_FORMAT)
    {
        return Err(ruling_disagreement());
    }
    validate_pack(pack)?;
    let recommendation = pack.get("recommendation");
    let expected_fields: &[&str] = if recommendation.is_some() {
        &[
            "format",
            "repo",
            "check",
            "ruling",
            "state_after_ruling",
            "expiry",
            "floor",
            "recommendation",
            "signature",
        ]
    } else {
        &[
            "format",
            "repo",
            "check",
            "ruling",
            "state_after_ruling",
            "expiry",
            "floor",
            "signature",
        ]
    };
    let fields: Vec<&str> = record
        .as_object()
        .expect("the record stands as an object")
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();
    if fields.len() != expected_fields.len()
        || !expected_fields.iter().all(|name| fields.contains(name))
    {
        return Err(ruling_disagreement());
    }
    if record.get("repo") != pack.get("repo") || record.get("check") != pack.get("check") {
        return Err(ruling_disagreement());
    }
    if record.get("floor") != pack.get("floor") {
        return Err(ruling_disagreement());
    }
    let recomputed = packs::recompute_floor(pack)?;
    if pack.get("floor").unwrap_or(&JsonValue::Null) != &recomputed
        || !matches!(recomputed.get("meets_floor"), Some(JsonValue::Bool(true)))
    {
        return Err(ruling_disagreement());
    }
    let verdict = record
        .get("ruling")
        .and_then(|item| item.as_text())
        .unwrap_or("");
    let state = match verdict_state(verdict) {
        Some(state) => state,
        None => return Err(ruling_disagreement()),
    };
    if record
        .get("state_after_ruling")
        .and_then(|item| item.as_text())
        != Some(state)
    {
        return Err(ruling_disagreement());
    }
    match verdict {
        "quarantine" => {
            let expiry = match record.get("expiry") {
                Some(value) => value,
                None => return Err(ruling_disagreement()),
            };
            calendar_date(expiry, "expiry")?;
        }
        "kill" => {
            let recommended = recommendation
                .and_then(|held| held.get("recommendation"))
                .and_then(|word| word.as_text())
                == Some("kill");
            if !recommended {
                return Err(ruling_disagreement());
            }
            if !matches!(record.get("expiry"), Some(JsonValue::Null)) {
                return Err(ruling_disagreement());
            }
        }
        _ => {
            if !matches!(record.get("expiry"), Some(JsonValue::Null)) {
                return Err(ruling_disagreement());
            }
        }
    }
    if let Some(held) = recommendation {
        if record.get("recommendation") != Some(held) {
            return Err(ruling_disagreement());
        }
    }
    validate_signature_block(record)?;
    Ok(true)
}

/// Validate a supplied ruling body against the body the rule
/// door renders from the same record and pack. The comparison
/// stands over the bytes themselves, because a body is its
/// rendering and a respelling of it is a different document.
/// The record stands validated under validate_ruling before
/// this reading runs at the door, so the body is rendered
/// from the record as supplied standing validated, and the
/// proof limit the fourth slice's amendment states stands
/// unchanged.
pub fn validate_ruling_body(supplied: &[u8], record: &JsonValue, pack: &JsonValue) -> Result<bool> {
    let rendered = render_body(pack, record);
    if supplied != rendered.as_bytes() {
        return Err(TameshiError::pack_validation(
            "the ruling body disagrees with the ruling record and its pack",
        ));
    }
    Ok(true)
}
