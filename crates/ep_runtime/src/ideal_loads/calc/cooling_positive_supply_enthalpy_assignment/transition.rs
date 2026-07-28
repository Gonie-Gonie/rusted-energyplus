//! Pure CP335-to-CP336 Cooling positive-supply enthalpy-assignment transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot;
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentActiveInput
{
    pub supply_temperature_c: f64,
    pub supply_humidity_ratio: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_enthalpy_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    active_input: Option<PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentActiveInput>,
) -> PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
    let assignment_executed =
        predecessor.supply_humidity_ratio_mixed_air_assignment_executed;
    debug_assert_eq!(assignment_executed, active_input.is_some());

    let supply_temperature_c = active_input.map(|input| input.supply_temperature_c);
    let supply_humidity_ratio = active_input.map(|input| input.supply_humidity_ratio);
    let psychrometric_supply_enthalpy_result_j_per_kg = active_input.map(|input| {
        energyplus_psy_h_fn_tdb_w(input.supply_temperature_c, input.supply_humidity_ratio)
    });
    let supply_enthalpy_j_per_kg = psychrometric_supply_enthalpy_result_j_per_kg;

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRetainedRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRetainedRoute::
            PositiveGuardFalseFallthrough
    } else {
        state.supply_enthalpy_assignment_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER.len();
        state.supply_temperature_for_enthalpy_read_count += 1;
        state.supply_humidity_ratio_for_enthalpy_read_count += 1;
        state.psychrometric_supply_enthalpy_evaluation_count += 1;
        state.supply_enthalpy_assignment_write_count += 1;
        state.witnessed_supply_enthalpy_assignment_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRetainedRoute::
            SupplyEnthalpyAssigned
    };

    let snapshot = PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
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
        psychrometric_supply_enthalpy_result_j_per_kg,
        supply_enthalpy_assigned: assignment_executed,
        supply_enthalpy_j_per_kg,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
