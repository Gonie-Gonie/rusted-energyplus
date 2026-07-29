//! Model-bound CP359 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_cooling_humidistat_moisture_demand_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot,
) -> Result<
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingHumidistatMoistureDemandAssignment,
    )
}
