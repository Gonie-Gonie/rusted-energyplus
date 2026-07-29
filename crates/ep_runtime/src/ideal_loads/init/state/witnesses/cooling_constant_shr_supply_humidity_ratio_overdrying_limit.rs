//! Private CP354 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot> {
        self.cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
    ) {
        self.cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witnesses
            .insert(system, snapshot);
    }
}
