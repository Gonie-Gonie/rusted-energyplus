//! Model-bound CP358 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_cooling_humidistat_case_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
) -> Result<
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_humidistat_case_entry(runtime, system, predecessor).map_err(
        DirectZonePurchasedAirScheduledCouplingError::CalculationCoolingHumidistatCaseEntry,
    )
}
