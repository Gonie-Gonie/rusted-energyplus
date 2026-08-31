//! Model-bound CP435 heating outdoor-air maximum-flow guard adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_guard,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_outdoor_air_maximum_flow_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp434: PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_guard(
        runtime,
        system,
        predecessor_cp434,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::CalculationHeatingOutdoorAirMaximumFlowGuard,
    )
}
