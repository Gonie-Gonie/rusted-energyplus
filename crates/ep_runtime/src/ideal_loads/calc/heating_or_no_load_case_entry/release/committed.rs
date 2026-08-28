//! Sealed CP430 route capability for CP431.

use super::super::transition::{
    PurchasedAirCalcHeatingOrNoLoadCaseEntryRetainedRoute as Route,
    heating_or_no_load_case_entry_route_from_committed_predecessor,
};
use super::prefix::predecessor_cp429_snapshot;
use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{
    prefix_and_local_shape_match, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

/// Returns CP430's bounded committed route without recursive derivation.
pub(in crate::ideal_loads::calc) fn heating_or_no_load_case_entry_committed_latest_route(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
    cp329_witness: Option<Cp329Snapshot>,
) -> Option<Route> {
    let state = &unit.calc_heating_or_no_load_case_entry;
    let latest = state.latest?;
    let route = state.latest_route?;
    let predecessor = predecessor_cp429_snapshot(latest);
    let predecessor_route =
        cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_committed_latest_route(
            unit,
            predecessor,
            cp329_witness,
        )?;
    let expected_route =
        heating_or_no_load_case_entry_route_from_committed_predecessor(
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
        && prefix_and_local_shape_match(latest, predecessor, route)
        && route == expected_route
        && state_counts_are_consistent(state)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment,
        ))
    .then_some(route)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp430_committed_route_hot_path_is_bounded() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        for forbidden in [
            "completed_",
            "snapshot_is_exact",
            "private_characterization",
            "retained_route_matches_snapshot_bounded(",
            "_snapshot_route(",
        ] {
            assert!(!hot.contains(forbidden), "{forbidden}");
        }
        assert_eq!(
            hot.matches("total_output_positive_zero_assignment_committed_latest_route(")
                .count(),
            1,
        );
    }
}
