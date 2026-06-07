---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# Output Registry

`ep_conformance::OutputRegistry` normalizes requested output variables from
case manifests and rejects duplicate requests.

It is part of the conformance evidence contract. A comparison report should use
the registry rather than ad hoc variable discovery.

`ep_runtime::RuntimeOutputRegistry` is the execution-side counterpart. It is
derived from the typed model, assigns output handles, and feeds
`ExecutionStep::WriteOutput` entries. Requested outputs should be resolved to
handles before timestep execution.

Missing variables become `OutputVariableUnavailable` diagnostics instead of
empty successful-looking series. Meters are routed through
`RuntimeMeterRegistry`; until meter algorithms are ported, meter requests become
explicit `MeterUnavailable` diagnostics rather than empty result columns.
