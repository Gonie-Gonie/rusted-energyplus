//! Sealed bounded CP436 route capability for CP437.

use super::super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute as Route;
use super::prefix::predecessor_cp435_snapshot;
use super::runtime_validation::{predecessor_counts_match, state_counts_are_consistent};
use super::snapshot_validation::{prefix_and_local_shape_match, snapshots_match_bit_exact};
use crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_guard_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

/// Returns CP436's committed route without recursive snapshot-route replay.
pub(in crate::ideal_loads::calc) fn heating_outdoor_air_maximum_flow_body_volume_flow_assignment_committed_latest_route(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
    cp329_witness: Option<Cp329Snapshot>,
) -> Option<Route> {
    let state = &unit.calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment;
    let latest = state.latest?;
    let route = state.latest_route?;
    let predecessor = predecessor_cp435_snapshot(latest);
    let predecessor_route = heating_outdoor_air_maximum_flow_guard_committed_latest_route(
        unit,
        predecessor,
        cp329_witness,
    )?;
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count > 0
        && state.latest_transition_ordinal == Some(state.transition_count)
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == unit.system
        && unit.controlled_zone == Some(latest.controlled_zone)
        && snapshots_match_bit_exact(latest, witness)
        && prefix_and_local_shape_match(latest, predecessor, predecessor_route, route)
        && state_counts_are_consistent(state)
        && predecessor_counts_match(state, &unit.calc_heating_outdoor_air_maximum_flow_guard))
    .then_some(route)
}
