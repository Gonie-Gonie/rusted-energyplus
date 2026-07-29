use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    #[allow(dead_code)]
    pub(in crate::ideal_loads) fn cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot> {
        self.cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
    ) {
        self.cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witnesses
            .insert(system, snapshot);
    }
}
