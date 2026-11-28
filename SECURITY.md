# Security Policy

## Supported Versions

Security fixes land on the latest `0.34.x` and subsequent `0.x` release line on `main`. Earlier tags do not receive backported patches.

## Security Model

Tameshi makes zero network calls, spawns zero external processes, and deletes nothing from disk. It reads local `catchledger`, `canaryfield`, `blindgate`, and `tameshi-decision/1` JSON files under strict duplicate-key refusal and a `4000` nesting bound, enforces the mandatory two-leg evidence floor (`30` `canaryfield`-scored planted defects and `14` UTC days of `catchledger` CI history) before any ruling can issue, and emits *unsigned* ruling records (`signature` block with `status` set to `"unsigned"`, `signed_by` set to `null`, and `signed_at` set to `null`) so human sign-off always remains outside the tool. Never commit real repository payloads, usernames, or private paths into test fixtures.

## Reporting a Vulnerability

Do not open a public issue for a suspected security vulnerability. Report vulnerabilities privately through [GitHub Security Advisories](https://github.com/david-engelmann/tameshi/security/advisories/new) with synthetic inputs only, naming the affected version and the relevant section of `CONTRACT.md`. Every report receives an initial review within seven days, and confirmed fixes ship with a regression pin in the test suite before public disclosure.
