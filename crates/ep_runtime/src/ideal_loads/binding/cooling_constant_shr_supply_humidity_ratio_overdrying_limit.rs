//! Model-bound CP354 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
) -> Result<
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingConstantShrSupplyHumidityRatioOverdryingLimit,
    )
}
