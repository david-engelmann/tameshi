# tameshi

The evidence pack tool for checks, where catch record, canary performance, false block rate, cost per catch, and blindness traces are assembled into one evidence pack per check and the ruling is issued as a pull request.

## What this is

Checks have no lifecycle in mainstream tooling. A check is required or it is absent, with no evidenced path through advisory, probation, and quarantine with expiry, so rational teams keep everything forever and suites grow into an hour of tests that stop nothing. Tameshi assembles the evidence the gate instruments produce and rules on it, keep, demote to advisory, or quarantine with a date. Nothing is deleted by the tool, rulings stay human signed, and a check can win a stay by producing fresh evidence. The boundary with its sibling is one sentence. Tameshi judges checks, verdictloop judges changes.

## Status

- Slice 2 is built and proven on synthetic targets only
- Slice 2 assembles one evidence pack per check in the format tameshi-pack/1 from a catchledger pack or ledger, reading canaryfield results directly or from the pack and blindness traces where blindgate evidence is supplied
- Slice 2 accounts every pack against the evidence floor of 30 canaryfield-scored planted defects and 14 days of catchledger CI history, read as the span of each check own catch record timestamps, and marks a pack below the floor
- Slice 2 generates the ruling pull request for a pack at or above the floor as a local ruling record in the format tameshi-ruling/1 and a body whose numbers all come from the pack, refuses a ruling below the floor naming each failed leg and its shortfall, and emits unsigned records only
- Slice 2 is the Round 1 work increment and is built on a local branch, and the round closes only after audit and two-axis review and red team and owner approval
- Round 2 completes the rename from keeporkill to tameshi across the tree, refuses any JSON document that carries the same key twice in one object under obligation TM2-2, and opens the Rust port with slice 1 under obligations PORT-1 to PORT-6, carrying the rendering and the document reader and the pack assembly and the ruling generation into a standard library only crate proven by fifty one recorded conformance cases against the landed Python, with the catchledger document reading and the command line staying Python in this slice for the reasons the contract states
- Round 3 opens the second Rust port slice under obligations PORT2-1 to PORT2-6, carrying the catchledger document reading whole into the crate, the pack and ledger validation and the ledger derivation under the read entry the pack assembly stands on, proven by one hundred further recorded conformance cases against the landed Python, and the slice is built on a local branch and the round closes only after audit and two-axis review and red team and owner approval, with the command line and the package front doors staying Python for the reasons the contract states
- Round 5 lands slice 3, the decision record and the kill ruling, carrying a recorded decision model recommendation into the pack as tameshi-pack/2 and issuing a kill ruling only above the floor on a pack that recommends it, with one live decision run over a synthetic pack standing in the examples
- Round 4 closes the Rust port under obligations PORT3-1 to PORT3-6, carrying the command line whole into the binary with the three doors the Python carried and retiring the Python package and suite from the tree, the recorded battery and the earlier recordings standing as the specification from here on, with the reader bound standing as a stated port limit in the one band the round's probes found the Python completing past it
- Closed rounds stand at 5 of 100
- Seeded in October 2026. Part of the impatient programming portfolio, in the Gate Truth arc

## Licence

MIT. See LICENSE.
