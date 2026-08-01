//! Private CP378 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot> {
        self.cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    ) {
        self.cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witnesses
            .insert(system, snapshot);
    }
}
