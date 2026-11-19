//! Owner discharge diag for tameshi Round 14.

const PACK: &str = r##"{"below_floor":false,"blindness_traces":[{"check":"unit checks","detail":"synthetic trace","observation":"read","surface":"settings file"},{"check":"unit checks","detail":"second synthetic trace","observation":"read","surface":"settings file"}],"canary_performance":{"catch_rate":0.666667,"counts":{"caught":20,"gate-broken":4,"not-applicable":3,"waved-through":6},"gate":"unit checks","results":[{"defect_class":"synthetic class","name":"synthetic defect 0","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-00"},{"defect_class":"synthetic class","name":"synthetic defect 1","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-01"},{"defect_class":"synthetic class","name":"synthetic defect 2","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-02"},{"defect_class":"synthetic class","name":"synthetic defect 3","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-03"},{"defect_class":"synthetic class","name":"synthetic defect 4","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-04"},{"defect_class":"synthetic class","name":"synthetic defect 5","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-05"},{"defect_class":"synthetic class","name":"synthetic defect 6","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-06"},{"defect_class":"synthetic class","name":"synthetic defect 7","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-07"},{"defect_class":"synthetic class","name":"synthetic defect 8","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-08"},{"defect_class":"synthetic class","name":"synthetic defect 9","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-09"},{"defect_class":"synthetic class","name":"synthetic defect 10","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-10"},{"defect_class":"synthetic class","name":"synthetic defect 11","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-11"},{"defect_class":"synthetic class","name":"synthetic defect 12","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-12"},{"defect_class":"synthetic class","name":"synthetic defect 13","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-13"},{"defect_class":"synthetic class","name":"synthetic defect 14","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-14"},{"defect_class":"synthetic class","name":"synthetic defect 15","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-15"},{"defect_class":"synthetic class","name":"synthetic defect 16","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-16"},{"defect_class":"synthetic class","name":"synthetic defect 17","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-17"},{"defect_class":"synthetic class","name":"synthetic defect 18","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-18"},{"defect_class":"synthetic class","name":"synthetic defect 19","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-19"},{"defect_class":"synthetic class","name":"synthetic defect 20","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-20"},{"defect_class":"synthetic class","name":"synthetic defect 21","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-21"},{"defect_class":"synthetic class","name":"synthetic defect 22","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-22"},{"defect_class":"synthetic class","name":"synthetic defect 23","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-23"},{"defect_class":"synthetic class","name":"synthetic defect 24","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-24"},{"defect_class":"synthetic class","name":"synthetic defect 25","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-25"},{"defect_class":"synthetic class","name":"synthetic defect 26","planted_files":["src/synthetic.py"],"result":"gate-broken","seed":"seed-26"},{"defect_class":"synthetic class","name":"synthetic defect 27","planted_files":["src/synthetic.py"],"result":"gate-broken","seed":"seed-27"},{"defect_class":"synthetic class","name":"synthetic defect 28","planted_files":["src/synthetic.py"],"result":"gate-broken","seed":"seed-28"},{"defect_class":"synthetic class","name":"synthetic defect 29","planted_files":["src/synthetic.py"],"result":"gate-broken","seed":"seed-29"},{"defect_class":"synthetic class","name":"synthetic defect 30","planted_files":["src/synthetic.py"],"result":"not-applicable","seed":"seed-30"},{"defect_class":"synthetic class","name":"synthetic defect 31","planted_files":["src/synthetic.py"],"result":"not-applicable","seed":"seed-31"},{"defect_class":"synthetic class","name":"synthetic defect 32","planted_files":["src/synthetic.py"],"result":"not-applicable","seed":"seed-32"}],"scored":30},"catch_record":[{"block_run_id":"100","blocked_commit":"0000000000000000000000000000000000000001","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-01T00:10:00Z"},{"block_run_id":"101","blocked_commit":"0000000000000000000000000000000000000002","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-02T00:11:00Z"},{"block_run_id":"102","blocked_commit":"0000000000000000000000000000000000000003","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-03T00:12:00Z"},{"block_run_id":"103","blocked_commit":"0000000000000000000000000000000000000004","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-04T00:13:00Z"},{"block_run_id":"104","blocked_commit":"0000000000000000000000000000000000000005","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-05T00:14:00Z"},{"block_run_id":"105","blocked_commit":"0000000000000000000000000000000000000006","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-06T00:15:00Z"},{"block_run_id":"106","blocked_commit":"0000000000000000000000000000000000000007","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-07T00:16:00Z"},{"block_run_id":"107","blocked_commit":"0000000000000000000000000000000000000008","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-08T00:17:00Z"},{"block_run_id":"108","blocked_commit":"0000000000000000000000000000000000000009","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-09T00:18:00Z"},{"block_run_id":"109","blocked_commit":"000000000000000000000000000000000000000a","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-10T00:19:00Z"},{"block_run_id":"110","blocked_commit":"000000000000000000000000000000000000000b","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-11T00:20:00Z"},{"block_run_id":"111","blocked_commit":"000000000000000000000000000000000000000c","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-12T00:21:00Z"},{"block_run_id":"112","blocked_commit":"000000000000000000000000000000000000000d","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-13T00:22:00Z"},{"block_run_id":"113","blocked_commit":"000000000000000000000000000000000000000e","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-14T00:23:00Z"}],"check":"unit checks","cost_per_catch_usd":0.04,"false_block_rate":0.166667,"floor":{"canary_defects":30,"canary_shortfall":0,"ci_history_days":14,"ci_history_shortfall":0,"meets_floor":true,"required_canary_defects":30,"required_ci_history_days":14},"format":"tameshi-pack/1","history_window":{"first_run_started_at":"2026-01-01T00:10:00Z","last_run_updated_at":"2026-01-14T23:50:00Z"},"repo":"synthetic-repo","scorecard":{"blocks":6,"catches":3,"cost_per_catch_usd":0.04,"false_block_rate":0.166667,"false_blocks":1,"fires":40,"flake_rate":0.05,"flaky_commits":1,"judged_commits":20,"runtime_minutes":15.0,"runtime_seconds":900,"total_cost_usd":0.12,"unattributed_blocks":2},"sources":{"blindgate_evidence_supplied":true,"blindgate_unmatched_trace_count":0,"canary_origin":"direct","catchledger_source":{"canaryfield_results_sha256":null,"ci_history_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","git_history_sha256":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}}}"##;

