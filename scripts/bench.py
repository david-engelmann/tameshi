#!/usr/bin/env python3
"""Reproducible synthetic benchmark harness for tameshi."""
import argparse, json, pathlib, shutil, statistics, subprocess, tempfile, time

def median_ms(fn, repeats=5):
    fn()
    samples = []
    for _ in range(repeats):
        t0 = time.perf_counter()
        fn()
        samples.append((time.perf_counter() - t0) * 1000.0)
    return statistics.median(samples)

def main():
    ap = argparse.ArgumentParser(description="Run synthetic tameshi benchmarks")
    ap.add_argument("--bin", default="target/release/tameshi", help="Path to tameshi binary")
    args = ap.parse_args()
    bin_path = str(pathlib.Path(args.bin).resolve())

    print("| Workload | assemble (ms) | rule (ms) |")
    print("| --- | ---: | ---: |")
    with tempfile.TemporaryDirectory() as tmp:
        tdir = pathlib.Path(tmp)
        for n_checks, n_seeds in [(10, 40), (50, 100), (200, 200)]:
            cl_p = tdir / f"cl-{n_checks}.json"
            cf_p = tdir / f"cf-{n_checks}.json"
            pdir = tdir / f"packs-{n_checks}"
            rdir = tdir / f"rule-{n_checks}"
            gates = [f"gate-{c:03d}" for c in range(n_checks)]
            checks = [{
                "name": g,
                "canaryfield_results": None,
                "evidence_pack_file": f"{c + 1:03d}-pack.json",
                "scorecard": {
                    "blocks": 6, "catches": 3, "cost_per_catch_usd": 0.04,
                    "false_block_rate": 0.166667, "false_blocks": 1, "fires": 40,
                    "flake_rate": 0.05, "flaky_commits": 1, "judged_commits": 20,
                    "runtime_minutes": 15.0, "runtime_seconds": 900,
                    "total_cost_usd": 0.12, "unattributed_blocks": 2,
                },
                "catch_record": [
                    {
                        "block_run_id": f"{c * 10 + 1}", "blocked_commit": f"{1:040x}",
                        "started_at": "2026-01-01T00:10:00Z", "flagged_paths": ["src/lib.rs"],
                        "outcome": "catch",
                        "evidence": {"fix_commits": [f"{2:040x}"], "reason": None, "type": "flagged_fix_then_success", "validating_run_id": f"{c * 10 + 2}"},
                    },
                    {
                        "block_run_id": f"{c * 10 + 3}", "blocked_commit": f"{3:040x}",
                        "started_at": "2026-01-20T00:10:00Z", "flagged_paths": ["src/lib.rs"],
                        "outcome": "catch",
                        "evidence": {"fix_commits": [f"{4:040x}"], "reason": None, "type": "flagged_fix_then_success", "validating_run_id": f"{c * 10 + 4}"},
                    },
                ],
            } for c, g in enumerate(gates)]
            rows = [{
                "seed": f"seed-{s:04d}", "name": f"synthetic defect {s}",
                "defect_class": "logic", "planted_files": ["src/lib.rs"],
                "gates": {g: ("caught" if (s + idx) % 3 != 0 else "waved-through") for idx, g in enumerate(gates)},
            } for s in range(n_seeds)]
            cl_p.write_text(json.dumps({
                "format": "catchledger-ledger/1", "repo": "synthetic-bench",
                "pricing": {"cost_per_minute": "0.008", "currency": "USD"},
                "source": {"canaryfield_results_sha256": None, "ci_history_sha256": "a" * 64, "git_history_sha256": "b" * 64},
                "history_window": {"first_run_started_at": "2026-01-01T00:10:00Z", "last_run_updated_at": "2026-01-20T23:50:00Z"},
                "totals": {}, "checks": checks,
            }), encoding="utf-8")
            cf_p.write_text(json.dumps({"gates": gates, "rows": rows}), encoding="utf-8")

            def do_asm():
                shutil.rmtree(pdir, ignore_errors=True)
                pdir.mkdir(parents=True, exist_ok=True)
                subprocess.run([bin_path, "assemble", "--catchledger", str(cl_p), "--canaryfield", str(cf_p), "--packs-dir", str(pdir)], check=True, capture_output=True)

            a_ms = median_ms(do_asm)
            rdir.mkdir(parents=True, exist_ok=True)
            r_ms = median_ms(lambda: subprocess.run([bin_path, "rule", "--pack", str(pdir / "001-gate-000.json"), "--ruling", "quarantine", "--expires", "2027-03-01", "--today", "2027-01-30", "--out", str(rdir)], check=True, capture_output=True))
            print(f"| {n_checks} checks / {n_seeds} seeds | {a_ms:.2f} | {r_ms:.2f} |")

if __name__ == "__main__":
    main()
