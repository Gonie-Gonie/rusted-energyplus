//! Bounded committed CP419 route capability.

use super::super::transition::{
    RetainedRoute,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_route_from_validated_predecessor,
};
use super::{
    Snapshot, committed_predecessor_counts_match, cp418_shape, direct_subset_values_are_valid,
    snapshot_shape_is_exact, snapshots_match_bit_exact, state_counts_are_consistent,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_committed_latest_route;
use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Returns CP419's sealed committed route without recursively validating CP418.
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<RetainedRoute> {
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air(
        unit, witness,
    )
    .map(|(route, _)| route)
}

/// Returns CP419's sealed route and local `CpAir` owner for CP423.
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route_and_cp_air(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<(RetainedRoute, Option<f64>)> {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment;
    let latest = state.latest?;
    let route = state.latest_route?;
    let cp_air = latest.cp_air_j_per_kg_k;
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && latest.system == unit.system
        && latest.parent_call_ordinal == state.transition_count
        && unit.controlled_zone == Some(latest.controlled_zone)
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
                .transition_count
        && snapshots_match_bit_exact(latest, witness)
        && snapshot_shape_is_exact(latest)
        && direct_subset_values_are_valid(latest)
        && retained_route_matches_snapshot_bounded(unit, latest, route)
        && route.active
            == latest
                .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        && if route.active {
            latest.cp_air_assigned && cp_air.is_some()
        } else {
            !latest.cp_air_assigned && cp_air.is_none()
        }
        && state_counts_are_consistent(state)
        && committed_predecessor_counts_match(
            state,
            &unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
        ))
    .then_some((route, cp_air))
}

fn retained_route_matches_snapshot_bounded(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    route: RetainedRoute,
) -> bool {
    let predecessor = cp418_shape(snapshot);
    let Some(predecessor_route) =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_committed_latest_route(
            unit,
            predecessor,
        )
    else {
        return false;
    };
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_route_from_validated_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp419_route_hot_path_has_no_recursive_exact_validation() {
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
        assert!(hot.contains("retained_route_matches_snapshot_bounded("));
        assert_eq!(
            hot.matches("guard_else_branch_entry_committed_latest_route(")
                .count(),
            1,
        );
        assert_eq!(
            hot.matches("committed_latest_route_and_cp_air(").count(),
            2,
        );
    }
}
