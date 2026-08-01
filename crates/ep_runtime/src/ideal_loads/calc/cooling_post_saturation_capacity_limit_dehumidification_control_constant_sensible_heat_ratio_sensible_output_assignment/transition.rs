//! Pure CP387-to-CP388 constant-SHR sensible-output assignment.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Owner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Corroborator,
};

mod accounting;
mod owners;
pub(in crate::ideal_loads::calc) mod routes;

use accounting::{increment_counts, next_transition_fits};
use owners::cooling_total_output_from_exact_owner;
use routes::predecessor_route;

/// Exact CP384/CP385 owner bundle plus the selected-system `CoolSHR` value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput
{
    pub cooling_total_output_owner: Owner,
    pub cooling_total_output_corroborator: Corroborator,
    pub cooling_sensible_heat_ratio: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    active_input: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let prepared = prepare_values(predecessor, route.active, active_input)?;
    if !next_transition_fits(state, route) {
        return None;
    }
    state.transition_count += 1;
    increment_counts(state, route);

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: predecessor.dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed: predecessor.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        predecessor_mixed_air_humidity_ratio_read: predecessor.mixed_air_humidity_ratio_read,
        predecessor_mixed_air_humidity_ratio: predecessor.mixed_air_humidity_ratio,
        predecessor_psychrometric_cp_air_evaluated: predecessor.psychrometric_cp_air_evaluated,
        predecessor_psychrometric_cp_air_result_j_per_kg_k: predecessor.psychrometric_cp_air_result_j_per_kg_k,
        predecessor_cp_air_assigned: predecessor.cp_air_assigned,
        predecessor_cp_air_j_per_kg_k: predecessor.cp_air_j_per_kg_k,
        dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed: route.active,
        cp384_retained_cooling_total_output_owned_read: route.active,
        cp385_cooling_total_output_bit_corroborated: route.active,
        cooling_total_output_read: route.active,
        cooling_total_output_w: prepared.cooling_total_output_w,
        cooling_sensible_heat_ratio_read: route.active,
        cooling_sensible_heat_ratio: prepared.cooling_sensible_heat_ratio,
        cooling_sensible_output_calculated: route.active,
        calculated_cooling_sensible_output_w: prepared.cooling_sensible_output_w,
        cooling_sensible_output_assigned: route.active,
        cooling_sensible_output_w: prepared.cooling_sensible_output_w,
        resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

fn prepare_values(
    predecessor: Predecessor,
    active: bool,
    active_input: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput>,
) -> Option<PreparedValues> {
    if !active {
        return active_input.is_none().then_some(PreparedValues::empty());
    }
    let input = active_input?;
    let cooling_total_output_w = cooling_total_output_from_exact_owner(
        predecessor,
        input.cooling_total_output_owner,
        input.cooling_total_output_corroborator,
    )?;
    let cooling_sensible_heat_ratio = input.cooling_sensible_heat_ratio;
    let cooling_sensible_output_w = cooling_total_output_w * cooling_sensible_heat_ratio;
    Some(PreparedValues {
        cooling_total_output_w: Some(cooling_total_output_w),
        cooling_sensible_heat_ratio: Some(cooling_sensible_heat_ratio),
        cooling_sensible_output_w: Some(cooling_sensible_output_w),
    })
}

#[derive(Clone, Copy)]
struct PreparedValues {
    cooling_total_output_w: Option<f64>,
    cooling_sensible_heat_ratio: Option<f64>,
    cooling_sensible_output_w: Option<f64>,
}

impl PreparedValues {
    const fn empty() -> Self {
        Self {
            cooling_total_output_w: None,
            cooling_sensible_heat_ratio: None,
            cooling_sensible_output_w: None,
        }
    }
}
