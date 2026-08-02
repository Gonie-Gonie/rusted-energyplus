//! Exact CP391 snapshot, route, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_route,
};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot as Predecessor,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        !route.active
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
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
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER
    {
        return None;
    }
    let predecessor = predecessor_snapshot(snapshot);
    let route = predecessor_route(predecessor)?;
    let active_flags = [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed,
        snapshot.cp390_retained_supply_enthalpy_owned_read,
        snapshot.supply_enthalpy_for_overdrying_limit_maximum_read,
        snapshot.cp390_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_humidity_ratio_enthalpy_read,
        snapshot.psychrometric_minimum_supply_enthalpy_evaluated,
        snapshot.source_shaped_two_argument_maximum_evaluated,
        snapshot.supply_enthalpy_assignment_performed,
    ];
    let has_enthalpy = predecessor_has_supply_enthalpy(route.predecessor_index);
    if active_flags.into_iter().any(|flag| flag != route.active)
        || snapshot.cp390_retained_supply_enthalpy_state_owned != has_enthalpy
        || snapshot.preexisting_supply_enthalpy_j_per_kg.is_some() != has_enthalpy
        || !option_bits_match(
            snapshot.preexisting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_match(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
    {
        return None;
    }
    if route.active {
        let (
            Some(preexisting),
            Some(left),
            Some(temperature),
            Some(psychrometric),
            Some(maximum),
            Some(assigned),
            Some(resulting),
        ) = (
            snapshot.preexisting_supply_enthalpy_j_per_kg,
            snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg,
            snapshot.supply_temperature_c,
            snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg,
            snapshot.maximum_supply_enthalpy_j_per_kg,
            snapshot.assigned_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
        )
        else {
            return None;
        };
        let expected_psychrometric = energyplus_psy_h_fn_tdb_w(temperature, 1.0e-5);
        let expected_maximum = source_shaped_two_argument_maximum(left, expected_psychrometric);
        if left.to_bits() != preexisting.to_bits()
            || temperature.to_bits() != predecessor.resulting_supply_temperature_c?.to_bits()
            || psychrometric.to_bits() != expected_psychrometric.to_bits()
            || maximum.to_bits() != expected_maximum.to_bits()
            || assigned.to_bits() != maximum.to_bits()
            || resulting.to_bits() != assigned.to_bits()
        {
            return None;
        }
    } else {
        let active_values = [
            snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg,
            snapshot.supply_temperature_c,
            snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg,
            snapshot.maximum_supply_enthalpy_j_per_kg,
            snapshot.assigned_supply_enthalpy_j_per_kg,
        ];
        if active_values.into_iter().any(|value| value.is_some())
            || !option_bits_match(
                snapshot.resulting_supply_enthalpy_j_per_kg,
                snapshot.preexisting_supply_enthalpy_j_per_kg,
            )
        {
            return None;
        }
    }
    Some(route)
}

pub(super) fn predecessor_snapshot(snapshot: Snapshot) -> Predecessor {
    Predecessor {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot
            .predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot
            .dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot
            .dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: snapshot
            .predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: snapshot
            .predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: snapshot
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: snapshot
            .predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed: snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        predecessor_mixed_air_humidity_ratio_read: snapshot
            .predecessor_mixed_air_humidity_ratio_read,
        predecessor_mixed_air_humidity_ratio: snapshot.predecessor_mixed_air_humidity_ratio,
        predecessor_psychrometric_cp_air_evaluated: snapshot
            .predecessor_psychrometric_cp_air_evaluated,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: snapshot
            .predecessor_psychrometric_cp_air_result_j_per_kg_k,
        predecessor_cp_air_assigned: snapshot.predecessor_cp_air_assigned,
        predecessor_cp_air_j_per_kg_k: snapshot.predecessor_cp_air_j_per_kg_k,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed: snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        predecessor_cp384_retained_cooling_total_output_owned_read: snapshot
            .predecessor_cp384_retained_cooling_total_output_owned_read,
        predecessor_cp385_cooling_total_output_bit_corroborated: snapshot
            .predecessor_cp385_cooling_total_output_bit_corroborated,
        predecessor_cooling_total_output_read: snapshot.predecessor_cooling_total_output_read,
        predecessor_cooling_total_output_w: snapshot.predecessor_cooling_total_output_w,
        predecessor_cooling_sensible_heat_ratio_read: snapshot
            .predecessor_cooling_sensible_heat_ratio_read,
        predecessor_cooling_sensible_heat_ratio: snapshot
            .predecessor_cooling_sensible_heat_ratio,
        predecessor_cooling_sensible_output_calculated: snapshot
            .predecessor_cooling_sensible_output_calculated,
        predecessor_calculated_cooling_sensible_output_w: snapshot
            .predecessor_calculated_cooling_sensible_output_w,
        predecessor_cooling_sensible_output_assigned: snapshot
            .predecessor_cooling_sensible_output_assigned,
        predecessor_cooling_sensible_output_w: snapshot.predecessor_cooling_sensible_output_w,
        predecessor_resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_resulting_supply_enthalpy_j_per_kg,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed: snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed,
        predecessor_cp379_retained_supply_temperature_state_owned: snapshot
            .predecessor_cp379_retained_supply_temperature_state_owned,
        predecessor_preexisting_supply_temperature_c: snapshot
            .predecessor_preexisting_supply_temperature_c,
        predecessor_cp329_retained_mixed_air_temperature_owned_read: snapshot
            .predecessor_cp329_retained_mixed_air_temperature_owned_read,
        predecessor_mixed_air_temperature_read: snapshot.predecessor_mixed_air_temperature_read,
        predecessor_mixed_air_temperature_c: snapshot.predecessor_mixed_air_temperature_c,
        predecessor_cp388_retained_cooling_sensible_output_owned_read: snapshot
            .predecessor_cp388_retained_cooling_sensible_output_owned_read,
        predecessor_cooling_sensible_output_read: snapshot
            .predecessor_cooling_sensible_output_read,
        predecessor_cp389_cooling_sensible_output_w: snapshot
            .predecessor_cp389_cooling_sensible_output_w,
        predecessor_cp387_retained_cp_air_owned_read: snapshot
            .predecessor_cp387_retained_cp_air_owned_read,
        predecessor_cp_air_read: snapshot.predecessor_cp_air_read,
        predecessor_cp389_cp_air_j_per_kg_k: snapshot.predecessor_cp389_cp_air_j_per_kg_k,
        predecessor_cp330_retained_supply_mass_flow_rate_owned_read: snapshot
            .predecessor_cp330_retained_supply_mass_flow_rate_owned_read,
        predecessor_cp329_supply_mass_flow_rate_bit_corroborated: snapshot
            .predecessor_cp329_supply_mass_flow_rate_bit_corroborated,
        predecessor_supply_mass_flow_rate_read: snapshot.predecessor_supply_mass_flow_rate_read,
        predecessor_supply_mass_flow_rate_kg_per_s: snapshot
            .predecessor_supply_mass_flow_rate_kg_per_s,
        predecessor_cp_air_times_supply_mass_flow_rate_calculated: snapshot
            .predecessor_cp_air_times_supply_mass_flow_rate_calculated,
        predecessor_cp_air_times_supply_mass_flow_rate_w_per_k: snapshot
            .predecessor_cp_air_times_supply_mass_flow_rate_w_per_k,
        predecessor_cooling_sensible_output_over_air_capacity_rate_calculated: snapshot
            .predecessor_cooling_sensible_output_over_air_capacity_rate_calculated,
        predecessor_cooling_sensible_output_over_air_capacity_rate_k: snapshot
            .predecessor_cooling_sensible_output_over_air_capacity_rate_k,
        predecessor_supply_temperature_calculated: snapshot.predecessor_supply_temperature_calculated,
        predecessor_calculated_supply_temperature_c: snapshot
            .predecessor_calculated_supply_temperature_c,
        predecessor_supply_temperature_assigned: snapshot.predecessor_supply_temperature_assigned,
        predecessor_assigned_supply_temperature_c: snapshot
            .predecessor_assigned_supply_temperature_c,
        predecessor_resulting_supply_temperature_c: snapshot
            .predecessor_resulting_supply_temperature_c,
        resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp390_resulting_supply_enthalpy_j_per_kg,
        dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed: snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed,
        cp389_retained_supply_temperature_state_owned: snapshot
            .cp389_retained_supply_temperature_state_owned,
        preexisting_supply_temperature_c: snapshot.preexisting_supply_temperature_c,
        cp389_retained_supply_temperature_owned_read: snapshot
            .cp389_retained_supply_temperature_owned_read,
        supply_temperature_for_minimum_read: snapshot.supply_temperature_for_minimum_read,
        supply_temperature_before_mixed_air_limit_c: snapshot
            .supply_temperature_before_mixed_air_limit_c,
        cp329_retained_mixed_air_temperature_owned_read: snapshot
            .cp329_retained_mixed_air_temperature_owned_read,
        cp389_mixed_air_temperature_bit_corroborated: snapshot
            .cp389_mixed_air_temperature_bit_corroborated,
        mixed_air_temperature_for_minimum_read: snapshot.mixed_air_temperature_for_minimum_read,
        mixed_air_temperature_c: snapshot.mixed_air_temperature_c,
        source_shaped_two_argument_minimum_evaluated: snapshot
            .source_shaped_two_argument_minimum_evaluated,
        minimum_supply_temperature_c: snapshot.minimum_supply_temperature_c,
        supply_temperature_assignment_performed: snapshot.supply_temperature_assignment_performed,
        assigned_supply_temperature_c: snapshot.assigned_supply_temperature_c,
        resulting_supply_temperature_c: snapshot
            .predecessor_cp390_resulting_supply_temperature_c,
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
        && compare_clear!(predecessor_cp390_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(preexisting_supply_temperature_c)
        && compare_clear!(supply_temperature_before_mixed_air_limit_c)
        && compare_clear!(mixed_air_temperature_c)
        && compare_clear!(minimum_supply_temperature_c)
        && compare_clear!(assigned_supply_temperature_c)
        && compare_clear!(predecessor_cp390_resulting_supply_temperature_c)
        && compare_clear!(preexisting_supply_enthalpy_j_per_kg)
        && compare_clear!(supply_enthalpy_before_overdrying_limit_j_per_kg)
        && compare_clear!(supply_temperature_c)
        && compare_clear!(psychrometric_minimum_supply_enthalpy_j_per_kg)
        && compare_clear!(maximum_supply_enthalpy_j_per_kg)
        && compare_clear!(assigned_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
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
