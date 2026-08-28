//! Model-bound CP432 heating operating-mode Heat assignment adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingModeGuardSnapshot,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_heating_operating_mode_heat_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_operating_mode_heat_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp431: PurchasedAirCalcHeatingModeGuardSnapshot,
) -> Result<
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_heating_operating_mode_heat_assignment(
        runtime,
        system,
        predecessor_cp431,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::CalculationHeatingOperatingModeHeatAssignment,
    )
}
