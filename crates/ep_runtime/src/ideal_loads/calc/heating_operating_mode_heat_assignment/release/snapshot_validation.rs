//! Exact CP432 prefix, route, local-shape, and bitwise validation.

use super::prefix::predecessor_cp431_snapshot;
use super::super::{
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Snapshot,
};
use super::super::transition::{
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentRetainedRoute as Route,
    heating_operating_mode_heat_assignment_route_from_committed_predecessor,
};
use crate::ideal_loads::calc::{
    IdealLoadsSensibleMode,
    PurchasedAirCalcHeatingModeGuardRetainedRoute as PredecessorRoute,
    heating_mode_guard_snapshot_route,
};
use crate::ideal_loads::PurchasedAirCalcHeatingModeGuardSnapshot as Predecessor;

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
        && crate::ideal_loads::heating_mode_guard_snapshot_is_exact(
            predecessor_cp431_snapshot(snapshot),
        )
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp431_snapshot(snapshot);
    let predecessor_route = heating_mode_guard_snapshot_route(predecessor)?;
    let route = heating_operating_mode_heat_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    prefix_and_local_shape_match(snapshot, predecessor, route).then_some(route)
}

pub(super) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp431_snapshot(snapshot);
    heating_operating_mode_heat_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
        && local_shape_is_exact(snapshot, predecessor, route)
}

pub(super) fn retained_route_matches_prior_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    retained_route_matches_snapshot_bounded(
        snapshot,
        PredecessorRoute {
            logical_index: route.logical_index,
            predecessor_active: route.predecessor_active,
            predecessor_assignment_executed: route.predecessor_assignment_executed,
            predecessor_entered: route.predecessor_entered,
            predecessor_total_output_assignment_executed: route
                .predecessor_total_output_assignment_executed,
            predecessor_heating_or_no_load_case_entered: route
                .predecessor_heating_or_no_load_case_entered,
            guard_evaluated: route.predecessor_heating_mode_guard_evaluated,
            sensible_comparison_satisfied: route.predecessor_sensible_comparison_satisfied,
            single_cool_blocked: route.predecessor_single_cool_blocked,
            body_entered: route.predecessor_heating_operating_mode_body_entered,
            false_fallthrough: route.predecessor_heating_mode_guard_false_fallthrough,
        },
        route,
    )
}

pub(super) fn prefix_and_local_shape_match(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    crate::ideal_loads::heating_mode_guard_snapshots_match_bit_exact(
        predecessor_cp431_snapshot(snapshot),
        predecessor,
    ) && local_shape_is_exact(snapshot, predecessor, route)
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor, route: Route) -> bool {
    let assignment = route.assignment_executed;
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.heating_operating_mode_heat_assignment_executed == assignment
        && snapshot.cp431_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp431_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp431_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.heating_operating_mode_heat_assignment_performed == assignment
        && snapshot.assigned_heating_operating_mode
            == assignment.then_some(IdealLoadsSensibleMode::Heating)
        && same(
            snapshot.predecessor_cp431_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.predecessor_cp431_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            snapshot.predecessor_cp431_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
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
        && crate::ideal_loads::heating_mode_guard_snapshots_match_bit_exact(
            predecessor_cp431_snapshot(left),
            predecessor_cp431_snapshot(right),
        )
        && left.heating_operating_mode_heat_assignment_executed
            == right.heating_operating_mode_heat_assignment_executed
        && left.cp431_retained_supply_humidity_ratio_state_owned
            == right.cp431_retained_supply_humidity_ratio_state_owned
        && left.cp431_retained_supply_enthalpy_state_owned
            == right.cp431_retained_supply_enthalpy_state_owned
        && left.cp431_retained_supply_temperature_state_owned
            == right.cp431_retained_supply_temperature_state_owned
        && left.heating_operating_mode_heat_assignment_performed
            == right.heating_operating_mode_heat_assignment_performed
        && left.assigned_heating_operating_mode == right.assigned_heating_operating_mode
        && same(
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        )
        && same(
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        )
        && same(
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        )
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
