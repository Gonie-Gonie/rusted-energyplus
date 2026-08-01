//! Model-bound CP379 post-saturation supply-enthalpy transition adapter.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment,
};

use super::DirectZonePurchasedAirScheduledCouplingError;

pub(in crate::ideal_loads) fn advance_cooling_supply_enthalpy_post_saturation_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
        runtime,
        system,
        predecessor,
    )
    .map_err(
        DirectZonePurchasedAirScheduledCouplingError::
            CalculationCoolingSupplyEnthalpyPostSaturationAssignment,
    )
}
