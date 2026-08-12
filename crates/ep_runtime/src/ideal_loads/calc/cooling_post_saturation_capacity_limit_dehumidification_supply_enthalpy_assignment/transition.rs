//! Pure CP416-to-CP417 supply-enthalpy psychrometric assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot as Predecessor;
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

mod accounting;
use accounting::{increment_counts, next_transition_fits};

/// One retained CP417 route over the lossless 36-wide CP416 partition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub logical_index: usize,
    pub predecessor_guard_false_fallthrough: bool,
    pub predecessor_guard_body_entered: bool,
    pub predecessor_saturation_temperature_assignment_executed: bool,
    pub predecessor_saturation_temperature_mixed_air_limit_executed: bool,
    pub predecessor_supply_humidity_ratio_assignment_executed: bool,
    pub active: bool,
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<RetainedRoute> {
    let route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        logical_index: route.logical_index,
        predecessor_guard_false_fallthrough: route.predecessor_guard_false_fallthrough,
        predecessor_guard_body_entered: route.predecessor_guard_body_entered,
        predecessor_saturation_temperature_assignment_executed: route
            .predecessor_saturation_temperature_assignment_executed,
        predecessor_saturation_temperature_mixed_air_limit_executed: route
            .predecessor_saturation_temperature_mixed_air_limit_executed,
        predecessor_supply_humidity_ratio_assignment_executed: route.active,
        active: route.active,
    })
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let operands = if route.active {
        Some((
            predecessor.resulting_supply_temperature_c?,
            predecessor.resulting_supply_humidity_ratio?,
        ))
    } else {
        None
    };
    if !next_transition_fits(state, predecessor, route) {
        return None;
    }

    let psychrometric_supply_enthalpy = operands
        .map(|(temperature, humidity_ratio)| energyplus_psy_h_fn_tdb_w(temperature, humidity_ratio));
    let resulting_supply_enthalpy_j_per_kg =
        psychrometric_supply_enthalpy.or(predecessor.resulting_supply_enthalpy_j_per_kg);
    let transition_ordinal = state.transition_count + 1;
    let snapshot = Snapshot {
        source: SOURCE,
        first_excluded_source: EXCLUDED,
        source_order: ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: predecessor.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: predecessor.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: predecessor.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: predecessor.dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: predecessor.predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: predecessor.predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: predecessor.predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: predecessor.predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: predecessor.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: predecessor.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: predecessor.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: predecessor.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: predecessor.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break: predecessor.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        predecessor_cp409_resulting_supply_humidity_ratio: predecessor.predecessor_cp409_resulting_supply_humidity_ratio,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg: predecessor.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp409_resulting_supply_temperature_c: predecessor.predecessor_cp409_resulting_supply_temperature_c,
        predecessor_dehumidification_control_default_case_exited_via_break: predecessor.predecessor_dehumidification_control_default_case_exited_via_break,
        predecessor_cp410_resulting_supply_humidity_ratio: predecessor.predecessor_cp410_resulting_supply_humidity_ratio,
        predecessor_cp410_resulting_supply_enthalpy_j_per_kg: predecessor.predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp410_resulting_supply_temperature_c: predecessor.predecessor_cp410_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed: predecessor.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
        cp410_retained_supply_humidity_ratio_state_owned: predecessor.cp410_retained_supply_humidity_ratio_state_owned,
        cp410_retained_supply_enthalpy_state_owned: predecessor.cp410_retained_supply_enthalpy_state_owned,
        cp410_retained_supply_temperature_state_owned: predecessor.cp410_retained_supply_temperature_state_owned,
        cp410_retained_supply_humidity_ratio_owned_read: predecessor.cp410_retained_supply_humidity_ratio_owned_read,
        purchased_air_supply_humidity_ratio_read: predecessor.purchased_air_supply_humidity_ratio_read,
        purchased_air_supply_humidity_ratio_before_saturation_check: predecessor.purchased_air_supply_humidity_ratio_before_saturation_check,
        local_supply_humidity_ratio_original_assignment_performed: predecessor.local_supply_humidity_ratio_original_assignment_performed,
        assigned_supply_humidity_ratio_original: predecessor.assigned_supply_humidity_ratio_original,
        resulting_supply_humidity_ratio_original: predecessor.resulting_supply_humidity_ratio_original,
        predecessor_cp411_resulting_supply_humidity_ratio: predecessor.predecessor_cp411_resulting_supply_humidity_ratio,
        predecessor_cp411_resulting_supply_enthalpy_j_per_kg: predecessor.predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp411_resulting_supply_temperature_c: predecessor.predecessor_cp411_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed: predecessor.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed,
        cp411_retained_supply_humidity_ratio_state_owned: predecessor.cp411_retained_supply_humidity_ratio_state_owned,
        cp411_retained_supply_enthalpy_state_owned: predecessor.cp411_retained_supply_enthalpy_state_owned,
        cp411_retained_supply_temperature_state_owned: predecessor.cp411_retained_supply_temperature_state_owned,
        cp411_retained_supply_temperature_owned_read: predecessor.cp411_retained_supply_temperature_owned_read,
        purchased_air_supply_temperature_for_saturation_humidity_ratio_read: predecessor.purchased_air_supply_temperature_for_saturation_humidity_ratio_read,
        supply_temperature_for_saturation_humidity_ratio_c: predecessor.supply_temperature_for_saturation_humidity_ratio_c,
        environment_outdoor_barometric_pressure_owned_read: predecessor.environment_outdoor_barometric_pressure_owned_read,
        environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: predecessor.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read,
        outdoor_barometric_pressure_pa: predecessor.outdoor_barometric_pressure_pa,
        psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: predecessor.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated,
        saturation_supply_humidity_ratio: predecessor.saturation_supply_humidity_ratio,
        local_saturation_supply_humidity_ratio_assignment_performed: predecessor.local_saturation_supply_humidity_ratio_assignment_performed,
        assigned_saturation_supply_humidity_ratio: predecessor.assigned_saturation_supply_humidity_ratio,
        resulting_saturation_supply_humidity_ratio: predecessor.resulting_saturation_supply_humidity_ratio,
        predecessor_cp412_resulting_supply_humidity_ratio: predecessor.predecessor_cp412_resulting_supply_humidity_ratio,
        predecessor_cp412_resulting_supply_enthalpy_j_per_kg: predecessor.predecessor_cp412_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp412_resulting_supply_temperature_c: predecessor.predecessor_cp412_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated: predecessor.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated,
        cp412_saturation_supply_humidity_ratio_owned_read: predecessor.cp412_saturation_supply_humidity_ratio_owned_read,
        saturation_supply_humidity_ratio_for_guard_read: predecessor.saturation_supply_humidity_ratio_for_guard_read,
        saturation_supply_humidity_ratio_for_guard: predecessor.saturation_supply_humidity_ratio_for_guard,
        cp411_original_supply_humidity_ratio_owned_read: predecessor.cp411_original_supply_humidity_ratio_owned_read,
        cp412_same_call_original_supply_humidity_ratio_bit_corroborated: predecessor.cp412_same_call_original_supply_humidity_ratio_bit_corroborated,
        original_supply_humidity_ratio_for_guard_read: predecessor.original_supply_humidity_ratio_for_guard_read,
        original_supply_humidity_ratio_for_guard: predecessor.original_supply_humidity_ratio_for_guard,
        saturation_original_supply_humidity_ratio_comparison_evaluated: predecessor.saturation_original_supply_humidity_ratio_comparison_evaluated,
        saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio: predecessor.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
        saturation_supply_humidity_ratio_guard_body_entered: predecessor.saturation_supply_humidity_ratio_guard_body_entered,
        saturation_supply_humidity_ratio_guard_false_fallthrough: predecessor.saturation_supply_humidity_ratio_guard_false_fallthrough,
        cp412_retained_supply_humidity_ratio_state_owned: predecessor.cp412_retained_supply_humidity_ratio_state_owned,
        cp412_retained_supply_enthalpy_state_owned: predecessor.cp412_retained_supply_enthalpy_state_owned,
        cp412_retained_supply_temperature_state_owned: predecessor.cp412_retained_supply_temperature_state_owned,
        predecessor_cp413_resulting_supply_humidity_ratio: predecessor.predecessor_cp413_resulting_supply_humidity_ratio,
        predecessor_cp413_resulting_supply_enthalpy_j_per_kg: predecessor.predecessor_cp413_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp413_resulting_supply_temperature_c: predecessor.predecessor_cp413_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed: predecessor.post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed,
        cp413_retained_supply_humidity_ratio_state_owned: predecessor.cp413_retained_supply_humidity_ratio_state_owned,
        cp413_retained_supply_enthalpy_state_owned: predecessor.cp413_retained_supply_enthalpy_state_owned,
        cp413_retained_supply_temperature_state_owned: predecessor.cp413_retained_supply_temperature_state_owned,
        cp413_retained_supply_enthalpy_owned_read: predecessor.cp413_retained_supply_enthalpy_owned_read,
        supply_enthalpy_for_saturation_temperature_read: predecessor.supply_enthalpy_for_saturation_temperature_read,
        supply_enthalpy_for_saturation_temperature_j_per_kg: predecessor.supply_enthalpy_for_saturation_temperature_j_per_kg,
        environment_outdoor_barometric_pressure_for_saturation_temperature_owned_read: predecessor.environment_outdoor_barometric_pressure_for_saturation_temperature_owned_read,
        environment_outdoor_barometric_pressure_for_saturation_temperature_read: predecessor.environment_outdoor_barometric_pressure_for_saturation_temperature_read,
        outdoor_barometric_pressure_for_saturation_temperature_pa: predecessor.outdoor_barometric_pressure_for_saturation_temperature_pa,
        psy_tsat_fn_h_pb_evaluated: predecessor.psy_tsat_fn_h_pb_evaluated,
        psychrometric_saturation_supply_temperature_result_c: predecessor.psychrometric_saturation_supply_temperature_result_c,
        purchased_air_supply_temperature_saturation_assignment_performed: predecessor.purchased_air_supply_temperature_saturation_assignment_performed,
        assigned_saturation_supply_temperature_c: predecessor.assigned_saturation_supply_temperature_c,
        predecessor_cp414_resulting_supply_humidity_ratio: predecessor.predecessor_cp414_resulting_supply_humidity_ratio,
        predecessor_cp414_resulting_supply_enthalpy_j_per_kg: predecessor.predecessor_cp414_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp414_resulting_supply_temperature_c: predecessor.predecessor_cp414_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed: predecessor.post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed,
        cp414_retained_supply_temperature_state_owned: predecessor.cp414_retained_supply_temperature_state_owned,
        preexisting_supply_temperature_c: predecessor.preexisting_supply_temperature_c,
        cp414_retained_supply_temperature_owned_read: predecessor.cp414_retained_supply_temperature_owned_read,
        supply_temperature_for_minimum_read: predecessor.supply_temperature_for_minimum_read,
        supply_temperature_before_mixed_air_limit_c: predecessor.supply_temperature_before_mixed_air_limit_c,
        cp329_retained_mixed_air_temperature_owned_read: predecessor.cp329_retained_mixed_air_temperature_owned_read,
        mixed_air_temperature_for_minimum_read: predecessor.mixed_air_temperature_for_minimum_read,
        mixed_air_temperature_c: predecessor.mixed_air_temperature_c,
        source_shaped_two_argument_minimum_evaluated: predecessor.source_shaped_two_argument_minimum_evaluated,
        minimum_supply_temperature_c: predecessor.minimum_supply_temperature_c,
        supply_temperature_assignment_performed: predecessor.supply_temperature_assignment_performed,
        assigned_supply_temperature_c: predecessor.assigned_supply_temperature_c,
        predecessor_cp415_resulting_supply_humidity_ratio: predecessor.predecessor_cp415_resulting_supply_humidity_ratio,
        predecessor_cp415_resulting_supply_enthalpy_j_per_kg: predecessor.predecessor_cp415_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp415_resulting_supply_temperature_c: predecessor.predecessor_cp415_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed: predecessor.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed,
        cp415_retained_supply_humidity_ratio_state_owned: predecessor.cp415_retained_supply_humidity_ratio_state_owned,
        cp415_retained_supply_enthalpy_state_owned: predecessor.cp415_retained_supply_enthalpy_state_owned,
        cp415_retained_supply_temperature_state_owned: predecessor.cp415_retained_supply_temperature_state_owned,
        cp415_retained_supply_temperature_owned_read: predecessor.cp415_retained_supply_temperature_owned_read,
        supply_temperature_for_humidity_ratio_inversion_read: predecessor.supply_temperature_for_humidity_ratio_inversion_read,
        supply_temperature_c: predecessor.supply_temperature_c,
        cp415_retained_supply_enthalpy_owned_read: predecessor.cp415_retained_supply_enthalpy_owned_read,
        supply_enthalpy_for_humidity_ratio_inversion_read: predecessor.supply_enthalpy_for_humidity_ratio_inversion_read,
        supply_enthalpy_j_per_kg: predecessor.supply_enthalpy_j_per_kg,
        psychrometric_supply_humidity_ratio_evaluated: predecessor.psychrometric_supply_humidity_ratio_evaluated,
        psychrometric_supply_humidity_ratio: predecessor.psychrometric_supply_humidity_ratio,
        supply_humidity_ratio_assignment_performed: predecessor.supply_humidity_ratio_assignment_performed,
        assigned_supply_humidity_ratio: predecessor.assigned_supply_humidity_ratio,
        predecessor_cp416_resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        predecessor_cp416_resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        predecessor_cp416_resulting_supply_temperature_c: predecessor.resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed: route.active,
        cp416_retained_supply_humidity_ratio_state_owned: predecessor.resulting_supply_humidity_ratio.is_some(),
        cp416_retained_supply_enthalpy_state_owned: predecessor.resulting_supply_enthalpy_j_per_kg.is_some(),
        cp416_retained_supply_temperature_state_owned: predecessor.resulting_supply_temperature_c.is_some(),
        cp416_retained_supply_temperature_owned_read: route.active,
        supply_temperature_for_enthalpy_read: route.active,
        supply_temperature_for_enthalpy_c: operands.map(|value| value.0),
        cp416_retained_supply_humidity_ratio_owned_read: route.active,
        supply_humidity_ratio_for_enthalpy_read: route.active,
        supply_humidity_ratio_for_enthalpy: operands.map(|value| value.1),
        psychrometric_supply_enthalpy_evaluated: route.active,
        psychrometric_supply_enthalpy_j_per_kg: psychrometric_supply_enthalpy,
        supply_enthalpy_assignment_performed: route.active,
        assigned_supply_enthalpy_j_per_kg: psychrometric_supply_enthalpy,
        resulting_supply_humidity_ratio: predecessor.resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: predecessor.resulting_supply_temperature_c,
    };

    state.transition_count = transition_ordinal;
    increment_counts(state, predecessor, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(transition_ordinal);
    Some(snapshot)
}
