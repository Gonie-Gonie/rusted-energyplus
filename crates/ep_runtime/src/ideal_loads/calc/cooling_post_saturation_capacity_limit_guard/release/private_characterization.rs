//! Restricted pure CP380 counterfactual characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_guard_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Predecessor;

/// Characterizes a non-public CP380 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_guard_characterization(
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_guard_state(&mut state, predecessor, input)
}
