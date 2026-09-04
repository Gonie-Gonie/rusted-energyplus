//! Model-bound CP440 heating maximum-flow continue-warning call-site adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_continue_warning_call,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_outdoor_air_maximum_flow_continue_warning_call(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp439: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot,
) -> Result<
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_continue_warning_call(
        runtime,
        system,
        predecessor_cp439,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationHeatingOutdoorAirMaximumFlowContinueWarningCall,
    )
}