const RULING: &str = r##"{"check":"unit checks","expiry":null,"floor":{"canary_defects":30,"canary_shortfall":0,"ci_history_days":14,"ci_history_shortfall":0,"meets_floor":true,"required_canary_defects":30,"required_ci_history_days":14},"format":"tameshi-ruling/1","repo":"synthetic-repo","ruling":"keep","signature":{"signed_at":null,"signed_by":null,"status":"unsigned"},"state_after_ruling":"required"}"##;

const VPACK: &str = r##"{"below_floor":true,"blindness_traces":[],"canary_performance":{"catch_rate":1.0,"counts":{"caught":3,"gate-broken":0,"not-applicable":0,"waved-through":0},"gate":"unit checks","results":[{"defect_class":"synthetic class","name":"synthetic defect 0","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-00"},{"defect_class":"synthetic class","name":"synthetic defect 1","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-01"},{"defect_class":"synthetic class","name":"synthetic defect 2","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-02"}],"scored":3},"catch_record":[{"block_run_id":"100","blocked_commit":"0000000000000000000000000000000000000001","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-01T00:10:00Z"},{"block_run_id":"101","blocked_commit":"0000000000000000000000000000000000000002","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-02T00:11:00Z"},{"block_run_id":"102","blocked_commit":"0000000000000000000000000000000000000003","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-03T00:12:00Z"},{"block_run_id":"103","blocked_commit":"0000000000000000000000000000000000000004","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-04T00:13:00Z"},{"block_run_id":"104","blocked_commit":"0000000000000000000000000000000000000005","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-05T00:14:00Z"},{"block_run_id":"105","blocked_commit":"0000000000000000000000000000000000000006","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-06T00:15:00Z"},{"block_run_id":"106","blocked_commit":"0000000000000000000000000000000000000007","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-07T00:16:00Z"},{"block_run_id":"107","blocked_commit":"0000000000000000000000000000000000000008","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-08T00:17:00Z"},{"block_run_id":"108","blocked_commit":"0000000000000000000000000000000000000009","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-09T00:18:00Z"},{"block_run_id":"109","blocked_commit":"000000000000000000000000000000000000000a","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-10T00:19:00Z"},{"block_run_id":"110","blocked_commit":"000000000000000000000000000000000000000b","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-11T00:20:00Z"},{"block_run_id":"111","blocked_commit":"000000000000000000000000000000000000000c","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-12T00:21:00Z"},{"block_run_id":"112","blocked_commit":"000000000000000000000000000000000000000d","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-13T00:22:00Z"},{"block_run_id":"113","blocked_commit":"000000000000000000000000000000000000000e","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-14T00:23:00Z"}],"check":"unit checks","cost_per_catch_usd":0.04,"false_block_rate":0.166667,"floor":{"canary_defects":3,"canary_shortfall":27,"ci_history_days":14,"ci_history_shortfall":0,"meets_floor":false,"required_canary_defects":30,"required_ci_history_days":14},"format":"tameshi-pack/1","history_window":{"first_run_started_at":"2026-01-01T00:10:00Z","last_run_updated_at":"2026-01-14T23:50:00Z"},"repo":"synthetic-repo","scorecard":{"blocks":6,"catches":3,"cost_per_catch_usd":0.04,"false_block_rate":0.166667,"false_blocks":1,"fires":40,"flake_rate":0.05,"flaky_commits":1,"judged_commits":20,"runtime_minutes":15.0,"runtime_seconds":900,"total_cost_usd":0.12,"unattributed_blocks":2},"sources":{"blindgate_evidence_supplied":false,"blindgate_unmatched_trace_count":0,"canary_origin":"direct","catchledger_source":{"canaryfield_results_sha256":null,"ci_history_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","git_history_sha256":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}}}"##;

