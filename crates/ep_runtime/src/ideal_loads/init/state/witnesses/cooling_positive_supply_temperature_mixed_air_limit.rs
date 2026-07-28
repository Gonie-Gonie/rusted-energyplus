//! Private CP334 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot, PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_positive_supply_temperature_mixed_air_limit_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot> {
        self.cooling_positive_supply_temperature_mixed_air_limit_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    ) {
        self.cooling_positive_supply_temperature_mixed_air_limit_latest_witnesses
            .insert(system, snapshot);
    }
}
