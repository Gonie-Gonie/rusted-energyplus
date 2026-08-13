//! Sealed CP420 route and sensible-output owner.

use super::runtime_validation::{
    committed_predecessor_counts_match, state_counts_are_consistent,
};
use super::snapshot::{snapshot_shape_is_exact, snapshots_match_bit_exact};
use super::route_commitment::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_route_matches_snapshot as route_matches_snapshot;
use super::super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentCommittedRoute as Route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_latest_route_and_cooling_sensible_output(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<(Route, Option<f64>)> {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment;
    let latest = state.latest?;
    let route = state.latest_route?;
    let output = latest.cooling_sensible_output_w;
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count > 0
        && state.latest_transition_ordinal == Some(state.transition_count)
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == unit.system
        && unit.controlled_zone == Some(latest.controlled_zone)
        && snapshots_match_bit_exact(latest, witness)
        && snapshot_shape_is_exact(latest)
        && route_matches_snapshot(latest, route)
        && state_counts_are_consistent(state)
        && committed_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment,
        )
        && route.active
            == latest.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed
        && if route.active {
            latest.cooling_sensible_output_assigned && output.is_some()
        } else {
            !latest.cooling_sensible_output_assigned && output.is_none()
        })
        .then_some((route, output))
}
