//! Model-bound CP434 heating operating-mode Deadband-assignment adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_heating_operating_mode_deadband_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_operating_mode_deadband_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp433: PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
) -> Result<
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_heating_operating_mode_deadband_assignment(
        runtime,
        system,
        predecessor_cp433,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationHeatingOperatingModeDeadbandAssignment,
    )
}
