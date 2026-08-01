//! Restricted pure CP387 route characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state,
};
use super::prefix_validation::active_input_from_retained_owner;
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Predecessor;
use crate::ideal_loads::PurchasedAirRuntimeState;
use ep_model::IdealLoadsAirSystem;

/// Characterizes one non-public CP387 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_characterization(
    runtime: &PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    if system.id != predecessor.system {
        return None;
    }
    let route = super::super::transition::routes::predecessor_route(predecessor)?;
    let active_input = route
        .active
        .then(|| active_input_from_retained_owner(runtime, system, predecessor))
        .flatten();
    if route.active && active_input.is_none() {
        return None;
    }
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
        &mut state,
        predecessor,
        active_input,
    )
}
