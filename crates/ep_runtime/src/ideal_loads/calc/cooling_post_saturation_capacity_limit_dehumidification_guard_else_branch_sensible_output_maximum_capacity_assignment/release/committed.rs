//! Sealed CP422 route and assigned sensible-output capability for CP423.

use super::super::transition::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRetainedRoute as Route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_route_from_committed_predecessor,
};
use super::prefix::predecessor_cp421_snapshot;
use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{
    retained_route_matches_snapshot_bounded, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_committed_latest_route_and_assignment_values;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

/// Returns CP422's bounded route and line-2333 assigned output for CP423.
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_committed_latest_route_and_assigned_cooling_sensible_output(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<(Route, Option<f64>)> {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment;
    let latest = state.latest?;
    let route = state.latest_route?;
    let predecessor = predecessor_cp421_snapshot(latest);
    let (predecessor_route, _, committed_capacity) =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_committed_latest_route_and_assignment_values(
            unit, predecessor,
        )?;
    let expected_route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    let assigned = if route.assignment_executed {
        let output = latest
            .resulting_cooling_sensible_output_after_maximum_capacity_assignment_w?;
        let capacity = committed_capacity?;
        if output.to_bits() != capacity.to_bits() {
            return None;
        }
        Some(output)
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
        && route == expected_route
        && state_counts_are_consistent(state)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard,
        )
        && if route.assignment_executed {
            assigned.is_some()
        } else {
            assigned.is_none()
        })
    .then_some((route, assigned))
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp422_committed_owner_hot_path_is_bounded_and_calls_cp421_once() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        for forbidden in [
            "completed_",
            "snapshot_is_exact",
            "private_characterization",
            "predecessor_route(",
        ] {
            assert!(!hot.contains(forbidden), "{forbidden}");
        }
        assert_eq!(
            hot.matches("guard_committed_latest_route_and_assignment_values(")
                .count(),
            1,
        );
        for coordinated_forgery_barrier in [
            "resulting_cooling_sensible_output_after_maximum_capacity_assignment_w",
            "committed_capacity?",
            "output.to_bits() != capacity.to_bits()",
            "snapshots_match_bit_exact(latest, witness)",
        ] {
            assert!(hot.contains(coordinated_forgery_barrier), "{coordinated_forgery_barrier}");
        }
    }
}
