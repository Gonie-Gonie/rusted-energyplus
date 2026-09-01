//! Sealed bounded CP437 route and canonical warning-counter capability for CP438.

use super::super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute as Route;
use super::prefix::predecessor_cp436_snapshot;
use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{prefix_and_local_shape_match, snapshots_match_bit_exact};
use crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_body_volume_flow_assignment_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

/// Returns CP437's committed route and canonical counter without recursive route replay.
pub(in crate::ideal_loads::calc) fn heating_outdoor_air_maximum_flow_first_warning_guard_committed_latest_route_and_outdoor_air_flow_maximum_heating_output_error_count(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
    cp329_witness: Option<Cp329Snapshot>,
) -> Option<(Route, usize)> {
    let state = &unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard;
    let latest = state.latest?;
    let route = state.latest_route?;
    let predecessor = predecessor_cp436_snapshot(latest);
    let predecessor_route =
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_committed_latest_route(
            unit,
            predecessor,
            cp329_witness,
        )?;
    let counter = state.outdoor_air_flow_maximum_heating_output_error_count;
    let latest_counter_matches_owner = if route.guard_evaluated {
        latest.outdoor_air_flow_maximum_heating_output_error_count_before == Some(counter)
    } else {
        latest
            .outdoor_air_flow_maximum_heating_output_error_count_before
            .is_none()
    };
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count > 0
        && state.latest_transition_ordinal == Some(state.transition_count)
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == unit.system
        && unit.controlled_zone == Some(latest.controlled_zone)
        && snapshots_match_bit_exact(latest, witness)
        && prefix_and_local_shape_match(latest, predecessor, predecessor_route, route)
        && counter <= 1
        && latest_counter_matches_owner
        && state_counts_are_consistent(state)
        && predecessor_counts_match(
            state,
            &unit.calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment,
        ))
    .then_some((route, counter))
}
