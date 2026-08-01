use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot,
    supply_mass_flow_owner: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mixed_air_owner: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    early_total_corroborator: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    supply_enthalpy_owner: PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot
{
    let active = predecessor.dehumidification_body_entered;
    let supply_mass_flow_rate_kg_per_s = active.then(|| {
        let value = supply_mass_flow_owner
            .supply_mass_flow_rate_kg_per_s
            .expect("active CP330 supply-mass-flow owner");
        debug_assert_eq!(
            value.to_bits(),
            mixed_air_owner
                .supply_mass_flow_rate_kg_per_s
                .expect("active CP329 supply-mass-flow corroborator")
                .to_bits(),
        );
        debug_assert_eq!(
            value.to_bits(),
            early_total_corroborator
                .supply_mass_flow_rate_kg_per_s
                .expect("active CP339 supply-mass-flow corroborator")
                .to_bits(),
        );
        debug_assert_eq!(
            value.to_bits(),
            mixed_air_owner
                .resulting_recirculation_mass_flow_rate_kg_per_s
                .expect("active CP329 recirculation-mass-flow corroborator")
                .to_bits(),
        );
        value
    });
    let mixed_air_enthalpy_j_per_kg = active.then(|| {
        let value = mixed_air_owner
            .mixed_air_enthalpy_projection_j_per_kg
            .expect("active CP329 mixed-air-enthalpy owner");
        debug_assert_eq!(
            value.to_bits(),
            mixed_air_owner
                .recirculation_enthalpy_projection_j_per_kg
                .expect("active CP329 recirculation-enthalpy corroborator")
                .to_bits(),
        );
        debug_assert_eq!(
            value.to_bits(),
            early_total_corroborator
                .mixed_air_enthalpy_j_per_kg
                .expect("active CP339 mixed-air-enthalpy corroborator")
                .to_bits(),
        );
        value
    });
    let supply_enthalpy_j_per_kg = active.then(|| {
        let value = supply_enthalpy_owner
            .resulting_supply_enthalpy_j_per_kg
            .expect("active CP379 supply-enthalpy owner");
        debug_assert_eq!(
            value.to_bits(),
            supply_enthalpy_owner
                .assigned_supply_enthalpy_j_per_kg
                .expect("active CP379 supply-enthalpy corroborator")
                .to_bits(),
        );
        debug_assert_eq!(
            value.to_bits(),
            supply_enthalpy_owner
                .psychrometric_supply_enthalpy_j_per_kg
                .expect("active CP379 psychrometric supply-enthalpy corroborator")
                .to_bits(),
        );
        value
    });
    let mixed_air_minus_supply_enthalpy_j_per_kg = mixed_air_enthalpy_j_per_kg
        .zip(supply_enthalpy_j_per_kg)
        .map(|(mixed_air, supply)| mixed_air - supply);
    let cooling_total_output_w = supply_mass_flow_rate_kg_per_s
        .zip(mixed_air_minus_supply_enthalpy_j_per_kg)
        .map(|(mass_flow, difference)| mass_flow * difference);

    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
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
            .dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor.dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .dehumidification_guard_false_fallthrough,
        dehumidification_total_output_assignment_executed: active,
        cp330_supply_mass_flow_rate_owned_read: active,
        cp329_same_call_supply_mass_flow_rate_bit_corroborated: active,
        cp339_same_call_supply_mass_flow_rate_bit_corroborated: active,
        supply_mass_flow_rate_read: active,
        supply_mass_flow_rate_kg_per_s,
        cp329_mixed_air_enthalpy_owned_read: active,
        cp329_same_call_recirculation_enthalpy_bit_corroborated: active,
        cp339_same_call_mixed_air_enthalpy_bit_corroborated: active,
        mixed_air_enthalpy_read: active,
        mixed_air_enthalpy_j_per_kg,
        cp379_post_saturation_supply_enthalpy_owned_read: active,
        cp379_same_call_supply_enthalpy_bits_corroborated: active,
        supply_enthalpy_read: active,
        supply_enthalpy_j_per_kg,
        enthalpy_difference_calculated: active,
        mixed_air_minus_supply_enthalpy_j_per_kg,
        cooling_total_output_calculated: active,
        calculated_cooling_total_output_w: cooling_total_output_w,
        cooling_total_output_assigned: active,
        cooling_total_output_w,
    }
}
