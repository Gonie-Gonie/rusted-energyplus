//! Private CP356 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot> {
        self.cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
    ) {
        self.cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_latest_witnesses
            .insert(system, snapshot);
    }
}
