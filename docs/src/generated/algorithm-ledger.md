<!-- DO NOT EDIT.
     Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py. -->

# Algorithm Ledger

Algorithm status is maintained in `specs/algorithm_ledger.toml`.

| ID | Domain | Status | Source map | EnergyPlus source | Rust target | First case | Proof variables | Claim level |
|---|---|---|---|---|---|---|---|---|
| zone_air_heat_balance | heat_balance | conformance | docs/src/porting-map/heat-balance-source-map.md | src/EnergyPlus/HeatBalanceManager.cc, src/EnergyPlus/ZoneTempPredictorCorrector.cc | crates/ep_runtime/src/runtime.rs::ZoneHeatBalanceState, crates/ep_runtime/src/runtime.rs::simulate_heat_balance | heat_balance_nomass_001 | Zone Mean Air Temperature | limited-conformance |
| surface_inside_temperature | heat_balance | conformance | docs/src/porting-map/heat-balance-source-map.md | src/EnergyPlus/HeatBalanceSurfaceManager.cc | crates/ep_runtime/src/runtime.rs::SurfaceHeatBalanceState, crates/ep_runtime/src/runtime.rs::simulate_heat_balance | surface_temperature_nomass_001 | Surface Inside Face Temperature | limited-conformance |
| air_side_node_state | hvac | diagnostic_only | docs/src/porting-map/node-state-source-map.md | src/EnergyPlus/DataLoopNode.hh, src/EnergyPlus/OutputProcessor.cc | crates/ep_runtime/src/runtime.rs::NodeStateStore, crates/ep_runtime/src/runtime.rs::simulate_ideal_loads_node_state_projection | air_side_node_diagnostic_001 | System Node Temperature, System Node Mass Flow Rate | none |
| plant_loop_state_projection | plant | diagnostic_only | docs/src/porting-map/plant-source-map.md | src/EnergyPlus/Plant/PlantManager.cc, src/EnergyPlus/Plant/Loop.cc, src/EnergyPlus/PlantUtilities.cc | crates/ep_runtime/src/runtime.rs::PlantStateStore, crates/ep_runtime/src/runtime.rs::simulate_plant_state_projection | plant_loop_diagnostic_001 | Plant Supply Side Inlet Temperature, Plant Supply Side Outlet Temperature | none |
