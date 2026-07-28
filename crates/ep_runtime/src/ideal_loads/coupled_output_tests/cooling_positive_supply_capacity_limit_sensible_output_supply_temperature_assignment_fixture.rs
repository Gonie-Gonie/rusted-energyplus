use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};

pub(super) fn calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    temperature_owner: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    humidity_owner: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot
{
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment = predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let active_prefix = guard_false || assignment;
    let preexisting = active_prefix
        .then_some(temperature_owner.assigned_supply_temperature_c)
        .flatten();
    let active = assignment.then(|| {
        let enthalpy = predecessor
            .resulting_supply_enthalpy_j_per_kg
            .expect("active CP342 enthalpy");
        let humidity = humidity_owner
            .assigned_supply_humidity_ratio
            .expect("active CP335 humidity ratio");
        let result = crate::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
        (enthalpy, humidity, result)
    });

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough:
            predecessor.predecessor_active_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated:
            predecessor.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered:
            predecessor.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough:
            predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_capacity_limit_cp_air_assignment_executed:
            predecessor.predecessor_capacity_limit_cp_air_assignment_executed,
        predecessor_capacity_limit_sensible_output_assignment_executed:
            predecessor.predecessor_capacity_limit_sensible_output_assignment_executed,
        predecessor_capacity_limit_sensible_output_guard_evaluated:
            predecessor.predecessor_capacity_limit_sensible_output_guard_evaluated,
        predecessor_capacity_limit_sensible_output_guard_false_fallthrough:
            predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough,
        predecessor_capacity_limit_sensible_output_adjustment_body_entered:
            predecessor.predecessor_capacity_limit_sensible_output_adjustment_body_entered,
        predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed:
            predecessor
                .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed,
        predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed:
            assignment,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        capacity_limit_guard_false_fallthrough_skipped:
            predecessor.capacity_limit_guard_false_fallthrough_skipped,
        capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
        capacity_limit_sensible_output_supply_temperature_assignment_executed:
            assignment,
        preexisting_supply_temperature_c: preexisting,
        supply_enthalpy_for_dry_bulb_inversion_read: assignment,
        supply_enthalpy_j_per_kg: active.map(|values| values.0),
        supply_humidity_ratio_for_dry_bulb_inversion_read: assignment,
        supply_humidity_ratio: active.map(|values| values.1),
        psychrometric_supply_temperature_evaluated: assignment,
        psychrometric_supply_temperature_result_c: active.map(|values| values.2),
        supply_temperature_assigned: assignment,
        assigned_supply_temperature_c: active.map(|values| values.2),
        resulting_supply_temperature_c: if assignment {
            active.map(|values| values.2)
        } else {
            preexisting
        },
    }
}
