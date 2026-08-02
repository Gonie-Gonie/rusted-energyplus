//! Exact CP390 snapshot, route, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, predecessor_has_supply_temperature, predecessor_route,
};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Predecessor,
};

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        !route.active
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_snapshot_is_exact_direct_release(
                predecessor_snapshot(snapshot),
            )
    })
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
    {
        return None;
    }
    let predecessor = predecessor_snapshot(snapshot);
    let route = predecessor_route(predecessor)?;
    let active_flags = [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed,
        snapshot.cp389_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_read,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.cp389_mixed_air_temperature_bit_corroborated,
        snapshot.mixed_air_temperature_for_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_temperature_assignment_performed,
    ];
    let has_temperature = predecessor_has_supply_temperature(route.predecessor_index);
    if active_flags.into_iter().any(|flag| flag != route.active)
        || snapshot.cp389_retained_supply_temperature_state_owned != has_temperature
        || snapshot.preexisting_supply_temperature_c.is_some() != has_temperature
        || !option_bits_match(
            snapshot.preexisting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        || !option_bits_match(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
    {
        return None;
    }
    if route.active {
        let (
            Some(preexisting),
            Some(left),
            Some(right),
            Some(minimum),
            Some(assigned),
            Some(resulting),
        ) = (
            snapshot.preexisting_supply_temperature_c,
            snapshot.supply_temperature_before_mixed_air_limit_c,
            snapshot.mixed_air_temperature_c,
            snapshot.minimum_supply_temperature_c,
            snapshot.assigned_supply_temperature_c,
            snapshot.resulting_supply_temperature_c,
        )
        else {
            return None;
        };
        let expected = source_shaped_two_argument_minimum(left, right);
        if !right.is_finite()
            || left.to_bits() != preexisting.to_bits()
            || right.to_bits() != predecessor.mixed_air_temperature_c?.to_bits()
            || minimum.to_bits() != expected.to_bits()
            || assigned.to_bits() != minimum.to_bits()
            || resulting.to_bits() != assigned.to_bits()
        {
            return None;
        }
    } else {
        let active_values = [
            snapshot.supply_temperature_before_mixed_air_limit_c,
            snapshot.mixed_air_temperature_c,
            snapshot.minimum_supply_temperature_c,
            snapshot.assigned_supply_temperature_c,
        ];
        if active_values.into_iter().any(|value| value.is_some())
            || !option_bits_match(
                snapshot.resulting_supply_temperature_c,
                snapshot.preexisting_supply_temperature_c,
            )
        {
            return None;
        }
    }
    Some(route)
}

pub(super) fn predecessor_snapshot(snapshot: Snapshot) -> Predecessor {
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: snapshot.predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: snapshot.predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: snapshot.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: snapshot.predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        predecessor_mixed_air_humidity_ratio_read: snapshot.predecessor_mixed_air_humidity_ratio_read,
        predecessor_mixed_air_humidity_ratio: snapshot.predecessor_mixed_air_humidity_ratio,
        predecessor_psychrometric_cp_air_evaluated: snapshot.predecessor_psychrometric_cp_air_evaluated,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        predecessor_cp_air_assigned: snapshot.predecessor_cp_air_assigned,
        predecessor_cp_air_j_per_kg_k: snapshot.predecessor_cp_air_j_per_kg_k,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        predecessor_cp384_retained_cooling_total_output_owned_read: snapshot.predecessor_cp384_retained_cooling_total_output_owned_read,
        predecessor_cp385_cooling_total_output_bit_corroborated: snapshot.predecessor_cp385_cooling_total_output_bit_corroborated,
        predecessor_cooling_total_output_read: snapshot.predecessor_cooling_total_output_read,
        predecessor_cooling_total_output_w: snapshot.predecessor_cooling_total_output_w,
        predecessor_cooling_sensible_heat_ratio_read: snapshot.predecessor_cooling_sensible_heat_ratio_read,
        predecessor_cooling_sensible_heat_ratio: snapshot.predecessor_cooling_sensible_heat_ratio,
        predecessor_cooling_sensible_output_calculated: snapshot.predecessor_cooling_sensible_output_calculated,
        predecessor_calculated_cooling_sensible_output_w: snapshot.predecessor_calculated_cooling_sensible_output_w,
        predecessor_cooling_sensible_output_assigned: snapshot.predecessor_cooling_sensible_output_assigned,
        predecessor_cooling_sensible_output_w: snapshot.predecessor_cooling_sensible_output_w,
        resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
        dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed,
        cp379_retained_supply_temperature_state_owned: snapshot.predecessor_cp379_retained_supply_temperature_state_owned,
        preexisting_supply_temperature_c: snapshot.predecessor_preexisting_supply_temperature_c,
        cp329_retained_mixed_air_temperature_owned_read: snapshot.predecessor_cp329_retained_mixed_air_temperature_owned_read,
        mixed_air_temperature_read: snapshot.predecessor_mixed_air_temperature_read,
        mixed_air_temperature_c: snapshot.predecessor_mixed_air_temperature_c,
        cp388_retained_cooling_sensible_output_owned_read: snapshot.predecessor_cp388_retained_cooling_sensible_output_owned_read,
        cooling_sensible_output_read: snapshot.predecessor_cooling_sensible_output_read,
        cooling_sensible_output_w: snapshot.predecessor_cp389_cooling_sensible_output_w,
        cp387_retained_cp_air_owned_read: snapshot.predecessor_cp387_retained_cp_air_owned_read,
        cp_air_read: snapshot.predecessor_cp_air_read,
        cp_air_j_per_kg_k: snapshot.predecessor_cp389_cp_air_j_per_kg_k,
        cp330_retained_supply_mass_flow_rate_owned_read: snapshot.predecessor_cp330_retained_supply_mass_flow_rate_owned_read,
        cp329_supply_mass_flow_rate_bit_corroborated: snapshot.predecessor_cp329_supply_mass_flow_rate_bit_corroborated,
        supply_mass_flow_rate_read: snapshot.predecessor_supply_mass_flow_rate_read,
        supply_mass_flow_rate_kg_per_s: snapshot.predecessor_supply_mass_flow_rate_kg_per_s,
        cp_air_times_supply_mass_flow_rate_calculated: snapshot.predecessor_cp_air_times_supply_mass_flow_rate_calculated,
        cp_air_times_supply_mass_flow_rate_w_per_k: snapshot.predecessor_cp_air_times_supply_mass_flow_rate_w_per_k,
        cooling_sensible_output_over_air_capacity_rate_calculated: snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_calculated,
        cooling_sensible_output_over_air_capacity_rate_k: snapshot.predecessor_cooling_sensible_output_over_air_capacity_rate_k,
        supply_temperature_calculated: snapshot.predecessor_supply_temperature_calculated,
        calculated_supply_temperature_c: snapshot.predecessor_calculated_supply_temperature_c,
        supply_temperature_assigned: snapshot.predecessor_supply_temperature_assigned,
        assigned_supply_temperature_c: snapshot.predecessor_assigned_supply_temperature_c,
        resulting_supply_temperature_c: snapshot.predecessor_resulting_supply_temperature_c,
    }
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    macro_rules! compare_clear {
        ($field:ident) => {{
            let matches = option_bits_match(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }
    let values_match = compare_clear!(predecessor_mixed_air_humidity_ratio)
        && compare_clear!(predecessor_psychrometric_cp_air_result_j_per_kg_k)
        && compare_clear!(predecessor_cp_air_j_per_kg_k)
        && compare_clear!(predecessor_cooling_total_output_w)
        && compare_clear!(predecessor_cooling_sensible_heat_ratio)
        && compare_clear!(predecessor_calculated_cooling_sensible_output_w)
        && compare_clear!(predecessor_cooling_sensible_output_w)
        && compare_clear!(predecessor_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_preexisting_supply_temperature_c)
        && compare_clear!(predecessor_mixed_air_temperature_c)
        && compare_clear!(predecessor_cp389_cooling_sensible_output_w)
        && compare_clear!(predecessor_cp389_cp_air_j_per_kg_k)
        && compare_clear!(predecessor_supply_mass_flow_rate_kg_per_s)
        && compare_clear!(predecessor_cp_air_times_supply_mass_flow_rate_w_per_k)
        && compare_clear!(predecessor_cooling_sensible_output_over_air_capacity_rate_k)
        && compare_clear!(predecessor_calculated_supply_temperature_c)
        && compare_clear!(predecessor_assigned_supply_temperature_c)
        && compare_clear!(predecessor_resulting_supply_temperature_c)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(preexisting_supply_temperature_c)
        && compare_clear!(supply_temperature_before_mixed_air_limit_c)
        && compare_clear!(mixed_air_temperature_c)
        && compare_clear!(minimum_supply_temperature_c)
        && compare_clear!(assigned_supply_temperature_c)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
