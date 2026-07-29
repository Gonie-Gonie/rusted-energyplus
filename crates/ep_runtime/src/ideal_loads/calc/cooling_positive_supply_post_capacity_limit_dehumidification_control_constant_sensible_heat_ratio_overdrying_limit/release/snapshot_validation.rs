//! Exact CP353 snapshot validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit::transition::source_shaped_two_argument_maximum;
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

pub(in crate::ideal_loads) fn cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some()
        && (!active_prefix(snapshot)
            || snapshot.predecessor_dehumidification_control_type
                == Some(DehumidificationControlType::None))
        && !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed
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
                    .predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed,
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
                .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed,
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed
        && snapshot
            .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed
        && predecessor_count == 1
        && local_count == 1
    {
        Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted
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
        if route == Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted {
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
            left.supply_enthalpy_before_overdrying_limit_j_per_kg,
            right.supply_enthalpy_before_overdrying_limit_j_per_kg,
        ),
        option_bits_match(
            left.supply_temperature_c,
            right.supply_temperature_c,
        ),
        option_bits_match(
            left.psychrometric_minimum_supply_enthalpy_j_per_kg,
            right.psychrometric_minimum_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.maximum_supply_enthalpy_j_per_kg,
            right.maximum_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.assigned_supply_enthalpy_j_per_kg,
            right.assigned_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg = None;
        snapshot.supply_temperature_c = None;
        snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg = None;
        snapshot.maximum_supply_enthalpy_j_per_kg = None;
        snapshot.assigned_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
    }
    values_match && left == right
}

fn assigned_values_are_exact(snapshot: Snapshot) -> bool {
    let (
        Some(pre_limit),
        Some(temperature),
        Some(psychrometric_minimum),
        Some(maximum),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg,
        snapshot.supply_temperature_c,
        snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg,
        snapshot.maximum_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    )
    else {
        return false;
    };
    let expected_psychrometric_minimum =
        energyplus_psy_h_fn_tdb_w(temperature, 1.0e-5);
    let expected_maximum =
        source_shaped_two_argument_maximum(pre_limit, expected_psychrometric_minimum);
    snapshot.supply_enthalpy_for_overdrying_limit_maximum_read
        && snapshot.supply_temperature_for_minimum_humidity_ratio_enthalpy_read
        && snapshot.psychrometric_minimum_supply_enthalpy_evaluated
        && psychrometric_minimum.to_bits() == expected_psychrometric_minimum.to_bits()
        && snapshot.source_shaped_two_argument_maximum_evaluated
        && maximum.to_bits() == expected_maximum.to_bits()
        && snapshot.supply_enthalpy_assignment_performed
        && assigned.to_bits() == maximum.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

fn skipped_values_are_exact(snapshot: Snapshot) -> bool {
    !snapshot.supply_enthalpy_for_overdrying_limit_maximum_read
        && snapshot
            .supply_enthalpy_before_overdrying_limit_j_per_kg
            .is_none()
        && !snapshot.supply_temperature_for_minimum_humidity_ratio_enthalpy_read
        && snapshot.supply_temperature_c.is_none()
        && !snapshot.psychrometric_minimum_supply_enthalpy_evaluated
        && snapshot
            .psychrometric_minimum_supply_enthalpy_j_per_kg
            .is_none()
        && !snapshot.source_shaped_two_argument_maximum_evaluated
        && snapshot.maximum_supply_enthalpy_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_assignment_performed
        && snapshot.assigned_supply_enthalpy_j_per_kg.is_none()
        && snapshot.resulting_supply_enthalpy_j_per_kg.is_none()
}

fn provenance_is_exact(snapshot: Snapshot) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER
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
