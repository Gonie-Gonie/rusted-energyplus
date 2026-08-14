//! Persistent CP337 runtime-state validation.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit};

use super::snapshot_validation::{
    cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release, snapshot_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_positive_supply_enthalpy_assignment
            .system
            == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_guard
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_positive_supply_capacity_limit_guard
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_positive_supply_enthalpy_assignment
            .transition_count
            == ordinal
}

pub(in crate::ideal_loads::calc) fn pending_capacity_limit_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_capacity_limit_guard;
    let predecessor_state = &unit.calc_cooling_positive_supply_enthalpy_assignment;
    state_is_consistent(state, witness, predecessor.system, system.cooling_limit)
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
            .positive_guard_false_fallthrough_skip_count
            .checked_add(usize::from(
                predecessor.positive_guard_false_fallthrough_skipped,
            ))
            == Some(predecessor_state.positive_guard_false_fallthrough_skip_count)
        && state
            .capacity_limit_guard_evaluation_count
            .checked_add(usize::from(
                predecessor.supply_enthalpy_assignment_executed,
            ))
            == Some(predecessor_state.supply_enthalpy_assignment_count)
}

pub(in crate::ideal_loads::calc) fn next_capacity_limit_guard_transition_fits(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_capacity_limit_guard;
    if state.transition_count.checked_add(1).is_none() {
        return false;
    }
    if predecessor.unit_off_skipped {
        return state.unit_off_skip_count.checked_add(1).is_some();
    }
    if predecessor.non_cooling_skipped {
        return state.non_cooling_skip_count.checked_add(1).is_some();
    }
    if predecessor.positive_guard_false_fallthrough_skipped {
        return state
            .positive_guard_false_fallthrough_skip_count
            .checked_add(1)
            .is_some()
            && state
                .witnessed_positive_guard_false_fallthrough_skip_count
                .checked_add(1)
                .is_some();
    }

    let capacity = cooling_limit == IdealLoadsLimit::LimitCapacity;
    let combined = cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let second = !capacity;
    let body = capacity || combined;
    let source_sites = 2 + 2 * usize::from(second) + usize::from(body);
    state
        .capacity_limit_guard_evaluation_count
        .checked_add(1)
        .is_some()
        && state
            .source_site_execution_count
            .checked_add(source_sites)
            .is_some()
        && state.first_cooling_limit_read_count.checked_add(1).is_some()
        && state
            .cooling_limit_capacity_comparison_count
            .checked_add(1)
            .is_some()
        && (!capacity
            || state
                .cooling_limit_capacity_match_count
                .checked_add(1)
                .is_some())
        && (!second
            || (state.second_cooling_limit_read_count.checked_add(1).is_some()
                && state
                    .cooling_limit_flow_rate_and_capacity_comparison_count
                    .checked_add(1)
                    .is_some()))
        && (!combined
            || state
                .cooling_limit_flow_rate_and_capacity_match_count
                .checked_add(1)
                .is_some())
        && if body {
            state.capacity_limit_body_entry_count.checked_add(1).is_some()
                && state
                    .witnessed_capacity_limit_body_entry_count
                    .checked_add(1)
                    .is_some()
        } else {
            state.cooling_limit_rejected_count.checked_add(1).is_some()
                && state
                    .active_guard_false_fallthrough_count
                    .checked_add(1)
                    .is_some()
                && state
                    .witnessed_active_guard_false_fallthrough_count
                    .checked_add(1)
                    .is_some()
        }
}

pub(super) fn completed_capacity_limit_guard_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_capacity_limit_guard;
    let predecessor = &unit.calc_cooling_positive_supply_enthalpy_assignment;
    state_is_consistent(state, witness, snapshot.system, system.cooling_limit)
        && state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == predecessor.positive_guard_false_fallthrough_skip_count
        && state.capacity_limit_guard_evaluation_count
            == predecessor.supply_enthalpy_assignment_count
        && state.latest == Some(snapshot)
}

/// Bounded committed CP337 snapshot/state proof for later selector corroboration.
pub(in crate::ideal_loads::calc) fn committed_latest_snapshot_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    witness: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_capacity_limit_guard;
    system.id == unit.system
        && state.system == unit.system
        && witness.system == system.id
        && state.transition_count > 0
        && state.transition_count == unit.init_call_count
        && state.transition_count == unit.calc_entry.call_count
        && witness.parent_call_ordinal == state.transition_count
        && unit.controlled_zone == Some(witness.controlled_zone)
        && completed_capacity_limit_guard_state_is_consistent(
            unit,
            system,
            witness,
            Some(witness),
        )
}

fn state_is_consistent(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot>,
    expected_system: IdealLoadsAirSystemId,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let Some(route_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.positive_guard_false_fallthrough_skip_count))
        .and_then(|count| count.checked_add(state.capacity_limit_guard_evaluation_count))
    else {
        return false;
    };
    let Some(active_partition) = state
        .capacity_limit_body_entry_count
        .checked_add(state.active_guard_false_fallthrough_count)
    else {
        return false;
    };
    let Some(selector_partition) = state
        .cooling_limit_capacity_match_count
        .checked_add(state.cooling_limit_flow_rate_and_capacity_match_count)
        .and_then(|count| count.checked_add(state.cooling_limit_rejected_count))
    else {
        return false;
    };
    let Some(expected_source_sites) = state
        .capacity_limit_guard_evaluation_count
        .checked_mul(2)
        .and_then(|count| {
            state
                .second_cooling_limit_read_count
                .checked_mul(2)
                .and_then(|second| count.checked_add(second))
        })
        .and_then(|count| count.checked_add(state.capacity_limit_body_entry_count))
    else {
        return false;
    };
    let active = state.capacity_limit_guard_evaluation_count;
    let expected_capacity =
        if cooling_limit == IdealLoadsLimit::LimitCapacity { active } else { 0 };
    let expected_second = active - expected_capacity;
    let expected_combined =
        if cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity {
            active
        } else {
            0
        };
    let Some(expected_body) = expected_capacity.checked_add(expected_combined) else {
        return false;
    };
    let expected_rejected = active - expected_body;
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && active_partition == active
        && selector_partition == active
        && state.source_site_execution_count == expected_source_sites
        && state.first_cooling_limit_read_count == active
        && state.cooling_limit_capacity_comparison_count == active
        && state.cooling_limit_capacity_match_count == expected_capacity
        && state.second_cooling_limit_read_count == expected_second
        && state.cooling_limit_flow_rate_and_capacity_comparison_count == expected_second
        && state.cooling_limit_flow_rate_and_capacity_match_count == expected_combined
        && state.cooling_limit_rejected_count == expected_rejected
        && state.capacity_limit_body_entry_count == expected_body
        && state.active_guard_false_fallthrough_count == expected_rejected
        && state.cooling_limit_rejected_count
            == state.active_guard_false_fallthrough_count
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_capacity_limit_body_entry_count
            == state.capacity_limit_body_entry_count
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
            count > 0
                && state.latest_transition_ordinal == Some(count)
                && snapshot_route(latest) == state.latest_route
                && latest.system == expected_system
                && latest.parent_call_ordinal == count
                && cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(
                    latest,
                )
                && latest == witness
                && latest_input_matches(latest, cooling_limit)
        }
        _ => false,
    }
}

fn latest_input_matches(
    latest: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    if !latest.capacity_limit_guard_evaluated {
        return true;
    }
    latest.first_cooling_limit == Some(cooling_limit)
        && (!latest.second_cooling_limit_read
            || latest.second_cooling_limit == Some(cooling_limit))
}
