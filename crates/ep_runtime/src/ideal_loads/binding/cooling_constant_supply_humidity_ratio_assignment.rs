//! Model-bound CP365 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_constant_supply_humidity_ratio_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
) -> Result<
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingConstantSupplyHumidityRatioAssignment,
    )
}
