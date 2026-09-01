//! Exact CP437 snapshot and retained-route validation.

use super::prefix::predecessor_cp436_snapshot;
use super::{Predecessor, Snapshot};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute as PredecessorRoute,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRetainedRoute as Route,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_route,
    heating_outdoor_air_maximum_flow_first_warning_guard_route_from_committed_predecessor,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE_ORDER as ORDER,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_is_exact,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact,
};

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    let predecessor = predecessor_cp436_snapshot(snapshot);
    let Some(predecessor_route) =
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_route(predecessor)
    else {
        return false;
    };
    let counter = snapshot
        .outdoor_air_flow_maximum_heating_output_error_count_before
        .unwrap_or(0);
    let Some(route) =
        heating_outdoor_air_maximum_flow_first_warning_guard_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
            counter,
        )
    else {
        return false;
    };
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_is_exact(predecessor)
        && prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route)
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp436_snapshot(snapshot);
    let predecessor_route =
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_route(predecessor)?;
    let counter = snapshot
        .outdoor_air_flow_maximum_heating_output_error_count_before
        .unwrap_or(0);
    let route =
        heating_outdoor_air_maximum_flow_first_warning_guard_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
            counter,
        )?;
    prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp436_snapshot(snapshot);
    let Some(predecessor_route) =
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_route(predecessor)
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
    let counter = snapshot
        .outdoor_air_flow_maximum_heating_output_error_count_before
        .unwrap_or(0);
    let exact_route =
        heating_outdoor_air_maximum_flow_first_warning_guard_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
            counter,
        );
    let execution = route.guard_evaluated;
    let expected_counter = execution.then_some(counter);
    let expected_comparison = expected_counter.map(|counter| counter < 1);
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact(
            predecessor_cp436_snapshot(snapshot),
            predecessor,
        )
        && exact_route == Some(route)
        && snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_evaluated == execution
        && snapshot.cp436_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp436_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp436_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.outdoor_air_flow_maximum_heating_output_error_count_state_owned == execution
        && snapshot.outdoor_air_flow_maximum_heating_output_error_count_read == execution
        && snapshot.outdoor_air_flow_maximum_heating_output_error_count_before == expected_counter
        && snapshot
            .outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated
            == execution
        && snapshot.outdoor_air_flow_maximum_heating_output_error_count_less_than_one
            == expected_comparison
        && snapshot.heating_outdoor_air_maximum_flow_first_warning_branch_entered
            == route.first_warning_branch_entered
        && snapshot.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
            == route.guard_false_fallthrough
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
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact(
        predecessor_cp436_snapshot(left),
        predecessor_cp436_snapshot(right),
    ) && left.heating_outdoor_air_maximum_flow_first_warning_guard_evaluated
        == right.heating_outdoor_air_maximum_flow_first_warning_guard_evaluated
        && left.cp436_retained_supply_humidity_ratio_state_owned
            == right.cp436_retained_supply_humidity_ratio_state_owned
        && left.cp436_retained_supply_enthalpy_state_owned
            == right.cp436_retained_supply_enthalpy_state_owned
        && left.cp436_retained_supply_temperature_state_owned
            == right.cp436_retained_supply_temperature_state_owned
        && left.outdoor_air_flow_maximum_heating_output_error_count_state_owned
            == right.outdoor_air_flow_maximum_heating_output_error_count_state_owned
        && left.outdoor_air_flow_maximum_heating_output_error_count_read
            == right.outdoor_air_flow_maximum_heating_output_error_count_read
        && left.outdoor_air_flow_maximum_heating_output_error_count_before
            == right.outdoor_air_flow_maximum_heating_output_error_count_before
        && left
            .outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated
            == right
                .outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated
        && left.outdoor_air_flow_maximum_heating_output_error_count_less_than_one
            == right.outdoor_air_flow_maximum_heating_output_error_count_less_than_one
        && left.heating_outdoor_air_maximum_flow_first_warning_branch_entered
            == right.heating_outdoor_air_maximum_flow_first_warning_branch_entered
        && left.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
            == right.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough
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
