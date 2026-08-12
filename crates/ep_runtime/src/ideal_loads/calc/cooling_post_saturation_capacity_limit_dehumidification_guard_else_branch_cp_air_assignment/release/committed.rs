//! Bounded committed CP419 route capability.

use super::super::transition::RetainedRoute;
use super::{
    Snapshot, committed_predecessor_counts_match, direct_subset_values_are_valid,
    snapshot_shape_is_exact, snapshots_match_bit_exact, state_counts_are_consistent,
};
use crate::ideal_loads::PurchasedAirUnitRuntimeState;

/// Returns CP419's sealed committed route without recursively validating CP418.
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<RetainedRoute> {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment;
    let latest = state.latest?;
    let route = state.latest_route?;
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
                .transition_count
        && snapshots_match_bit_exact(latest, witness)
        && snapshot_shape_is_exact(latest)
        && direct_subset_values_are_valid(latest)
        && route.active
            == latest
                .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        && state_counts_are_consistent(state)
        && committed_predecessor_counts_match(
            state,
            &unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
        ))
    .then_some(route)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp419_route_hot_path_has_no_recursive_exact_validation() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        assert!(!hot.contains("completed_"));
        assert!(!hot.contains("snapshot_is_exact"));
        assert!(!hot.contains("predecessor_route("));
    }
}
