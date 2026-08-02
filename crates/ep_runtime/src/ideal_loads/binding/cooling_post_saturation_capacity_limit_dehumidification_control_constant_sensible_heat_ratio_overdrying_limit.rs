//! Model-bound CP391 constant-SHR supply-enthalpy overdrying-limit adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp390: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
>{
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit(
        runtime,
        system,
        predecessor_cp390,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimit,
    )
}
