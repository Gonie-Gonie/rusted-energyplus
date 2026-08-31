//! Exact CP435 prefix, route, local-shape, enum, and bitwise validation.

use ep_model::IdealLoadsLimit;

use super::super::transition::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRetainedRoute as Route,
    heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor,
};
use super::super::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE_ORDER as ORDER,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Snapshot,
};
use super::prefix::predecessor_cp434_snapshot;
use crate::ideal_loads::calc::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRetainedRoute as PredecessorRoute;
use crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot as Predecessor;

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
        && crate::ideal_loads::heating_operating_mode_deadband_assignment_snapshot_is_exact(
            predecessor_cp434_snapshot(snapshot),
        )
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp434_snapshot(snapshot);
    let predecessor_route =
        crate::ideal_loads::calc::heating_operating_mode_deadband_assignment_snapshot_route(
            predecessor,
        )?;
    let (heating_limit, outdoor, maximum) = retained_inputs(snapshot)?;
    let route = heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
        heating_limit,
        outdoor,
        maximum,
    )?;
    prefix_and_local_shape_match(snapshot, predecessor, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp434_snapshot(snapshot);
    let predecessor_route = predecessor_route(route);
    let Some((heating_limit, outdoor, maximum)) = retained_inputs(snapshot) else {
        return false;
    };
    crate::ideal_loads::calc::heating_operating_mode_deadband_assignment_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
        heating_limit,
        outdoor,
        maximum,
    ) == Some(route)
        && local_shape_is_exact(snapshot, predecessor, route)
}

pub(super) fn retained_route_matches_prior_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    retained_route_matches_snapshot_bounded(snapshot, route)
}

#[allow(dead_code)]
pub(super) fn committed_prefix_and_local_route_shape_match(
    snapshot: Snapshot,
    predecessor: Predecessor,
    predecessor_route: PredecessorRoute,
    route: Route,
) -> bool {
    let Some((heating_limit, outdoor, maximum)) = retained_inputs(snapshot) else {
        return false;
    };
    crate::ideal_loads::heating_operating_mode_deadband_assignment_snapshots_match_bit_exact(
        predecessor_cp434_snapshot(snapshot),
        predecessor,
    ) && crate::ideal_loads::calc::heating_operating_mode_deadband_assignment_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
        heating_limit,
        outdoor,
        maximum,
    ) == Some(route)
        && local_shape_is_exact(snapshot, predecessor, route)
}

pub(super) fn prefix_and_local_shape_match(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    crate::ideal_loads::heating_operating_mode_deadband_assignment_snapshots_match_bit_exact(
        predecessor_cp434_snapshot(snapshot),
        predecessor,
    ) && local_shape_is_exact(snapshot, predecessor, route)
}

