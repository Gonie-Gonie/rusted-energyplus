//! Exact CP427 prefix, route, assignment, and bitwise validation.

use super::prefix::predecessor_cp426_snapshot;
use super::super::{
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_ZERO_SUPPLY_MASS_FLOW_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot as Snapshot,
};
use super::super::transition::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentRetainedRoute as Route,
    cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_from_committed_predecessor,
};

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
        && crate::ideal_loads::cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact(
            predecessor_cp426_snapshot(snapshot),
        )
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp426_snapshot(snapshot);
    let predecessor_route =
        crate::ideal_loads::calc::cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_snapshot_route(
            predecessor,
        )?;
    let route =
        cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_from_committed_predecessor(
            predecessor,
            predecessor_route,
        )?;
    prefix_and_local_shape_match(snapshot, predecessor, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp426_snapshot(snapshot);
    let predecessor_route =
        crate::ideal_loads::calc::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentRetainedRoute {
            logical_index: route.logical_index,
            active: route.active,
            predecessor_assignment_executed: route.predecessor_assignment_executed,
            predecessor_entered: route.predecessor_entered,
            assignment_executed: route.assignment_executed,
        };
    crate::ideal_loads::calc::cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    ) == Some(route)
        && local_shape_is_exact(snapshot, predecessor, route)
}

