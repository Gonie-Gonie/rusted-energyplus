//! Model-bound CP430 Heating-or-no-load case-entry adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_heating_or_no_load_case_entry,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_heating_or_no_load_case_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp429: PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_heating_or_no_load_case_entry(runtime, system, predecessor_cp429)
        .map_err(DirectZonePurchasedAirScheduledCouplingError::CalculationHeatingOrNoLoadCaseEntry)
}
