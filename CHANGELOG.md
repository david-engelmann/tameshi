# Changelog

All notable changes to tameshi are recorded in this file.

## Unreleased

- Align `Cargo.toml` package metadata (`version = "0.34.0"`, `rust-version = "1.61"`, `repository`, `homepage`, `documentation`, `readme`, `keywords`, `categories`, `binstall`) and `.crate` archive `exclude` rules (reducing `.crate` payload size by excluding `tests/`).
- Add storefront installation (with explicit `crates.io` third-party name collision guidance), quickstart, and lifecycle workflow sections in `README.md`, full six-document mdBook handbook under `docs/` and `book/`, bespoke dark and light SVG architecture diagrams under `docs/media/`, rate-limit-safe checksum-verifying `install.sh`, `SECURITY.md`, `CONTRIBUTING.md`, `CITATION.cff`, `deny.toml`, `.editorconfig`, `.gitattributes`, GitHub issue templates, and `.github/workflows/{release.yml,docs.yml,clean-install.yml}`.

## 0.34.0 on 2026-11-27

- Complete 37 closed verification rounds covering `tameshi-pack/1` and `tameshi-pack/2` evidence pack assembly from `catchledger`, `canaryfield`, `blindgate`, and `tameshi-decision/1` inputs, strict 30-defect and 14-day evidence floor accounting, unsigned `tameshi-ruling/1` and pull-request body generation across `keep`, `demote`, `quarantine`, and `kill` rulings, and four CLI doors (`assemble`, `rule`, `validate`, `validate-ruling`).
