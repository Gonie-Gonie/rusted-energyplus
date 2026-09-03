//! Exact CP439 snapshot and retained-route validation.

use super::prefix::predecessor_cp438_snapshot;
use super::{Predecessor, Snapshot};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallRetainedRoute as Route,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRetainedRoute as PredecessorRoute,
    heating_outdoor_air_maximum_flow_first_warning_call_route_from_committed_predecessor,
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot_route,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_CALL_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_CALL_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_CALL_SOURCE_ORDER as ORDER,
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot_is_exact,
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshots_match_bit_exact,
};

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    let predecessor = predecessor_cp438_snapshot(snapshot);
    let Some(predecessor_route) =
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot_route(predecessor)
    else {
        return false;
    };
    let Some(route) =
        heating_outdoor_air_maximum_flow_first_warning_call_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )
    else {
        return false;
    };
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot_is_exact(predecessor)
        && prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route)
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp438_snapshot(snapshot);
    let predecessor_route =
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot_route(predecessor)?;
    let route = heating_outdoor_air_maximum_flow_first_warning_call_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp438_snapshot(snapshot);
    let Some(predecessor_route) =
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot_route(predecessor)
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
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshots_match_bit_exact(
            predecessor_cp438_snapshot(snapshot),
            predecessor,
        )
        && heating_outdoor_air_maximum_flow_first_warning_call_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        ) == Some(route)
        && snapshot.heating_outdoor_air_maximum_flow_first_warning_call_site_reached
            == route.first_warning_call_site_reached
}

pub(super) fn snapshots_match_bit_exact(left: Snapshot, right: Snapshot) -> bool {
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshots_match_bit_exact(
        predecessor_cp438_snapshot(left),
        predecessor_cp438_snapshot(right),
    ) && left.heating_outdoor_air_maximum_flow_first_warning_call_site_reached
        == right.heating_outdoor_air_maximum_flow_first_warning_call_site_reached
}