const VLEDGER: &str = r##"{"canaryfield_results":null,"catch_record":[{"block_run_id":"100","blocked_commit":"0000000000000000000000000000000000000001","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-01T00:10:00Z"},{"block_run_id":"101","blocked_commit":"0000000000000000000000000000000000000002","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-02T00:11:00Z"},{"block_run_id":"102","blocked_commit":"0000000000000000000000000000000000000003","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-03T00:12:00Z"},{"block_run_id":"103","blocked_commit":"0000000000000000000000000000000000000004","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-04T00:13:00Z"},{"block_run_id":"104","blocked_commit":"0000000000000000000000000000000000000005","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-05T00:14:00Z"},{"block_run_id":"105","blocked_commit":"0000000000000000000000000000000000000006","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-06T00:15:00Z"},{"block_run_id":"106","blocked_commit":"0000000000000000000000000000000000000007","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-07T00:16:00Z"},{"block_run_id":"107","blocked_commit":"0000000000000000000000000000000000000008","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-08T00:17:00Z"},{"block_run_id":"108","blocked_commit":"0000000000000000000000000000000000000009","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-09T00:18:00Z"},{"block_run_id":"109","blocked_commit":"000000000000000000000000000000000000000a","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-10T00:19:00Z"},{"block_run_id":"110","blocked_commit":"000000000000000000000000000000000000000b","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-11T00:20:00Z"},{"block_run_id":"111","blocked_commit":"000000000000000000000000000000000000000c","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-12T00:21:00Z"},{"block_run_id":"112","blocked_commit":"000000000000000000000000000000000000000d","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-13T00:22:00Z"},{"block_run_id":"113","blocked_commit":"000000000000000000000000000000000000000e","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-14T00:23:00Z"}],"check":"unit checks","false_block_rate":0.166667,"format":"catchledger-pack/1","history_window":{"first_run_started_at":"2026-01-01T00:10:00Z","last_run_updated_at":"2026-01-14T23:50:00Z"},"pricing":{"cost_per_minute":"0.008","currency":"USD"},"repo":"synthetic-repo","scorecard":{"blocks":6,"catches":3,"cost_per_catch_usd":0.04,"false_block_rate":0.166667,"false_blocks":1,"fires":40,"flake_rate":0.05,"flaky_commits":1,"judged_commits":20,"runtime_minutes":15.0,"runtime_seconds":900,"total_cost_usd":0.12,"unattributed_blocks":2},"source":{"canaryfield_results_sha256":null,"ci_history_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","git_history_sha256":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}}"##;

