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
| v0.9 | Surface/fenestration/radiation expansion release | declared cases only |
| v0.10 | IdealLoads/thermostat release | declared cases only |
| v0.11 | Air-side node/simple HVAC component release | declared cases only |
| v0.12 | Plant loop skeleton/node flow release | declared cases only |
| v1.0 | Declared compatibility subset release | locked supported subset only |

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

- [ ] keep `compare zone-temperature` diagnostic-only until v0.8 heat-balance conformance
- [ ] decide whether `run first-zone` moves under a dev-only CLI namespace
- [ ] make v0.7 source maps a blocking gate before heat-balance algorithm work

## Immediate Work Order

1. Keep this reset plan linked from the mdBook.
2. Split README quick start into public release, developer diagnostics, and conformance infrastructure.
3. Keep v0.2 and v0.3 scope separate from v0.4/v0.5 evidence.
4. Add verify scripts for each active milestone before calling it ready.
5. Move heat-balance conformance attempts to v0.8 or later.
