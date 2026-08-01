//! Pure CP389-to-CP390 supply-temperature mixed-air limit.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Predecessor,
};

mod accounting;
mod owners;
pub(in crate::ideal_loads::calc) mod routes;

use accounting::{increment_counts, next_transition_fits};
use owners::prepare_exact_input;
use routes::{predecessor_has_supply_temperature, predecessor_route};

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state(
    state: &mut State,
    predecessor: Predecessor,
    mixed_air_owner: Option<MixedAirOwner>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let prepared = prepare_exact_input(predecessor, route, mixed_air_owner)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    let minimum_supply_temperature_c = prepared.active.map(|active| {
        source_shaped_two_argument_minimum(
            active.supply_temperature_c,
            active.mixed_air_temperature_c,
        )
    });
    let resulting_supply_temperature_c =
        minimum_supply_temperature_c.or(prepared.preexisting_supply_temperature_c);

    state.transition_count += 1;
    increment_counts(state, route);
    let active = prepared.active;
    let has_temperature = predecessor_has_supply_temperature(route.predecessor_index);
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
        system: state.system,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        predecessor_mixed_air_humidity_ratio_read: predecessor.predecessor_mixed_air_humidity_ratio_read,
        predecessor_mixed_air_humidity_ratio: predecessor.predecessor_mixed_air_humidity_ratio,
        predecessor_psychrometric_cp_air_evaluated: predecessor.predecessor_psychrometric_cp_air_evaluated,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: predecessor.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        predecessor_cp_air_assigned: predecessor.predecessor_cp_air_assigned,
        predecessor_cp_air_j_per_kg_k: predecessor.predecessor_cp_air_j_per_kg_k,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        predecessor_cp384_retained_cooling_total_output_owned_read: predecessor.predecessor_cp384_retained_cooling_total_output_owned_read,
        predecessor_cp385_cooling_total_output_bit_corroborated: predecessor.predecessor_cp385_cooling_total_output_bit_corroborated,
        predecessor_cooling_total_output_read: predecessor.predecessor_cooling_total_output_read,
        predecessor_cooling_total_output_w: predecessor.predecessor_cooling_total_output_w,
        predecessor_cooling_sensible_heat_ratio_read: predecessor.predecessor_cooling_sensible_heat_ratio_read,
        predecessor_cooling_sensible_heat_ratio: predecessor.predecessor_cooling_sensible_heat_ratio,
        predecessor_cooling_sensible_output_calculated: predecessor.predecessor_cooling_sensible_output_calculated,
        predecessor_calculated_cooling_sensible_output_w: predecessor.predecessor_calculated_cooling_sensible_output_w,
        predecessor_cooling_sensible_output_assigned: predecessor.predecessor_cooling_sensible_output_assigned,
        predecessor_cooling_sensible_output_w: predecessor.predecessor_cooling_sensible_output_w,
        predecessor_resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed: predecessor.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed,
        predecessor_cp379_retained_supply_temperature_state_owned: predecessor.cp379_retained_supply_temperature_state_owned,
        predecessor_preexisting_supply_temperature_c: predecessor.preexisting_supply_temperature_c,
        predecessor_cp329_retained_mixed_air_temperature_owned_read: predecessor.cp329_retained_mixed_air_temperature_owned_read,
        predecessor_mixed_air_temperature_read: predecessor.mixed_air_temperature_read,
        predecessor_mixed_air_temperature_c: predecessor.mixed_air_temperature_c,
        predecessor_cp388_retained_cooling_sensible_output_owned_read: predecessor.cp388_retained_cooling_sensible_output_owned_read,
        predecessor_cooling_sensible_output_read: predecessor.cooling_sensible_output_read,
        predecessor_cp389_cooling_sensible_output_w: predecessor.cooling_sensible_output_w,
        predecessor_cp387_retained_cp_air_owned_read: predecessor.cp387_retained_cp_air_owned_read,
        predecessor_cp_air_read: predecessor.cp_air_read,
        predecessor_cp389_cp_air_j_per_kg_k: predecessor.cp_air_j_per_kg_k,
        predecessor_cp330_retained_supply_mass_flow_rate_owned_read: predecessor.cp330_retained_supply_mass_flow_rate_owned_read,
        predecessor_cp329_supply_mass_flow_rate_bit_corroborated: predecessor.cp329_supply_mass_flow_rate_bit_corroborated,
        predecessor_supply_mass_flow_rate_read: predecessor.supply_mass_flow_rate_read,
        predecessor_supply_mass_flow_rate_kg_per_s: predecessor.supply_mass_flow_rate_kg_per_s,
        predecessor_cp_air_times_supply_mass_flow_rate_calculated: predecessor.cp_air_times_supply_mass_flow_rate_calculated,
        predecessor_cp_air_times_supply_mass_flow_rate_w_per_k: predecessor.cp_air_times_supply_mass_flow_rate_w_per_k,
        predecessor_cooling_sensible_output_over_air_capacity_rate_calculated: predecessor.cooling_sensible_output_over_air_capacity_rate_calculated,
        predecessor_cooling_sensible_output_over_air_capacity_rate_k: predecessor.cooling_sensible_output_over_air_capacity_rate_k,
        predecessor_supply_temperature_calculated: predecessor.supply_temperature_calculated,
        predecessor_calculated_supply_temperature_c: predecessor.calculated_supply_temperature_c,
        predecessor_supply_temperature_assigned: predecessor.supply_temperature_assigned,
        predecessor_assigned_supply_temperature_c: predecessor.assigned_supply_temperature_c,
        predecessor_resulting_supply_temperature_c: predecessor.resulting_supply_temperature_c,
        resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed: route.active,
        cp389_retained_supply_temperature_state_owned: has_temperature,
        preexisting_supply_temperature_c: prepared.preexisting_supply_temperature_c,
        cp389_retained_supply_temperature_owned_read: route.active,
        supply_temperature_for_minimum_read: route.active,
        supply_temperature_before_mixed_air_limit_c: active.map(|value| value.supply_temperature_c),
        cp329_retained_mixed_air_temperature_owned_read: route.active,
        cp389_mixed_air_temperature_bit_corroborated: route.active,
        mixed_air_temperature_for_minimum_read: route.active,
        mixed_air_temperature_c: active.map(|value| value.mixed_air_temperature_c),
        source_shaped_two_argument_minimum_evaluated: route.active,
        minimum_supply_temperature_c,
        supply_temperature_assignment_performed: route.active,
        assigned_supply_temperature_c: minimum_supply_temperature_c,
        resulting_supply_temperature_c,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