const VCANARY: &str = r##"{"gates":["unit checks"],"rows":[{"defect_class":"synthetic class","gates":{"unit checks":"caught"},"name":"synthetic defect 0","planted_file":"src/synthetic.py","planted_files":["src/synthetic.py"],"seed":"seed-00"},{"defect_class":"synthetic class","gates":{"unit checks":"caught"},"name":"synthetic defect 1","planted_file":"src/synthetic.py","planted_files":["src/synthetic.py"],"seed":"seed-01"},{"defect_class":"synthetic class","gates":{"unit checks":"caught"},"name":"synthetic defect 2","planted_file":"src/synthetic.py","planted_files":["src/synthetic.py"],"seed":"seed-02"}]}"##;

const PACK_NESTED: &str = r##"{"below_floor":false,"blindness_traces":[{"check":"unit checks","detail":"synthetic trace","observation":"read","surface":"settings file"},{"check":"unit checks","detail":"second synthetic trace","observation":"read","surface":"settings file"}],"canary_performance":{"catch_rate":0.666667,"counts":{"caught":20,"gate-broken":4,"not-applicable":3,"waved-through":6},"gate":"unit checks","results":[{"defect_class":"synthetic class","name":"synthetic defect 0","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-00"},{"defect_class":"synthetic class","name":"synthetic defect 1","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-01"},{"defect_class":"synthetic class","name":"synthetic defect 2","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-02"},{"defect_class":"synthetic class","name":"synthetic defect 3","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-03"},{"defect_class":"synthetic class","name":"synthetic defect 4","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-04"},{"defect_class":"synthetic class","name":"synthetic defect 5","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-05"},{"defect_class":"synthetic class","name":"synthetic defect 6","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-06"},{"defect_class":"synthetic class","name":"synthetic defect 7","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-07"},{"defect_class":"synthetic class","name":"synthetic defect 8","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-08"},{"defect_class":"synthetic class","name":"synthetic defect 9","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-09"},{"defect_class":"synthetic class","name":"synthetic defect 10","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-10"},{"defect_class":"synthetic class","name":"synthetic defect 11","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-11"},{"defect_class":"synthetic class","name":"synthetic defect 12","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-12"},{"defect_class":"synthetic class","name":"synthetic defect 13","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-13"},{"defect_class":"synthetic class","name":"synthetic defect 14","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-14"},{"defect_class":"synthetic class","name":"synthetic defect 15","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-15"},{"defect_class":"synthetic class","name":"synthetic defect 16","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-16"},{"defect_class":"synthetic class","name":"synthetic defect 17","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-17"},{"defect_class":"synthetic class","name":"synthetic defect 18","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-18"},{"defect_class":"synthetic class","name":"synthetic defect 19","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-19"},{"defect_class":"synthetic class","name":"synthetic defect 20","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-20"},{"defect_class":"synthetic class","name":"synthetic defect 21","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-21"},{"defect_class":"synthetic class","name":"synthetic defect 22","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-22"},{"defect_class":"synthetic class","name":"synthetic defect 23","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-23"},{"defect_class":"synthetic class","name":"synthetic defect 24","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-24"},{"defect_class":"synthetic class","name":"synthetic defect 25","planted_files":["src/synthetic.py"],"result":"waved-through","seed":"seed-25"},{"defect_class":"synthetic class","name":"synthetic defect 26","planted_files":["src/synthetic.py"],"result":"gate-broken","seed":"seed-26"},{"defect_class":"synthetic class","name":"synthetic defect 27","planted_files":["src/synthetic.py"],"result":"gate-broken","seed":"seed-27"},{"defect_class":"synthetic class","name":"synthetic defect 28","planted_files":["src/synthetic.py"],"result":"gate-broken","seed":"seed-28"},{"defect_class":"synthetic class","name":"synthetic defect 29","planted_files":["src/synthetic.py"],"result":"gate-broken","seed":"seed-29"},{"defect_class":"synthetic class","name":"synthetic defect 30","planted_files":["src/synthetic.py"],"result":"not-applicable","seed":"seed-30"},{"defect_class":"synthetic class","name":"synthetic defect 31","planted_files":["src/synthetic.py"],"result":"not-applicable","seed":"seed-31"},{"defect_class":"synthetic class","name":"synthetic defect 32","planted_files":["src/synthetic.py"],"result":"not-applicable","seed":"seed-32"}],"scored":30},"catch_record":[{"block_run_id":"100","blocked_commit":"0000000000000000000000000000000000000001","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-01T00:10:00Z"},{"block_run_id":"101","blocked_commit":"0000000000000000000000000000000000000002","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-02T00:11:00Z"},{"block_run_id":"102","blocked_commit":"0000000000000000000000000000000000000003","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-03T00:12:00Z"},{"block_run_id":"103","blocked_commit":"0000000000000000000000000000000000000004","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-04T00:13:00Z"},{"block_run_id":"104","blocked_commit":"0000000000000000000000000000000000000005","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-05T00:14:00Z"},{"block_run_id":"105","blocked_commit":"0000000000000000000000000000000000000006","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-06T00:15:00Z"},{"block_run_id":"106","blocked_commit":"0000000000000000000000000000000000000007","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-07T00:16:00Z"},{"block_run_id":"107","blocked_commit":"0000000000000000000000000000000000000008","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-08T00:17:00Z"},{"block_run_id":"108","blocked_commit":"0000000000000000000000000000000000000009","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-09T00:18:00Z"},{"block_run_id":"109","blocked_commit":"000000000000000000000000000000000000000a","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-10T00:19:00Z"},{"block_run_id":"110","blocked_commit":"000000000000000000000000000000000000000b","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-11T00:20:00Z"},{"block_run_id":"111","blocked_commit":"000000000000000000000000000000000000000c","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-12T00:21:00Z"},{"block_run_id":"112","blocked_commit":"000000000000000000000000000000000000000d","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-13T00:22:00Z"},{"block_run_id":"113","blocked_commit":"000000000000000000000000000000000000000e","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-14T00:23:00Z"}],"check":"unit checks","cost_per_catch_usd":0.04,"false_block_rate":0.166667,"floor":{"canary_defects":30,"canary_shortfall":0,"ci_history_days":14,"ci_history_shortfall":0,"meets_floor":true,"required_canary_defects":30,"required_ci_history_days":14},"format":"tameshi-pack/1","history_window":{"first_run_started_at":"2026-01-01T00:10:00Z","last_run_updated_at":"2026-01-14T23:50:00Z"},"repo":"synthetic-repo","scorecard":{"blocks":6,"catches":3,"cost_per_catch_usd":0.04,"false_block_rate":0.166667,"false_blocks":1,"fires":40,"flake_rate":0.05,"flaky_commits":1,"judged_commits":20,"runtime_minutes":15.0,"runtime_seconds":900,"total_cost_usd":0.12,"unattributed_blocks":2},"sources":{"blindgate_evidence_supplied":true,"blindgate_unmatched_trace_count":0,"blindness_traces":["a bare string standing where no trace reading looks"],"canary_origin":"direct","catchledger_source":{"canaryfield_results_sha256":null,"ci_history_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","git_history_sha256":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}}}"##;

