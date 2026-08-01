//! Restricted pure CP377 counterfactual characterization.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as Owner,
    advance_cooling_supply_humidity_ratio_saturation_assignment_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor;

/// Characterizes a non-public CP377 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_supply_humidity_ratio_saturation_assignment_characterization(
    predecessor: Predecessor,
    supply_temperature_c: Option<f64>,
    temperature_owner: Option<Owner>,
    outdoor_barometric_pressure_pa: Option<f64>,
) -> Option<Snapshot> {
    let input = match (
        supply_temperature_c,
        temperature_owner,
        outdoor_barometric_pressure_pa,
    ) {
        (
            Some(supply_temperature_c),
            Some(temperature_owner),
            Some(outdoor_barometric_pressure_pa),
        ) => Some(ActiveInput {
            supply_temperature_c,
            temperature_owner,
            outdoor_barometric_pressure_pa,
        }),
        (None, None, None) => None,
        _ => return None,
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_saturation_assignment_state(
        &mut state,
        predecessor,
        input,
    )
}
