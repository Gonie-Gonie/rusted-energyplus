---
status: active
claim_level: none
owner: runtime
last_reviewed: 2026-06-07
---

# Execution Plan

`ExecutionPlan` records coarse runtime stages and graph-derived work ordering.
It is currently an architecture boundary and diagnostic summary.

The plan is useful for:

- validating graph connectivity
- making runtime stages inspectable
- preparing trace/report infrastructure

It does not imply that all EnergyPlus algorithms behind those stages have been
ported.

`ExecutionPlan` is the place where compatibility-mode ordering becomes
explicit. The default compatibility path must preserve EnergyPlus-aligned
barriers before fast or experimental scheduling is considered.

v0.10 adds thermostat and IdealLoads ordering placeholders:

- `EvaluateZoneThermostat`
- `EvaluateIdealLoadsAirSystem`

They are typed-graph readiness markers for `ideal_loads_thermostat_001`, not
HVAC load-conformance markers.

Future execution stages should represent environment, warmup, zone, system,
plant, reporting, callback, and output barriers. EMS, PythonPlugin, API
actuators, and other callbacks must become explicit invalidation points before
dependent caches can be reused across them.

For the official `1ZoneUncontrolled` dynamic heat-balance work,
`ExecutionPlan.compatibility_stages` now records the EnergyPlus source routine
contract separately from the coarse Rust work stages. The initial contract is:

1. `HeatBalanceManager.cc::GetHeatBalanceInput`
2. `HeatBalanceManager.cc::EMS BeginZoneTimestepBeforeInitHeatBalance`
3. `HeatBalanceManager.cc::InitHeatBalance`
4. `HeatBalanceManager.cc::EMS BeginZoneTimestepAfterInitHeatBalance`
5. `HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance`
6. `HeatBalanceSurfaceManager.cc::InitSurfaceHeatBalance`
7. `HeatBalanceSurfaceManager.cc::CalcHeatBalanceOutsideSurf`
8. `HeatBalanceSurfaceManager.cc::CalcHeatBalanceInsideSurf`
9. `HeatBalanceAirManager.cc::ManageAirHeatBalance`
10. `HeatBalanceSurfaceManager.cc::UpdateFinalSurfaceHeatBalance`
11. `HeatBalanceSurfaceManager.cc::UpdateThermalHistories`
12. `HeatBalanceSurfaceManager.cc::ReportSurfaceHeatBalance`
13. `HeatBalanceManager.cc::EMS EndZoneTimestepBeforeZoneReporting`
14. `HeatBalanceManager.cc::RecKeepHeatBalance`
15. `HeatBalanceManager.cc::ReportHeatBalance`
16. `HeatBalanceManager.cc::EMS EndZoneTimestepAfterZoneReporting`
17. `HeatBalanceManager.cc::CheckWarmupConvergence`

The entries are an ordering contract and trace/report scaffold. They do not
claim that every routine has full EnergyPlus numerical parity yet.
