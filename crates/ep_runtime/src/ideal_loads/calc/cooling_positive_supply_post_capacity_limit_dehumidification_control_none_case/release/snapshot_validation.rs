//! Exact CP347 snapshot validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
};
use super::prefix_validation::option_bits_match;

pub(in crate::ideal_loads) fn cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    snapshot_shape_is_exact(snapshot)
        && (!snapshot
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
            || snapshot.predecessor_dehumidification_control_type
                == Some(DehumidificationControlType::None))
}

pub(super) fn snapshot_route(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> Option<Route> {
    if !snapshot_shape_is_exact(snapshot) {
        return None;
    }
    if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.positive_guard_false_fallthrough_skipped {
        Some(Route::PositiveGuardFalseFallthrough)
    } else {
        match snapshot.predecessor_dehumidification_control_type? {
            DehumidificationControlType::None => {
                Some(Route::DehumidificationControlNoneCaseCompleted)
            }
            DehumidificationControlType::ConstantSensibleHeatRatio => {
                Some(Route::DehumidificationControlConstantSensibleHeatRatioCaseSelected)
            }
            DehumidificationControlType::Humidistat => {
                Some(Route::DehumidificationControlHumidistatCaseSelected)
            }
            DehumidificationControlType::ConstantSupplyHumidityRatio => {
                Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelected)
            }
        }
    }
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.predecessor_assigned_supply_humidity_ratio,
            right.predecessor_assigned_supply_humidity_ratio,
        ),
        option_bits_match(
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        ),
        option_bits_match(
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        option_bits_match(
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_assigned_supply_humidity_ratio = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn snapshot_shape_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_prefix(snapshot);
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_prefix(snapshot);
    let positive_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.positive_guard_false_fallthrough_skipped
        && inactive_capacity_join(snapshot);
    let active = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && usize::from(snapshot.predecessor_capacity_limit_guard_false_fallthrough)
            + usize::from(
                snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
            )
            + usize::from(
                snapshot
                    .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            )
            == 1
        && snapshot
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        && snapshot.predecessor_dehumidification_control_type_read
        && snapshot
            .predecessor_dehumidification_control_type
            .is_some()
        && snapshot.predecessor_dehumidification_control_switch_dispatched;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(positive_false)
            + usize::from(active)
            == 1
        && if active {
            snapshot
                .predecessor_assigned_supply_humidity_ratio
                .is_some()
                && active_case_values_are_exact(snapshot)
        } else {
            snapshot
                .predecessor_assigned_supply_humidity_ratio
                .is_none()
                && !snapshot.predecessor_dehumidification_control_type_read
                && snapshot.predecessor_dehumidification_control_type.is_none()
                && !snapshot.predecessor_dehumidification_control_switch_dispatched
                && inactive_none_case_values_are_exact(snapshot)
        }
}

fn active_case_values_are_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    if snapshot.predecessor_dehumidification_control_type != Some(DehumidificationControlType::None)
    {
        return inactive_none_case_values_are_exact(snapshot);
    }
    snapshot.dehumidification_control_none_case_entered
        && snapshot.mixed_air_humidity_ratio_read
        && snapshot.supply_humidity_ratio_assignment_performed
        && snapshot.dehumidification_control_none_case_exited_via_break
        && option_bits_match(
            snapshot.predecessor_assigned_supply_humidity_ratio,
            snapshot.mixed_air_humidity_ratio,
        )
        && option_bits_match(
            snapshot.mixed_air_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        )
        && snapshot.mixed_air_humidity_ratio.is_some()
}

fn inactive_none_case_values_are_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    !snapshot.dehumidification_control_none_case_entered
        && !snapshot.mixed_air_humidity_ratio_read
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.supply_humidity_ratio_assignment_performed
        && snapshot.assigned_supply_humidity_ratio.is_none()
        && snapshot.resulting_supply_humidity_ratio.is_none()
        && !snapshot.dehumidification_control_none_case_exited_via_break
}

fn inactive_prefix(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && inactive_capacity_join(snapshot)
}

fn inactive_capacity_join(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    !snapshot.predecessor_capacity_limit_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        && !snapshot
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
}
