//! Pure CP394-to-CP395 Humidistat supply-humidity-ratio assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot as Predecessor;
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

mod accounting;
mod owners;
pub(in crate::ideal_loads::calc) mod routes;

use accounting::{increment_counts, next_transition_fits};
use owners::prepare_exact_input;
use routes::{
    predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature, predecessor_route,
};

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let prepared = prepare_exact_input(predecessor, route)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    let psychrometric_supply_humidity_ratio = prepared.active.map(|active| {
        energyplus_psy_w_fn_tdb_h(active.supply_temperature_c, active.supply_enthalpy_j_per_kg)
    });
    let resulting_supply_humidity_ratio = if route.active {
        psychrometric_supply_humidity_ratio
    } else {
        prepared.predecessor_supply_humidity_ratio
    };
    let index = route.predecessor_index;

    state.transition_count += 1;
    increment_counts(state, route);
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_cp393_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp393_resulting_supply_humidity_ratio,
        predecessor_cp393_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp393_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp393_resulting_supply_temperature_c: predecessor
            .predecessor_cp393_resulting_supply_temperature_c,
        predecessor_dehumidification_control_humidistat_case_entered: predecessor
            .dehumidification_control_humidistat_case_entered,
        predecessor_cp394_resulting_supply_humidity_ratio: prepared
            .predecessor_supply_humidity_ratio,
        predecessor_cp394_resulting_supply_enthalpy_j_per_kg: prepared
            .predecessor_supply_enthalpy_j_per_kg,
        predecessor_cp394_resulting_supply_temperature_c: prepared
            .predecessor_supply_temperature_c,
        dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: route
            .active,
        cp394_retained_supply_humidity_ratio_state_owned:
            predecessor_has_supply_humidity_ratio(index),
        cp394_retained_supply_temperature_state_owned: predecessor_has_supply_temperature(index),
        cp394_retained_supply_enthalpy_state_owned: predecessor_has_supply_enthalpy(index),
        cp394_retained_supply_temperature_owned_read: route.active,
        supply_temperature_for_humidity_ratio_inversion_read: route.active,
        supply_temperature_c: prepared.active.map(|active| active.supply_temperature_c),
        cp394_retained_supply_enthalpy_owned_read: route.active,
        supply_enthalpy_for_humidity_ratio_inversion_read: route.active,
        supply_enthalpy_j_per_kg: prepared
            .active
            .map(|active| active.supply_enthalpy_j_per_kg),
        psychrometric_supply_humidity_ratio_evaluated: route.active,
        psychrometric_supply_humidity_ratio,
        supply_humidity_ratio_assignment_performed: route.active,
        assigned_supply_humidity_ratio: psychrometric_supply_humidity_ratio,
        resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: prepared.predecessor_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: prepared.predecessor_supply_temperature_c,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