pub(super) fn prefix_and_local_shape_match(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    route: Route,
) -> bool {
    crate::ideal_loads::cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_snapshots_match_bit_exact(
        predecessor_cp426_snapshot(snapshot),
        predecessor,
    ) && local_shape_is_exact(snapshot, predecessor, route)
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    route: Route,
) -> bool {
    let assigned = snapshot.assigned_supply_temperature_from_mixed_air_c;
    let expected_temperature = if route.assignment_executed {
        assigned
    } else {
        predecessor.resulting_supply_temperature_c
    };
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && route.active == route.assignment_executed
        && route.predecessor_entered
            == predecessor.cooling_supply_mass_flow_positive_guard_else_branch_entered
        && predecessor
            .cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed
            == route.assignment_executed
        && snapshot
            .cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_executed
            == route.assignment_executed
        && snapshot.cp426_retained_supply_humidity_ratio_state_owned
            == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp426_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp426_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
        && snapshot.cp329_retained_mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_owned_read
            == route.assignment_executed
        && snapshot.mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_read
            == route.assignment_executed
        && snapshot.zero_supply_mass_flow_supply_temperature_mixed_air_assignment_performed
            == route.assignment_executed
        && (route.assignment_executed
            == snapshot
                .mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c
                .is_some())
        && same(
            snapshot.mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c,
            assigned,
        )
        && same(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && same(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && same(snapshot.resulting_supply_temperature_c, expected_temperature)
}

fn same(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

pub(super) fn snapshots_match_bit_exact(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! clear {
        ($field:ident) => {
            if !same(left.$field, right.$field) {
                return false;
            }
            left.$field = None;
            right.$field = None;
        };
    }
    clear!(predecessor_cp409_resulting_supply_humidity_ratio);
    clear!(predecessor_cp409_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp409_resulting_supply_temperature_c);
    clear!(predecessor_cp410_resulting_supply_humidity_ratio);
    clear!(predecessor_cp410_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp410_resulting_supply_temperature_c);
    clear!(purchased_air_supply_humidity_ratio_before_saturation_check);
    clear!(assigned_supply_humidity_ratio_original);
    clear!(resulting_supply_humidity_ratio_original);
    clear!(predecessor_cp411_resulting_supply_humidity_ratio);
    clear!(predecessor_cp411_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp411_resulting_supply_temperature_c);
    clear!(supply_temperature_for_saturation_humidity_ratio_c);
    clear!(outdoor_barometric_pressure_pa);
    clear!(saturation_supply_humidity_ratio);
    clear!(assigned_saturation_supply_humidity_ratio);
    clear!(resulting_saturation_supply_humidity_ratio);
    clear!(predecessor_cp412_resulting_supply_humidity_ratio);
    clear!(predecessor_cp412_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp412_resulting_supply_temperature_c);
    clear!(saturation_supply_humidity_ratio_for_guard);
    clear!(original_supply_humidity_ratio_for_guard);
    clear!(predecessor_cp413_resulting_supply_humidity_ratio);
    clear!(predecessor_cp413_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp413_resulting_supply_temperature_c);
    clear!(supply_enthalpy_for_saturation_temperature_j_per_kg);
    clear!(outdoor_barometric_pressure_for_saturation_temperature_pa);
    clear!(psychrometric_saturation_supply_temperature_result_c);
    clear!(assigned_saturation_supply_temperature_c);
    clear!(predecessor_cp414_resulting_supply_humidity_ratio);
    clear!(predecessor_cp414_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp414_resulting_supply_temperature_c);
    clear!(preexisting_supply_temperature_c);
    clear!(supply_temperature_before_mixed_air_limit_c);
    clear!(mixed_air_temperature_c);
    clear!(minimum_supply_temperature_c);
    clear!(assigned_supply_temperature_c);
    clear!(predecessor_cp415_resulting_supply_humidity_ratio);
    clear!(predecessor_cp415_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp415_resulting_supply_temperature_c);
    clear!(supply_temperature_c);
    clear!(supply_enthalpy_j_per_kg);
    clear!(psychrometric_supply_humidity_ratio);
    clear!(assigned_supply_humidity_ratio);
    clear!(predecessor_cp416_resulting_supply_humidity_ratio);
    clear!(predecessor_cp416_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp416_resulting_supply_temperature_c);
    clear!(supply_temperature_for_enthalpy_c);
    clear!(supply_humidity_ratio_for_enthalpy);
    clear!(psychrometric_supply_enthalpy_j_per_kg);
    clear!(assigned_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp418_resulting_supply_humidity_ratio);
    clear!(predecessor_cp418_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp418_resulting_supply_temperature_c);
    clear!(mixed_air_humidity_ratio_for_cp_air);
    clear!(psychrometric_cp_air_result_j_per_kg_k);
    clear!(cp_air_j_per_kg_k);
    clear!(predecessor_cp419_resulting_supply_humidity_ratio);
    clear!(predecessor_cp419_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp419_resulting_supply_temperature_c);
    clear!(supply_mass_flow_rate_kg_per_s);
    clear!(cp419_cp_air_for_sensible_output_j_per_kg_k);
    clear!(supply_mass_flow_rate_times_cp_air_w_per_k);
    clear!(mixed_air_temperature_for_sensible_output_c);
    clear!(supply_temperature_for_sensible_output_c);
    clear!(mixed_air_minus_supply_temperature_k);
    clear!(calculated_cooling_sensible_output_w);
    clear!(cooling_sensible_output_w);
    clear!(predecessor_cp420_resulting_supply_humidity_ratio);
    clear!(predecessor_cp420_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp420_resulting_supply_temperature_c);
    clear!(cp420_cooling_sensible_output_for_capacity_guard_w);
    clear!(maximum_total_cooling_capacity_w);
    clear!(predecessor_cp421_resulting_supply_humidity_ratio);
    clear!(predecessor_cp421_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp421_resulting_supply_temperature_c);
    clear!(preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w);
    clear!(maximum_total_cooling_capacity_for_sensible_output_assignment_w);
    clear!(assigned_cooling_sensible_output_from_maximum_capacity_w);
    clear!(resulting_cooling_sensible_output_after_maximum_capacity_assignment_w);
    clear!(predecessor_cp422_resulting_supply_humidity_ratio);
    clear!(predecessor_cp422_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp422_resulting_supply_temperature_c);
    clear!(mixed_air_temperature_for_sensible_output_supply_temperature_c);
    clear!(cooling_sensible_output_for_supply_temperature_w);
    clear!(supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s);
    clear!(cp_air_for_sensible_output_supply_temperature_j_per_kg_k);
    clear!(supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k);
    clear!(cooling_sensible_output_over_air_capacity_rate_k);
    clear!(calculated_sensible_output_supply_temperature_c);
    clear!(assigned_sensible_output_supply_temperature_c);
    clear!(predecessor_cp424_resulting_supply_humidity_ratio);
    clear!(predecessor_cp424_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp424_resulting_supply_temperature_c);
    clear!(mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg);
    clear!(assigned_supply_enthalpy_from_mixed_air_j_per_kg);
    clear!(predecessor_cp425_resulting_supply_humidity_ratio);
    clear!(predecessor_cp425_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp425_resulting_supply_temperature_c);
    clear!(mixed_air_humidity_ratio_for_zero_supply_mass_flow_supply_humidity_ratio_assignment);
    clear!(assigned_supply_humidity_ratio_from_mixed_air);
    clear!(predecessor_cp426_resulting_supply_humidity_ratio);
    clear!(predecessor_cp426_resulting_supply_enthalpy_j_per_kg);
    clear!(predecessor_cp426_resulting_supply_temperature_c);
    clear!(mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_c);
    clear!(assigned_supply_temperature_from_mixed_air_c);
    clear!(resulting_supply_humidity_ratio);
    clear!(resulting_supply_enthalpy_j_per_kg);
    clear!(resulting_supply_temperature_c);
    left == right
}
