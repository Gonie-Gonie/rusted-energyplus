//! Pure CP399-to-CP400 shared-case sensible-output assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot as FlowOwner,
};

mod accounting;
mod owners;
pub(in crate::ideal_loads::calc) mod routes;

use accounting::{increment_counts, next_transition_fits};
use owners::prepare_exact_input;
use routes::{
    predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature, predecessor_route,
};

/// Same-call CP329/CP330 owner bundle required only on active CP400 routes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentActiveOwners
{
    pub mixed_air_owner: MixedAirOwner,
    pub supply_mass_flow_owner: FlowOwner,
}

use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentActiveOwners as ActiveOwners;

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    active_owners: Option<ActiveOwners>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let prepared = prepare_exact_input(predecessor, route, active_owners)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    let active_values = prepared.active.map(|active| {
        let first_product = active.supply_mass_flow_rate_kg_per_s * active.cp_air_j_per_kg_k;
        let difference = active.mixed_air_temperature_c - active.supply_temperature_c;
        let output = first_product * difference;
        (active, first_product, difference, output)
    });
    let index = route.predecessor_index;

    state.transition_count += 1;
    increment_counts(state, route);
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: predecessor
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: predecessor
            .dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: predecessor
            .predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: predecessor
            .predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: predecessor
            .predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: predecessor
            .predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: predecessor
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_cp397_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp397_resulting_supply_humidity_ratio,
        predecessor_cp397_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp397_resulting_supply_temperature_c: predecessor
            .predecessor_cp397_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_case_entered: predecessor
            .predecessor_dehumidification_control_none_case_entered,
        predecessor_cp398_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp398_resulting_supply_humidity_ratio,
        predecessor_cp398_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp398_resulting_supply_temperature_c: predecessor
            .predecessor_cp398_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed: predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        predecessor_mixed_air_humidity_ratio_read: predecessor.mixed_air_humidity_ratio_read,
        predecessor_mixed_air_humidity_ratio: predecessor.mixed_air_humidity_ratio,
        predecessor_psychrometric_cp_air_evaluated: predecessor.psychrometric_cp_air_evaluated,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: predecessor
            .psychrometric_cp_air_result_j_per_kg_k,
        predecessor_cp_air_assigned: predecessor.cp_air_assigned,
        predecessor_cp_air_j_per_kg_k: predecessor.cp_air_j_per_kg_k,
        predecessor_cp399_resulting_supply_humidity_ratio: prepared
            .predecessor_supply_humidity_ratio,
        predecessor_cp399_resulting_supply_enthalpy_j_per_kg: prepared
            .predecessor_supply_enthalpy_j_per_kg,
        predecessor_cp399_resulting_supply_temperature_c: prepared
            .predecessor_supply_temperature_c,
        dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed: route.active,
        cp399_retained_supply_humidity_ratio_state_owned:
            predecessor_has_supply_humidity_ratio(index),
        cp399_retained_supply_enthalpy_state_owned: predecessor_has_supply_enthalpy(index),
        cp399_retained_supply_temperature_state_owned: predecessor_has_supply_temperature(index),
        cp330_retained_supply_mass_flow_rate_owned_read: route.active,
        cp329_supply_mass_flow_rate_bit_corroborated: route.active,
        supply_mass_flow_rate_read: route.active,
        supply_mass_flow_rate_kg_per_s: active_values
            .map(|(active, _, _, _)| active.supply_mass_flow_rate_kg_per_s),
        cp399_retained_cp_air_owned_read: route.active,
        cp_air_read: route.active,
        cp_air_j_per_kg_k: active_values.map(|(active, _, _, _)| active.cp_air_j_per_kg_k),
        supply_mass_flow_rate_times_cp_air_calculated: route.active,
        supply_mass_flow_rate_times_cp_air_w_per_k: active_values
            .map(|(_, first_product, _, _)| first_product),
        cp329_retained_mixed_air_temperature_owned_read: route.active,
        mixed_air_temperature_read: route.active,
        mixed_air_temperature_c: active_values
            .map(|(active, _, _, _)| active.mixed_air_temperature_c),
        cp399_retained_supply_temperature_owned_read: route.active,
        supply_temperature_read: route.active,
        supply_temperature_c: active_values
            .map(|(active, _, _, _)| active.supply_temperature_c),
        mixed_air_minus_supply_temperature_calculated: route.active,
        mixed_air_minus_supply_temperature_k: active_values
            .map(|(_, _, difference, _)| difference),
        cooling_sensible_output_calculated: route.active,
        calculated_cooling_sensible_output_w: active_values.map(|(_, _, _, output)| output),
        cooling_sensible_output_assigned: route.active,
        cooling_sensible_output_w: active_values.map(|(_, _, _, output)| output),
        resulting_supply_humidity_ratio: prepared.predecessor_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: prepared.predecessor_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: prepared.predecessor_supply_temperature_c,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
