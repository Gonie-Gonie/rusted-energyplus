use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    supply_flow: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    supply_enthalpy: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
    let assignment_executed = predecessor.capacity_limit_cp_air_assignment_executed;
    let values = assignment_executed.then(|| {
        let supply_mass_flow_rate_kg_per_s = supply_flow
            .supply_mass_flow_rate_kg_per_s
            .expect("active fixture supply mass flow");
        let mixed_air_enthalpy_j_per_kg = mixed_air
            .mixed_air_enthalpy_projection_j_per_kg
            .expect("active fixture mixed-air enthalpy");
        let supply_enthalpy_j_per_kg = supply_enthalpy
            .supply_enthalpy_j_per_kg
            .expect("active fixture supply enthalpy");
        let mixed_air_minus_supply_enthalpy_j_per_kg =
            mixed_air_enthalpy_j_per_kg - supply_enthalpy_j_per_kg;
        let cooling_sensible_output_w =
            supply_mass_flow_rate_kg_per_s * mixed_air_minus_supply_enthalpy_j_per_kg;
        (
            supply_mass_flow_rate_kg_per_s,
            mixed_air_enthalpy_j_per_kg,
            supply_enthalpy_j_per_kg,
            mixed_air_minus_supply_enthalpy_j_per_kg,
            cooling_sensible_output_w,
        )
    });

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
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
            .capacity_limit_cp_air_assignment_executed,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        capacity_limit_guard_false_fallthrough_skipped: predecessor
            .capacity_limit_guard_false_fallthrough_skipped,
        capacity_limit_sensible_output_assignment_executed: assignment_executed,
        supply_mass_flow_rate_read: assignment_executed,
        supply_mass_flow_rate_kg_per_s: values.map(|values| values.0),
        mixed_air_enthalpy_read: assignment_executed,
        mixed_air_enthalpy_j_per_kg: values.map(|values| values.1),
        supply_enthalpy_read: assignment_executed,
        supply_enthalpy_j_per_kg: values.map(|values| values.2),
        enthalpy_difference_calculated: assignment_executed,
        mixed_air_minus_supply_enthalpy_j_per_kg: values.map(|values| values.3),
        cooling_sensible_output_calculated: assignment_executed,
        calculated_cooling_sensible_output_w: values.map(|values| values.4),
        cooling_sensible_output_assigned: assignment_executed,
        cooling_sensible_output_w: values.map(|values| values.4),
    }
}
