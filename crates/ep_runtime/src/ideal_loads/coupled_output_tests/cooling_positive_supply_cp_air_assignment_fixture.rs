use crate::{
    ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    },
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

pub(super) fn calculation_cooling_positive_supply_cp_air_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    source_humidity_ratio: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
    let assignment_executed = predecessor.positive_supply_mass_flow_body_entered;
    let zone_humidity_ratio = assignment_executed.then_some(source_humidity_ratio);
    let cp_air = zone_humidity_ratio.map(energyplus_psy_cp_air_fn_w);

    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.active_guard_false_fallthrough,
        cp_air_assignment_executed: assignment_executed,
        zone_humidity_ratio_read: assignment_executed,
        zone_humidity_ratio,
        psychrometric_cp_air_evaluated: assignment_executed,
        psychrometric_cp_air_result_j_per_kg_k: cp_air,
        cp_air_assigned: assignment_executed,
        cp_air_j_per_kg_k: cp_air,
    }
}
