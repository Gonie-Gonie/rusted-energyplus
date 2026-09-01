//! Model-bound CP437 heating maximum-flow first-warning guard adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_first_warning_guard,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_outdoor_air_maximum_flow_first_warning_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp436: PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_first_warning_guard(
        runtime,
        system,
        predecessor_cp436,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationHeatingOutdoorAirMaximumFlowFirstWarningGuard,
    )
}
