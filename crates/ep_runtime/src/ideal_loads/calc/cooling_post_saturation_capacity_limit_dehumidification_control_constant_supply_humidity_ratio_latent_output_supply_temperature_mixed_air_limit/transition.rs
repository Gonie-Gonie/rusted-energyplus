//! Pure CP407-to-CP408 supply-temperature mixed-air limit.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
};

mod accounting;
mod owners;
pub(in crate::ideal_loads::calc) mod routes;

use accounting::{increment_counts, next_transition_fits};
use owners::prepare_exact_input;
pub(in crate::ideal_loads::calc) use routes::RetainedRoute;
use routes::predecessor_route;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use routes::{
    logical_route_index, predecessor_index_is_active, predecessor_index_is_public,
};

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_state(
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

    let minimum_supply_temperature_c = prepared
        .active
        .map(|active| source_minimum(active.supply_temperature_c, active.mixed_air_temperature_c));
    let resulting_supply_temperature_c =
        minimum_supply_temperature_c.or(prepared.predecessor_supply_temperature_c);
    let active = prepared.active;
    let transition_ordinal = state.transition_count + 1;

    let snapshot = Snapshot {
        source: SOURCE,
        first_excluded_source: EXCLUDED,
        source_order: ORDER,
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
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
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
        predecessor_dehumidification_control_none_case_entered: predecessor
            .predecessor_dehumidification_control_none_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered,
        predecessor_cp406_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp406_resulting_supply_humidity_ratio,
        predecessor_cp406_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp406_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp406_resulting_supply_temperature_c: predecessor
            .predecessor_cp406_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed: predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed,
        predecessor_cp385_retained_supply_enthalpy_owned_read: predecessor
            .cp385_retained_supply_enthalpy_owned_read,
        predecessor_cp406_same_call_supply_enthalpy_bit_corroborated: predecessor
            .cp406_same_call_supply_enthalpy_bit_corroborated,
        predecessor_supply_enthalpy_for_dry_bulb_inversion_read: predecessor
            .supply_enthalpy_for_dry_bulb_inversion_read,
        predecessor_supply_enthalpy_j_per_kg: predecessor.supply_enthalpy_j_per_kg,
        predecessor_cp378_retained_supply_humidity_ratio_owned_read: predecessor
            .cp378_retained_supply_humidity_ratio_owned_read,
        predecessor_supply_humidity_ratio_for_dry_bulb_inversion_read: predecessor
            .supply_humidity_ratio_for_dry_bulb_inversion_read,
        predecessor_supply_humidity_ratio: predecessor.supply_humidity_ratio,
        predecessor_cp406_retained_supply_temperature_state_owned: predecessor
            .cp406_retained_supply_temperature_state_owned,
        predecessor_preexisting_supply_temperature_c: predecessor
            .preexisting_supply_temperature_c,
        predecessor_psychrometric_supply_temperature_evaluated: predecessor
            .psychrometric_supply_temperature_evaluated,
        predecessor_psychrometric_supply_temperature_result_c: predecessor
            .psychrometric_supply_temperature_result_c,
        predecessor_supply_temperature_assigned: predecessor.supply_temperature_assigned,
        predecessor_assigned_supply_temperature_c: predecessor.assigned_supply_temperature_c,
        predecessor_resulting_supply_humidity_ratio: prepared.predecessor_supply_humidity_ratio,
        predecessor_resulting_supply_enthalpy_j_per_kg: prepared
            .predecessor_supply_enthalpy_j_per_kg,
        predecessor_resulting_supply_temperature_c: prepared.predecessor_supply_temperature_c,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_executed: route.active,
        cp407_retained_supply_temperature_state_owned: prepared
            .predecessor_supply_temperature_c
            .is_some(),
        preexisting_supply_temperature_c: prepared.predecessor_supply_temperature_c,
        cp407_retained_supply_temperature_owned_read: route.active,
        supply_temperature_for_minimum_read: route.active,
        supply_temperature_before_mixed_air_limit_c: active
            .map(|value| value.supply_temperature_c),
        cp329_retained_mixed_air_temperature_owned_read: route.active,
        mixed_air_temperature_for_minimum_read: route.active,
        mixed_air_temperature_c: active.map(|value| value.mixed_air_temperature_c),
        source_shaped_two_argument_minimum_evaluated: route.active,
        minimum_supply_temperature_c,
        supply_temperature_assignment_performed: route.active,
        assigned_supply_temperature_c: minimum_supply_temperature_c,
        resulting_supply_humidity_ratio: prepared.predecessor_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: prepared.predecessor_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c,
    };
    increment_counts(state, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(transition_ordinal);
    Some(snapshot)
}

/// Evaluates the source-shaped C++-compatible two-argument minimum.
pub(in crate::ideal_loads::calc) fn source_minimum(left: f64, right: f64) -> f64 {
    source_shaped_two_argument_minimum(left, right)
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_increment_counts(state: &mut State, route: RetainedRoute) {
    accounting::increment_counts(state, route);
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_next_transition_fits(
    state: &State,
    route: RetainedRoute,
) -> bool {
    accounting::next_transition_fits(state, route)
}
