//! Tameshi assembles per-check evidence packs and generates
//! floor-enforced, unsigned ruling records for human sign-off and
//! pull request receipts from local files.
//!
//! Catchledger packs or ledgers read through `ingest` alongside
//! optional `canaryfield` catch tables, `blindgate` blindness
//! traces, and `tameshi-decision/1` recommendation records. The
//! `packs` module computes floor accounting against the 30-defect
//! and 14-day floor, marks packs below the floor without discarding
//! them, and validates assembled `tameshi-pack/1` and
//! `tameshi-pack/2` documents. The `rulings` module recomputes the
//! floor from raw pack evidence, refuses below-floor packs, enforces
//! the kill recommendation gate, emits unsigned `tameshi-ruling/1`
//! records and deterministic Markdown pull request bodies, and
//! validates signed or unsigned rulings and bodies against their
//! originating packs.

pub mod dates;
pub mod errors;
pub mod ingest;
pub mod jsonvalue;
pub mod names;
pub mod packs;
pub mod reader;
pub mod render;
pub mod rulings;

pub use errors::{ErrorKind, Result, TameshiError};
pub use ingest::{
    normalize_canary_document, read_blindness_traces, read_catchledger_packs,
    validate_canary_document, BLINDNESS_FORMAT, CANARY_SOURCE_FORMAT, CATCHLEDGER_LEDGER_FORMAT,
    CATCHLEDGER_PACK_FORMAT,
};
pub use jsonvalue::JsonValue;
pub use names::slugify_check;
pub use packs::{
    assemble_packs, canary_performance, catch_record_days, floor_accounting, read_decision,
    recompute_floor, validate_canary_performance, validate_pack, DECISION_FORMAT, PACK_FORMAT,
    PACK_FORMAT_2, RECOMMENDATION_FIELDS, RECOMMENDATION_WORDS, REQUIRED_CANARY_DEFECTS,
    REQUIRED_CI_HISTORY_DAYS,
};
pub use reader::{read_file_bytes, read_json_document, read_json_file};
pub use render::render_json;
pub use rulings::{
    generate_ruling, render_body, ruling_slug, validate_ruling, validate_ruling_body, RULING_FORMAT,
};
