---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Milestone Reset Plan

This document records the June 2026 milestone reset. The central correction is
that implemented code does not automatically make a milestone complete. Each
milestone must state its evidence level, required cases, output variables,
tolerances, artifacts, and blocking gate.

Core rule:

```text
No conformance claim without case + variable list + tolerance + report + gate.
```

## Current Layering

The repository currently contains four different kinds of work. They must stay
separate in docs, scripts, and release language.

| Layer | Examples | Evidence level |
|---|---|---|
| foundation | Rust workspace, pinned toolchain, oracle/runtime setup, reference source checkout | setup |
| harness | case/suite schema, baseline generation, report skeletons, output request schema | baseline-only |
| input/static smoke | RunPeriod, EPW fields, schedules, geometry, construction/materials, nominal gains | smoke |
| diagnostic runtime plumbing | `run first-zone`, `compare zone-temperature`, toy ResultStore output | diagnostic-only |

## Reclassified Milestones

| Milestone | Meaning | Claim boundary |
|---|---|---|
| F0 | Foundation only, no public tag | setup only |
| v0.1 | Model intake release | RawModel/TypedModel preview only |
| v0.2 | Conformance harness release | baseline/report infrastructure only |
| v0.3 | Input interpretation contract release | typed input interpretation only |
| v0.4 | Time/weather/schedule evidence release | listed variables only, smoke until tolerance-gated reports exist |
| v0.5 | Static geometry/construction/internal-gain release | input/static evidence only |
| v0.6 | Output/trace/compare infrastructure release | no heat-balance claim |
| v0.7 | EnergyPlus source mapping and algorithm porting plan release | planning guard before heat-balance code |
| v0.8 | Uncontrolled zone heat-balance port release | first tolerance-gated heat-balance subset |
| v0.9 | Surface/fenestration/radiation expansion release | `surface_temperature_nomass_001` surface temperatures only |
| v0.10 | IdealLoads/thermostat release | `ideal_loads_thermostat_001` typed graph and baseline-only output availability only |
| v0.11 | Air-side node diagnostic release | baseline-only node evidence and diagnostic Rust projection only |
| v0.12 | Node source mapping release | planning guard before node or IdealLoads numerical conformance |
| v0.13 | Plant loop skeleton release | typed graph smoke only; no plant numerical conformance |
| v0.14 | Plant source mapping release | planning guard before plant diagnostics or numerical conformance |
| v0.15 | Plant loop diagnostic release | oracle baseline and report skeleton only; no plant numerical conformance |
| v0.16 | Versioning and evidence cleanup release | planning/documentation gate; no new numerical conformance |
| v0.17 | Case manifest and output request schema v2 | infrastructure only |
| v0.18 | Output request injection and oracle baseline pipeline | baseline-only |
| v0.19 | Series reader and compare engine v2 | comparison infrastructure |
| v0.20 | Conformance report generator | reporting infrastructure |
| v0.21 | Source map and algorithm ledger v1 | planning guard |
| v0.22-v0.59 | Road to v1.0 numerical conformance expansion and stabilization | declared cases and variables only |
| v0.80-v0.99 | v1.0 release candidates and freeze | v1 candidate claims only |
| v1.0 | Substantial compatibility draft | locked supported subset only |
| v2.0 | EnergyPlus 26.1 full compatibility target | compatibility mode only, with evidence |
| v3.0 | fast modernized successor target | mode-specific claims only |

The Rust plant-state projection artifacts added after v0.15 are retained as an
additional diagnostic addendum. They are useful runtime plumbing, but they are
not the official meaning of v0.16 and do not create plant numerical
conformance.

## Retroactive Audit Rules

Already-written code must be handled in one of three ways:

| Choice | Use when | Required action |
|---|---|---|
| promote | the milestone should claim the behavior | add case, output list, tolerance, Rust artifact, report, and blocking gate |
| demote | the behavior is diagnostic/dev-only | remove it from public scope and keep `conformance_claim: false` |
| defer | the behavior exists but is ungated | move it to a future milestone as existing but ungated |

## ExampleFiles-Based Conformance Expansion

Milestones must eventually be backed by selected EnergyPlus ExampleFiles or
testfiles, not only reduced hand-built fixtures. ExampleFiles evidence expands
by tier:

| Tier | Purpose | Release use |
|---|---|---|
| Tier A | small deterministic release-gate candidates | may become blocking after reports and tolerances exist |
| Tier B | scheduled diagnostics and broader coverage | non-blocking by default |
| Tier C | complex coverage exploration | baseline-only by default |

Every ExampleFiles case must declare:

- source IDF and weather file
- feature flags such as surfaces, fenestration, HVAC, plant, EMS, plugins, and
  daylighting
- requested output variables and meters
- frequency and source artifact: EIO, ESO, MTR, SQL, or selected CSV
- evidence level for each output: baseline-only, diagnostic-only, or
  conformance
- report and summary artifact paths
- release gate and CI gate policy

The detailed policy lives in:

- `docs/src/conformance/examplefiles-coverage-plan.md`
- `docs/src/conformance/case-tier-policy.md`
- `docs/src/conformance/output-variable-matrix.md`
- `docs/src/conformance/report-format.md`

Forbidden shortcuts:

- treating finite samples as conformance
- printing deltas and marking a diagnostic comparison as pass
- running an ExampleFile without recording requested outputs
- comparing only total energy while omitting declared zone, surface, node,
  component, or meter outputs
- using `matches` without a tolerance policy and report

## Required Evidence By Class

