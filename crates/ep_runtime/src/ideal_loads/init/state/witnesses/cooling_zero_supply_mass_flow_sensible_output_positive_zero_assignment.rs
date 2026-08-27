use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<
        PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot,
    > {
        self.cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<
            PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot,
        >,
    ) {
        match witness {
            Some(witness) => {
                self.cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
