//! Release-bound CP442 first-warning guard else-branch-entry evidence.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::transition::{
    advance_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_state as advance,
    advance_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_state_with_validated_route as advance_with_route,
    heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_from_committed_predecessor,
};
use super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot as Snapshot,
};
use crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact,
};

mod error;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryError;
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{
    prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact,
};
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    retained_route_matches_snapshot_bounded as heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_retained_route_matches_snapshot_bounded,
    snapshot_route as heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshot_route,
};

/// Losslessly reconstructs the CP441 predecessor retained in CP442.
#[must_use]
pub fn heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_predecessor_cp441_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp441_snapshot(snapshot)
}

/// Executes CP442 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp441: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryError,
> {
    use PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryError as Error;

    let selected = predecessor_cp441.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_latest_witness(
            selected,
        );
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
    if unit.controlled_zone != Some(predecessor_cp441.controlled_zone)
        || unit
            .calc_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call
            .latest
            .is_none_or(|latest| {
                !heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact(
                    latest,
                    predecessor_cp441,
                )
            })
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshots_match_bit_exact(
        predecessor_cp441,
        predecessor_witness,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_route =
        heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_committed_latest_route(
            unit,
            predecessor_witness,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route = heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_from_committed_predecessor(
        predecessor_cp441,
        predecessor_route,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if route.entered {
        return Err(Error::ExactReleaseReductionViolated { system: selected });
    }
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp441.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = unit
        .calc_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry
        .clone();
    let snapshot = advance_with_route(&mut next_state, predecessor_cp441, predecessor_route, route)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state
        .latest_route
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp441, predecessor_route, post_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            post_route,
            &unit.calc_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry = next_state;
    runtime
        .set_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_latest_witness(
            selected,
            Some(snapshot),
        );
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryError {
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryError::
        HeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryError {
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call
            .transition_count,
        transition_count: unit
            .calc_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry
            .transition_count,
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
