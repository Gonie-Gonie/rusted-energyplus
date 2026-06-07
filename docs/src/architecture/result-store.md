---
status: active
claim_level: diagnostic
owner: runtime
last_reviewed: 2026-06-07
---

# Result Store

`ResultStore` is the runtime-native container for output series.

The current `first-zone` command writes a diagnostic `ResultStore`. That path
exercises runtime plumbing only and is not heat-balance conformance evidence.

v0.6 keeps this command as developer diagnostic infrastructure. The release
evidence is that native output series can be written and inspected; it is not a
public claim that zone air temperature matches EnergyPlus.

Future conformance cases should write Rust result artifacts from `ResultStore`
or successor output stores and compare them through declared output requests.

## Output Handles

The long-term result path should use an `OutputRegistry` to resolve output
requests during initialization, then write series through stable handles. The
runtime hot path should avoid ad hoc string lookup for every reported sample.

Selected output can be stored column-wise or through another handle-based
layout as long as the exported artifacts keep timestamp, key, variable,
frequency, class, and tolerance mapping intact for comparison reports.
