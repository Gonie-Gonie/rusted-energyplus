//! Model-bound CP390 constant-SHR supply-temperature mixed-air-limit adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp389: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
>{
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit(
        runtime,
        system,
        predecessor_cp389,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimit,
    )
}
