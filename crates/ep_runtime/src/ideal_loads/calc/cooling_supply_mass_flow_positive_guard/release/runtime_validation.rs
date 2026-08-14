//! Persistent CP330 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::snapshot_validation::{
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot, PurchasedAirUnitRuntimeState,
};

use super::super::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute;

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_cooling_mixed_air_call.system == system
        && unit.calc_cooling_supply_mass_flow_positive_guard.system == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_supply_mass_flow_positive_guard
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_cooling_mixed_air_call.transition_count == ordinal
}

pub(in crate::ideal_loads::calc) fn pending_positive_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_positive_guard;
    let predecessor_state = &unit.calc_cooling_mixed_air_call;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && state
            .unit_off_skip_count
            .checked_add(usize::from(predecessor.unit_off_skipped))
            == Some(predecessor_state.unit_off_skip_count)
        && state
            .non_cooling_skip_count
            .checked_add(usize::from(predecessor.non_cooling_skipped))
            == Some(predecessor_state.non_cooling_skip_count)
        && state
            .cooling_body_entry_count
            .checked_add(usize::from(predecessor.cooling_call_executed))
            == Some(predecessor_state.cooling_call_count)
}

pub(in crate::ideal_loads::calc) fn next_positive_guard_transition_fits(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_positive_guard;
    if state.transition_count.checked_add(1).is_none() {
        return false;
    }
    if predecessor.unit_off_skipped {
        return state.unit_off_skip_count.checked_add(1).is_some();
    }
    if predecessor.non_cooling_skipped {
        return state.non_cooling_skip_count.checked_add(1).is_some();
    }

    let positive = predecessor
        .supply_mass_flow_rate_kg_per_s
        .is_some_and(|supply| supply > 0.0);
    state.cooling_body_entry_count.checked_add(1).is_some()
        && state
            .source_site_execution_count
            .checked_add(2 + usize::from(positive))
            .is_some()
        && state
            .supply_mass_flow_rate_read_count
            .checked_add(1)
            .is_some()
        && state
            .supply_mass_flow_rate_strictly_positive_comparison_count
            .checked_add(1)
            .is_some()
        && if positive {
            state
                .positive_supply_mass_flow_body_entry_count
                .checked_add(1)
                .is_some()
                && state
                    .witnessed_positive_supply_mass_flow_body_entry_count
                    .checked_add(1)
                    .is_some()
        } else {
            state
                .active_guard_false_fallthrough_count
                .checked_add(1)
                .is_some()
                && state
                    .witnessed_active_guard_false_fallthrough_count
                    .checked_add(1)
                    .is_some()
        }
}

pub(super) fn completed_positive_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_mass_flow_positive_guard;
    let predecessor = &unit.calc_cooling_mixed_air_call;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.cooling_body_entry_count == predecessor.cooling_call_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

pub(in crate::ideal_loads::calc) fn committed_latest_snapshot_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    witness: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.init_call_count != 0
        && unit.init_call_count == unit.calc_entry.call_count
        && snapshot.system == system
        && snapshot.parent_call_ordinal == unit.init_call_count
        && unit.controlled_zone == Some(snapshot.controlled_zone)
        && snapshots_match_bit_exact(snapshot, witness)
        && completed_positive_guard_state_is_consistent(unit, snapshot, Some(witness))
        && cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(snapshot)
}

fn state_is_consistent(
    state: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    witness: Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let Some(transition_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.cooling_body_entry_count))
    else {
        return false;
    };
    let Some(active_partition) = state
        .positive_supply_mass_flow_body_entry_count
        .checked_add(state.active_guard_false_fallthrough_count)
    else {
        return false;
    };
    let Some(expected_source_sites) = state
        .cooling_body_entry_count
        .checked_mul(2)
        .and_then(|count| count.checked_add(state.positive_supply_mass_flow_body_entry_count))
    else {
        return false;
    };
    let counters_match = state.system == expected_system
        && transition_partition == state.transition_count
        && active_partition == state.cooling_body_entry_count
        && state.source_site_execution_count == expected_source_sites
        && state.supply_mass_flow_rate_read_count == state.cooling_body_entry_count
        && state.supply_mass_flow_rate_strictly_positive_comparison_count
            == state.cooling_body_entry_count
        && state.witnessed_positive_supply_mass_flow_body_entry_count
            == state.positive_supply_mass_flow_body_entry_count
        && state.witnessed_active_guard_false_fallthrough_count
            == state.active_guard_false_fallthrough_count;
    if !counters_match {
        return false;
    }
    match (state.transition_count, state.latest, witness) {
        (0, None, None) => {
            state.latest_route.is_none() && state.latest_transition_ordinal.is_none()
        }
        (count, Some(latest), Some(witness)) => {
            let expected_route = if latest.unit_off_skipped {
                PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::UnitOff
            } else if latest.non_cooling_skipped {
                PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::NonCooling
            } else if latest.positive_supply_mass_flow_body_entered {
                PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::
                    PositiveSupplyMassFlowBodyEntered
            } else {
                PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute::
                    ActiveGuardFalseFallthrough
            };
            count > 0
                && state.latest_transition_ordinal == Some(count)
                && state.latest_route == Some(expected_route)
                && latest.system == expected_system
                && latest.parent_call_ordinal == count
                && cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(latest)
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}
