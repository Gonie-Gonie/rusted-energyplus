//! Model-bound CP363 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_humidistat_case_break,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_humidistat_case_break(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
) -> Result<
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_humidistat_case_break(runtime, system, predecessor).map_err(
        DirectZonePurchasedAirScheduledCouplingError::CalculationCoolingHumidistatCaseBreak,
    )
}
