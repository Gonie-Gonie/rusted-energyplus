//! Persistent CP329 state, predecessor-history, and increment validation.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallRetainedRoute,
    PurchasedAirCalcCoolingMixedAirCallRuntimeState, PurchasedAirCalcCoolingMixedAirCallSnapshot,
};
use super::{
    cooling_mixed_air_call_snapshots_match_bit_exact, mixed_air_call_links_to_predecessor,
};
use crate::ideal_loads::calc::cooling_supply_mass_flow_very_small_guard::completed_direct_cooling_supply_mass_flow_very_small_guard_body_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot, PurchasedAirRuntimeState,
    PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
};

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_mixed_air_call_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    witness: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_supply_mass_flow_very_small_guard_body
        .latest
    else {
        return false;
    };
    completed_mixed_air_predecessor_is_consistent(runtime, unit, system, predecessor)
        && mixed_air_call_links_to_predecessor(snapshot, predecessor)
        && completed_mixed_air_history_links_to_predecessor(unit)
        && system.id == snapshot.system
        && unit.system == snapshot.system
        && classify_no_oa_sensible_subset(system).is_supported()
        && unit.controlled_zone == Some(snapshot.controlled_zone)
        && (!snapshot.cooling_call_executed
            || unit.recirculation_node == snapshot.recirculation_node)
        && unit
            .calc_cooling_mixed_air_call
            .latest
            .is_some_and(|latest| {
                cooling_mixed_air_call_snapshots_match_bit_exact(latest, snapshot)
            })
        && state_is_consistent(&unit.calc_cooling_mixed_air_call, witness, snapshot.system)
}

pub(super) fn completed_mixed_air_predecessor_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    completed_direct_cooling_supply_mass_flow_very_small_guard_body_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_supply_mass_flow_very_small_guard_body_latest_witness(system.id),
    )
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_mixed_air_call
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_minimum_oa_prefix.transition_count == ordinal
        && unit
            .calc_cooling_supply_mass_flow_very_small_guard_body
            .transition_count
            == ordinal
}

pub(in crate::ideal_loads::calc) fn pending_mixed_air_history_links_to_predecessor(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    let state = &unit.calc_cooling_mixed_air_call;
    let predecessor_state = &unit.calc_cooling_supply_mass_flow_very_small_guard_body;
    state
        .unit_off_skip_count
        .checked_add(usize::from(predecessor.unit_off_skipped))
        == Some(predecessor_state.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(predecessor_state.non_cooling_skip_count)
        && state
            .cooling_call_count
            .checked_add(usize::from(predecessor.cooling_body_entered))
            == Some(predecessor_state.cooling_body_entry_count)
}

pub(in crate::ideal_loads::calc) fn next_mixed_air_transition_fits(
    state: &PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    if state.transition_count.checked_add(1).is_none() {
        return false;
    }
    if predecessor.unit_off_skipped {
        return state.unit_off_skip_count.checked_add(1).is_some();
    }
    if predecessor.non_cooling_skipped {
        return state.non_cooling_skip_count.checked_add(1).is_some();
    }

    state.cooling_call_count.checked_add(1).is_some()
        && state
            .caller_source_site_execution_count
            .checked_add(PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER.len())
            .is_some()
        && state
            .child_source_site_execution_count
            .checked_add(PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER.len())
            .is_some()
        && state.state_reference_bind_count.checked_add(1).is_some()
        && state
            .purchased_air_number_read_count
            .checked_add(1)
            .is_some()
        && state
            .outdoor_air_mass_flow_rate_read_count
            .checked_add(1)
            .is_some()
        && state
            .supply_mass_flow_rate_read_count
            .checked_add(1)
            .is_some()
        && state
            .mixed_air_output_reference_bind_count
            .checked_add(3)
            .is_some()
        && state.operating_mode_read_count.checked_add(1).is_some()
        && state.mixed_air_child_call_count.checked_add(1).is_some()
        && state.no_outdoor_air_fallback_count.checked_add(1).is_some()
        && state
            .recirculation_enthalpy_projection_count
            .checked_add(1)
            .is_some()
        && state
            .mixed_air_output_assignment_count
            .checked_add(3)
            .is_some()
        && state
            .heat_recovery_output_positive_zero_assignment_count
            .checked_add(2)
            .is_some()
}

pub(super) fn state_is_consistent(
    state: &PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    witness: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let Some(route_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.cooling_call_count))
    else {
        return false;
    };
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && counter_product_matches(
            state.caller_source_site_execution_count,
            state.cooling_call_count,
            PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER.len(),
        )
        && counter_product_matches(
            state.child_source_site_execution_count,
            state.cooling_call_count,
            PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER.len(),
        )
        && state.state_reference_bind_count == state.cooling_call_count
        && state.purchased_air_number_read_count == state.cooling_call_count
        && state.outdoor_air_mass_flow_rate_read_count == state.cooling_call_count
        && state.supply_mass_flow_rate_read_count == state.cooling_call_count
        && counter_product_matches(
            state.mixed_air_output_reference_bind_count,
            state.cooling_call_count,
            3,
        )
        && state.operating_mode_read_count == state.cooling_call_count
        && state.mixed_air_child_call_count == state.cooling_call_count
        && state.no_outdoor_air_fallback_count == state.cooling_call_count
        && state.recirculation_enthalpy_projection_count == state.cooling_call_count
        && counter_product_matches(
            state.mixed_air_output_assignment_count,
            state.cooling_call_count,
            3,
        )
        && counter_product_matches(
            state.heat_recovery_output_positive_zero_assignment_count,
            state.cooling_call_count,
            2,
        );
    if !counters_match {
        return false;
    }
    match (state.transition_count, state.latest, witness) {
        (0, None, None) => {
            state.latest_route.is_none() && state.latest_transition_ordinal.is_none()
        }
        (count, Some(latest), Some(witness)) => {
            let expected_route = if latest.unit_off_skipped {
                PurchasedAirCalcCoolingMixedAirCallRetainedRoute::UnitOff
            } else if latest.non_cooling_skipped {
                PurchasedAirCalcCoolingMixedAirCallRetainedRoute::NonCooling
            } else {
                PurchasedAirCalcCoolingMixedAirCallRetainedRoute::NoOutdoorAirFallback
            };
            count > 0
                && state.latest_transition_ordinal == Some(count)
                && state.latest_route == Some(expected_route)
                && latest.system == expected_system
                && latest.parent_call_ordinal == count
                && cooling_mixed_air_call_snapshot_is_exact_direct_release(latest)
                && cooling_mixed_air_call_snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}

fn completed_mixed_air_history_links_to_predecessor(unit: &PurchasedAirUnitRuntimeState) -> bool {
    let state = &unit.calc_cooling_mixed_air_call;
    let predecessor = &unit.calc_cooling_supply_mass_flow_very_small_guard_body;
    state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.cooling_call_count == predecessor.cooling_body_entry_count
}

pub(in crate::ideal_loads::calc) fn counter_product_matches(
    actual: usize,
    count: usize,
    factor: usize,
) -> bool {
    count
        .checked_mul(factor)
        .is_some_and(|expected| actual == expected)
}
