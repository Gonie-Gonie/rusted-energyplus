//! Pure CP400-to-CP401 shared-case latent-output assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as TotalOutputOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as TotalOutputCorroborator,
};

mod accounting;
mod owners;
pub(in crate::ideal_loads::calc) mod routes;

use accounting::{increment_counts, next_transition_fits};
use owners::cooling_total_output_from_exact_owner;
use routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature, predecessor_route,
};

/// Same-call CP384/CP385 owner bundle required only on active CP401 routes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentActiveOwners
{
    pub cooling_total_output_owner: TotalOutputOwner,
    pub cooling_total_output_corroborator: TotalOutputCorroborator,
}

use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentActiveOwners as ActiveOwners;

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_state(
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
        let total = active.cooling_total_output_w;
        let sensible = active.cooling_sensible_output_w;
        let output = total - sensible;
        (active, output)
    });
    let index = route.predecessor_index;

    state.transition_count += 1;
    increment_counts(state, route);
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
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
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        predecessor_mixed_air_humidity_ratio_read: predecessor
            .predecessor_mixed_air_humidity_ratio_read,
        predecessor_mixed_air_humidity_ratio: predecessor.predecessor_mixed_air_humidity_ratio,
        predecessor_psychrometric_cp_air_evaluated: predecessor
            .predecessor_psychrometric_cp_air_evaluated,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: predecessor
            .predecessor_psychrometric_cp_air_result_j_per_kg_k,
        predecessor_cp_air_assigned: predecessor.predecessor_cp_air_assigned,
        predecessor_cp_air_j_per_kg_k: predecessor.predecessor_cp_air_j_per_kg_k,
        predecessor_cp399_resulting_supply_humidity_ratio: predecessor
            .predecessor_cp399_resulting_supply_humidity_ratio,
        predecessor_cp399_resulting_supply_enthalpy_j_per_kg: predecessor
            .predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp399_resulting_supply_temperature_c: predecessor
            .predecessor_cp399_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed: predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
        predecessor_cp399_retained_supply_humidity_ratio_state_owned: predecessor
            .cp399_retained_supply_humidity_ratio_state_owned,
        predecessor_cp399_retained_supply_enthalpy_state_owned: predecessor
            .cp399_retained_supply_enthalpy_state_owned,
        predecessor_cp399_retained_supply_temperature_state_owned: predecessor
            .cp399_retained_supply_temperature_state_owned,
        predecessor_cp330_retained_supply_mass_flow_rate_owned_read: predecessor
            .cp330_retained_supply_mass_flow_rate_owned_read,
        predecessor_cp329_supply_mass_flow_rate_bit_corroborated: predecessor
            .cp329_supply_mass_flow_rate_bit_corroborated,
        predecessor_supply_mass_flow_rate_read: predecessor.supply_mass_flow_rate_read,
        predecessor_supply_mass_flow_rate_kg_per_s: predecessor.supply_mass_flow_rate_kg_per_s,
        predecessor_cp399_retained_cp_air_owned_read: predecessor
            .cp399_retained_cp_air_owned_read,
        predecessor_cp_air_read: predecessor.cp_air_read,
        predecessor_cp400_cp_air_j_per_kg_k: predecessor.cp_air_j_per_kg_k,
        predecessor_supply_mass_flow_rate_times_cp_air_calculated: predecessor
            .supply_mass_flow_rate_times_cp_air_calculated,
        predecessor_supply_mass_flow_rate_times_cp_air_w_per_k: predecessor
            .supply_mass_flow_rate_times_cp_air_w_per_k,
        predecessor_cp329_retained_mixed_air_temperature_owned_read: predecessor
            .cp329_retained_mixed_air_temperature_owned_read,
        predecessor_mixed_air_temperature_read: predecessor.mixed_air_temperature_read,
        predecessor_mixed_air_temperature_c: predecessor.mixed_air_temperature_c,
        predecessor_cp399_retained_supply_temperature_owned_read: predecessor
            .cp399_retained_supply_temperature_owned_read,
        predecessor_supply_temperature_read: predecessor.supply_temperature_read,
        predecessor_supply_temperature_c: predecessor.supply_temperature_c,
        predecessor_mixed_air_minus_supply_temperature_calculated: predecessor
            .mixed_air_minus_supply_temperature_calculated,
        predecessor_mixed_air_minus_supply_temperature_k: predecessor
            .mixed_air_minus_supply_temperature_k,
        predecessor_cooling_sensible_output_calculated: predecessor
            .cooling_sensible_output_calculated,
        predecessor_calculated_cooling_sensible_output_w: predecessor
            .calculated_cooling_sensible_output_w,
        predecessor_cooling_sensible_output_assigned: predecessor.cooling_sensible_output_assigned,
        predecessor_cooling_sensible_output_w: predecessor.cooling_sensible_output_w,
        predecessor_cp400_resulting_supply_humidity_ratio: prepared
            .predecessor_supply_humidity_ratio,
        predecessor_cp400_resulting_supply_enthalpy_j_per_kg: prepared
            .predecessor_supply_enthalpy_j_per_kg,
        predecessor_cp400_resulting_supply_temperature_c: prepared
            .predecessor_supply_temperature_c,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed: route.active,
        cp400_retained_supply_humidity_ratio_state_owned:
            predecessor_has_supply_humidity_ratio(index),
        cp400_retained_supply_enthalpy_state_owned: predecessor_has_supply_enthalpy(index),
        cp400_retained_supply_temperature_state_owned: predecessor_has_supply_temperature(index),
        cp384_retained_cooling_total_output_owned_read: route.active,
        cp385_cooling_total_output_bit_corroborated: route.active,
        cooling_total_output_read: route.active,
        cooling_total_output_w: active_values.map(|(active, _)| active.cooling_total_output_w),
        cp400_retained_cooling_sensible_output_owned_read: route.active,
        cooling_sensible_output_read: route.active,
        cooling_sensible_output_w: active_values
            .map(|(active, _)| active.cooling_sensible_output_w),
        cooling_latent_output_calculated: route.active,
        calculated_cooling_latent_output_w: active_values.map(|(_, output)| output),
        cooling_latent_output_assigned: route.active,
        cooling_latent_output_w: active_values.map(|(_, output)| output),
        resulting_supply_humidity_ratio: prepared.predecessor_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: prepared.predecessor_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: prepared.predecessor_supply_temperature_c,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

