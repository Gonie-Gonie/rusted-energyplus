use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as OperandBundle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Snapshot,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot(
    predecessor: Predecessor,
    operands: OperandBundle,
) -> Snapshot {
    let assignment = predecessor.dehumidification_total_output_maximum_capacity_assignment_executed;
    let retained =
        predecessor.dehumidification_total_output_capacity_guard_false_fallthrough || assignment;
    let preexisting = retained
        .then_some(operands.supply_enthalpy_j_per_kg)
        .flatten();
    let active = assignment
        .then(|| {
            let mixed = operands.mixed_air_enthalpy_j_per_kg?;
            let total = predecessor.resulting_cooling_total_output_w?;
            let flow = operands.supply_mass_flow_rate_kg_per_s?;
            let specific = total / flow;
            let calculated = mixed - specific;
            Some((mixed, total, flow, specific, calculated))
        })
        .flatten();

    Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
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
        dehumidification_total_output_maximum_capacity_assignment_executed: assignment,
        supply_enthalpy_assignment_executed: assignment,
        preexisting_supply_enthalpy_j_per_kg: preexisting,
        cp379_retained_supply_enthalpy_owned_read: retained,
        cp329_retained_mixed_air_enthalpy_owned_read: assignment,
        mixed_air_enthalpy_read: assignment,
        mixed_air_enthalpy_j_per_kg: active.map(|values| values.0),
        cp384_retained_cooling_total_output_owned_read: assignment,
        cooling_total_output_read: assignment,
        cooling_total_output_w: active.map(|values| values.1),
        cp330_retained_supply_mass_flow_rate_owned_read: assignment,
        supply_mass_flow_rate_read: assignment,
        supply_mass_flow_rate_kg_per_s: active.map(|values| values.2),
        specific_cooling_output_calculated: assignment,
        specific_cooling_output_j_per_kg: active.map(|values| values.3),
        supply_enthalpy_difference_calculated: assignment,
        calculated_supply_enthalpy_j_per_kg: active.map(|values| values.4),
        supply_enthalpy_assigned: assignment,
        assigned_supply_enthalpy_j_per_kg: active.map(|values| values.4),
        resulting_supply_enthalpy_j_per_kg: if assignment {
            active.map(|values| values.4)
        } else {
            preexisting
        },
    }
}
