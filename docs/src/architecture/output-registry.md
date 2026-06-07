---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# Output Registry

`OutputRegistry` normalizes requested output variables from case manifests and
rejects duplicate requests.

It is part of the conformance evidence contract. A comparison report should use
the registry rather than ad hoc variable discovery.

At runtime, requested outputs should be resolved to handles before timestep
execution. Missing variables should become `OutputVariableUnavailable`
diagnostics instead of empty successful-looking series.
