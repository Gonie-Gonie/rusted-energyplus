//! Pure CP388-to-CP389 constant-SHR supply-temperature assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as CpAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as TemperatureOwner,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot as FlowOwner,
};

mod accounting;
mod owners;
pub(in crate::ideal_loads::calc) mod routes;

use accounting::{increment_counts, next_transition_fits};
use owners::prepare_exact_input;
use routes::{predecessor_has_supply_temperature, predecessor_route};

/// Formula-owner bundle required only on the three active CP389 routes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentActiveOwners
{
    pub mixed_air_owner: MixedAirOwner,
    pub supply_mass_flow_owner: FlowOwner,
    pub cp_air_owner: CpAirOwner,
}

/// Exact CP379 state carrier plus optional active formula owners.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRetainedInput
{
    pub cp379_temperature_owner: TemperatureOwner,
    pub active_owners: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentActiveOwners>,
}

use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentActiveOwners as ActiveOwners;
use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRetainedInput as RetainedInput;

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    retained_input: RetainedInput,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let prepared = prepare_exact_input(predecessor, route, retained_input)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    let (denominator, drop, calculated) = if let Some(active) = prepared.active {
        let denominator = active.cp_air_j_per_kg_k * active.supply_mass_flow_rate_kg_per_s;
        let drop = active.cooling_sensible_output_w / denominator;
        let calculated = active.mixed_air_temperature_c - drop;
        (Some(denominator), Some(drop), Some(calculated))
    } else {
        (None, None, None)
    };
    let resulting_supply_temperature_c = calculated.or(prepared.preexisting_supply_temperature_c);

    state.transition_count += 1;
    increment_counts(state, route);
    let active = prepared.active;
    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed: predecessor.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed,
        predecessor_cp384_retained_cooling_total_output_owned_read: predecessor.cp384_retained_cooling_total_output_owned_read,
        predecessor_cp385_cooling_total_output_bit_corroborated: predecessor.cp385_cooling_total_output_bit_corroborated,
        predecessor_cooling_total_output_read: predecessor.cooling_total_output_read,
        predecessor_cooling_total_output_w: predecessor.cooling_total_output_w,
        predecessor_cooling_sensible_heat_ratio_read: predecessor.cooling_sensible_heat_ratio_read,
        predecessor_cooling_sensible_heat_ratio: predecessor.cooling_sensible_heat_ratio,
        predecessor_cooling_sensible_output_calculated: predecessor.cooling_sensible_output_calculated,
        predecessor_calculated_cooling_sensible_output_w: predecessor.calculated_cooling_sensible_output_w,
        predecessor_cooling_sensible_output_assigned: predecessor.cooling_sensible_output_assigned,
        predecessor_cooling_sensible_output_w: predecessor.cooling_sensible_output_w,
        resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed: route.active,
        cp379_retained_supply_temperature_state_owned: predecessor_has_supply_temperature(route.predecessor_index),
        preexisting_supply_temperature_c: prepared.preexisting_supply_temperature_c,
        cp329_retained_mixed_air_temperature_owned_read: route.active,
        mixed_air_temperature_read: route.active,
        mixed_air_temperature_c: active.map(|value| value.mixed_air_temperature_c),
        cp388_retained_cooling_sensible_output_owned_read: route.active,
        cooling_sensible_output_read: route.active,
        cooling_sensible_output_w: active.map(|value| value.cooling_sensible_output_w),
        cp387_retained_cp_air_owned_read: route.active,
        cp_air_read: route.active,
        cp_air_j_per_kg_k: active.map(|value| value.cp_air_j_per_kg_k),
        cp330_retained_supply_mass_flow_rate_owned_read: route.active,
        cp329_supply_mass_flow_rate_bit_corroborated: route.active,
        supply_mass_flow_rate_read: route.active,
        supply_mass_flow_rate_kg_per_s: active.map(|value| value.supply_mass_flow_rate_kg_per_s),
        cp_air_times_supply_mass_flow_rate_calculated: route.active,
        cp_air_times_supply_mass_flow_rate_w_per_k: denominator,
        cooling_sensible_output_over_air_capacity_rate_calculated: route.active,
        cooling_sensible_output_over_air_capacity_rate_k: drop,
        supply_temperature_calculated: route.active,
        calculated_supply_temperature_c: calculated,
        supply_temperature_assigned: route.active,
        assigned_supply_temperature_c: calculated,
        resulting_supply_temperature_c,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}
