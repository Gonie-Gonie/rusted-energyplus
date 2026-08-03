//! Restricted pure CP410 route characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor;

/// Characterizes one CP410 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_state(
        &mut state,
        predecessor,
    )
}
