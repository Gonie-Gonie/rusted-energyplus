//! Private CP379 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_supply_enthalpy_post_saturation_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot> {
        self.cooling_supply_enthalpy_post_saturation_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_enthalpy_post_saturation_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
    ) {
        self.cooling_supply_enthalpy_post_saturation_assignment_latest_witnesses
            .insert(system, snapshot);
    }
}
