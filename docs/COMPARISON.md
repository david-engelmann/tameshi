# Tameshi Comparison

In most repositories, required status checks are one-way doors. Once added to branch protection, slow or flaky checks stay forever because nobody has an audited process to demote or quarantine them. Tameshi turns check governance into a reproducible, floor-gated pull-request workflow.

## Capability Matrix

| Approach | Primary target | Enforces minimum evidence floor? | Lifecycle states beyond on or off? | Human sign-off preserved? | Offline & deterministic? |
| --- | --- | --- | --- | --- | --- |
| **tameshi** | CI check lifecycle rulings | Yes (`30` defects + `14` CI days) | `keep`, `demote` (`advisory`), `quarantine` (dated), `kill` | Yes (emits unsigned PR records) | Yes (`tameshi-pack/1,2` + `tameshi-ruling/1`) |
| GitHub branch protection UI | Toggle required status checks | No | Required or absent only | Manual click, no evidence receipt | Cloud UI |
| Ad-hoc workflow deletion PRs | Delete slow `.github/workflows` jobs | No | Deleted immediately | Code review without structured floor | Manual |
| **verdictloop** (sibling portfolio repo) | Rulings on individual code *changes* | Change-level evidence | Merge or hold per change | Change-level workflow | Yes |
| **greenharvest** (sibling Gate Truth repo) | Individual *test cases* inside one suite | Unique-catcher holdout block | Surviving vs `suitediet-quarantine/1` | Deletes nothing | Yes |

## Boundary Within the Gate Truth Suite

- **Tameshi judges checks, verdictloop judges changes.** Verdictloop evaluates whether a specific pull request or patch should land, whereas Tameshi evaluates whether a *gate itself* earns required status on `main`.
- **Greenharvest diets tests inside a suite, Tameshi governs whole checks.** Use Greenharvest to quarantine redundant test cases inside a test runner under a millisecond budget, and use Tameshi to rule on entire status checks (`unit checks`, `style checks`, `lint`) using `catchledger`, `canaryfield`, and `blindgate` packs.