const VPACK_NESTED: &str = r##"{"below_floor":true,"blindness_traces":[],"canary_performance":{"catch_rate":1.0,"counts":{"caught":3,"gate-broken":0,"not-applicable":0,"waved-through":0},"gate":"unit checks","results":[{"defect_class":"synthetic class","name":"synthetic defect 0","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-00"},{"defect_class":"synthetic class","name":"synthetic defect 1","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-01"},{"defect_class":"synthetic class","name":"synthetic defect 2","planted_files":["src/synthetic.py"],"result":"caught","seed":"seed-02"}],"scored":3},"catch_record":[{"block_run_id":"100","blocked_commit":"0000000000000000000000000000000000000001","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-01T00:10:00Z"},{"block_run_id":"101","blocked_commit":"0000000000000000000000000000000000000002","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-02T00:11:00Z"},{"block_run_id":"102","blocked_commit":"0000000000000000000000000000000000000003","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-03T00:12:00Z"},{"block_run_id":"103","blocked_commit":"0000000000000000000000000000000000000004","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-04T00:13:00Z"},{"block_run_id":"104","blocked_commit":"0000000000000000000000000000000000000005","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-05T00:14:00Z"},{"block_run_id":"105","blocked_commit":"0000000000000000000000000000000000000006","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-06T00:15:00Z"},{"block_run_id":"106","blocked_commit":"0000000000000000000000000000000000000007","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-07T00:16:00Z"},{"block_run_id":"107","blocked_commit":"0000000000000000000000000000000000000008","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-08T00:17:00Z"},{"block_run_id":"108","blocked_commit":"0000000000000000000000000000000000000009","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-09T00:18:00Z"},{"block_run_id":"109","blocked_commit":"000000000000000000000000000000000000000a","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-10T00:19:00Z"},{"block_run_id":"110","blocked_commit":"000000000000000000000000000000000000000b","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-11T00:20:00Z"},{"block_run_id":"111","blocked_commit":"000000000000000000000000000000000000000c","evidence":{"fix_commits":[],"reason":"no_later_outcome","type":"none","validating_run_id":null},"flagged_paths":["src/synthetic.py"],"outcome":"unattributed","started_at":"2026-01-12T00:21:00Z"},{"block_run_id":"112","blocked_commit":"000000000000000000000000000000000000000d","evidence":{"fix_commits":["ffffffffffffffffffffffffffffffffffffffff"],"reason":null,"type":"flagged_fix_then_success","validating_run_id":"900"},"flagged_paths":["src/synthetic.py"],"outcome":"catch","started_at":"2026-01-13T00:22:00Z"},{"block_run_id":"113","blocked_commit":"000000000000000000000000000000000000000e","evidence":{"fix_commits":[],"reason":null,"type":"same_commit_success_after_block","validating_run_id":"901"},"flagged_paths":["src/synthetic.py"],"outcome":"false_block","started_at":"2026-01-14T00:23:00Z"}],"check":"unit checks","cost_per_catch_usd":0.04,"false_block_rate":0.166667,"floor":{"canary_defects":3,"canary_shortfall":27,"ci_history_days":14,"ci_history_shortfall":0,"meets_floor":false,"required_canary_defects":30,"required_ci_history_days":14},"format":"tameshi-pack/1","history_window":{"first_run_started_at":"2026-01-01T00:10:00Z","last_run_updated_at":"2026-01-14T23:50:00Z"},"repo":"synthetic-repo","scorecard":{"blocks":6,"catches":3,"cost_per_catch_usd":0.04,"false_block_rate":0.166667,"false_blocks":1,"fires":40,"flake_rate":0.05,"flaky_commits":1,"judged_commits":20,"runtime_minutes":15.0,"runtime_seconds":900,"total_cost_usd":0.12,"unattributed_blocks":2},"sources":{"blindgate_evidence_supplied":false,"blindgate_unmatched_trace_count":0,"blindness_traces":["a bare string standing where no trace reading looks"],"canary_origin":"direct","catchledger_source":{"canaryfield_results_sha256":null,"ci_history_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","git_history_sha256":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}}}"##;

