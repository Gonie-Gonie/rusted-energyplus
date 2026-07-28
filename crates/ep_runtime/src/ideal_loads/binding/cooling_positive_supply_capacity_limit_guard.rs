//! Model-bound CP337 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_positive_supply_capacity_limit_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPositiveSupplyCapacityLimitGuard,
    )
}
