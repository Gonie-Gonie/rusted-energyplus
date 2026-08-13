//! Exact CP421 local-shape and bitwise validation.

use super::prefix::predecessor_cp420_snapshot;
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Snapshot,
};
use super::super::transition::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute as Route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_route_from_committed_predecessor,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentCommittedRoute as Cp420Route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_route_matches_snapshot,
};

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
        && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_is_exact(
            predecessor_cp420_snapshot(snapshot),
        )
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp420_snapshot(snapshot);
    let predecessor_route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_route(predecessor)?;
    let mut route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    route.body_entered = snapshot
        .cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity
        == Some(true);
    local_shape_is_exact(snapshot, predecessor, route).then_some(route)
}

pub(super) fn prefix_and_local_shape_match(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact(
        predecessor_cp420_snapshot(snapshot),
        predecessor,
    ) && local_shape_is_exact(snapshot, predecessor, route)
}

pub(super) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp420_snapshot(snapshot);
    let cp420_route = Cp420Route {
        logical_index: route.logical_index,
        predecessor_guard_false_fallthrough: predecessor
            .saturation_supply_humidity_ratio_guard_false_fallthrough,
        predecessor_guard_body_entered: predecessor
            .saturation_supply_humidity_ratio_guard_body_entered,
        predecessor_saturation_temperature_assignment_executed: predecessor
            .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed,
        predecessor_saturation_temperature_mixed_air_limit_executed: predecessor
            .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed,
        predecessor_supply_humidity_ratio_assignment_executed: predecessor
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: predecessor
            .post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed,
        active: route.active,
    };
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_route_matches_snapshot(
        predecessor,
        cp420_route,
    ) && local_shape_is_exact(snapshot, predecessor, route)
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor, route: Route) -> bool {
    let active = route.active;
    let result =
        snapshot.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity;
    let supply_is_preserved = option_bits_match(
        snapshot.predecessor_cp420_resulting_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ) && option_bits_match(
        snapshot.predecessor_cp420_resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        snapshot.predecessor_cp420_resulting_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    );
    let owner_shape = snapshot.cp420_retained_supply_humidity_ratio_state_owned
        == snapshot.predecessor_cp420_resulting_supply_humidity_ratio.is_some()
        && snapshot.cp420_retained_supply_enthalpy_state_owned
            == snapshot.predecessor_cp420_resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp420_retained_supply_temperature_state_owned
            == snapshot.predecessor_cp420_resulting_supply_temperature_c.is_some();
    let provenance = snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER;
    if !provenance
        || !supply_is_preserved
        || !owner_shape
        || route.logical_index >= 36
        || route.body_entered != (result == Some(true))
        || active
            != predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed
    {
        return false;
    }
    if !active {
        return !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
            && !snapshot.cp420_retained_cooling_sensible_output_owned_read
            && !snapshot.cooling_sensible_output_read
            && snapshot.cp420_cooling_sensible_output_for_capacity_guard_w.is_none()
            && !snapshot.cp321_maximum_total_cooling_capacity_owned_read
            && !snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
            && !snapshot.maximum_total_cooling_capacity_read
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && !snapshot.cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated
            && result.is_none()
            && !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
            && !snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough;
    }
    let (Some(predecessor_output), Some(output), Some(capacity), Some(result)) = (
        predecessor.cooling_sensible_output_w,
        snapshot.cp420_cooling_sensible_output_for_capacity_guard_w,
        snapshot.maximum_total_cooling_capacity_w,
        result,
    ) else {
        return false;
    };
    snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
        && snapshot.cp420_retained_cooling_sensible_output_owned_read
        && snapshot.cooling_sensible_output_read
        && output.to_bits() == predecessor_output.to_bits()
        && snapshot.cp321_maximum_total_cooling_capacity_owned_read
        && snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
        && snapshot.maximum_total_cooling_capacity_read
        && snapshot.cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated
        && result == (output >= capacity)
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered == result
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough != result
}

