//! Private CP355 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_constant_shr_supply_humidity_ratio_minimum_limit_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot> {
        self.cooling_constant_shr_supply_humidity_ratio_minimum_limit_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_constant_shr_supply_humidity_ratio_minimum_limit_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
    ) {
        self.cooling_constant_shr_supply_humidity_ratio_minimum_limit_latest_witnesses
            .insert(system, snapshot);
    }
}
