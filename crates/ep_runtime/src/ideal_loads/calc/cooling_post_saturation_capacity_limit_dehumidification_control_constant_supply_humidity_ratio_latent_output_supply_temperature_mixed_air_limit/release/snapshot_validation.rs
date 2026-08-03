//! Exact CP408 compact snapshot, route, and binary64 validation.

use super::super::transition::routes::{
    RetainedRoute, predecessor_index_is_public, predecessor_route,
};
use super::super::transition::source_minimum;
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as PREDECESSOR_EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE as PREDECESSOR_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER as PREDECESSOR_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
};

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some_and(|route| {
        predecessor_index_is_public(route.predecessor_index)
            && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release(
                predecessor_snapshot(snapshot),
            )
    })
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    if snapshot.source != SOURCE
        || snapshot.first_excluded_source != EXCLUDED
        || snapshot.source_order != ORDER
    {
        return None;
    }
    let predecessor = predecessor_snapshot(snapshot);
    let route = predecessor_route(predecessor)?;
    let active_flags = [
        snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_executed,
        snapshot.cp407_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_read,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.mixed_air_temperature_for_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_temperature_assignment_performed,
    ];
    if active_flags.into_iter().any(|flag| flag != route.active)
        || snapshot.cp407_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
        || !option_bits_match(
            snapshot.preexisting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        || !option_bits_match(
            snapshot.predecessor_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        || !option_bits_match(
            snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        || !option_bits_match(
            snapshot.predecessor_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        || !option_bits_match(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
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
        let expected = source_minimum(left, right);
        if left.to_bits() != preexisting.to_bits()
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
        source: PREDECESSOR_SOURCE,
        first_excluded_source: PREDECESSOR_EXCLUDED,
        source_order: PREDECESSOR_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: snapshot
            .predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: snapshot
            .predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_dehumidification_control_none_case_entered: snapshot
            .predecessor_dehumidification_control_none_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered,
        predecessor_cp406_resulting_supply_humidity_ratio: snapshot
            .predecessor_cp406_resulting_supply_humidity_ratio,
        predecessor_cp406_resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp406_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp406_resulting_supply_temperature_c: snapshot
            .predecessor_cp406_resulting_supply_temperature_c,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed,
        cp385_retained_supply_enthalpy_owned_read: snapshot
            .predecessor_cp385_retained_supply_enthalpy_owned_read,
        cp406_same_call_supply_enthalpy_bit_corroborated: snapshot
            .predecessor_cp406_same_call_supply_enthalpy_bit_corroborated,
        supply_enthalpy_for_dry_bulb_inversion_read: snapshot
            .predecessor_supply_enthalpy_for_dry_bulb_inversion_read,
        supply_enthalpy_j_per_kg: snapshot.predecessor_supply_enthalpy_j_per_kg,
        cp378_retained_supply_humidity_ratio_owned_read: snapshot
            .predecessor_cp378_retained_supply_humidity_ratio_owned_read,
        supply_humidity_ratio_for_dry_bulb_inversion_read: snapshot
            .predecessor_supply_humidity_ratio_for_dry_bulb_inversion_read,
        supply_humidity_ratio: snapshot.predecessor_supply_humidity_ratio,
        cp406_retained_supply_temperature_state_owned: snapshot
            .predecessor_cp406_retained_supply_temperature_state_owned,
        preexisting_supply_temperature_c: snapshot.predecessor_preexisting_supply_temperature_c,
        psychrometric_supply_temperature_evaluated: snapshot
            .predecessor_psychrometric_supply_temperature_evaluated,
        psychrometric_supply_temperature_result_c: snapshot
            .predecessor_psychrometric_supply_temperature_result_c,
        supply_temperature_assigned: snapshot.predecessor_supply_temperature_assigned,
        assigned_supply_temperature_c: snapshot.predecessor_assigned_supply_temperature_c,
        resulting_supply_humidity_ratio: snapshot.predecessor_resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_resulting_supply_enthalpy_j_per_kg,
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
    let values_match = compare_clear!(predecessor_cp406_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp406_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp406_resulting_supply_temperature_c)
        && compare_clear!(predecessor_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_supply_humidity_ratio)
        && compare_clear!(predecessor_preexisting_supply_temperature_c)
        && compare_clear!(predecessor_psychrometric_supply_temperature_result_c)
        && compare_clear!(predecessor_assigned_supply_temperature_c)
        && compare_clear!(predecessor_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_resulting_supply_temperature_c)
        && compare_clear!(preexisting_supply_temperature_c)
        && compare_clear!(supply_temperature_before_mixed_air_limit_c)
        && compare_clear!(mixed_air_temperature_c)
        && compare_clear!(minimum_supply_temperature_c)
        && compare_clear!(assigned_supply_temperature_c)
        && compare_clear!(resulting_supply_humidity_ratio)
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
