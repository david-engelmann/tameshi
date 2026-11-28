# Tameshi Operator Guide

Tameshi is an offline, zero-dependency Rust CLI and library that gives CI checks an evidenced lifecycle (`keep`, `demote`, `quarantine`, and model-backed `kill`). It assembles local outputs from `catchledger`, `canaryfield`, and `blindgate`, refuses to issue any ruling below the 30-defect and 14-day evidence floor, deletes nothing on disk, and emits unsigned pull-request records for human sign-off.

## Installation

Important package-registry notice. Do **not** run bare `cargo install tameshi` from `crates.io`, because the `tameshi` name on `crates.io` belongs to an unrelated third-party library. Always install `david-engelmann/tameshi` directly from this GitHub repository or via `install.sh`.

### Build from GitHub with Cargo

Requires Rust `1.61` or newer on Linux or macOS.

```sh
cargo install --git https://github.com/david-engelmann/tameshi --locked
```

### Prebuilt Release Archives

On Linux and macOS (`x86_64` and `aarch64`), install checksum-verified prebuilt archives via `install.sh` or download prebuilt archives via `cargo-binstall --git` once release assets are attached (`v0.34.0` and `v0.35.0` onward).

```sh
curl -fsSL https://raw.githubusercontent.com/david-engelmann/tameshi/main/install.sh | sh
export PATH="$HOME/.local/bin:$PATH"
```

```sh
cargo binstall --git https://github.com/david-engelmann/tameshi tameshi
```

## All Four CLI Doors

```text
tameshi assemble --catchledger PATH [--canaryfield PATH] [--blindgate PATH] [--decision PATH] --packs-dir DIR
tameshi rule --pack PATH --ruling WORD [--expires DATE] [--today DATE] --out DIR
tameshi validate --pack PATH --catchledger PATH [--canaryfield PATH] [--blindgate PATH] [--decision PATH]
tameshi validate-ruling --pack PATH --ruling PATH [--body PATH]
```

Every door exits `0` in silence on success, exits `2` on CLI flag usage errors, and exits `1` on domain or validation refusals (such as duplicate JSON keys, malformed inputs, floor shortfalls, or unfaithful ruling fields).

## Step 1 Assembling Evidence Packs

Pass either a single-check `catchledger-pack/1` file or a multi-check `catchledger-ledger/1` file to `tameshi assemble`, optionally joining `canaryfield` defect scores, `blindgate-evidence/1` blindness traces, and a `tameshi-decision/1` model recommendation.

```sh
mkdir -p /tmp/tm-demo/packs
tameshi assemble \
  --catchledger examples/synthetic-catchledger-ledger.json \
  --canaryfield examples/synthetic-canaryfield-results.json \
  --blindgate examples/synthetic-blindgate-evidence.json \
  --packs-dir /tmp/tm-demo/packs
```

Tameshi writes numbered pack files in check order (`001-style-checks.json` and `002-unit-checks.json`). Without `--decision`, assembled packs carry format `tameshi-pack/1`. When `--decision` is supplied, the pack matching the decision's `check` carries format `tameshi-pack/2` (embedding `recommendation`) while any other checks in a multi-check ledger emit `tameshi-pack/1`.

### How the Evidence Floor Works

Every assembled pack records boolean `below_floor` (`false` or `true`) and a seven-field `floor` object (`meets_floor`, `canary_defects`, `required_canary_defects`, `canary_shortfall`, `ci_history_days`, `required_ci_history_days`, `ci_history_shortfall`) across two mandatory legs

1. **Planted defect leg** At least `30` `canaryfield`-scored planted defects (`caught`, `waved-through`, `gate-broken`, or `timed-out`, excluding `not-applicable`).
2. **CI history leg** At least `14` UTC calendar days spanned by the check's own `catch_record` timestamps in `catchledger`.

Packs below either leg still assemble cleanly (`"below_floor": true`) so teams can inspect what evidence is missing, and `tameshi rule` refuses to issue a ruling on any `below_floor` pack while naming every failed leg and its exact shortfall.

## Step 2 Generating an Unsigned Ruling

Run `tameshi rule` against a pack where `below_floor` is `false` (`002-unit-checks.json`). Supported `--ruling` CLI words are `keep`, `demote` (which emits `"ruling": "demote-to-advisory"` and `"state_after_ruling": "advisory"` in `tameshi-ruling/1`), `quarantine` (which unconditionally requires `--expires YYYY-MM-DD`), and `kill` (permitted only on a `tameshi-pack/2` pack whose `recommendation.recommendation` equals `"kill"`).

```sh
mkdir -p /tmp/tm-demo/rulings
tameshi rule \
  --pack /tmp/tm-demo/packs/002-unit-checks.json \
  --ruling quarantine \
  --expires 2027-03-01 \
  --today 2027-01-30 \
  --out /tmp/tm-demo/rulings
```

This writes two files inside `--out`

- `ruling-unit-checks.json` (`tameshi-ruling/1`) with `"signature": {"status": "unsigned", "signed_by": null, "signed_at": null}`.
- `ruling-unit-checks.md`, a ready-to-open pull request body whose receipt table derives every figure from the pack.

## Step 3 Human Sign-Off and CI Validation

Tameshi never signs rulings automatically. After reviewing the receipts, a human operator updates the nested `signature` object in `ruling-<slug>.json` outside the tool so `"status"` becomes `"signed"` and both `"signed_by"` and `"signed_at"` hold non-empty strings, then opens the ruling pull request.

Important validation note. `tameshi validate-ruling --body PATH` compares the Markdown file byte for byte against the canonical body rendered from the pack and ruling record, so leave `ruling-<slug>.md` unedited if passing `--body` in CI, and record human sign-off inside `ruling-<slug>.json`'s `signature` block (`status` set to `"signed"` with non-empty `signed_by` and `signed_at`). Both the generated outputs in `/tmp/tm-demo` and the committed reference files under `examples/` can be validated directly.

```sh
tameshi validate \
  --pack /tmp/tm-demo/packs/002-unit-checks.json \
  --catchledger examples/synthetic-catchledger-ledger.json \
  --canaryfield examples/synthetic-canaryfield-results.json \
  --blindgate examples/synthetic-blindgate-evidence.json

tameshi validate-ruling \
  --pack /tmp/tm-demo/packs/002-unit-checks.json \
  --ruling /tmp/tm-demo/rulings/ruling-unit-checks.json \
  --body /tmp/tm-demo/rulings/ruling-unit-checks.md
```

## GitHub Actions Ruling Audit Workflow

Verify every committed ruling JSON and Markdown body in `examples/` (or your repository's rulings directory) against its matching evidence pack on pull requests.

```yaml
name: verify-check-rulings

on:
  pull_request:

permissions:
  contents: read

jobs:
  validate-ruling:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install tameshi from GitHub
        run: cargo install --git https://github.com/david-engelmann/tameshi --locked
      - name: Verify committed ruling records against their evidence packs
        run: |
          set -eu
          for ruling in examples/ruling-*.json; do
            slug="${ruling#examples/ruling-}"
            slug="${slug%.json}"
            pack="$(ls examples/packs/[0-9][0-9][0-9]-"${slug}".json)"
            body="examples/ruling-${slug}.md"
            tameshi validate-ruling --pack "$pack" --ruling "$ruling" --body "$body"
          done
```
