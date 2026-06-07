---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Milestone Map

Milestones are organized around evidence, not demos.

| Milestone | Purpose | Claim boundary |
|---|---|---|
| F0 Foundation | Reproducible toolchain, oracle, reference source, and workspace | setup only |
| v0.1 Model Intake | RawModel/TypedModel preview and missing-reference diagnostics | no runtime conformance |
| v0.2 Conformance Harness | case/suite/output/tolerance/report contracts and oracle baseline generation | harness only |
| v0.3 Input Interpretation Parity | object/default/name/reference interpretation evidence | input interpretation only |
| v0.4 Time, Weather, Schedule Evidence | RunPeriod, EPW, and schedule comparisons | listed variables only; smoke until tolerance-gated reports exist |
| v0.5 Static Geometry, Construction, Internal Gains | EIO/internal-variable comparisons for static properties and nominal gains | input/static evidence only |
| v0.6 Output, Trace, Compare Infrastructure | ResultStore, OutputRegistry, trace/report schema, compare artifact contract | no heat-balance claim |
| v0.7 EnergyPlus Source Mapping | source-function maps, call order, data maps, and proof variables before algorithms | planning guard |
| v0.8 Uncontrolled Heat Balance Port | first tolerance-gated heat-balance subset | `heat_balance_nomass_001` MAT only |
| v0.9 Surface, Fenestration, Radiation Expansion | first surface-temperature conformance subset, with fenestration and solar still separate | `surface_temperature_nomass_001` surface temperatures only |
| v0.10 IdealLoads and Thermostat | thermostat, zone equipment, and IdealLoads typed graph foundation | `ideal_loads_thermostat_001` baseline-only smoke, no load conformance |
| v0.11 Air-side Node Diagnostic | baseline-only node output diagnostics for the typed IdealLoads node graph | `air_side_node_diagnostic_001` diagnostic-only; no node conformance |
| v0.12 Node Source Mapping | source-function map for node registration, update, and output sampling paths | planning guard only |
| v0.13 Plant Loop Skeleton | plant graph, node, flow, pump/boiler/chiller subset comparisons | declared cases only |
| v1.0 Stable Compatibility Subset | public subset with locked object matrix and CI reports | declared subset only |

Historical readiness notes are archived when they describe diagnostic runtime
paths rather than active public conformance milestones.
