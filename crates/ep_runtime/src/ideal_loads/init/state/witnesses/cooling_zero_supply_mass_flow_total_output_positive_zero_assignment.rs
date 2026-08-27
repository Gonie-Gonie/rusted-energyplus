use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot>
    {
        self.cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<
            PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot,
        >,
    ) {
        match witness {
            Some(witness) => {
                self.cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
