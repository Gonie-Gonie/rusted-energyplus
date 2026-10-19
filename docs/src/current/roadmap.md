---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-23
---

# Roadmap

Work is organized by explicit source/owner/branch contracts. Historical version
milestones remain in `specs/milestones.toml` and generated documentation; an
existing file, source map or successful demo is not a completion claim.

## Completed Fixed CON-01 Port

The [64-card plan](../../../energyplus_porting_plan/README.md) covers the fixed opaque one-zone
A and direct no-OA sensible hard-sized IdealLoads B inputs. The selected Source
path now includes the real CTF, Surface/air, PurchasedAir, report and normal
completion producers. Retained unit comparisons remain tied to their actual
Source and inputs; unchanged units are not described as rerun on every later
binary.

The final four-gate decisions bind the same-current-Source
fifteen normal runs, Surface cadence/first-two/exit states, direct warmup
handoffs, exact named input outcomes, output round trips and separately scoped
ancillary differences in the [final validation](../../../energyplus_porting_plan/evidence/SRC-06-final-current-source-validation-60/summary.json).
The current finite results are **15 normal cases (A3/B12), 16427880 reported samples and own JSON/CSV records; tolerance failures 0, RMSE failures 0** and
**A 54/54, B 64/64**. These are explicit per-card
finite decisions, not general EnergyPlus equivalence. Preserve every failed attempt, unavailable state and excluded
branch; do not relax a numerical policy to close a card.

## Keep Promotion Boundaries Separate

A completed finite card does not promote a conformance manifest, make a routine
`family_gated`/`complete`, or establish full-domain readiness. Declared-variable
release claims continue to require their own reports and blocking gates.
`specs/algorithm_ledger.toml`, `specs/project_contract.toml`, the capability
registry and generated coverage remain authoritative for those broader claims.

Keep `ep_run` as the CLI-independent orchestration layer, explicit support/run
states and unsupported-feature boundaries, and source-order barriers in
`ExecutionPlan`. Diagnostic probes and fixture-demand adapters stay distinct
from the normal compatible Source route.

## Expand Only from a New Explicit Contract

Broader humidity/OA/economizer/heat-recovery, sizing, mixed HVAC, Plant, EMS,
AFN, fenestration and general ExampleFiles require their own active object
graphs, state ownership, source ranges, support rules and evidence gates.
An ExampleFile must map to supported capabilities or produce a clean blocked
or explicitly diagnostic partial report. Exploratory unit inputs do not widen
the fixed production domain.

## Keep Validation Proportional to the Change

Use the existing focused cohort or ordinary run that reaches changed code.
Preserve accepted unchanged-body/readset evidence instead of repeating old
large suites, annual runs or Original captures. Run relevant quality checks
once at a code milestone; documentation adoption uses the existing plan/view
checks. A genuinely missing input/state observation warrants a bounded new
capture, not a new generic parent hierarchy or an invented expected value.
