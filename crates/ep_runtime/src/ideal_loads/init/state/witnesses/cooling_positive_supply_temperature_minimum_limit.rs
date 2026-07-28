//! Private CP333 snapshot witnesses.

use ep_model::IdealLoadsAirSystemId;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot, PurchasedAirRuntimeState,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_positive_supply_temperature_minimum_limit_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot> {
        self.cooling_positive_supply_temperature_minimum_limit_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_positive_supply_temperature_minimum_limit_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    ) {
        self.cooling_positive_supply_temperature_minimum_limit_latest_witnesses
            .insert(system, snapshot);
    }
}
