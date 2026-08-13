//! Exact CP422 local-shape and bitwise validation.

use super::prefix::predecessor_cp421_snapshot;
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Snapshot,
};
use super::super::transition::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRetainedRoute as Route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_route_from_committed_predecessor,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Predecessor;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute as Cp421Route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_retained_route_matches_snapshot_bounded,
};

pub(super) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
        && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshot_is_exact(
            predecessor_cp421_snapshot(snapshot),
        )
}
pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let predecessor = predecessor_cp421_snapshot(snapshot);
    let predecessor_route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshot_route(predecessor)?;
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_route_from_committed_predecessor(
        predecessor,
        predecessor_route,
    )?;
    local_shape_is_exact(snapshot, predecessor, route).then_some(route)
}

pub(super) fn prefix_and_local_shape_match(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: Route,
) -> bool {
    crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshots_match_bit_exact(
        predecessor_cp421_snapshot(snapshot),
        predecessor,
    ) && local_shape_is_exact(snapshot, predecessor, route)
}

pub(super) fn retained_route_matches_snapshot_bounded(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    let predecessor = predecessor_cp421_snapshot(snapshot);
    let predecessor_route = Cp421Route {
        logical_index: route.logical_index,
        active: route.active,
        body_entered: route.assignment_executed,
    };
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_retained_route_matches_snapshot_bounded(
        predecessor,
        predecessor_route,
    ) && local_shape_is_exact(snapshot, predecessor, route)
}

fn local_shape_is_exact(snapshot: Snapshot, predecessor: Predecessor, route: Route) -> bool {
    let assignment = route.assignment_executed;
    let active = route.active;
    let supply_is_preserved = option_bits_match(
        snapshot.predecessor_cp421_resulting_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ) && option_bits_match(
        snapshot.predecessor_cp421_resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        snapshot.predecessor_cp421_resulting_supply_temperature_c,
        snapshot.resulting_supply_temperature_c,
    );
    let owner_shape = snapshot.cp421_retained_supply_humidity_ratio_state_owned
        == snapshot.predecessor_cp421_resulting_supply_humidity_ratio.is_some()
        && snapshot.cp421_retained_supply_enthalpy_state_owned
            == snapshot.predecessor_cp421_resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp421_retained_supply_temperature_state_owned
            == snapshot.predecessor_cp421_resulting_supply_temperature_c.is_some();
    if snapshot.source != SOURCE
        || snapshot.first_excluded_source != EXCLUDED
        || snapshot.source_order != ORDER
        || !supply_is_preserved
        || !owner_shape
        || route.logical_index >= 36
        || active != matches!(route.logical_index, 4 | 7 | 10 | 13 | 16)
        || assignment
            != predecessor
                .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered
        || active
            != predecessor
                .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated
        || snapshot
            .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed
            != assignment
    {
        return false;
    }
    if !active {
        return snapshot
            .preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w
            .is_none()
            && !snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read
            && !snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read
            && snapshot
                .maximum_total_cooling_capacity_for_sensible_output_assignment_w
                .is_none()
            && !snapshot.cooling_sensible_output_maximum_capacity_assignment_performed
            && snapshot
                .assigned_cooling_sensible_output_from_maximum_capacity_w
                .is_none()
            && snapshot
                .resulting_cooling_sensible_output_after_maximum_capacity_assignment_w
                .is_none();
    }
    let (Some(preexisting), Some(predecessor_output)) = (
        snapshot.preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w,
        predecessor.cp420_cooling_sensible_output_for_capacity_guard_w,
    ) else {
        return false;
    };
    if preexisting.to_bits() != predecessor_output.to_bits() {
        return false;
    }
    if !assignment {
        return !snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read
            && !snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read
            && snapshot
                .maximum_total_cooling_capacity_for_sensible_output_assignment_w
                .is_none()
            && !snapshot.cooling_sensible_output_maximum_capacity_assignment_performed
            && snapshot
                .assigned_cooling_sensible_output_from_maximum_capacity_w
                .is_none()
            && snapshot
                .resulting_cooling_sensible_output_after_maximum_capacity_assignment_w
                .is_some_and(|result| result.to_bits() == preexisting.to_bits());
    }
    let (Some(capacity), Some(predecessor_capacity), Some(assigned), Some(resulting)) = (
        snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_w,
        predecessor.maximum_total_cooling_capacity_w,
        snapshot.assigned_cooling_sensible_output_from_maximum_capacity_w,
        snapshot.resulting_cooling_sensible_output_after_maximum_capacity_assignment_w,
    ) else {
        return false;
    };
    snapshot.cp421_retained_maximum_total_cooling_capacity_owned_read
        && snapshot.maximum_total_cooling_capacity_for_sensible_output_assignment_read
        && snapshot.cooling_sensible_output_maximum_capacity_assignment_performed
        && capacity.to_bits() == predecessor_capacity.to_bits()
        && assigned.to_bits() == capacity.to_bits()
        && resulting.to_bits() == capacity.to_bits()
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
        && clear!(predecessor_cp421_resulting_supply_humidity_ratio)
        && clear!(predecessor_cp421_resulting_supply_enthalpy_j_per_kg)
        && clear!(predecessor_cp421_resulting_supply_temperature_c)
        && clear!(preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w)
        && clear!(maximum_total_cooling_capacity_for_sensible_output_assignment_w)
        && clear!(assigned_cooling_sensible_output_from_maximum_capacity_w)
        && clear!(resulting_cooling_sensible_output_after_maximum_capacity_assignment_w)
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
