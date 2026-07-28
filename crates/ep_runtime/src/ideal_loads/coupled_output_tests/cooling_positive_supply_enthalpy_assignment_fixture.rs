use crate::{
    ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    },
    psychrometrics::energyplus_psy_h_fn_tdb_w,
};

pub(super) fn calculation_cooling_positive_supply_enthalpy_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    temperature: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
    let assignment_executed = predecessor.supply_humidity_ratio_mixed_air_assignment_executed;
    let supply_temperature_c = assignment_executed
        .then_some(temperature.assigned_supply_temperature_c)
        .flatten();
    let supply_humidity_ratio = assignment_executed
        .then_some(predecessor.assigned_supply_humidity_ratio)
        .flatten();
    let enthalpy = supply_temperature_c.zip(supply_humidity_ratio).map(
        |(supply_temperature_c, supply_humidity_ratio)| {
            energyplus_psy_h_fn_tdb_w(supply_temperature_c, supply_humidity_ratio)
        },
    );

    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
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
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        supply_enthalpy_assignment_executed: assignment_executed,
        supply_temperature_for_enthalpy_read: assignment_executed,
        supply_temperature_c,
        supply_humidity_ratio_for_enthalpy_read: assignment_executed,
        supply_humidity_ratio,
        psychrometric_supply_enthalpy_evaluated: assignment_executed,
        psychrometric_supply_enthalpy_result_j_per_kg: enthalpy,
        supply_enthalpy_assigned: assignment_executed,
        supply_enthalpy_j_per_kg: enthalpy,
    }
}
