//! Sealed CP428 route and positive-zero sensible-output carrier for CP429.

use super::super::transition::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRetainedRoute as Route,
    cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_from_committed_predecessor,
};
use super::prefix::predecessor_cp427_snapshot;
use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{
    retained_route_matches_snapshot_bounded, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

/// Returns CP428's bounded committed route without recursive derivation.
pub(in crate::ideal_loads::calc) fn cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_committed_latest_route(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
    cp329_witness: Option<Cp329Snapshot>,
) -> Option<Route> {
    let state =
        &unit.calc_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment;
    let latest = state.latest?;
    let route = state.latest_route?;
    let predecessor = predecessor_cp427_snapshot(latest);
    let predecessor_route =
        cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_committed_latest_route(
            unit,
            predecessor,
            cp329_witness,
        )?;
    let expected_route =
        cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )?;
    let assigned_sensible_output_is_exact = if route.assignment_executed {
        latest
            .assigned_cooling_sensible_output_w
            .is_some_and(|value| value.to_bits() == 0.0_f64.to_bits())
    } else {
        latest.assigned_cooling_sensible_output_w.is_none()
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
        && assigned_sensible_output_is_exact
        && state_counts_are_consistent(state)
        && predecessor_counts_match(
            state,
            &unit.calc_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment,
        ))
    .then_some(route)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp428_committed_route_hot_path_is_bounded() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        for forbidden in [
            "completed_",
            "snapshot_is_exact",
            "private_characterization",
            "snapshot_route(",
            "calc_cooling_mixed_air_call.latest",
            "committed_latest_mixed_air_temperature(",
        ] {
            assert!(!hot.contains(forbidden), "{forbidden}");
        }
        assert_eq!(
            hot.matches("supply_temperature_mixed_air_assignment_committed_latest_route(")
                .count(),
            1,
        );
    }
}
