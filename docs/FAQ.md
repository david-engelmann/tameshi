# Tameshi FAQ

## Why must I install from GitHub instead of running `cargo install tameshi`?

An unrelated third-party crate already occupies the name `tameshi` on `crates.io`. Always install with `cargo install --git https://github.com/david-engelmann/tameshi --locked`, `install.sh`, or `cargo binstall --git https://github.com/david-engelmann/tameshi tameshi`.

## Why did `tameshi rule` refuse my pack as below floor?

No lifecycle ruling can issue until a check satisfies **both** legs of the evidence floor

1. At least `30` planted defects scored by `canaryfield` (`caught`, `waved-through`, `gate-broken`, or `timed-out`).
2. At least `14` UTC calendar days between the earliest and latest entries in that check's own `catch_record` from `catchledger`.

If a check has fewer than 30 scored canary defects or fewer than 14 days of recorded blocks in `catch_record`, `tameshi assemble` still writes the pack with `"below_floor": true` (recording `canary_shortfall` and `ci_history_shortfall` in `floor`) so you can inspect the exact shortfall, and `tameshi rule` refuses to generate a ruling until sufficient history and canary runs accumulate.

## When is `--ruling kill` allowed?

`keep`, `demote` (emitted in `tameshi-ruling/1` as `"demote-to-advisory"`), and `quarantine` (with `--expires YYYY-MM-DD`) can be requested via `--ruling` on any pack meeting the floor (`tameshi-pack/1` or `tameshi-pack/2`). By contract, `--ruling kill` is refused unless the pack is a `tameshi-pack/2` pack carrying a `recommendation` object whose `"recommendation"` is `"kill"` alongside `below_floor: false`.

## Does Tameshi edit branch protection or delete workflows?

No. Tameshi makes zero network calls and deletes nothing on disk. It emits `ruling-<slug>.json` and `ruling-<slug>.md` locally so a human reviewer can sign `ruling-<slug>.json`, open a pull request with the full evidence table, and apply the branch-protection or quarantine change under normal code review.

## Why did `tameshi validate-ruling` fail after human sign-off or Markdown edits?

Two rules govern `validate-ruling`. First, in `ruling-<slug>.json`, human sign-off must update `"signature"` from `{"status": "unsigned", "signed_by": null, "signed_at": null}` to `{"status": "signed", "signed_by": "<reviewer>", "signed_at": "<timestamp>"}` (changing `signed_by` without setting `status` to `"signed"` is refused). Second, `validate-ruling --body` compares `ruling-<slug>.md` byte for byte against the canonical body rendered from the pack and ruling record, so keep `ruling-<slug>.md` verbatim when passing `--body` or omit `--body` when checking JSON fidelity alone.
