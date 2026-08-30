//! Exact CP433 prefix, route, local-shape, and bitwise validation.

use super::prefix::predecessor_cp432_snapshot;
use super::super::{
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot as Snapshot,
};
use super::super::transition::{
    PurchasedAirCalcHeatingModeGuardElseBranchEntryRetainedRoute as Route,
    heating_mode_guard_else_branch_entry_route_from_committed_predecessor,
};
use crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOperatingModeHeatAssignmentRetainedRoute as PredecessorRoute;

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
        && crate::ideal_loads::heating_operating_mode_heat_assignment_snapshot_is_exact(
            predecessor_cp432_snapshot(snapshot),
        )
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp432_snapshot(snapshot);
    let predecessor_route =
        crate::ideal_loads::calc::heating_operating_mode_heat_assignment_snapshot_route(
            predecessor,
        )?;
    let route = heating_mode_guard_else_branch_entry_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    prefix_and_local_shape_match(snapshot, predecessor, route).then_some(route)
}

pub(super) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp432_snapshot(snapshot);
    let predecessor_route = predecessor_route(route);
    crate::ideal_loads::calc::heating_operating_mode_heat_assignment_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && heating_mode_guard_else_branch_entry_route_from_committed_predecessor(
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
    route: Route,
) -> bool {
    crate::ideal_loads::heating_operating_mode_heat_assignment_snapshots_match_bit_exact(
        predecessor_cp432_snapshot(snapshot),
        predecessor,
    ) && local_shape_is_exact(snapshot, predecessor, route)
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor, route: Route) -> bool {
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.heating_mode_guard_else_branch_entered == route.entered
        && route.entered == predecessor.heating_mode_guard_false_fallthrough
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
        && crate::ideal_loads::heating_operating_mode_heat_assignment_snapshots_match_bit_exact(
            predecessor_cp432_snapshot(left),
            predecessor_cp432_snapshot(right),
        )
        && left.heating_mode_guard_else_branch_entered
            == right.heating_mode_guard_else_branch_entered
}

fn predecessor_route(route: Route) -> PredecessorRoute {
    PredecessorRoute {
        logical_index: route.logical_index,
        predecessor_active: route.predecessor_active,
        predecessor_assignment_executed: route.predecessor_assignment_executed,
        predecessor_entered: route.predecessor_entered,
        predecessor_total_output_assignment_executed: route
            .predecessor_total_output_assignment_executed,
        predecessor_heating_or_no_load_case_entered: route
            .predecessor_heating_or_no_load_case_entered,
        predecessor_heating_mode_guard_evaluated: route
            .predecessor_heating_mode_guard_evaluated,
        predecessor_sensible_comparison_satisfied: route
            .predecessor_sensible_comparison_satisfied,
        predecessor_single_cool_blocked: route.predecessor_single_cool_blocked,
        predecessor_heating_operating_mode_body_entered: route
            .predecessor_heating_operating_mode_body_entered,
        predecessor_heating_mode_guard_false_fallthrough: route
            .predecessor_heating_mode_guard_false_fallthrough,
        assignment_executed: route.assignment_executed,
    }
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
