use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn heating_operating_mode_deadband_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot> {
        self.heating_operating_mode_deadband_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_heating_operating_mode_deadband_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot>,
    ) {
        match witness {
            Some(witness) => {
                self.heating_operating_mode_deadband_assignment_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.heating_operating_mode_deadband_assignment_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
