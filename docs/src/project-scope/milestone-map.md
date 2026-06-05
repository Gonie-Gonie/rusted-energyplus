---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-05
---

# Milestone Map

Milestones are organized around evidence, not demos.

| Milestone | Purpose | Claim boundary |
|---|---|---|
| F0 Foundation | Reproducible toolchain, oracle, reference source, and workspace | setup only |
| v0.1 Model Intake | RawModel/TypedModel preview and missing-reference diagnostics | no runtime conformance |
| v0.2 Conformance Harness | case/suite/output/tolerance/report contracts and oracle baseline generation | harness only |
| v0.3 Input Interpretation Parity | object/default/name/reference interpretation evidence | input interpretation only |
| v0.4 Time, Weather, Schedule Parity | RunPeriod, EPW, and schedule comparisons | variables listed in reports only |
| v0.5 Geometry and Internal Variables | EIO/internal-variable comparisons for static properties and nominal gains | input/static evidence only |
| v0.6 Output and Trace Infrastructure | ResultStore, OutputRegistry, trace/report schema | no heat-balance claim |
| v0.7 EnergyPlus Source Porting Map | source-function maps and proof variables before algorithms | planning guard |
| v0.8 Uncontrolled Heat Balance Conformance | first tolerance-gated heat-balance subset | declared cases only |
| v0.9 IdealLoads Conformance | thermostat and load-calculation comparisons | declared cases only |
| v1.0 Stable Compatibility Subset | public subset with locked object matrix and CI reports | declared subset only |

Historical readiness notes are archived when they describe diagnostic runtime
paths rather than active public conformance milestones.

