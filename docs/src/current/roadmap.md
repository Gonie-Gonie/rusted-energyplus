---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-23
---

# Roadmap

The roadmap is phase-led. Historical milestones remain in
`specs/milestones.toml` and generated docs, but new work should be planned
around source-order porting stages rather than version-number accumulation.

## P0: Current-State Judgment Correction

Treat existing surfaces as implementation to audit and harden, not as completed
checklist items. `ep_run`, support assessment, launcher scripts, PDF/evidence
scripts, and IdealLoads branch candidates already exist, so the next work is
classification, source-order alignment, capability integration, and evidence
hardening.

## Phase 1: Source-Order Runtime Reset

Tighten the contract, docs, specs, and runtime boundaries. Split diagnostic
probes away from compatibility paths, make `ExecutionPlan` an actual runtime
barrier, and keep unsupported features typed and explicit.

## Phase 2: 1ZoneUncontrolled Clean Conformance

Port the heat-balance path around EnergyPlus routines: `ManageHeatBalance`,
`ManageSurfaceHeatBalance`, `ManageAirHeatBalance`,
`ZoneTempPredictorCorrector`, and the CTF history handoff. Promote only
declared official `1ZoneUncontrolled` variables with reports and blocking
gates.

## Phase 3: IdealLoads Clean Conformance

Port `ZoneEquipmentManager` dispatch and `PurchasedAirManager` source-order
paths. Start with no-OA/no-limit sensible, finite limits, and
ConstantSensibleHeatRatio before expanding humidity, outdoor-air, economizer,
heat-recovery, and meter branches.

## Phase 4: Arbitrary IDF Run Framework

Harden the existing `ep_run` crate as the only CLI-independent orchestration
layer. Keep `ep_cli` as command dispatch, connect `SupportAssessment` to the
capability registry, add branch-specific runtime classes, and verify output
layout, run states, exit codes, oracle baseline, and oracle compare.

## Phase 5: ExampleFiles Coverage Expansion

Add ExampleFiles only when their active object graph maps to supported
capabilities or produces a clean blocked/partial support report.

## Phase 6: HVAC/Plant Expansion

Expand beyond IdealLoads and heat balance only after source maps, algorithm
ledger entries, support rules, runtime ownership, and evidence gates exist for
the target subsystem.
