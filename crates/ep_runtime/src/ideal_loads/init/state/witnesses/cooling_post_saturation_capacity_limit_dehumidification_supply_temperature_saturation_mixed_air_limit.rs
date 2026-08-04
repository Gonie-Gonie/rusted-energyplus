//! Private CP415 latest-witness storage.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot>
    {
        self.cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot,
    ) {
        self.cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witnesses
            .insert(system, snapshot);
    }

    #[cfg(test)]
    pub(in crate::ideal_loads) fn clear_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witness_for_test(
        &mut self,
        system: IdealLoadsAirSystemId,
    ) {
        self.cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witnesses
            .remove(&system);
    }
}
