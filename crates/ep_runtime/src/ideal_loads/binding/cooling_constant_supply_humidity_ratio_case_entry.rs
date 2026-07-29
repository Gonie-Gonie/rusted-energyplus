//! Model-bound CP364 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_constant_supply_humidity_ratio_case_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
) -> Result<
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingConstantSupplyHumidityRatioCaseEntry,
    )
}
