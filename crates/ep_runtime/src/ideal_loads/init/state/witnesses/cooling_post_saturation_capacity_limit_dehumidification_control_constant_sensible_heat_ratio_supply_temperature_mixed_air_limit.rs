//! Private CP390 latest-witness storage.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot>
    {
        self.cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot,
    ) {
        self.cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_latest_witnesses
            .insert(system, snapshot);
    }
}
