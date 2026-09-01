//! Exact CP436 snapshot and retained-route validation.

use super::prefix::predecessor_cp435_snapshot;
use super::{Predecessor, Snapshot};
use crate::ideal_loads::calc::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute as Route,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRetainedRoute as PredecessorRoute,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor,
    heating_outdoor_air_maximum_flow_guard_snapshot_route,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE_ORDER as ORDER,
    heating_outdoor_air_maximum_flow_guard_snapshot_is_exact,
    heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact,
};

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    let predecessor = predecessor_cp435_snapshot(snapshot);
    let Some(predecessor_route) = heating_outdoor_air_maximum_flow_guard_snapshot_route(predecessor)
    else {
        return false;
    };
    let Some(route) =
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )
    else {
        return false;
    };
    heating_outdoor_air_maximum_flow_guard_snapshot_is_exact(predecessor)
        && prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route)
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp435_snapshot(snapshot);
    let predecessor_route = heating_outdoor_air_maximum_flow_guard_snapshot_route(predecessor)?;
    let route =
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )?;
    prefix_and_local_shape_match(snapshot, predecessor, predecessor_route, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp435_snapshot(snapshot);
    let Some(predecessor_route) = heating_outdoor_air_maximum_flow_guard_snapshot_route(predecessor)
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
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        );
    let execution = route.assignment_executed;
    let numerator = predecessor
        .outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s;
    let density = snapshot.standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3;
    let calculated = match (execution, numerator, density) {
        (true, Some(numerator), Some(density)) if density.is_finite() && density > 0.0 => {
            Some(numerator / density)
        }
        (false, _, None) => None,
        _ => return false,
    };
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact(
            predecessor_cp435_snapshot(snapshot),
            predecessor,
        )
        && exact_route == Some(route)
        && snapshot.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed
            == execution
        && snapshot.cp435_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp435_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp435_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.cp435_retained_outdoor_air_mass_flow_rate_owned_read == execution
        && snapshot.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read
            == execution
        && option_bits_eq(
            snapshot.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s,
            execution.then_some(numerator).flatten(),
        )
        && snapshot.begin_environment_standard_air_density_owned_read == execution
        && snapshot.standard_air_density_for_outdoor_air_volume_flow_division_read == execution
        && snapshot.outdoor_air_mass_flow_rate_standard_air_density_division_evaluated
            == execution
        && option_bits_eq(
            snapshot.calculated_outdoor_air_volume_flow_rate_m3_per_s,
            calculated,
        )
        && snapshot.local_outdoor_air_volume_flow_rate_assignment_performed == execution
        && option_bits_eq(snapshot.assigned_outdoor_air_volume_flow_rate_m3_per_s, calculated)
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
    heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact(
        predecessor_cp435_snapshot(left),
        predecessor_cp435_snapshot(right),
    ) && left.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed
        == right.heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed
        && left.cp435_retained_supply_humidity_ratio_state_owned
            == right.cp435_retained_supply_humidity_ratio_state_owned
        && left.cp435_retained_supply_enthalpy_state_owned
            == right.cp435_retained_supply_enthalpy_state_owned
        && left.cp435_retained_supply_temperature_state_owned
            == right.cp435_retained_supply_temperature_state_owned
        && left.cp435_retained_outdoor_air_mass_flow_rate_owned_read
            == right.cp435_retained_outdoor_air_mass_flow_rate_owned_read
        && left.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read
            == right.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read
        && option_bits_eq(
            left.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s,
            right.outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s,
        )
        && left.begin_environment_standard_air_density_owned_read
            == right.begin_environment_standard_air_density_owned_read
        && left.standard_air_density_for_outdoor_air_volume_flow_division_read
            == right.standard_air_density_for_outdoor_air_volume_flow_division_read
        && option_bits_eq(
            left.standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3,
            right.standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3,
        )
        && left.outdoor_air_mass_flow_rate_standard_air_density_division_evaluated
            == right.outdoor_air_mass_flow_rate_standard_air_density_division_evaluated
        && option_bits_eq(
            left.calculated_outdoor_air_volume_flow_rate_m3_per_s,
            right.calculated_outdoor_air_volume_flow_rate_m3_per_s,
        )
        && left.local_outdoor_air_volume_flow_rate_assignment_performed
            == right.local_outdoor_air_volume_flow_rate_assignment_performed
        && option_bits_eq(
            left.assigned_outdoor_air_volume_flow_rate_m3_per_s,
            right.assigned_outdoor_air_volume_flow_rate_m3_per_s,
        )
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
