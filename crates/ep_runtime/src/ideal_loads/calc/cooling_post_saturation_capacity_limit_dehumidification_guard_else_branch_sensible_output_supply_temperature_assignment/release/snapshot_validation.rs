//! Exact CP423 prefix, local IEEE-AST, and bitwise validation.

use super::prefix::predecessor_cp422_snapshot;
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
};
use super::super::transition::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRetainedRoute as Route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRetainedRoute as PredecessorRoute;

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
        && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshot_is_exact(
            predecessor_cp422_snapshot(snapshot),
        )
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp422_snapshot(snapshot);
    let predecessor_route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshot_route(predecessor)?;
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    prefix_and_local_shape_match(snapshot, predecessor, route).then_some(route)
}

pub(super) fn retained_route_matches_snapshot_bounded(snapshot: Snapshot, route: Route) -> bool {
    let predecessor = predecessor_cp422_snapshot(snapshot);
    let predecessor_route = PredecessorRoute {
        logical_index: route.logical_index,
        active: route.active,
        assignment_executed: route.assignment_executed,
    };
    route.logical_index < 36
        && crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_retained_route_matches_snapshot_bounded(
            predecessor,
            predecessor_route,
        )
        && local_shape_is_exact(snapshot, predecessor, route)
}

pub(super) fn prefix_and_local_shape_match(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshots_match_bit_exact(
        predecessor_cp422_snapshot(snapshot),
        predecessor,
    ) && local_shape_is_exact(snapshot, predecessor, route)
}

fn local_shape_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    let assignment = route.assignment_executed;
    let markers = [
        snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_executed,
        snapshot.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read,
        snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_read,
        snapshot.cp422_retained_cooling_sensible_output_owned_read,
        snapshot.cooling_sensible_output_for_supply_temperature_read,
        snapshot.cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read,
        snapshot.cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroborated,
        snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_read,
        snapshot.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read,
        snapshot.cp_air_for_sensible_output_supply_temperature_read,
        snapshot.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculated,
        snapshot.cooling_sensible_output_over_air_capacity_rate_calculated,
        snapshot.sensible_output_supply_temperature_calculated,
        snapshot.sensible_output_supply_temperature_assignment_performed,
    ];
    if snapshot.source != SOURCE
        || snapshot.first_excluded_source != EXCLUDED
        || snapshot.source_order != ORDER
        || snapshot.system != predecessor.system
        || snapshot.parent_call_ordinal != predecessor.parent_call_ordinal
        || snapshot.controlled_zone != predecessor.controlled_zone
        || markers.into_iter().any(|marker| marker != assignment)
        || snapshot.cp422_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp422_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp422_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
        || !same(snapshot.predecessor_cp422_resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !same(snapshot.predecessor_cp422_resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
        || !same(snapshot.predecessor_cp422_resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c)
        || !same(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio)
        || !same(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg)
    {
        return false;
    }
    if !assignment {
        return [
            snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_c,
            snapshot.cooling_sensible_output_for_supply_temperature_w,
            snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s,
            snapshot.cp_air_for_sensible_output_supply_temperature_j_per_kg_k,
            snapshot.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k,
            snapshot.cooling_sensible_output_over_air_capacity_rate_k,
            snapshot.calculated_sensible_output_supply_temperature_c,
            snapshot.assigned_sensible_output_supply_temperature_c,
        ]
        .into_iter()
        .all(|value| value.is_none())
            && same(snapshot.resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c);
    }
    let (Some(mixed), Some(output), Some(flow), Some(cp_air), Some(denominator), Some(drop), Some(calculated), Some(assigned)) = (
        snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_c,
        snapshot.cooling_sensible_output_for_supply_temperature_w,
        snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s,
        snapshot.cp_air_for_sensible_output_supply_temperature_j_per_kg_k,
        snapshot.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_w_per_k,
        snapshot.cooling_sensible_output_over_air_capacity_rate_k,
        snapshot.calculated_sensible_output_supply_temperature_c,
        snapshot.assigned_sensible_output_supply_temperature_c,
    ) else {
        return false;
    };
    let expected_denominator = flow * cp_air;
    let expected_drop = output / expected_denominator;
    let expected_calculated = mixed - expected_drop;
    same(Some(mixed), predecessor.mixed_air_temperature_for_sensible_output_c)
        && same(Some(output), predecessor.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w)
        && same(Some(flow), predecessor.supply_mass_flow_rate_kg_per_s)
        && same(Some(cp_air), predecessor.cp_air_j_per_kg_k)
        && denominator.to_bits() == expected_denominator.to_bits()
        && drop.to_bits() == expected_drop.to_bits()
        && calculated.to_bits() == expected_calculated.to_bits()
        && assigned.to_bits() == calculated.to_bits()
        && same(snapshot.resulting_supply_temperature_c, Some(assigned))
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
    clear!(resulting_supply_humidity_ratio);
    clear!(resulting_supply_enthalpy_j_per_kg);
    clear!(resulting_supply_temperature_c);
    left == right
}
