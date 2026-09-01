//! Exact CP438 snapshot and retained-route validation.

use super::prefix::predecessor_cp437_snapshot;
use super::{Predecessor, Snapshot};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRetainedRoute as Route,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute as PredecessorRoute,
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor,
    heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_route,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE_ORDER as ORDER,
    heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_is_exact,
    heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact,
};

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    let predecessor = predecessor_cp437_snapshot(snapshot);
    let Some(predecessor_route) =
        heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_route(predecessor)
    else {
        return false;
    };
    let Some(route) =
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )
    else {
        return false;
    };
    heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_is_exact(predecessor)
        && prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route)
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp437_snapshot(snapshot);
    let predecessor_route =
        heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_route(predecessor)?;
    let route =
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )?;
    prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp437_snapshot(snapshot);
    let Some(predecessor_route) =
        heating_outdoor_air_maximum_flow_first_warning_guard_snapshot_route(predecessor)
    else {
        return false;
    };
    prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route)
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
    let exact_route =
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        );
    let increment = route.counter_increment_executed;
    let expected_assigned_counter = increment.then_some(1);
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact(
            predecessor_cp437_snapshot(snapshot),
            predecessor,
        )
        && exact_route == Some(route)
        && option_bits_eq(
            snapshot.predecessor_cp437_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_eq(
            snapshot.predecessor_cp437_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_eq(
            snapshot.predecessor_cp437_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && snapshot.heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed
            == increment
        && snapshot.cp437_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp437_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp437_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.cp437_retained_outdoor_air_flow_maximum_heating_output_error_count_state_owned
            == increment
        && snapshot.outdoor_air_flow_maximum_heating_output_error_count_increment_performed
            == increment
        && snapshot.assigned_outdoor_air_flow_maximum_heating_output_error_count
            == expected_assigned_counter
        && option_bits_eq(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_eq(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_eq(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

pub(super) fn snapshots_match_bit_exact(left: Snapshot, right: Snapshot) -> bool {
    heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact(
        predecessor_cp437_snapshot(left),
        predecessor_cp437_snapshot(right),
    ) && option_bits_eq(
        left.predecessor_cp437_resulting_supply_humidity_ratio,
        right.predecessor_cp437_resulting_supply_humidity_ratio,
    ) && option_bits_eq(
        left.predecessor_cp437_resulting_supply_enthalpy_j_per_kg,
        right.predecessor_cp437_resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_eq(
        left.predecessor_cp437_resulting_supply_temperature_c,
        right.predecessor_cp437_resulting_supply_temperature_c,
    ) && left.heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed
        == right.heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed
        && left.cp437_retained_supply_humidity_ratio_state_owned
            == right.cp437_retained_supply_humidity_ratio_state_owned
        && left.cp437_retained_supply_enthalpy_state_owned
            == right.cp437_retained_supply_enthalpy_state_owned
        && left.cp437_retained_supply_temperature_state_owned
            == right.cp437_retained_supply_temperature_state_owned
        && left.cp437_retained_outdoor_air_flow_maximum_heating_output_error_count_state_owned
            == right.cp437_retained_outdoor_air_flow_maximum_heating_output_error_count_state_owned
        && left.outdoor_air_flow_maximum_heating_output_error_count_increment_performed
            == right.outdoor_air_flow_maximum_heating_output_error_count_increment_performed
        && left.assigned_outdoor_air_flow_maximum_heating_output_error_count
            == right.assigned_outdoor_air_flow_maximum_heating_output_error_count
        && option_bits_eq(
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        )
        && option_bits_eq(
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_eq(
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        )
}

fn option_bits_eq(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
