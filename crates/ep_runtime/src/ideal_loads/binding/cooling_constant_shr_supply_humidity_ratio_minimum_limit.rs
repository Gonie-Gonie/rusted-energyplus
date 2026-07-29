//! Model-bound CP355 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
) -> Result<
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingConstantShrSupplyHumidityRatioMinimumLimit,
    )
}
