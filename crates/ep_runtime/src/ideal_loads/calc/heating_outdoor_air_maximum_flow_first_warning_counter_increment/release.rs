//! Release-bound CP438 heating outdoor-air maximum-flow first-warning counter increment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::transition::{
    advance_heating_outdoor_air_maximum_flow_first_warning_counter_increment_state as advance,
    advance_heating_outdoor_air_maximum_flow_first_warning_counter_increment_state_with_validated_route as advance_with_route,
    heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor,
};
use super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot as Snapshot,
};
use crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_first_warning_guard_committed_latest_route_and_outdoor_air_flow_maximum_heating_output_error_count;
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as CounterOwner,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact,
};

mod error;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError;
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{
    prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact,
};
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    retained_route_matches_snapshot_bounded as heating_outdoor_air_maximum_flow_first_warning_counter_increment_retained_route_matches_snapshot_bounded,
    snapshot_route as heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot_route,
};

/// Losslessly reconstructs the CP437 predecessor retained in CP438.
#[must_use]
pub fn heating_outdoor_air_maximum_flow_first_warning_counter_increment_predecessor_cp437_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp437_snapshot(snapshot)
}

/// Executes CP438 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp437: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError>
{
    use PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError as Error;

    let selected = predecessor_cp437.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .heating_outdoor_air_maximum_flow_first_warning_counter_increment_latest_witness(selected);
    if system.id != selected {
        return Err(Error::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(Error::InitializationNotReady { system: selected });
    }
    if unit.controlled_zone != Some(predecessor_cp437.controlled_zone)
        || unit
            .calc_heating_outdoor_air_maximum_flow_first_warning_guard
            .latest
            .is_none_or(|latest| {
                !heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact(
                    latest,
                    predecessor_cp437,
                )
            })
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .heating_outdoor_air_maximum_flow_first_warning_guard_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact(
        predecessor_cp437,
        predecessor_witness,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let mixed_air_witness = if predecessor_cp437
        .cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_executed
    {
        Some(
            runtime
                .cooling_mixed_air_call_latest_witness(selected)
                .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?,
        )
    } else {
        None
    };
    let (predecessor_route, sealed_counter) =
        heating_outdoor_air_maximum_flow_first_warning_guard_committed_latest_route_and_outdoor_air_flow_maximum_heating_output_error_count(
            unit,
            predecessor_witness,
            mixed_air_witness,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route =
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_from_committed_predecessor(
            predecessor_cp437,
            predecessor_route,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if sealed_counter
        != unit
            .calc_heating_outdoor_air_maximum_flow_first_warning_guard
            .outdoor_air_flow_maximum_heating_output_error_count
        || route.counter_increment_executed
    {
        return Err(Error::ExactReleaseReductionViolated { system: selected });
    }
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp437.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
        .clone();
    let mut next_counter_owner = unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard
        .clone();
    let snapshot = advance_with_route(
        &mut next_state,
        &mut next_counter_owner,
        predecessor_cp437,
        predecessor_route,
        route,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state
        .latest_route
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp437, predecessor_route, post_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            post_route,
            &next_counter_owner,
            &unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard = next_counter_owner;
    unit.calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment = next_state;
    runtime.set_heating_outdoor_air_maximum_flow_first_warning_counter_increment_latest_witness(
        selected,
        Some(snapshot),
    );
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_heating_outdoor_air_maximum_flow_first_warning_counter_increment_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    let mut counter_owner = CounterOwner::new(predecessor.system);
    counter_owner.outdoor_air_flow_maximum_heating_output_error_count = predecessor
        .outdoor_air_flow_maximum_heating_output_error_count_before
        .unwrap_or(0);
    advance(&mut state, &mut counter_owner, predecessor)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError {
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError::HeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshotMismatch {
        system,
    }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError {
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_heating_outdoor_air_maximum_flow_first_warning_guard
            .transition_count,
        transition_count: unit
            .calc_heating_outdoor_air_maximum_flow_first_warning_counter_increment
            .transition_count,
    }
}

#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
