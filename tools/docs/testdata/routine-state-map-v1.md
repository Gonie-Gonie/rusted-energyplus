# Routine State Map Test Fixture

This fixture verifies that one checked-in state map can own separate structured
contracts for multiple routines. It is test evidence only, not a porting claim.

<!-- routine-state-contract:v1 begin manage_heat_balance -->
source_routine: `ManageHeatBalance`
read_state:
- `test.manage_heat_balance.read`
write_state:
- `test.manage_heat_balance.write`
history_state_ownership: `test.manage_heat_balance.history`
unsupported_state:
inactive_branches:
unsupported_active_branches:
not_claimed_branches:
<!-- routine-state-contract:v1 end manage_heat_balance -->

<!-- routine-state-contract:v1 begin sim_purchased_air -->
source_routine: `SimPurchasedAir`
read_state:
- `test.sim_purchased_air.read`
write_state:
- `test.sim_purchased_air.write`
history_state_ownership: `test.sim_purchased_air.history`
unsupported_state:
inactive_branches:
unsupported_active_branches:
not_claimed_branches:
<!-- routine-state-contract:v1 end sim_purchased_air -->

<!-- routine-state-contract:v1 begin manage_plant_loops -->
source_routine: `ManagePlantLoops`
read_state:
- `test.manage_plant_loops.read`
write_state:
- `test.manage_plant_loops.write`
history_state_ownership: `test.manage_plant_loops.history`
unsupported_state:
inactive_branches:
unsupported_active_branches:
not_claimed_branches:
<!-- routine-state-contract:v1 end manage_plant_loops -->
