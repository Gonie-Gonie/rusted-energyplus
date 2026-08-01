//! Restricted pure CP376 counterfactual characterization.

use super::super::{
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner as Owner,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Predecessor;

/// Characterizes a non-public CP376 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_supply_humidity_ratio_pre_saturation_original_assignment_characterization(
    predecessor: Predecessor,
    purchased_air_supply_humidity_ratio: Option<f64>,
    owner: Option<Owner>,
) -> Option<Snapshot> {
    let input = match (purchased_air_supply_humidity_ratio, owner) {
        (Some(purchased_air_supply_humidity_ratio), Some(owner)) => Some(ActiveInput {
            purchased_air_supply_humidity_ratio,
            owner,
        }),
        (None, None) => None,
        _ => return None,
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state(
        &mut state,
        predecessor,
        input,
    )
}
