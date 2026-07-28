use crate::{
    ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        PurchasedAirCalcCoolingMixedAirCallSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    },
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

pub(super) fn calculation_cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot {
    let assignment_executed = predecessor.capacity_limit_body_entered;
    let mixed_air_humidity_ratio = if assignment_executed {
        mixed_air.mixed_air_humidity_ratio
    } else {
        None
    };
    let cp_air = mixed_air_humidity_ratio.map(energyplus_psy_cp_air_fn_w);

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_capacity_limit_guard_evaluated: predecessor.capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .active_guard_false_fallthrough,
        capacity_limit_guard_false_fallthrough_skipped: predecessor
            .active_guard_false_fallthrough,
        capacity_limit_cp_air_assignment_executed: assignment_executed,
        mixed_air_humidity_ratio_read: assignment_executed,
        mixed_air_humidity_ratio,
        psychrometric_cp_air_evaluated: assignment_executed,
        psychrometric_cp_air_result_j_per_kg_k: cp_air,
        cp_air_assigned: assignment_executed,
        cp_air_j_per_kg_k: cp_air,
    }
}