use std::path::{Path, PathBuf};
use std::process::Command;

use tameshi::jsonvalue::{parse, JsonValue};
use tameshi::render::render_json;

/// Owner discharge diag for tameshi Round 14, probe sets A
/// and B, evidence only and never to be merged. On the built
/// head EXPECT_REFUSAL stands true and the tamper cells
/// refuse at both ruling doors with the naming wording. On
/// the contract head it stands false and the same cells pass
/// both ruling doors, proving the pins fail for the pinned
/// reason. The list cell and the validate door quartet stand
/// identical on both heads.
const EXPECT_REFUSAL: bool = true;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tameshi-r14-discharge-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch directory stands");
    dir
}

fn write(dir: &Path, name: &str, text: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the fixture writes");
    path
}

fn run(dir: &Path, args: &[&str]) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_tameshi"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the binary runs");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn rule(dir: &Path) -> (i32, String) {
    run(
        dir,
        &[
            "rule",
            "--pack",
            "pack.json",
            "--ruling",
            "keep",
            "--out",
            "out",
        ],
    )
}

fn validate_ruling(dir: &Path) -> (i32, String) {
    run(
        dir,
        &[
            "validate-ruling",
            "--pack",
            "pack.json",
            "--ruling",
            "ruling.json",
        ],
    )
}

