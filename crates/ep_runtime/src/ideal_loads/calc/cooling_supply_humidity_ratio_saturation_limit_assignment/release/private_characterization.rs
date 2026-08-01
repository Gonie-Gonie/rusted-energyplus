//! Restricted pure CP378 counterfactual characterization.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor;

/// Characterizes a non-public CP378 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_supply_humidity_ratio_saturation_limit_assignment_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state(&mut state, predecessor)
}
