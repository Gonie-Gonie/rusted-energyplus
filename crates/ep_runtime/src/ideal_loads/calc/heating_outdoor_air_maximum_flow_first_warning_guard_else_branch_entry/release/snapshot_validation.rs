//! Exact CP442 prefix, route, local-shape, and bitwise validation.

use super::super::transition::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRetainedRoute as Route,
    heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_from_committed_predecessor,
};
use super::super::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot as Snapshot,
};
use super::prefix::predecessor_cp441_snapshot;
use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallRetainedRoute as PredecessorRoute;

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
        && crate::ideal_loads::heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshot_is_exact(
            predecessor_cp441_snapshot(snapshot),
        )
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp441_snapshot(snapshot);
    let predecessor_route =
        crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshot_route(
            predecessor,
        )?;
    let route = heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp441_snapshot(snapshot);
    let predecessor_route = predecessor_route(route);
    crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
        && local_shape_is_exact(snapshot, predecessor, route)
}

pub(super) fn retained_route_matches_prior_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    retained_route_matches_snapshot_bounded(snapshot, route)
}

pub(super) fn prefix_and_local_shape_match(
    snapshot: Snapshot,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> bool {
    crate::ideal_loads::heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact(
        predecessor_cp441_snapshot(snapshot),
        predecessor,
    ) && heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
        && local_shape_is_exact(snapshot, predecessor, route)
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor, route: Route) -> bool {
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered
            == route.entered
        && route.entered
            == predecessor.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
        && same(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

pub(super) fn snapshots_match_bit_exact(left: Snapshot, right: Snapshot) -> bool {
    left.source == right.source
        && left.first_excluded_source == right.first_excluded_source
        && left.source_order == right.source_order
        && crate::ideal_loads::heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact(
            predecessor_cp441_snapshot(left),
            predecessor_cp441_snapshot(right),
        )
        && left.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered
            == right.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entered
}

fn predecessor_route(route: Route) -> PredecessorRoute {
    PredecessorRoute {
        logical_index: route.logical_index,
        predecessor_guard_false_fallthrough: route.predecessor_guard_false_fallthrough,
        predecessor_guard_body_entered: route.predecessor_guard_body_entered,
        predecessor_assignment_executed: route.predecessor_assignment_executed,
        predecessor_first_warning_guard_evaluated: route.predecessor_first_warning_guard_evaluated,
        predecessor_first_warning_branch_entered: route.predecessor_first_warning_branch_entered,
        predecessor_first_warning_guard_false_fallthrough: route
            .predecessor_first_warning_guard_false_fallthrough,
        predecessor_counter_increment_executed: route.predecessor_counter_increment_executed,
        predecessor_first_warning_call_site_reached: route
            .predecessor_first_warning_call_site_reached,
        predecessor_continue_warning_call_site_reached: route
            .predecessor_continue_warning_call_site_reached,
        continue_warning_timestamp_call_site_reached: route
            .predecessor_continue_warning_timestamp_call_site_reached,
    }
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
