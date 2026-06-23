---
status: active
claim_level: none
owner: runtime
last_reviewed: 2026-06-07
---

# Execution Plan

`ExecutionPlan` records EnergyPlus source-order runtime barriers and the
graph-derived work assigned to each barrier. It is currently an architecture
boundary, a diagnostic summary, and the compatibility-mode ordering contract.

The plan is useful for:

- validating graph connectivity
- making runtime stages inspectable
- preparing trace/report infrastructure

It does not imply that all EnergyPlus algorithms behind those stages have been
ported.

`ExecutionPlan` is the place where compatibility-mode ordering becomes
explicit. The default compatibility path must preserve EnergyPlus-aligned
barriers before fast or experimental scheduling is considered.

The plan assigns typed graph work to source-order stages:

- `EvaluateZoneThermostat`
- `SolveZone`
- `ManageZoneEquipment`
- `SimZoneEquipment`
- `InitIdealLoadsAirSystem`
- `EvaluateIdealLoadsAirSystem`
- `UpdateIdealLoadsAirSystem`
- `ReportIdealLoadsAirSystem`
- `WriteOutput`

They are ordering markers, not broad HVAC load-conformance markers by
themselves.

EMS, PythonPlugin, API actuators, and other callbacks must remain explicit
barriers or invalidation points before dependent caches can be reused across
them.

For the official `1ZoneUncontrolled` dynamic heat-balance work,
`ExecutionPlan.stages` follows the EnergyPlus heat-balance source routine order
and `ExecutionPlan.compatibility_stages` stores the same heat-balance contract
for reports. The initial contract is:

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

If IdealLoads equipment is active, `ExecutionPlan.stages` appends the
source-order `ZoneEquipmentManager` and `PurchasedAirManager` stages:

1. `ZoneEquipmentManager.cc::ManageZoneEquipment`
2. `PurchasedAirManager.cc::InitPurchasedAir`
3. `PurchasedAirManager.cc::CalcPurchAirLoads`
4. `PurchasedAirManager.cc::UpdatePurchasedAir`
5. `PurchasedAirManager.cc::ReportPurchasedAir`

The entries are an ordering contract and trace/report scaffold. They do not
claim that every routine has full EnergyPlus numerical parity yet.
