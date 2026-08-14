//! Persistent CP339 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::snapshot_validation::{
    cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_mixed_air_call.system == system
        && unit.calc_cooling_supply_mass_flow_positive_guard.system == system
        && unit.calc_cooling_positive_supply_enthalpy_assignment.system == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
            .system
            == system
        && unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
            .transition_count
            == ordinal
}

pub(in crate::ideal_loads::calc) fn pending_capacity_limit_sensible_output_assignment_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    >,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_capacity_limit_sensible_output_assignment;
    let predecessor_state =
        &unit.calc_cooling_positive_supply_capacity_limit_cp_air_assignment;
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
            .capacity_limit_guard_false_fallthrough_skip_count
            .checked_add(usize::from(
                predecessor.capacity_limit_guard_false_fallthrough_skipped,
            ))
            == Some(
                predecessor_state.capacity_limit_guard_false_fallthrough_skip_count,
            )
        && state
            .capacity_limit_sensible_output_assignment_count
            .checked_add(usize::from(
                predecessor.capacity_limit_cp_air_assignment_executed,
            ))
            == Some(predecessor_state.capacity_limit_cp_air_assignment_count)
}

pub(in crate::ideal_loads::calc) fn next_capacity_limit_sensible_output_assignment_transition_fits(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_capacity_limit_sensible_output_assignment;
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
    if predecessor.capacity_limit_guard_false_fallthrough_skipped {
        return state
            .capacity_limit_guard_false_fallthrough_skip_count
            .checked_add(1)
            .is_some()
            && state
                .witnessed_capacity_limit_guard_false_fallthrough_skip_count
                .checked_add(1)
                .is_some();
    }

    state
        .capacity_limit_sensible_output_assignment_count
        .checked_add(1)
        .is_some()
        && state.source_site_execution_count.checked_add(6).is_some()
        && state.supply_mass_flow_rate_read_count.checked_add(1).is_some()
        && state.mixed_air_enthalpy_read_count.checked_add(1).is_some()
        && state.supply_enthalpy_read_count.checked_add(1).is_some()
        && state
            .enthalpy_difference_calculation_count
            .checked_add(1)
            .is_some()
        && state
            .cooling_sensible_output_calculation_count
            .checked_add(1)
            .is_some()
        && state
            .cooling_sensible_output_assignment_write_count
            .checked_add(1)
            .is_some()
        && state
            .witnessed_capacity_limit_sensible_output_assignment_count
            .checked_add(1)
            .is_some()
}

pub(super) fn completed_capacity_limit_sensible_output_assignment_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    >,
) -> bool {
    let state =
        &unit.calc_cooling_positive_supply_capacity_limit_sensible_output_assignment;
    let predecessor =
        &unit.calc_cooling_positive_supply_capacity_limit_cp_air_assignment;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == predecessor.transition_count
        && state.unit_off_skip_count == predecessor.unit_off_skip_count
        && state.non_cooling_skip_count == predecessor.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == predecessor.positive_guard_false_fallthrough_skip_count
        && state.capacity_limit_guard_false_fallthrough_skip_count
            == predecessor.capacity_limit_guard_false_fallthrough_skip_count
        && state.capacity_limit_sensible_output_assignment_count
            == predecessor.capacity_limit_cp_air_assignment_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

pub(in crate::ideal_loads::calc) fn committed_latest_snapshot_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    witness: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.init_call_count != 0
        && unit.init_call_count == unit.calc_entry.call_count
        && snapshot.system == system
        && snapshot.parent_call_ordinal == unit.init_call_count
        && unit.controlled_zone == Some(snapshot.controlled_zone)
        && snapshots_match_bit_exact(snapshot, witness)
        && completed_capacity_limit_sensible_output_assignment_state_is_consistent(
            unit,
            snapshot,
            Some(witness),
        )
        && cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(snapshot)
}

fn state_is_consistent(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    >,
    expected_system: IdealLoadsAirSystemId,
) -> bool {
    let Some(route_partition) = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.positive_guard_false_fallthrough_skip_count))
        .and_then(|count| {
            count.checked_add(state.capacity_limit_guard_false_fallthrough_skip_count)
        })
        .and_then(|count| {
            count.checked_add(state.capacity_limit_sensible_output_assignment_count)
        })
    else {
        return false;
    };
    let Some(expected_source_sites) = state
        .capacity_limit_sensible_output_assignment_count
        .checked_mul(6)
    else {
        return false;
    };
    let assignment_count = state.capacity_limit_sensible_output_assignment_count;
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && state.source_site_execution_count == expected_source_sites
        && state.supply_mass_flow_rate_read_count == assignment_count
        && state.mixed_air_enthalpy_read_count == assignment_count
        && state.supply_enthalpy_read_count == assignment_count
        && state.enthalpy_difference_calculation_count == assignment_count
        && state.cooling_sensible_output_calculation_count == assignment_count
        && state.cooling_sensible_output_assignment_write_count == assignment_count
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_capacity_limit_guard_false_fallthrough_skip_count
            == state.capacity_limit_guard_false_fallthrough_skip_count
        && state.witnessed_capacity_limit_sensible_output_assignment_count
            == assignment_count;
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
                && cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
                    latest,
                )
                && snapshots_match_bit_exact(latest, witness)
        }
        _ => false,
    }
}
