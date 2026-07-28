//! Persistent CP332 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::snapshot_validation::{
    cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release,
    snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirUnitRuntimeState,
};

use super::super::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute;

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_sensible_flow.system == system
        && unit.calc_cooling_mixed_air_call.system == system
        && unit.calc_cooling_supply_mass_flow_positive_guard.system == system
        && unit.calc_cooling_positive_supply_cp_air_assignment.system == system
        && unit
            .calc_cooling_positive_supply_temperature_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_positive_supply_temperature_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_positive_supply_cp_air_assignment
            .transition_count
            == ordinal
}

pub(in crate::ideal_loads::calc) fn pending_supply_temperature_assignment_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_temperature_assignment;
    let predecessor_state = &unit.calc_cooling_positive_supply_cp_air_assignment;
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
            .supply_temperature_assignment_count
            .checked_add(usize::from(predecessor.cp_air_assignment_executed))
            == Some(predecessor_state.cp_air_assignment_count)
}

pub(in crate::ideal_loads::calc) fn next_supply_temperature_assignment_transition_fits(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_temperature_assignment;
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
        .supply_temperature_assignment_count
        .checked_add(1)
        .is_some()
        && state.source_site_execution_count.checked_add(8).is_some()
        && state
            .zone_cooling_setpoint_load_read_count
            .checked_add(1)
            .is_some()
        && state.cp_air_read_count.checked_add(1).is_some()
        && state
            .supply_mass_flow_rate_read_count
            .checked_add(1)
            .is_some()
        && state
            .cp_air_times_supply_mass_flow_rate_calculation_count
            .checked_add(1)
            .is_some()
        && state
            .zone_cooling_setpoint_load_over_denominator_calculation_count
            .checked_add(1)
            .is_some()
        && state
            .zone_node_temperature_read_count
            .checked_add(1)
            .is_some()
        && state
            .supply_temperature_calculation_count
            .checked_add(1)
            .is_some()
        && state
            .supply_temperature_assignment_write_count
            .checked_add(1)
            .is_some()
        && state
            .witnessed_supply_temperature_assignment_count
            .checked_add(1)
            .is_some()
}

pub(super) fn completed_supply_temperature_assignment_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_positive_supply_temperature_assignment;
    let predecessor = &unit.calc_cooling_positive_supply_cp_air_assignment;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == predecessor.positive_guard_false_fallthrough_skip_count
        && state.supply_temperature_assignment_count == predecessor.cp_air_assignment_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

fn state_is_consistent(
    state: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot>,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let Some(route_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.positive_guard_false_fallthrough_skip_count))
        .and_then(|count| count.checked_add(state.supply_temperature_assignment_count))
    else {
        return false;
    };
    let Some(expected_source_sites) = state.supply_temperature_assignment_count.checked_mul(8)
    else {
        return false;
    };
    let assignments = state.supply_temperature_assignment_count;
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && state.source_site_execution_count == expected_source_sites
        && state.zone_cooling_setpoint_load_read_count == assignments
        && state.cp_air_read_count == assignments
        && state.supply_mass_flow_rate_read_count == assignments
        && state.cp_air_times_supply_mass_flow_rate_calculation_count == assignments
        && state.zone_cooling_setpoint_load_over_denominator_calculation_count == assignments
        && state.zone_node_temperature_read_count == assignments
        && state.supply_temperature_calculation_count == assignments
        && state.supply_temperature_assignment_write_count == assignments
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_supply_temperature_assignment_count == assignments;
    if !counters_match {
        return false;
    }
    match (state.transition_count, state.latest, witness) {
        (0, None, None) => {
            state.latest_route.is_none() && state.latest_transition_ordinal.is_none()
        }
        (count, Some(latest), Some(witness)) => {
            let expected_route = if latest.unit_off_skipped {
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute::UnitOff
            } else if latest.non_cooling_skipped {
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute::NonCooling
            } else if latest.positive_guard_false_fallthrough_skipped {
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute::
                    PositiveGuardFalseFallthrough
            } else {
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute::
                    SupplyTemperatureAssigned
            };
            count > 0
                && state.latest_transition_ordinal == Some(count)
                && state.latest_route == Some(expected_route)
                && latest.system == expected_system
                && latest.parent_call_ordinal == count
                && cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}