fn validate_v(dir: &Path) -> (i32, String) {
    run(
        dir,
        &[
            "validate",
            "--pack",
            "v-pack.json",
            "--catchledger",
            "v-ledger.json",
            "--canaryfield",
            "v-canary.json",
        ],
    )
}

fn stage_ruling(dir: &Path, pack_text: &str) {
    write(dir, "pack.json", pack_text);
    write(dir, "ruling.json", RULING);
}

fn replace_field(document: &JsonValue, field: &str, value: JsonValue) -> JsonValue {
    let pairs: Vec<(String, JsonValue)> = document
        .as_object()
        .expect("the document stands as an object")
        .iter()
        .map(|(key, held)| {
            if key == field {
                (key.clone(), value.clone())
            } else {
                (key.clone(), held.clone())
            }
        })
        .collect();
    JsonValue::Object(pairs)
}

fn add_field(document: &JsonValue, field: &str, value: JsonValue) -> JsonValue {
    let mut pairs: Vec<(String, JsonValue)> = document
        .as_object()
        .expect("the document stands as an object")
        .clone();
    pairs.push((field.to_string(), value));
    JsonValue::Object(pairs)
}

fn without_field(document: &JsonValue, field: &str) -> JsonValue {
    let pairs: Vec<(String, JsonValue)> = document
        .as_object()
        .expect("the document stands as an object")
        .iter()
        .filter(|(key, _)| key != field)
        .map(|(key, held)| (key.clone(), held.clone()))
        .collect();
    JsonValue::Object(pairs)
}

fn sources_of(pack: &JsonValue) -> JsonValue {
    pack.get("sources")
        .expect("the sources record stands")
        .clone()
}

fn forged(pack_text: &str, sources: JsonValue) -> String {
    let pack = parse(pack_text).expect("the pack parses");
    render_json(&replace_field(&pack, "sources", sources))
}

fn further_text() -> String {
    let pack = parse(PACK).expect("the pack parses");
    let sources = add_field(
        &sources_of(&pack),
        "publisher_note",
        JsonValue::Text("recorded by the publisher".to_string()),
    );
    render_json(&replace_field(&pack, "sources", sources))
}

fn missing_text() -> String {
    let pack = parse(PACK).expect("the pack parses");
    let sources = without_field(&sources_of(&pack), "blindgate_unmatched_trace_count");
    render_json(&replace_field(&pack, "sources", sources))
}

fn empty_text() -> String {
    forged(PACK, JsonValue::Object(Vec::new()))
}

fn list_text() -> String {
    forged(PACK, JsonValue::List(Vec::new()))
}

fn check_ruling_doors(name: &str, pack_text: &str, wording: &str) {
    let dir = scratch(name);
    stage_ruling(&dir, pack_text);
    let (code, err) = rule(&dir);
    let (vcode, verr) = validate_ruling(&dir);
    if EXPECT_REFUSAL {
        assert_eq!(code, 1, "the rule door refuses: {err}");
        assert!(
            err.contains(wording),
            "the rule door refusal names the fields: {err}"
        );
        assert_eq!(vcode, 1, "the validate ruling door refuses: {verr}");
        assert!(
            verr.contains(wording),
            "the validate ruling door refusal names the fields: {verr}"
        );
    } else {
        assert_eq!(code, 0, "the rule door passes the cell: {err}");
        assert_eq!(vcode, 0, "the validate ruling door passes the cell: {verr}");
    }
}

