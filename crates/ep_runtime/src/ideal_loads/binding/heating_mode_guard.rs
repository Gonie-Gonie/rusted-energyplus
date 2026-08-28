//! Model-bound CP431 heating-mode guard adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingModeGuardSnapshot, PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_heating_mode_guard,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_mode_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp430: PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
) -> Result<PurchasedAirCalcHeatingModeGuardSnapshot, DirectZonePurchasedAirScheduledCouplingError>
{
    advance_direct_no_oa_calc_heating_mode_guard(runtime, system, predecessor_cp430)
        .map_err(DirectZonePurchasedAirScheduledCouplingError::CalculationHeatingModeGuard)
}
