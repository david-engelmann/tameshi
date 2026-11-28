# Tameshi Wire Formats

Tameshi reads `catchledger`, `canaryfield`, `blindgate`, and `tameshi-decision/1` JSON files and writes `tameshi-pack/1`, `tameshi-pack/2`, `tameshi-ruling/1`, and deterministic pull-request Markdown bodies. Every JSON reader refuses duplicate object keys at any nesting depth and enforces a `4000` nesting bound.

## 1. `tameshi-pack/1` and `tameshi-pack/2`

Written by `tameshi assemble` to `<NNN>-<slug>.json` inside `--packs-dir` and verified by `tameshi validate`.

- `format` equals `"tameshi-pack/1"` when assembled without a matching decision, and `"tameshi-pack/2"` when assembled with a matching `tameshi-decision/1` document.
- `repo` and `check` identify the repository label and check name from `catchledger`.
- `below_floor` holds boolean `false` when both floor legs pass and `true` otherwise.
- `floor` holds seven exact keys (`required_canary_defects` `30`, `required_ci_history_days` `14`, `canary_defects`, `ci_history_days`, `canary_shortfall`, `ci_history_shortfall`, and boolean `meets_floor`).
- `scorecard`, `catch_record`, `history_window`, `false_block_rate`, and `cost_per_catch_usd` carry the verified `catchledger` figures and block history.
- `canary_performance` carries `gate`, `counts` (with hyphenated keys `"caught"`, `"waved-through"`, `"gate-broken"`, and `"not-applicable"`), `scored` (including `"timed-out"` rows when present), `catch_rate`, and `results`.
- `blindness_traces` carries matching check entries from `blindgate-evidence/1` when `--blindgate` is supplied, and `sources` records input SHA-256 digests and provenance metadata.
- `recommendation` (`tameshi-pack/2` only) embeds the four-field object (`recommendation`, `rationale`, `model`, `run`) validated from `--decision`.

## 2. `tameshi-decision/1` (`--decision`)

Recorded model recommendation input accepted by `tameshi assemble` and `tameshi validate`.

```json
{
  "check": "unit checks",
  "format": "tameshi-decision/1",
  "model": "jev-1.13.0",
  "rationale": "The model chose keep with confidence 0.52 over the pack evidence.",
  "recommendation": "keep",
  "run": "round-5-live-1"
}
```

### Field Rules

- `format` must equal `"tameshi-decision/1"`.
- `check` must name a non-empty check present in `--catchledger`.
- `recommendation` must be one of `"keep"`, `"demote"`, `"quarantine"`, or `"kill"`.

## 3. `tameshi-ruling/1` (`ruling-<slug>.json`)

Unsigned ruling record emitted by `tameshi rule` and verified against its source pack by `tameshi validate-ruling`.

```json
{
  "check": "unit checks",
  "expiry": "2027-03-01",
  "floor": {
    "canary_defects": 30,
    "canary_shortfall": 0,
    "ci_history_days": 14,
    "ci_history_shortfall": 0,
    "meets_floor": true,
    "required_canary_defects": 30,
    "required_ci_history_days": 14
  },
  "format": "tameshi-ruling/1",
  "repo": "synthetic-repo",
  "ruling": "quarantine",
  "signature": {
    "signed_at": null,
    "signed_by": null,
    "status": "unsigned"
  },
  "state_after_ruling": "quarantined"
}
```

When ruled from a `tameshi-pack/2` pack, `tameshi-ruling/1` also carries the pack's `recommendation` object alongside `floor`. `ruling` and `state_after_ruling` stand in one of four pairs (`"keep"` / `"required"`, `"demote-to-advisory"` / `"advisory"`, `"quarantine"` / `"quarantined"`, or `"kill"` / `"removed"`), and `expiry` holds a `"YYYY-MM-DD"` date on `"quarantine"` and `null` on all other verdicts.

### Signature Consistency Rule

`tameshi rule` always emits `"signature": {"status": "unsigned", "signed_by": null, "signed_at": null}`. Under `tameshi validate-ruling`, `signature.status` must be either `"unsigned"` (with `signed_by` and `signed_at` both `null`) or `"signed"` (with `signed_by` and `signed_at` both non-empty strings), while every `repo`, `check`, `floor`, `ruling`, `state_after_ruling`, `expiry`, and optional `recommendation` field must match the source pack.
