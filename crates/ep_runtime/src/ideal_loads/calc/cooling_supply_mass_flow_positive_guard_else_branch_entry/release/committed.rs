//! Sealed CP424 route capability for its immediate successor.

use super::super::transition::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRetainedRoute as Route,
    cooling_supply_mass_flow_positive_guard_else_branch_entry_route_from_committed_predecessor,
};
use super::prefix::predecessor_cp423_snapshot;
use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{retained_route_matches_snapshot_bounded, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

/// Returns CP424's bounded committed route without recursive derivation.
pub(in crate::ideal_loads::calc) fn cooling_supply_mass_flow_positive_guard_else_branch_entry_committed_latest_route(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<Route> {
    let state = &unit.calc_cooling_supply_mass_flow_positive_guard_else_branch_entry;
    let latest = state.latest?;
    let route = state.latest_route?;
    let predecessor = predecessor_cp423_snapshot(latest);
    let predecessor_route =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_committed_latest_route(
            unit,
            predecessor,
        )?;
    let expected_route =
        cooling_supply_mass_flow_positive_guard_else_branch_entry_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )?;
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
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment,
        ))
    .then_some(route)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp424_committed_route_hot_path_is_bounded() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        for forbidden in [
            "completed_",
            "snapshot_is_exact",
            "private_characterization",
            "snapshot_route(",
        ] {
            assert!(!hot.contains(forbidden), "{forbidden}");
        }
        assert_eq!(
            hot.matches("supply_temperature_assignment_committed_latest_route(").count(),
            1,
        );
    }
}
