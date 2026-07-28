//! Model-bound CP338 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_positive_supply_capacity_limit_cp_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingPositiveSupplyCapacityLimitCpAirAssignment,
    )
}
