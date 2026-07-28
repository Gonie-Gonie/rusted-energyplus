//! Exact CP350 snapshot validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Snapshot,
};

pub(in crate::ideal_loads) fn cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some()
        && (!active_prefix(snapshot)
            || snapshot.predecessor_dehumidification_control_type
                == Some(DehumidificationControlType::None))
        && !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
        && !snapshot.dehumidification_control_humidistat_case_selected_skip
        && !snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if !provenance_is_exact(snapshot) {
        return None;
    }
    let predecessor_count =
        usize::from(snapshot.predecessor_dehumidification_control_none_case_completed_skip)
            + usize::from(
                snapshot
                    .predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
            )
            + usize::from(
                snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip,
            )
            + usize::from(
                snapshot
                    .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
            );
    let local_count = usize::from(snapshot.dehumidification_control_none_case_completed_skip)
        + usize::from(
            snapshot
                .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        )
        + usize::from(snapshot.dehumidification_control_humidistat_case_selected_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        );
    let route = if snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0
    {
        Route::UnitOff
    } else if !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0
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
        && predecessor_count == 0
        && local_count == 0
    {
        Route::PositiveGuardFalseFallthrough
    } else if active_prefix(snapshot)
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::None)
        && snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && snapshot.dehumidification_control_none_case_completed_skip
        && predecessor_count == 1
        && local_count == 1
    {
        Route::DehumidificationControlNoneCaseCompletedSkip
    } else if active_prefix(snapshot)
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::ConstantSensibleHeatRatio)
        && snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && snapshot
            .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
        && predecessor_count == 1
        && local_count == 1
    {
        Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned
    } else if active_prefix(snapshot)
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::Humidistat)
        && snapshot.predecessor_dehumidification_control_humidistat_case_selected_skip
        && snapshot.dehumidification_control_humidistat_case_selected_skip
        && predecessor_count == 1
        && local_count == 1
    {
        Route::DehumidificationControlHumidistatCaseSelectedSkip
    } else if active_prefix(snapshot)
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::ConstantSupplyHumidityRatio)
        && snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && snapshot.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && predecessor_count == 1
        && local_count == 1
    {
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip
    } else {
        return None;
    };
    let values_are_exact =
        if route == Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned {
            assigned_values_are_exact(snapshot)
        } else {
            skipped_values_are_exact(snapshot)
        };
    values_are_exact.then_some(route)
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = option_bits_match(
        left.supply_mass_flow_rate_kg_per_s,
        right.supply_mass_flow_rate_kg_per_s,
    ) && option_bits_match(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k)
        && option_bits_match(
            left.supply_mass_flow_rate_times_cp_air_w_per_k,
            right.supply_mass_flow_rate_times_cp_air_w_per_k,
        )
        && option_bits_match(left.mixed_air_temperature_c, right.mixed_air_temperature_c)
        && option_bits_match(left.supply_temperature_c, right.supply_temperature_c)
        && option_bits_match(
            left.mixed_air_minus_supply_temperature_k,
            right.mixed_air_minus_supply_temperature_k,
        )
        && option_bits_match(
            left.calculated_cooling_sensible_output_w,
            right.calculated_cooling_sensible_output_w,
        )
        && option_bits_match(
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        );
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.cp_air_j_per_kg_k = None;
        snapshot.supply_mass_flow_rate_times_cp_air_w_per_k = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.supply_temperature_c = None;
        snapshot.mixed_air_minus_supply_temperature_k = None;
        snapshot.calculated_cooling_sensible_output_w = None;
        snapshot.cooling_sensible_output_w = None;
    }
    values_match && left == right
}

fn assigned_values_are_exact(snapshot: Snapshot) -> bool {
    let (
        Some(flow),
        Some(cp_air),
        Some(first_product),
        Some(mixed),
        Some(supply),
        Some(difference),
        Some(calculated),
        Some(assigned),
    ) = (
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.cp_air_j_per_kg_k,
        snapshot.supply_mass_flow_rate_times_cp_air_w_per_k,
        snapshot.mixed_air_temperature_c,
        snapshot.supply_temperature_c,
        snapshot.mixed_air_minus_supply_temperature_k,
        snapshot.calculated_cooling_sensible_output_w,
        snapshot.cooling_sensible_output_w,
    )
    else {
        return false;
    };
    snapshot.supply_mass_flow_rate_read
        && flow > 0.0
        && !flow.is_nan()
        && snapshot.cp_air_read
        && cp_air.is_finite()
        && snapshot.supply_mass_flow_rate_times_cp_air_calculated
        && first_product.to_bits() == (flow * cp_air).to_bits()
        && snapshot.mixed_air_temperature_read
        && snapshot.supply_temperature_read
        && snapshot.mixed_air_minus_supply_temperature_calculated
        && difference.to_bits() == (mixed - supply).to_bits()
        && snapshot.cooling_sensible_output_calculated
        && calculated.to_bits() == (first_product * difference).to_bits()
        && snapshot.cooling_sensible_output_assigned
        && assigned.to_bits() == calculated.to_bits()
}

fn skipped_values_are_exact(snapshot: Snapshot) -> bool {
    !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.cp_air_read
        && snapshot.cp_air_j_per_kg_k.is_none()
        && !snapshot.supply_mass_flow_rate_times_cp_air_calculated
        && snapshot
            .supply_mass_flow_rate_times_cp_air_w_per_k
            .is_none()
        && !snapshot.mixed_air_temperature_read
        && snapshot.mixed_air_temperature_c.is_none()
        && !snapshot.supply_temperature_read
        && snapshot.supply_temperature_c.is_none()
        && !snapshot.mixed_air_minus_supply_temperature_calculated
        && snapshot.mixed_air_minus_supply_temperature_k.is_none()
        && !snapshot.cooling_sensible_output_calculated
        && snapshot.calculated_cooling_sensible_output_w.is_none()
        && !snapshot.cooling_sensible_output_assigned
        && snapshot.cooling_sensible_output_w.is_none()
}

fn provenance_is_exact(snapshot: Snapshot) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
}

fn active_prefix(snapshot: Snapshot) -> bool {
    !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_some()
}

fn inactive_prefix(snapshot: Snapshot) -> bool {
    !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
