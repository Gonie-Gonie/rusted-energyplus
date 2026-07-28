use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    retained_sensible_output:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot
{
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment =
        predecessor.capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    let preexisting_supply_enthalpy_j_per_kg = (guard_false || assignment)
        .then_some(retained_sensible_output.supply_enthalpy_j_per_kg)
        .flatten();
    let calculated = assignment.then(|| {
        let mixed_air_enthalpy_j_per_kg = retained_sensible_output
            .mixed_air_enthalpy_j_per_kg
            .expect("active fixture mixed-air enthalpy");
        let cooling_sensible_output_w = predecessor
            .resulting_cooling_sensible_output_w
            .expect("active fixture resulting sensible output");
        let supply_mass_flow_rate_kg_per_s = retained_sensible_output
            .supply_mass_flow_rate_kg_per_s
            .expect("active fixture supply mass flow");
        let specific_cooling_output_j_per_kg =
            cooling_sensible_output_w / supply_mass_flow_rate_kg_per_s;
        let supply_enthalpy_j_per_kg =
            mixed_air_enthalpy_j_per_kg - specific_cooling_output_j_per_kg;
        (
            mixed_air_enthalpy_j_per_kg,
            cooling_sensible_output_w,
            supply_mass_flow_rate_kg_per_s,
            specific_cooling_output_j_per_kg,
            supply_enthalpy_j_per_kg,
        )
    });

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor
            .predecessor_active_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_capacity_limit_cp_air_assignment_executed: predecessor
            .predecessor_capacity_limit_cp_air_assignment_executed,
        predecessor_capacity_limit_sensible_output_assignment_executed: predecessor
            .predecessor_capacity_limit_sensible_output_assignment_executed,
        predecessor_capacity_limit_sensible_output_guard_evaluated: predecessor
            .predecessor_capacity_limit_sensible_output_guard_evaluated,
        predecessor_capacity_limit_sensible_output_guard_false_fallthrough: predecessor
            .predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
        predecessor_capacity_limit_sensible_output_adjustment_body_entered: predecessor
            .predecessor_capacity_limit_sensible_output_adjustment_body_entered,
        predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed:
            assignment,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        capacity_limit_guard_false_fallthrough_skipped: predecessor
            .capacity_limit_guard_false_fallthrough_skipped,
        capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        capacity_limit_sensible_output_supply_enthalpy_assignment_executed: assignment,
        preexisting_supply_enthalpy_j_per_kg,
        mixed_air_enthalpy_read: assignment,
        mixed_air_enthalpy_j_per_kg: calculated.map(|values| values.0),
        cooling_sensible_output_read: assignment,
        cooling_sensible_output_w: calculated.map(|values| values.1),
        supply_mass_flow_rate_read: assignment,
        supply_mass_flow_rate_kg_per_s: calculated.map(|values| values.2),
        specific_cooling_output_calculated: assignment,
        specific_cooling_output_j_per_kg: calculated.map(|values| values.3),
        supply_enthalpy_calculated: assignment,
        calculated_supply_enthalpy_j_per_kg: calculated.map(|values| values.4),
        supply_enthalpy_assigned: assignment,
        assigned_supply_enthalpy_j_per_kg: calculated.map(|values| values.4),
        resulting_supply_enthalpy_j_per_kg: if assignment {
            calculated.map(|values| values.4)
        } else {
            preexisting_supply_enthalpy_j_per_kg
        },
    }
}
