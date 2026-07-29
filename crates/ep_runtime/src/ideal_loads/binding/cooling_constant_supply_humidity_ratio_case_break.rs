//! Model-bound CP366 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_constant_supply_humidity_ratio_case_break(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingConstantSupplyHumidityRatioCaseBreak,
    )
}
