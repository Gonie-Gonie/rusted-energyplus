use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot(
    predecessor: PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot,
    supply_owner: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    supply_corroborator: PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
    mixed_air_owner: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot {
    let active = predecessor.capacity_limit_body_entered;
    let supply_humidity_ratio = active
        .then_some(supply_owner.resulting_supply_humidity_ratio)
        .flatten();
    let mixed_air_humidity_ratio = active
        .then_some(mixed_air_owner.mixed_air_humidity_ratio)
        .flatten();
    let comparison = active.then(|| {
        supply_humidity_ratio.expect("active supply-humidity owner")
            < mixed_air_humidity_ratio.expect("active mixed-air-humidity owner")
    });
    debug_assert!(
        !active
            || supply_corroborator
                .supply_humidity_ratio
                .is_some_and(|value| value.to_bits() == supply_humidity_ratio.unwrap().to_bits())
    );

    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
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
        predecessor_capacity_limit_guard_evaluated: predecessor.capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .active_guard_false_fallthrough,
        dehumidification_guard_evaluated: active,
        cp378_supply_humidity_ratio_saturation_limit_owned_read: active,
        cp379_same_call_supply_humidity_ratio_bit_corroborated: active,
        purchased_air_supply_humidity_ratio_read: active,
        supply_humidity_ratio,
        cp329_mixed_air_humidity_ratio_owned_read: active,
        purchased_air_mixed_air_humidity_ratio_read: active,
        mixed_air_humidity_ratio,
        supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated: active,
        supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio: comparison,
        dehumidification_body_entered: comparison == Some(true),
        dehumidification_guard_false_fallthrough: comparison == Some(false),
    }
}
