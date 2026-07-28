//! Persistent CP333 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::snapshot_validation::{
    cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirUnitRuntimeState,
};

use super::super::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRetainedRoute;

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_sensible_flow.system == system
        && unit
            .calc_cooling_positive_supply_temperature_assignment
            .system
            == system
        && unit
            .calc_cooling_positive_supply_temperature_minimum_limit
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_positive_supply_temperature_minimum_limit
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_positive_supply_temperature_assignment
            .transition_count
            == ordinal
}

pub(in crate::ideal_loads::calc) fn pending_supply_temperature_minimum_limit_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_temperature_minimum_limit;
    let predecessor_state = &unit.calc_cooling_positive_supply_temperature_assignment;
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
            .positive_guard_false_fallthrough_skip_count
            .checked_add(usize::from(
                predecessor.positive_guard_false_fallthrough_skipped,
            ))
            == Some(predecessor_state.positive_guard_false_fallthrough_skip_count)
        && state
            .supply_temperature_minimum_limit_count
            .checked_add(usize::from(
                predecessor.supply_temperature_assignment_executed,
            ))
            == Some(predecessor_state.supply_temperature_assignment_count)
}

pub(in crate::ideal_loads::calc) fn next_supply_temperature_minimum_limit_transition_fits(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_temperature_minimum_limit;
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

    state
        .supply_temperature_minimum_limit_count
        .checked_add(1)
        .is_some()
        && state.source_site_execution_count.checked_add(4).is_some()
        && state
            .supply_temperature_for_maximum_read_count
            .checked_add(1)
            .is_some()
        && state
            .minimum_cooling_supply_air_temperature_for_maximum_read_count
            .checked_add(1)
            .is_some()
        && state
            .source_shaped_two_argument_maximum_evaluation_count
            .checked_add(1)
            .is_some()
        && state
            .supply_temperature_assignment_count
            .checked_add(1)
            .is_some()
        && state
            .witnessed_supply_temperature_minimum_limit_count
            .checked_add(1)
            .is_some()
}

pub(super) fn completed_supply_temperature_minimum_limit_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_temperature_minimum_limit;
    let predecessor = &unit.calc_cooling_positive_supply_temperature_assignment;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == predecessor.positive_guard_false_fallthrough_skip_count
        && state.supply_temperature_minimum_limit_count
            == predecessor.supply_temperature_assignment_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

fn state_is_consistent(
    state: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let Some(route_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.positive_guard_false_fallthrough_skip_count))
        .and_then(|count| count.checked_add(state.supply_temperature_minimum_limit_count))
    else {
        return false;
    };
    let Some(expected_source_sites) = state
        .supply_temperature_minimum_limit_count
        .checked_mul(4)
    else {
        return false;
    };
    let active = state.supply_temperature_minimum_limit_count;
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && state.source_site_execution_count == expected_source_sites
        && state.supply_temperature_for_maximum_read_count == active
        && state.minimum_cooling_supply_air_temperature_for_maximum_read_count == active
        && state.source_shaped_two_argument_maximum_evaluation_count == active
        && state.supply_temperature_assignment_count == active
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_supply_temperature_minimum_limit_count == active;
    if !counters_match {
        return false;
    }
    match (state.transition_count, state.latest, witness) {
        (0, None, None) => {
            state.latest_route.is_none() && state.latest_transition_ordinal.is_none()
        }
        (count, Some(latest), Some(witness)) => {
            let expected_route = if latest.unit_off_skipped {
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRetainedRoute::UnitOff
            } else if latest.non_cooling_skipped {
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRetainedRoute::
                    NonCooling
            } else if latest.positive_guard_false_fallthrough_skipped {
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRetainedRoute::
                    PositiveGuardFalseFallthrough
            } else {
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRetainedRoute::
                    SupplyTemperatureMinimumLimitExecuted
            };
            count > 0
                && state.latest_transition_ordinal == Some(count)
                && state.latest_route == Some(expected_route)
                && latest.system == expected_system
                && latest.parent_call_ordinal == count
                && cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}
