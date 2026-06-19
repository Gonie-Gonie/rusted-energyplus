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
`RuntimeMeterRegistry`. The declared no-OA IdealLoads
`DistrictHeatingWater:Facility` and `DistrictCooling:Facility` requests now
resolve to meter handles and produce compareable MTR-aligned series for the
hourly, monthly, annual, and run-period rows covered by the narrow meter
claims. Unsupported meter requests continue to become explicit
`MeterUnavailable` diagnostics rather than empty result columns.
