//! Model-bound CP377 transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_supply_humidity_ratio_saturation_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
    barometric_pressure_pa: f64,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment(
        runtime,
        system,
        predecessor,
        barometric_pressure_pa,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingSupplyHumidityRatioSaturationAssignment,
    )
}
