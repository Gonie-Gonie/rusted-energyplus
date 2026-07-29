//! Model-bound CP357 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_constant_shr_case_break,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(super) fn advance_cooling_constant_shr_case_break(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
) -> Result<
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_constant_shr_case_break(runtime, system, predecessor).map_err(
        DirectZonePurchasedAirScheduledCouplingError::CalculationCoolingConstantShrCaseBreak,
    )
}
