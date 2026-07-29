//! Exact CP355 snapshot validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;

pub(in crate::ideal_loads) fn cooling_constant_shr_supply_humidity_ratio_minimum_limit_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some()
        && (!active_prefix(snapshot)
            || snapshot.predecessor_dehumidification_control_type
                == Some(DehumidificationControlType::None))
        && !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed
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
                    .predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed,
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
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed,
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed
        && snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed
        && predecessor_count == 1
        && local_count == 1
    {
        Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMinimumLimitExecuted
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
        && snapshot
            .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && predecessor_count == 1
        && local_count == 1
    {
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip
    } else {
        return None;
    };
    let values_are_exact =
        if route == Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMinimumLimitExecuted {
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
    let values_match = [
        option_bits_match(
            left.supply_humidity_ratio_before_minimum_limit,
            right.supply_humidity_ratio_before_minimum_limit,
        ),
        option_bits_match(
            left.minimum_cooling_supply_air_humidity_ratio,
            right.minimum_cooling_supply_air_humidity_ratio,
        ),
        option_bits_match(
            left.maximum_supply_humidity_ratio,
            right.maximum_supply_humidity_ratio,
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
        snapshot.supply_humidity_ratio_before_minimum_limit = None;
        snapshot.minimum_cooling_supply_air_humidity_ratio = None;
        snapshot.maximum_supply_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn assigned_values_are_exact(snapshot: Snapshot) -> bool {
    let (Some(left), Some(right), Some(maximum), Some(assigned), Some(resulting)) = (
        snapshot.supply_humidity_ratio_before_minimum_limit,
        snapshot.minimum_cooling_supply_air_humidity_ratio,
        snapshot.maximum_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ) else {
        return false;
    };
    let expected = source_shaped_two_argument_maximum(left, right);
    snapshot.supply_humidity_ratio_for_minimum_limit_maximum_read
        && snapshot.minimum_cooling_supply_air_humidity_ratio_for_maximum_read
        && snapshot.source_shaped_two_argument_maximum_evaluated
        && maximum.to_bits() == expected.to_bits()
        && snapshot.supply_humidity_ratio_assignment_performed
        && assigned.to_bits() == maximum.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

fn skipped_values_are_exact(snapshot: Snapshot) -> bool {
    !snapshot.supply_humidity_ratio_for_minimum_limit_maximum_read
        && snapshot
            .supply_humidity_ratio_before_minimum_limit
            .is_none()
        && !snapshot.minimum_cooling_supply_air_humidity_ratio_for_maximum_read
        && snapshot.minimum_cooling_supply_air_humidity_ratio.is_none()
        && !snapshot.source_shaped_two_argument_maximum_evaluated
        && snapshot.maximum_supply_humidity_ratio.is_none()
        && !snapshot.supply_humidity_ratio_assignment_performed
        && snapshot.assigned_supply_humidity_ratio.is_none()
        && snapshot.resulting_supply_humidity_ratio.is_none()
}

fn provenance_is_exact(snapshot: Snapshot) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER
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
