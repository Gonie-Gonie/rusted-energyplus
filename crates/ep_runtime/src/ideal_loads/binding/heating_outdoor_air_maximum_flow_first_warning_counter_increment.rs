//! Model-bound CP438 heating maximum-flow first-warning counter-increment adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_outdoor_air_maximum_flow_first_warning_counter_increment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp437: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
) -> Result<
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment(
        runtime,
        system,
        predecessor_cp437,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrement,
    )
}
