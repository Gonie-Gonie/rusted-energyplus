use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_supply_mass_flow_positive_guard_else_branch_entry_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot> {
        self.cooling_supply_mass_flow_positive_guard_else_branch_entry_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_mass_flow_positive_guard_else_branch_entry_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot>,
    ) {
        match witness {
            Some(witness) => {
                self.cooling_supply_mass_flow_positive_guard_else_branch_entry_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.cooling_supply_mass_flow_positive_guard_else_branch_entry_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
