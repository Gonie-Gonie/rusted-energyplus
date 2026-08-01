//! Restricted pure CP386 route characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Predecessor;

/// Characterizes one non-public CP386 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_switch_characterization(
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
        &mut state,
        predecessor,
        active_input,
    )
}
