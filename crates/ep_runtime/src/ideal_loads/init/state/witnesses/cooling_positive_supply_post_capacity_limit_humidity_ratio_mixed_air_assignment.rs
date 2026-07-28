//! Private CP345 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    >{
        self.cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot:
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    ) {
        self.cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witnesses
            .insert(system, snapshot);
    }
}