pub(super) fn snapshots_match_bit_exact(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! clear {
        ($field:ident) => {{
            let matches = option_bits_match(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }
    let values_match = clear!(predecessor_cp409_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp409_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp409_resulting_supply_temperature_c)
        && clear!(predecessor_cp410_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp410_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp410_resulting_supply_temperature_c)
        && clear!(purchased_air_supply_humidity_ratio_before_saturation_check)
        && clear!(assigned_supply_humidity_ratio_original)
        && clear!(resulting_supply_humidity_ratio_original)
        && clear!(predecessor_cp411_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp411_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp411_resulting_supply_temperature_c)
        && clear!(supply_temperature_for_saturation_humidity_ratio_c)
        && clear!(outdoor_barometric_pressure_pa)
        && clear!(saturation_supply_humidity_ratio)
        && clear!(assigned_saturation_supply_humidity_ratio)
        && clear!(resulting_saturation_supply_humidity_ratio)
        && clear!(predecessor_cp412_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp412_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp412_resulting_supply_temperature_c)
        && clear!(saturation_supply_humidity_ratio_for_guard)
        && clear!(original_supply_humidity_ratio_for_guard)
        && clear!(predecessor_cp413_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp413_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp413_resulting_supply_temperature_c)
        && clear!(supply_enthalpy_for_saturation_temperature_j_per_kg)
        && clear!(outdoor_barometric_pressure_for_saturation_temperature_pa)
        && clear!(psychrometric_saturation_supply_temperature_result_c)
        && clear!(assigned_saturation_supply_temperature_c)
        && clear!(predecessor_cp414_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp414_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp414_resulting_supply_temperature_c)
        && clear!(preexisting_supply_temperature_c)
        && clear!(supply_temperature_before_mixed_air_limit_c)
        && clear!(mixed_air_temperature_c)
        && clear!(minimum_supply_temperature_c)
        && clear!(assigned_supply_temperature_c)
        && clear!(predecessor_cp415_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp415_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp415_resulting_supply_temperature_c)
        && clear!(supply_temperature_c)
        && clear!(supply_enthalpy_j_per_kg)
        && clear!(psychrometric_supply_humidity_ratio)
        && clear!(assigned_supply_humidity_ratio)
        && clear!(predecessor_cp416_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp416_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp416_resulting_supply_temperature_c)
        && clear!(supply_temperature_for_enthalpy_c)
        && clear!(supply_humidity_ratio_for_enthalpy)
        && clear!(psychrometric_supply_enthalpy_j_per_kg)
        && clear!(assigned_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp418_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp418_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp418_resulting_supply_temperature_c)
        && clear!(mixed_air_humidity_ratio_for_cp_air)
        && clear!(psychrometric_cp_air_result_j_per_kg_k)
        && clear!(cp_air_j_per_kg_k)
        && clear!(predecessor_cp419_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp419_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp419_resulting_supply_temperature_c)
        && clear!(supply_mass_flow_rate_kg_per_s)
        && clear!(cp419_cp_air_for_sensible_output_j_per_kg_k)
        && clear!(supply_mass_flow_rate_times_cp_air_w_per_k)
        && clear!(mixed_air_temperature_for_sensible_output_c)
        && clear!(supply_temperature_for_sensible_output_c)
        && clear!(mixed_air_minus_supply_temperature_k)
        && clear!(calculated_cooling_sensible_output_w)
        && clear!(cooling_sensible_output_w)
        && clear!(predecessor_cp420_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp420_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp420_resulting_supply_temperature_c)
        && clear!(cp420_cooling_sensible_output_for_capacity_guard_w)
        && clear!(maximum_total_cooling_capacity_w)
        && clear!(resulting_supply_humidity_ratio)
        && clear!(resulting_supply_enthalpy_j_per_kg)
        && clear!(resulting_supply_temperature_c);
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