fn retained_inputs(snapshot: Snapshot) -> Option<(IdealLoadsLimit, f64, f64)> {
    if !snapshot.heating_outdoor_air_maximum_flow_guard_evaluated {
        return Some((IdealLoadsLimit::NoLimit, 0.0, 0.0));
    }
    let heating_limit = snapshot.heating_limit_flow_rate_value?;
    if snapshot
        .outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated
    {
        Some((
            heating_limit,
            snapshot.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s?,
            snapshot.maximum_heating_air_mass_flow_rate_for_guard_kg_per_s?,
        ))
    } else {
        Some((heating_limit, 0.0, 0.0))
    }
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor, route: Route) -> bool {
    let first_value = snapshot.heating_limit_flow_rate_value;
    let second_value = snapshot.heating_limit_flow_rate_and_capacity_value;
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && same(snapshot.predecessor_cp434_resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        && same(snapshot.predecessor_cp434_resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        && same(snapshot.predecessor_cp434_resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        && snapshot.heating_outdoor_air_maximum_flow_guard_evaluated == route.guard_evaluated
        && snapshot.heating_limit_flow_rate_comparison_evaluated == route.guard_evaluated
        && first_value.is_some() == route.guard_evaluated
        && snapshot.heating_limit_flow_rate_comparison_satisfied
            == route.guard_evaluated.then_some(route.heating_limit_flow_rate_comparison_satisfied)
        && snapshot.heating_limit_flow_rate_and_capacity_comparison_evaluated
            == route.heating_limit_flow_rate_and_capacity_comparison_evaluated
        && second_value.is_some()
            == route.heating_limit_flow_rate_and_capacity_comparison_evaluated
        && (!route.heating_limit_flow_rate_and_capacity_comparison_evaluated
            || second_value == first_value)
        && snapshot.heating_limit_flow_rate_and_capacity_comparison_satisfied
            == route.heating_limit_flow_rate_and_capacity_comparison_evaluated
                .then_some(route.heating_limit_flow_rate_and_capacity_comparison_satisfied)
        && snapshot.heating_flow_limit_active
            == route.guard_evaluated.then_some(route.heating_flow_limit_active)
        && snapshot.heating_flow_limit_selector_rejected
            == route.heating_flow_limit_selector_rejected
        && snapshot.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated
            == route.strict_mass_flow_comparison_evaluated
        && snapshot.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit
            == route.strict_mass_flow_comparison_evaluated
        && snapshot.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit
            == route.strict_mass_flow_comparison_evaluated
        && snapshot.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s.is_some()
            == route.strict_mass_flow_comparison_evaluated
        && snapshot.maximum_heating_air_mass_flow_rate_for_guard_kg_per_s.is_some()
            == route.strict_mass_flow_comparison_evaluated
        && snapshot.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated
            == route.strict_mass_flow_comparison_evaluated
        && snapshot.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate
            == route.strict_mass_flow_comparison_evaluated.then_some(route.body_entered)
        && snapshot.maximum_heating_flow_body_entered == route.body_entered
        && snapshot.heating_outdoor_air_maximum_flow_guard_false_fallthrough
            == route.false_fallthrough
        && snapshot.cp434_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp434_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp434_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && same(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        && same(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        && same(snapshot.resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
}

pub(super) fn snapshots_match_bit_exact(left: Snapshot, right: Snapshot) -> bool {
    left.source == right.source
        && left.first_excluded_source == right.first_excluded_source
        && left.source_order == right.source_order
        && crate::ideal_loads::heating_operating_mode_deadband_assignment_snapshots_match_bit_exact(
            predecessor_cp434_snapshot(left),
            predecessor_cp434_snapshot(right),
        )
        && left.heating_outdoor_air_maximum_flow_guard_evaluated == right.heating_outdoor_air_maximum_flow_guard_evaluated
        && left.heating_limit_flow_rate_comparison_evaluated == right.heating_limit_flow_rate_comparison_evaluated
        && left.heating_limit_flow_rate_value == right.heating_limit_flow_rate_value
        && left.heating_limit_flow_rate_comparison_satisfied == right.heating_limit_flow_rate_comparison_satisfied
        && left.heating_limit_flow_rate_and_capacity_comparison_evaluated == right.heating_limit_flow_rate_and_capacity_comparison_evaluated
        && left.heating_limit_flow_rate_and_capacity_value == right.heating_limit_flow_rate_and_capacity_value
        && left.heating_limit_flow_rate_and_capacity_comparison_satisfied == right.heating_limit_flow_rate_and_capacity_comparison_satisfied
        && left.heating_flow_limit_active == right.heating_flow_limit_active
        && left.heating_flow_limit_selector_rejected == right.heating_flow_limit_selector_rejected
        && left.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated == right.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated
        && left.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit == right.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit
        && same(left.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s, right.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s)
        && left.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit == right.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit
        && same(left.maximum_heating_air_mass_flow_rate_for_guard_kg_per_s, right.maximum_heating_air_mass_flow_rate_for_guard_kg_per_s)
        && left.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated == right.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated
        && left.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate == right.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate
        && left.maximum_heating_flow_body_entered == right.maximum_heating_flow_body_entered
        && left.heating_outdoor_air_maximum_flow_guard_false_fallthrough == right.heating_outdoor_air_maximum_flow_guard_false_fallthrough
        && left.cp434_retained_supply_humidity_ratio_state_owned == right.cp434_retained_supply_humidity_ratio_state_owned
        && left.cp434_retained_supply_enthalpy_state_owned == right.cp434_retained_supply_enthalpy_state_owned
        && left.cp434_retained_supply_temperature_state_owned == right.cp434_retained_supply_temperature_state_owned
        && same(left.resulting_supply_humidity_ratio, right.resulting_supply_humidity_ratio)
        && same(left.resulting_supply_enthalpy_j_per_kg, right.resulting_supply_enthalpy_j_per_kg)
        && same(left.resulting_supply_temperature_c, right.resulting_supply_temperature_c)
}

fn predecessor_route(route: Route) -> PredecessorRoute {
    PredecessorRoute {
        logical_index: route.logical_index,
        predecessor_active: route.predecessor_active,
        predecessor_assignment_executed: route.predecessor_assignment_executed,
        predecessor_entered: route.predecessor_entered,
        predecessor_total_output_assignment_executed: route.predecessor_total_output_assignment_executed,
        predecessor_heating_or_no_load_case_entered: route.predecessor_heating_or_no_load_case_entered,
        predecessor_heating_mode_guard_evaluated: route.predecessor_heating_mode_guard_evaluated,
        predecessor_sensible_comparison_satisfied: route.predecessor_sensible_comparison_satisfied,
        predecessor_single_cool_blocked: route.predecessor_single_cool_blocked,
        predecessor_heating_operating_mode_body_entered: route.predecessor_heating_operating_mode_body_entered,
        predecessor_heating_mode_guard_false_fallthrough: route.predecessor_heating_mode_guard_false_fallthrough,
        predecessor_heating_operating_mode_heat_assignment_executed: route.predecessor_heating_operating_mode_heat_assignment_executed,
        predecessor_heating_mode_guard_else_branch_entered: route.predecessor_heating_mode_guard_else_branch_entered,
        assignment_executed: route.predecessor_heating_operating_mode_deadband_assignment_executed,
    }
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
