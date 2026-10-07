//! Model-bound CP442 heating maximum-flow first-warning guard else-entry adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp441: PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot,
) -> Result<
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry(
        runtime,
        system,
        predecessor_cp441,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntry,
    )
}
