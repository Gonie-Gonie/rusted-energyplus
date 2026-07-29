use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot,
    PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot,
    > {
        self.cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot:
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot,
    ) {
        self.cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witnesses
            .insert(system, snapshot);
    }
}
