//! Sealed CP421 route and assignment operands for its immediate successor.

use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{
    retained_route_matches_snapshot_bounded, snapshots_match_bit_exact,
};
use super::super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute as Route;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_guard_committed_latest_maximum_total_cooling_capacity;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_committed_latest_route_and_assignment_values(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<(Route, Option<f64>, Option<f64>)> {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard;
    let latest = state.latest?;
    let route = state.latest_route?;
    let cooling_sensible_output = latest.cp420_cooling_sensible_output_for_capacity_guard_w;
    let maximum_total_cooling_capacity = latest.maximum_total_cooling_capacity_w;
    let committed_capacity = if route.active {
        let cp321 = unit.calc_cooling_capacity_zero_flow_reset.latest?;
        let cp340 = unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
            .latest?;
        Some(
            cooling_positive_supply_capacity_limit_sensible_output_guard_committed_latest_maximum_total_cooling_capacity(
                unit, cp321, cp340,
            )?,
        )
    } else {
        None
    };
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count > 0
        && state.latest_transition_ordinal == Some(state.transition_count)
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == unit.system
        && unit.controlled_zone == Some(latest.controlled_zone)
        && snapshots_match_bit_exact(latest, witness)
        && retained_route_matches_snapshot_bounded(latest, route)
        && state_counts_are_consistent(state)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment,
        )
        && route.active
            == latest.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
        && route.body_entered
            == latest.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
        && if route.active {
            cooling_sensible_output.is_some()
                && maximum_total_cooling_capacity.is_some()
                && maximum_total_cooling_capacity
                    .zip(committed_capacity)
                    .is_some_and(|(retained, committed)| {
                        retained.to_bits() == committed.to_bits()
                    })
                && latest.cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated
                && latest
                    .cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity
                    == Some(route.body_entered)
        } else {
            cooling_sensible_output.is_none()
                && maximum_total_cooling_capacity.is_none()
                && !route.body_entered
                && !latest.cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated
                && latest
                    .cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity
                    .is_none()
        })
        .then_some((route, cooling_sensible_output, maximum_total_cooling_capacity))
}