#[derive(Clone, Copy)]
struct PreparedInput {
    predecessor_supply_humidity_ratio: Option<f64>,
    predecessor_supply_enthalpy_j_per_kg: Option<f64>,
    predecessor_supply_temperature_c: Option<f64>,
    active: Option<PreparedActive>,
}

#[derive(Clone, Copy)]
struct PreparedActive {
    cooling_total_output_w: f64,
    cooling_sensible_output_w: f64,
}

fn prepare_exact_input(
    predecessor: Predecessor,
    route: RetainedRoute,
    active_owners: Option<ActiveOwners>,
) -> Option<PreparedInput> {
    let index = route.predecessor_index;
    let predecessor_supply_humidity_ratio = predecessor.resulting_supply_humidity_ratio;
    let predecessor_supply_enthalpy_j_per_kg = predecessor.resulting_supply_enthalpy_j_per_kg;
    let predecessor_supply_temperature_c = predecessor.resulting_supply_temperature_c;
    if predecessor_supply_humidity_ratio.is_some() != predecessor_has_supply_humidity_ratio(index)
        || predecessor_supply_enthalpy_j_per_kg.is_some()
            != predecessor_has_supply_enthalpy(index)
        || predecessor_supply_temperature_c.is_some()
            != predecessor_has_supply_temperature(index)
    {
        return None;
    }
    let active = match (route.active, active_owners) {
        (false, None) => None,
        (true, Some(owners)) => {
            let cooling_total_output_w = cooling_total_output_from_exact_owner(
                predecessor,
                owners.cooling_total_output_owner,
                owners.cooling_total_output_corroborator,
            )?;
            let cooling_sensible_output_w = predecessor.cooling_sensible_output_w?;
            Some(PreparedActive {
                cooling_total_output_w,
                cooling_sensible_output_w,
            })
        }
        _ => return None,
    };
    Some(PreparedInput {
        predecessor_supply_humidity_ratio,
        predecessor_supply_enthalpy_j_per_kg,
        predecessor_supply_temperature_c,
        active,
    })
}
