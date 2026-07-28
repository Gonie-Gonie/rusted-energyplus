//! Exact CP349 snapshot validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(in crate::ideal_loads) fn cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    snapshot_route(snapshot).is_some()
        && (!active_prefix(snapshot)
            || snapshot.predecessor_dehumidification_control_type
                == Some(DehumidificationControlType::None))
        && !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && !snapshot.dehumidification_control_humidistat_case_selected_skip
        && !snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
}

pub(super) fn snapshot_route(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> Option<Route> {
    if !provenance_is_exact(snapshot) {
        return None;
    }
    let route = if snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_case_flags_are_inactive(snapshot)
        && local_route_flags_are_inactive(snapshot)
    {
        Route::UnitOff
    } else if !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_case_flags_are_inactive(snapshot)
        && local_route_flags_are_inactive(snapshot)
    {
        Route::NonCooling
    } else if !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
        && predecessor_case_flags_are_inactive(snapshot)
        && local_route_flags_are_inactive(snapshot)
    {
        Route::PositiveGuardFalseFallthrough
    } else if active_none_case(snapshot) {
        Route::DehumidificationControlNoneCaseCompletedSkip
    } else if active_constant_sensible(snapshot) {
        Route::DehumidificationControlConstantSensibleHeatRatioCpAirAssigned
    } else if active_humidistat(snapshot) {
        Route::DehumidificationControlHumidistatCaseSelectedSkip
    } else if active_constant_supply(snapshot) {
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip
    } else {
        return None;
    };
    let values_are_exact =
        if route == Route::DehumidificationControlConstantSensibleHeatRatioCpAirAssigned {
            assigned_values_are_exact(snapshot)
        } else {
            skipped_values_are_exact(snapshot)
        };
    values_are_exact.then_some(route)
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && option_bits_match(
        left.psychrometric_cp_air_result_j_per_kg_k,
        right.psychrometric_cp_air_result_j_per_kg_k,
    ) && option_bits_match(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k);
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.psychrometric_cp_air_result_j_per_kg_k = None;
        snapshot.cp_air_j_per_kg_k = None;
    }
    values_match && left == right
}

fn active_none_case(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    active_prefix(snapshot)
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::None)
        && snapshot.predecessor_dehumidification_control_none_case_completed
        && snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && !snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
        && !snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip
        && !snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && snapshot.dehumidification_control_none_case_completed_skip
        && !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && !snapshot.dehumidification_control_humidistat_case_selected_skip
        && !snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
}

fn active_constant_sensible(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    active_prefix(snapshot)
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::ConstantSensibleHeatRatio)
        && !snapshot.predecessor_dehumidification_control_none_case_completed
        && !snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
        && !snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip
        && !snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && !snapshot.dehumidification_control_none_case_completed_skip
        && snapshot.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && !snapshot.dehumidification_control_humidistat_case_selected_skip
        && !snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
}

fn active_humidistat(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    active_prefix(snapshot)
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::Humidistat)
        && !snapshot.predecessor_dehumidification_control_none_case_completed
        && !snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && !snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
        && snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip
        && !snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && !snapshot.dehumidification_control_none_case_completed_skip
        && !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && snapshot.dehumidification_control_humidistat_case_selected_skip
        && !snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
}

fn active_constant_supply(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    active_prefix(snapshot)
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::ConstantSupplyHumidityRatio)
        && !snapshot.predecessor_dehumidification_control_none_case_completed
        && !snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && !snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
        && !snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip
        && snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && !snapshot.dehumidification_control_none_case_completed_skip
        && !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && !snapshot.dehumidification_control_humidistat_case_selected_skip
        && snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
}

fn assigned_values_are_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    let (Some(humidity), Some(result), Some(assigned)) = (
        snapshot.mixed_air_humidity_ratio,
        snapshot.psychrometric_cp_air_result_j_per_kg_k,
        snapshot.cp_air_j_per_kg_k,
    ) else {
        return false;
    };
    let expected = energyplus_psy_cp_air_fn_w(humidity);
    snapshot.mixed_air_humidity_ratio_read
        && humidity.is_finite()
        && humidity >= 0.0
        && snapshot.psychrometric_cp_air_evaluated
        && result.is_finite()
        && result.to_bits() == expected.to_bits()
        && snapshot.cp_air_assigned
        && assigned.to_bits() == result.to_bits()
}

fn skipped_values_are_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    !snapshot.mixed_air_humidity_ratio_read
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.psychrometric_cp_air_evaluated
        && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
        && !snapshot.cp_air_assigned
        && snapshot.cp_air_j_per_kg_k.is_none()
}

fn provenance_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER
}

fn active_prefix(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_some()
}

fn inactive_prefix(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
}

fn predecessor_case_flags_are_inactive(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    !snapshot.predecessor_dehumidification_control_none_case_completed
        && !snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && !snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
        && !snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip
        && !snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
}

fn local_route_flags_are_inactive(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
) -> bool {
    !snapshot.dehumidification_control_none_case_completed_skip
        && !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && !snapshot.dehumidification_control_humidistat_case_selected_skip
        && !snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
