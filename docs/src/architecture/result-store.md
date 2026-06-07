---
status: active
claim_level: diagnostic
owner: runtime
last_reviewed: 2026-06-07
---

# Result Store

`ResultStore` is the runtime-native container for output series. Since v0.24,
it lives with output primitives in `crates/ep_runtime/src/output.rs`.

The current `first-zone` command writes a diagnostic `ResultStore`. That path
exercises runtime plumbing only and is not heat-balance conformance evidence.

v0.24 adds duplicate-handle, duplicate-series, and profile scaffolding to this
store. The release evidence is that native output series can be written,
inspected, and checked structurally; it is not a public claim that zone air
temperature matches EnergyPlus.

Future conformance cases should write Rust result artifacts from `ResultStore`
or successor output stores and compare them through declared output requests.

## Output Handles

The runtime result path now uses `RuntimeOutputRegistry` to resolve model-owned
output handles before execution-plan output steps are built. The runtime hot
path should avoid ad hoc string lookup for every reported sample.

Selected output can be stored column-wise or through another handle-based
layout as long as the exported artifacts keep timestamp, key, variable,
frequency, class, and tolerance mapping intact for comparison reports.
