//! Model-bound CP433 heating-mode guard else-branch-entry adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_heating_mode_guard_else_branch_entry,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_mode_guard_else_branch_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp432: PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_heating_mode_guard_else_branch_entry(
        runtime,
        system,
        predecessor_cp432,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::CalculationHeatingModeGuardElseBranchEntry,
    )
}
