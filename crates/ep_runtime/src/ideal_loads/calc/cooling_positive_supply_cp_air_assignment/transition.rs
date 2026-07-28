//! Pure CP330-to-CP331 Cooling positive-supply `CpAir` assignment transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRetainedRoute,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot;
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentActiveInput
{
    pub zone_humidity_ratio: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_positive_supply_cp_air_assignment_state(
    state: &mut PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    active_input: Option<PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentActiveInput>,
) -> PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
    let assignment_executed = predecessor.positive_supply_mass_flow_body_entered;
    debug_assert_eq!(assignment_executed, active_input.is_some());
    let zone_humidity_ratio = active_input.map(|input| input.zone_humidity_ratio);
    // The EnergyPlus last-call cache is outside this pure scalar characterization.
    let psychrometric_cp_air_result_j_per_kg_k =
        zone_humidity_ratio.map(energyplus_psy_cp_air_fn_w);
    let cp_air_j_per_kg_k = psychrometric_cp_air_result_j_per_kg_k;

    state.transition_count += 1;
    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRetainedRoute::NonCooling
    } else if predecessor.active_guard_false_fallthrough {
        state.positive_guard_false_fallthrough_skip_count += 1;
        state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRetainedRoute::
            PositiveGuardFalseFallthrough
    } else {
        state.cp_air_assignment_count += 1;
        state.source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER.len();
        state.zone_humidity_ratio_read_count += 1;
        state.psychrometric_cp_air_evaluation_count += 1;
        state.cp_air_assignment_write_count += 1;
        state.witnessed_cp_air_assignment_count += 1;
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRetainedRoute::CpAirAssigned
    };

    let snapshot = PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
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
        psychrometric_cp_air_result_j_per_kg_k,
        cp_air_assigned: assignment_executed,
        cp_air_j_per_kg_k,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
