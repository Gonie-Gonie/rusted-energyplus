//! Model-bound CP333 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_positive_supply_temperature_minimum_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPositiveSupplyTemperatureMinimumLimit,
    )
}
