//! Private CP365 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot, PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_constant_supply_humidity_ratio_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot> {
        self.cooling_constant_supply_humidity_ratio_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_constant_supply_humidity_ratio_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    ) {
        self.cooling_constant_supply_humidity_ratio_assignment_latest_witnesses
            .insert(system, snapshot);
    }

}