#[test]
fn test_the_sound_pack_passes_both_ruling_doors() {
    let dir = scratch("sound");
    stage_ruling(&dir, PACK);
    let (code, err) = rule(&dir);
    assert_eq!(code, 0, "the sound pack rules: {err}");
    let (vcode, verr) = validate_ruling(&dir);
    assert_eq!(vcode, 0, "the sound pack validates its ruling: {verr}");
}

#[test]
fn test_the_nested_member_cell_at_the_ruling_doors() {
    check_ruling_doors(
        "nested",
        PACK_NESTED,
        "the evidence pack sources record has the wrong fields (unexpected blindness_traces)",
    );
}

#[test]
fn test_the_further_member_cell_at_the_ruling_doors() {
    check_ruling_doors(
        "further",
        &further_text(),
        "the evidence pack sources record has the wrong fields (unexpected publisher_note)",
    );
}

#[test]
fn test_the_missing_field_cell_at_the_ruling_doors() {
    check_ruling_doors(
        "missing",
        &missing_text(),
        "the evidence pack sources record has the wrong fields (missing blindgate_unmatched_trace_count)",
    );
}

#[test]
fn test_the_empty_record_cell_at_the_ruling_doors() {
    check_ruling_doors(
        "empty",
        &empty_text(),
        "the evidence pack sources record has the wrong fields (missing blindgate_evidence_supplied, blindgate_unmatched_trace_count, canary_origin, catchledger_source)",
    );
}

#[test]
fn test_the_list_cell_refuses_with_the_standing_object_sentence_on_both_heads() {
    let dir = scratch("list");
    stage_ruling(&dir, &list_text());
    let (code, err) = rule(&dir);
    assert_eq!(code, 1, "the list cell refuses: {err}");
    assert!(
        err.contains("the evidence pack sources record must be an object"),
        "the refusal keeps the standing object sentence: {err}"
    );
}

#[test]
fn test_the_validate_door_quartet_stands_unchanged_on_both_heads() {
    let dir = scratch("validate-control");
    write(&dir, "v-pack.json", VPACK);
    write(&dir, "v-ledger.json", VLEDGER);
    write(&dir, "v-canary.json", VCANARY);
    let (code, err) = validate_v(&dir);
    assert_eq!(code, 0, "the control pack validates: {err}");

    let dir = scratch("validate-nested");
    write(&dir, "v-pack.json", VPACK_NESTED);
    write(&dir, "v-ledger.json", VLEDGER);
    write(&dir, "v-canary.json", VCANARY);
    let (code, err) = validate_v(&dir);
    assert_eq!(
        code, 1,
        "the nested pack refuses at the validate door: {err}"
    );
    assert!(
        err.contains("the evidence pack disagrees with its rebuilt pack"),
        "the refusal stands in the disagreement words: {err}"
    );

    let dir = scratch("validate-further");
    let pack = parse(VPACK).expect("the pack parses");
    let sources = add_field(
        &sources_of(&pack),
        "publisher_note",
        JsonValue::Text("recorded by the publisher".to_string()),
    );
    write(
        &dir,
        "v-pack.json",
        &render_json(&replace_field(&pack, "sources", sources)),
    );
    write(&dir, "v-ledger.json", VLEDGER);
    write(&dir, "v-canary.json", VCANARY);
    let (code, err) = validate_v(&dir);
    assert_eq!(
        code, 1,
        "the fifth field pack refuses at the validate door: {err}"
    );
    assert!(
        err.contains("the evidence pack disagrees with its rebuilt pack"),
        "the refusal stands in the disagreement words: {err}"
    );

    let dir = scratch("validate-missing");
    let pack = parse(VPACK).expect("the pack parses");
    let sources = without_field(&sources_of(&pack), "canary_origin");
    write(
        &dir,
        "v-pack.json",
        &render_json(&replace_field(&pack, "sources", sources)),
    );
    write(&dir, "v-ledger.json", VLEDGER);
    write(&dir, "v-canary.json", VCANARY);
    let (code, err) = validate_v(&dir);
    assert_eq!(
        code, 1,
        "the missing field pack refuses at the validate door: {err}"
    );
    assert!(
        err.contains("the evidence pack disagrees with its rebuilt pack"),
        "the refusal stands in the disagreement words: {err}"
    );
}
