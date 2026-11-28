//! Cross-platform portability and crate-root API surface pins.

use std::path::PathBuf;
use std::process::Command;

use tameshi::{
    assemble_packs, canary_performance, catch_record_days, floor_accounting, generate_ruling,
    normalize_canary_document, read_blindness_traces, read_catchledger_packs, read_decision,
    read_file_bytes, read_json_document, read_json_file, recompute_floor, render_body, render_json,
    ruling_slug, slugify_check, validate_canary_document, validate_canary_performance,
    validate_pack, validate_ruling, validate_ruling_body, JsonValue, BLINDNESS_FORMAT,
    CANARY_SOURCE_FORMAT, CATCHLEDGER_LEDGER_FORMAT, CATCHLEDGER_PACK_FORMAT, DECISION_FORMAT,
    PACK_FORMAT, PACK_FORMAT_2, RECOMMENDATION_FIELDS, RECOMMENDATION_WORDS,
    REQUIRED_CANARY_DEFECTS, REQUIRED_CI_HISTORY_DAYS, RULING_FORMAT,
};

fn scratch_dir(tag: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("tameshi-portability-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch directory stands");
    dir
}

#[test]
fn crate_root_reexports_and_inline_flags_work() {
    assert_eq!(PACK_FORMAT, "tameshi-pack/1");
    assert_eq!(PACK_FORMAT_2, "tameshi-pack/2");
    assert_eq!(DECISION_FORMAT, "tameshi-decision/1");
    assert_eq!(RULING_FORMAT, "tameshi-ruling/1");
    assert_eq!(BLINDNESS_FORMAT, "blindgate-evidence/1");
    assert_eq!(CANARY_SOURCE_FORMAT, "canaryfield-catch-table");
    assert_eq!(CATCHLEDGER_PACK_FORMAT, "catchledger-pack/1");
    assert_eq!(CATCHLEDGER_LEDGER_FORMAT, "catchledger-ledger/1");
    assert_eq!(REQUIRED_CANARY_DEFECTS, 30);
    assert_eq!(REQUIRED_CI_HISTORY_DAYS, 14);
    assert_eq!(RECOMMENDATION_WORDS.len(), 4);
    assert_eq!(RECOMMENDATION_FIELDS.len(), 4);
    assert_eq!(slugify_check("Unit Checks"), "unit-checks");
    assert_eq!(ruling_slug("Unit Checks"), "unit-checks");

    let dir = scratch_dir("reexports");
    let mut rows_json = Vec::new();
    for idx in 0..30 {
        rows_json.push(format!(
            "{{\"seed\": \"s{idx:02}\", \"name\": \"d{idx:02}\", \
             \"defect_class\": \"logic\", \"gates\": {{\"unit-checks\": \"caught\"}}, \
             \"planted_files\": [\"src/lib.rs\"]}}"
        ));
    }
    let canary_text = format!(
        "{{\"gates\": [\"unit-checks\"], \"rows\": [{}]}}",
        rows_json.join(", ")
    );
    let canary_doc = read_json_document(&canary_text, "synthetic canary").expect("canary parses");
    validate_canary_document(&canary_doc).expect("canary validates");
    let normalized = normalize_canary_document(&canary_doc, "unit-checks")
        .expect("canary normalizes")
        .expect("gate stands");
    let perf = canary_performance(&normalized)
        .expect("performance builds")
        .expect("performance present");
    validate_canary_performance(&perf).expect("performance validates");

    let blindness_doc = read_json_document(
        "{\"format\": \"blindgate-evidence/1\", \"traces\": []}",
        "synthetic blindness",
    )
    .expect("blindness parses");
    assert!(read_blindness_traces(&blindness_doc)
        .expect("blindness reads")
        .is_empty());

    let decision_doc = read_json_document(
        "{\"format\": \"tameshi-decision/1\", \"check\": \"unit-checks\", \
         \"recommendation\": \"keep\", \"rationale\": \"catches defects\", \
         \"model\": \"synthetic-1\", \"run\": \"run-1\"}",
        "synthetic decision",
    )
    .expect("decision parses");
    let decision = read_decision(&decision_doc).expect("decision validates");
    assert_eq!(decision.0, "unit-checks");

    let cl_pack = read_json_document(
        "{\"format\": \"catchledger-pack/1\", \"repo\": \"synthetic/demo\", \
         \"check\": \"unit-checks\", \
         \"scorecard\": {\"fires\": 2, \"blocks\": 2, \"judged_commits\": 2, \
         \"flaky_commits\": 0, \"flake_rate\": 0.0, \"runtime_seconds\": 20, \
         \"runtime_minutes\": 0.333333, \"catches\": 2, \"false_blocks\": 0, \
         \"unattributed_blocks\": 0, \"false_block_rate\": 0.0, \
         \"total_cost_usd\": 0.002667, \"cost_per_catch_usd\": 0.001333}, \
         \"catch_record\": [\
           {\"block_run_id\": \"1\", \"blocked_commit\": \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", \
            \"started_at\": \"2026-01-01T00:00:00Z\", \"outcome\": \"catch\", \
            \"flagged_paths\": [\"src/lib.rs\"], \
            \"evidence\": {\"type\": \"fix\", \"fix_commits\": [\"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\"], \
                           \"validating_run_id\": \"2\", \"reason\": \"fixed\"}},\
           {\"block_run_id\": \"3\", \"blocked_commit\": \"cccccccccccccccccccccccccccccccccccccccc\", \
            \"started_at\": \"2026-01-15T00:00:00Z\", \"outcome\": \"catch\", \
            \"flagged_paths\": [\"src/lib.rs\"], \
            \"evidence\": {\"type\": \"fix\", \"fix_commits\": [\"dddddddddddddddddddddddddddddddddddddddd\"], \
                           \"validating_run_id\": \"4\", \"reason\": \"fixed\"}}\
         ], \
         \"canaryfield_results\": null, \
         \"false_block_rate\": 0.0, \"pricing\": {}, \"source\": {}}",
        "synthetic catchledger pack",
    )
    .expect("catchledger pack parses");
    let cl_packs = read_catchledger_packs(&cl_pack).expect("catchledger pack reads");
    assert_eq!(
        catch_record_days(cl_packs[0].get("catch_record").expect("catch_record"))
            .expect("days compute"),
        15
    );
    let accounting = floor_accounting(
        &perf,
        cl_packs[0].get("catch_record").expect("catch_record"),
    )
    .expect("floor accounting computes");
    assert_eq!(accounting.get("meets_floor"), Some(&JsonValue::Bool(true)));

    let assembled = assemble_packs(
        &cl_packs,
        Some(&canary_doc),
        Some(&blindness_doc),
        Some(&decision),
    )
    .expect("packs assemble");
    assert_eq!(assembled.len(), 1);
    let pack = &assembled[0].1;
    assert!(validate_pack(
        pack,
        &cl_packs,
        Some(&canary_doc),
        Some(&blindness_doc),
        Some(&decision),
    )
    .expect("assembled pack validates"));

    let pack_path = dir.join("001-unit-checks.json");
    std::fs::write(&pack_path, render_json(pack)).expect("pack writes");
    let raw_bytes = read_file_bytes(&pack_path, "written pack").expect("pack bytes read");
    assert!(!raw_bytes.is_empty());
    let disk_pack = read_json_file(&pack_path, "written pack").expect("written pack parses");
    assert_eq!(&disk_pack, pack);

    let floor = recompute_floor(&disk_pack).expect("floor recomputes");
    assert_eq!(floor.get("meets_floor"), Some(&JsonValue::Bool(true)));
    let (record, body) = generate_ruling(
        &disk_pack,
        "keep",
        None,
        Some(&JsonValue::Text("2026-03-01".to_string())),
    )
    .expect("ruling generates");
    assert_eq!(body, render_body(&disk_pack, &record));
    assert!(validate_ruling(&record, &disk_pack).expect("ruling validates"));
    assert!(validate_ruling_body(body.as_bytes(), &record, &disk_pack).expect("body validates"));

    let cl_file = dir.join("cl-pack.json");
    let out_packs = dir.join("packs");
    std::fs::write(&cl_file, render_json(&cl_pack)).expect("cl file writes");
    let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
        .arg("assemble")
        .arg(format!("--catchledger={}", cl_file.display()))
        .arg(format!("--packs-dir={}", out_packs.display()))
        .output()
        .expect("tameshi runs");
    assert_eq!(output.status.code(), Some(0));
    assert!(out_packs.join("001-unit-checks.json").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(windows)]
#[test]
fn windows_inline_flag_preserves_unpaired_utf16_surrogate_path() {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};

    let dir = scratch_dir("wide-surrogate");
    let mut wide_name: Vec<u16> = "cl-".encode_utf16().collect();
    wide_name.push(0xD800);
    wide_name.extend(".json".encode_utf16());
    let cl_file = dir.join(OsString::from_wide(&wide_name));
    let out_packs = dir.join("packs");
    std::fs::write(
        &cl_file,
        "{\"format\": \"catchledger-pack/1\", \"repo\": \"synthetic/demo\", \
         \"check\": \"unit-checks\", \
         \"scorecard\": {\"fires\": 1, \"blocks\": 0, \"judged_commits\": 1, \
         \"flaky_commits\": 0, \"flake_rate\": 0.0, \"runtime_seconds\": 10, \
         \"runtime_minutes\": 0.166667, \"catches\": 0, \"false_blocks\": 0, \
         \"unattributed_blocks\": 0, \"false_block_rate\": null, \
         \"total_cost_usd\": 0.001333, \"cost_per_catch_usd\": null}, \
         \"catch_record\": [], \"canaryfield_results\": null, \
         \"false_block_rate\": null, \"pricing\": {}, \"source\": {}}\n",
    )
    .expect("surrogate-named pack writes on NTFS");

    let mut arg: Vec<u16> = "--catchledger=".encode_utf16().collect();
    arg.extend(cl_file.as_os_str().encode_wide());
    let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
        .arg("assemble")
        .arg(OsString::from_wide(&arg))
        .arg(format!("--packs-dir={}", out_packs.display()))
        .output()
        .expect("tameshi runs");
    assert_eq!(output.status.code(), Some(0));
    assert!(out_packs.join("001-unit-checks.json").exists());
    let _ = std::fs::remove_dir_all(&dir);
}