| Class | Meaning | Can support public compatibility claim? |
|---|---|---|
| setup | toolchain, oracle, or source exists | no |
| smoke | command runs and expected output/files exist | no |
| diagnostic-only | values are extracted and deltas may be reported, no tolerance is enforced | no |
| baseline-only | EnergyPlus oracle artifacts are generated from a manifest | no |
| conformance | Rust output is compared to EnergyPlus with declared tolerance and blocking gate | yes, only for that case and variable |
| regression | current Rust behavior is compared to an accepted Rust baseline | no EnergyPlus claim |
| performance | runtime, memory, or profiling counters are compared | no EnergyPlus claim |

## Retrospective Checklist

v0.1:

- [x] keep graph/runtime/ResultStore out of public v0.1 scope
- [x] ensure `v0.1-verify` checks only model intake and package basics
- [x] state no simulation claim in release notes

v0.2:

- [x] align plan/readiness around conformance harness only
- [x] add `v0.2-verify`
- [x] fix baseline artifact contract to include expanded manifest and report JSON
- [x] document source-file target for output requests: ESO/EIO/MTR/SQL/CSV

v0.3:

- [x] align plan/readiness around input interpretation only
- [x] add `v0.3-verify`
- [x] decide whether graph/ExecutionPlan remains v0.3 foundation or moves fully to v0.6
- [x] add duplicate-normalized-name and supported invalid-numeric negative fixtures; defer unit/range until validators exist

v0.4:

- [x] lock the complete weather variable list, including radiation fields
- [x] document timestamp alignment and hour-ending semantics
- [x] decide which weather/schedule cases are smoke and which become conformance
- [x] keep conformance language out until tolerance-gated report artifacts exist

v0.5:

- [x] add dedicated `construction_materials_001` and `internal_gains_001` manifests
- [x] document EIO parser trust boundaries
- [x] lock geometry, construction/material, and internal-gain variable lists
- [x] document unsupported `GlobalGeometryRules`, coordinate-system, rotation, and fenestration boundaries

v0.6+:

- [x] keep `compare zone-temperature` diagnostic-only until v0.8 heat-balance conformance
- [x] decide whether `run first-zone` moves under a dev-only CLI namespace
- [x] make v0.7 source maps a blocking gate before heat-balance algorithm work
- [x] promote only a declared v0.8 heat-balance case with tolerance, report, and blocking gate

v0.6 decision: keep `run first-zone` callable for developer diagnostics in the
current CLI, but require docs, smoke output, and release verification to label
it as diagnostic infrastructure rather than public simulation compatibility.

v0.7 decision: source-map documents and `v0.7-verify` are required before any
heat-balance algorithm work can be promoted toward conformance.

v0.8 decision: `heat_balance_nomass_001` is the first promoted heat-balance
case. The claim is limited to hourly `Zone Mean Air Temperature` for the
no-mass adiabatic equilibrium case and is enforced by
`compare-heat-balance-conformance`.

v0.9 decision: `surface_temperature_nomass_001` is the first promoted
surface-state case. The claim is limited to hourly `Zone Mean Air Temperature`,
`Surface Inside Face Temperature`, and `Surface Outside Face Temperature` for
the no-mass adiabatic equilibrium case and is enforced by
`compare-surface-temperature-conformance`. It is not a fenestration or
solar-radiation claim.

v0.10 decision: `ideal_loads_thermostat_001` is a blocking smoke gate for
thermostat, zone equipment, and IdealLoads typed graph coverage. It proves
baseline output availability and graph connectivity only. It is not an
IdealLoads load-conformance claim, and it keeps `comparison_class = "smoke"`,
`conformance_claim = false`, and `tolerance_policy: none`.

v0.11 decision: `air_side_node_diagnostic_001` is a diagnostic-only node gate.
It records EnergyPlus baseline node-state outputs and a `NodeStateStore`-backed
Rust projection with `algorithm_parity: false`, but it is not a node, HVAC, or
IdealLoads numerical conformance claim.

v0.12 decision: node source mapping is a planning guard. It locks EnergyPlus
26.1.0 node registration, storage, update, and output paths before any future
node-state numerical claim may be promoted.

v0.13 decision: `plant-loop-skeleton.epJSON` is a typed graph smoke fixture for
PlantLoop, branch, connector, and first pump/boiler/chiller identity records.
It is not an EnergyPlus baseline case and is not a plant numerical conformance
claim.

v0.14 decision: plant source mapping is a planning guard. It locks EnergyPlus
26.1.0 plant loop input, loop-side simulation, component dispatch, plant
utilities, and first pump/boiler/chiller output paths before any future plant
diagnostic or numerical claim may be promoted.

v0.15 decision: `plant_loop_diagnostic_001` is a diagnostic-only plant gate. It
records EnergyPlus oracle baseline rows for plant supply-side demand/state,
pump electricity, district heating rate, and load-profile heat transfer, but it
does not provide Rust plant result artifacts, tolerances, or plant numerical
conformance.

v0.16 decision: v0.16 is Versioning and Evidence Cleanup. It adds the
canonical versioning reset, legacy milestone interpretation, v1/v2/v3 target
scope documents, and first Road to v1.0 plan entries. The already-added
`run plant-state-projection` work remains an additional diagnostic addendum for
the v0.15 fixture, with `algorithm_parity: false`, `conformance_claim: false`,
and `tolerance_policy: none`. This is artifact plumbing for future plant work,
not plant algorithm parity.

## Immediate Work Order

1. Keep `versioning-reset-v2.md`, `legacy-milestones.md`, and this reset plan
   linked from the mdBook.
2. Keep README split into public release, developer diagnostics, and
   conformance infrastructure.
3. Keep v0.1-v0.15 language historical and evidence-level based.
4. Use v0.17 for manifest/output schema v2 rather than adding more cases on
   the old manifest contract.
5. Use v0.18 for official-file output injection and oracle baseline pipeline.
6. Add verify scripts for each active milestone before calling it ready.
