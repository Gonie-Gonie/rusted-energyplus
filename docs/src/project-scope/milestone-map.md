---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Milestone Map

Milestones are organized around compatibility evidence, not demos or apparent
feature maturity.

The canonical reset is `versioning-reset-v2.md`. The short version:

- v0.1 through v0.15 are Historical Pre-Alpha Evidence Series milestones.
- The plant-state projection work after v0.15 is an additional diagnostic
  artifact, not the official meaning of v0.16.
- v0.16 starts Road to v1.0 with versioning and evidence cleanup.
- v1.0 is a substantial declared compatibility draft.
- v2.0 targets EnergyPlus 26.1 full compatibility.
- v3.0 targets a fast modernized successor while retaining compatibility mode.

## Historical Pre-Alpha Evidence Series

| Milestone | Purpose | Claim boundary |
|---|---|---|
| F0 Foundation | reproducible toolchain, oracle, reference source, and workspace | setup only |
| v0.1 Model Intake | RawModel/TypedModel preview and missing-reference diagnostics | no runtime conformance |
| v0.2 Conformance Harness | case/suite/output/tolerance/report contracts and oracle baseline generation | harness only |
| v0.3 Input Interpretation Parity | object/default/name/reference interpretation evidence | input interpretation only |
| v0.4 Time, Weather, Schedule Evidence | RunPeriod, EPW, and schedule comparisons | listed variables only; smoke until tolerance-gated reports exist |
| v0.5 Static Geometry, Construction, Internal Gains | EIO/internal-variable comparisons for static properties and nominal gains | input/static evidence only |
| v0.6 Output, Trace, Compare Infrastructure | ResultStore, OutputRegistry, trace/report schema, compare artifact contract | no heat-balance claim |
| v0.7 EnergyPlus Source Mapping | source-function maps, call order, data maps, and proof variables before algorithms | planning guard |
| v0.8 Uncontrolled Heat Balance Port | first tolerance-gated heat-balance subset | `heat_balance_nomass_001` MAT only |
| v0.9 Surface State Expansion | first surface-temperature conformance subset | `surface_temperature_nomass_001` surface temperatures only |
| v0.10 IdealLoads and Thermostat | thermostat, zone equipment, and IdealLoads typed graph foundation | baseline-only smoke; no load conformance |
| v0.11 Air-side Node Diagnostic | baseline-only node outputs plus Rust projection plumbing | diagnostic-only; no node conformance |
| v0.12 Node Source Mapping | node registration, update, and output sampling source maps | planning guard |
| v0.13 Plant Loop Skeleton | typed plant loop graph and first equipment identity records | smoke only; no plant conformance |
| v0.14 Plant Source Mapping | plant loop input, loop-side simulation, component dispatch, plant utilities, and output source maps | planning guard |
| v0.15 Plant Loop Diagnostic Baseline | manifest-backed plant-only EnergyPlus baseline rows | diagnostic-only; no plant conformance |

The Rust plant-state projection added after v0.15 is tracked as a diagnostic
addendum in `legacy-milestones.md`.

## Road to v1.0

| Milestone | Purpose | Claim boundary |
|---|---|---|
| v0.16 Versioning and Evidence Cleanup | reclassify v0.1-v0.15, update v1/v2/v3 roadmap, isolate diagnostic addenda, require claim-boundary sections | planning/documentation gate; no new numerical conformance |
| v0.17 Case Manifest and Output Request Schema v2 | case/output/meter/tolerance/waiver schema and validation | infrastructure only |
| v0.18 Output Request Injection and Oracle Baseline Pipeline | patch official IDFs with requested outputs and generate selected oracle artifacts | baseline-only |
| v0.19 Series Reader and Compare Engine v2 | timestamp alignment, selected output/meter readers, metrics, first divergence, tolerance application | comparison infrastructure |
| v0.20 Conformance Report Generator | case-level and release-level reports, coverage matrices, known gaps, gate decisions | reporting infrastructure |
| v0.21 Source Map and Algorithm Ledger v1 | connect EnergyPlus source maps to Rust modules, status, and future gates | planning guard |
| v0.22 Time / Weather / Schedule Conformance Expansion | promote aligned time/weather/schedule variables for selected cases | declared variables only |
| v0.23 Static Model Evidence Expansion | geometry, materials, constructions, and internal gains across selected ExampleFiles | static evidence; no heat-balance claim unless declared |
| v0.24 Runtime State and Output Registry Hardening | SimulationState, OutputRegistry, MeterRegistry, ResultStore, diagnostics, profiling scaffold | runtime infrastructure |
| v0.25-v0.33 Building Physics and Controlled-Zone Expansion | no-mass generalization, massive construction, internal gains, fenestration, solar, warmup, uncontrolled-zone pack, thermostat/IdealLoads | declared cases and variables only |
| v0.34-v0.39 Air-side Expansion | node semantics, fans, coils, zone equipment, CAV/simple air loop, v1 air-side candidate report | declared subsets only |
| v0.40-v0.47 Plant and Meter Expansion | pumps, purchased energy, boiler, chiller, condenser loops, operation schemes, integrated HVAC/plant, meters | declared subsets only |
| v0.48-v0.59 Scope Lock and Evidence Packs | sizing boundary, diagnostics, v1 scope lock, stabilization, performance, stability, platform, docs | v1 readiness only |
| v0.80-v0.99 v1 Release Candidates | RC gates, full evidence regeneration, release freeze | v1 candidate claims only |

## Long-Term Targets

| Target | Meaning | Claim boundary |
|---|---|---|
| v1.0 Substantial Compatibility Draft | declared Tier A conformance suite plus Tier B diagnostics, coverage matrices, performance, stability, and known gaps | declared subset only |
| v2.0 EnergyPlus 26.1 Full Compatibility | broad/full compatibility mode for EnergyPlus 26.1 IDF/epJSON | compatibility mode only, with evidence |
| v3.0 Fast Modernized Successor | fast/modern modes with engineering validation, while preserving compatibility mode | mode-specific claims only |

## Required Claim Boundary

Every milestone must state whether its outputs are setup, smoke,
baseline-only, diagnostic-only, conformance, regression, or performance
evidence. A conformance claim requires:

```text
case + variable/meter list + tolerance + oracle baseline + Rust artifact + report + blocking gate
```
