<!-- DO NOT EDIT.
     Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py. -->

# Algorithm Ledger

Algorithm status is maintained in `specs/algorithm_ledger.toml`.

| ID | Domain | Status | EnergyPlus source | Rust target | First case | Proof variables | Claim level |
|---|---|---|---|---|---|---|---|
| zone_air_heat_balance | heat_balance | source_mapped | src/EnergyPlus/ZoneTempPredictorCorrector.cc | crates/ep_runtime/src/heat_balance/zone_air.rs | heat_balance_nomass_001 | Zone Mean Air Temperature, Zone Air Heat Balance Air Energy Storage Rate | limited-conformance |
| surface_inside_temperature | heat_balance | source_mapped | src/EnergyPlus/HeatBalanceSurfaceManager.cc | crates/ep_runtime/src/heat_balance/surface_inside.rs | surface_temperature_nomass_001 | Surface Inside Face Temperature | limited-conformance |
| air_side_node_state | hvac | diagnostic_only | src/EnergyPlus/DataLoopNode.hh, src/EnergyPlus/OutputProcessor.cc | crates/ep_runtime/src/hvac/node_state.rs | air_side_node_diagnostic_001 | System Node Temperature, System Node Mass Flow Rate | none |
| plant_loop_state_projection | plant | diagnostic_only | src/EnergyPlus/Plant/PlantLoopSolver.cc | crates/ep_runtime/src/plant/plant_loop.rs | plant_loop_diagnostic_001 | Plant Supply Side Inlet Temperature, Plant Supply Side Outlet Temperature | none |
