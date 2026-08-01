use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Predecessor,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot(
    predecessor: Predecessor,
    control: DehumidificationControlType,
) -> Snapshot {
    let active = predecessor.supply_enthalpy_assignment_executed;
    Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
        first_excluded_lexical_source: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
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
        predecessor_supply_enthalpy_assignment_executed: predecessor.supply_enthalpy_assignment_executed,
        predecessor_resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
        dehumidification_control_type_read: active,
        dehumidification_control_type: active.then_some(control),
        dehumidification_control_switch_dispatched: active,
        resulting_supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg,
    }
}
