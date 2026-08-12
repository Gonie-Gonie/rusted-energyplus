//! Release-bound CP418 not-dehumidifying else-branch entry.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::transition::{
    RetainedRoute,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_state_with_validated_route as advance_with_validated_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_route_from_validated_predecessor,
    predecessor_route,
};
use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_state as advance,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_committed_latest_route;

mod error;
mod runtime_validation;
mod snapshot;
pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError;
use runtime_validation::{
    completed_predecessor_counts_match, pending_predecessor_counts_match,
    state_counts_are_consistent,
};
use snapshot::{cp417_shape, snapshots_match_bit_exact};

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn cp417_shape_for_test(snapshot: Snapshot) -> Predecessor {
    cp417_shape(snapshot)
}

/// Executes CP418 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp417: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError,
> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError as Error;

    let selected = predecessor_cp417.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_latest_witness(selected);
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_latest_witness(selected);
    if system.id != selected {
        return Err(Error::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(Error::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(Error::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(Error::InitializationNotReady { system: selected });
    }
    let controlled_zone = unit
        .controlled_zone
        .ok_or(Error::InitializationNotReady { system: selected })?;
    if predecessor_cp417.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    let Some(retained_predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment
        .latest
    else {
        return Err(predecessor_mismatch(selected));
    };
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(
        predecessor_cp417,
        retained_predecessor,
    ) || !predecessor_witness.is_some_and(|predecessor_witness| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(
            predecessor_cp417,
            predecessor_witness,
        )
    }) {
        return Err(predecessor_mismatch(selected));
    }
    let committed_route =
        cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_committed_latest_route(
            unit,
            predecessor_cp417,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_route_from_validated_predecessor(
        predecessor_cp417,
        committed_route,
    )
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry;
    if !pending_state_is_consistent(unit, state, witness)
        || !pending_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment,
            route,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let Some(expected_predecessor_transition_count) = state.transition_count.checked_add(1) else {
        return Err(call_order_error(unit, selected));
    };
    if unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment
        .transition_count
        != expected_predecessor_transition_count
        || predecessor_cp417.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = state.clone();
    let snapshot = advance_with_validated_route(&mut next_state, predecessor_cp417, route)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !snapshot_matches_validated_predecessor(snapshot, predecessor_cp417, route)
        || !completed_state_matches_validated_snapshot(&next_state, snapshot, route)
        || !completed_predecessor_counts_match(
            &next_state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =
        next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_latest_witness(
        selected,
        snapshot,
    );
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
        cp417_shape(snapshot),
    ) && snapshot_shape_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    if !snapshot_shape_is_exact(snapshot) {
        return false;
    }
    cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact(
        cp417_shape(snapshot),
    )
}

fn snapshot_shape_is_exact(snapshot: Snapshot) -> bool {
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered
            == snapshot.predecessor_dehumidification_guard_false_fallthrough
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_has_exact_cp417_prefix_and_marker(
    snapshot: Snapshot,
    predecessor: Predecessor,
) -> bool {
    snapshot_shape_is_exact(snapshot)
        && cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(
            cp417_shape(snapshot),
            predecessor,
        )
}

fn snapshot_matches_validated_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: RetainedRoute,
) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_has_exact_cp417_prefix_and_marker(
        snapshot,
        predecessor,
    )
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered
            == route.active
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_route(
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    snapshot_shape_is_exact(snapshot).then_some(())?;
    predecessor_route(cp417_shape(snapshot))
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    witness: Option<Snapshot>,
) -> bool {
    state.system == unit.system
        && state_counts_are_consistent(state)
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment
                .transition_count
                .saturating_sub(1)
        && match (state.latest, witness) {
            (Some(latest), Some(witness)) => {
                snapshots_match_bit_exact(latest, witness)
                    && snapshot_shape_is_exact(latest)
                    && state.latest_transition_ordinal == Some(state.transition_count)
                    && state
                        .latest_route
                        .is_some_and(|route| route.active
                            == latest.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered)
            }
            (None, None) => {
                state.transition_count == 0
                    && state.latest_route.is_none()
                    && state.latest_transition_ordinal.is_none()
            }
            _ => false,
        }
}

fn completed_state_matches_validated_snapshot(
    state: &State,
    snapshot: Snapshot,
    route: RetainedRoute,
) -> bool {
    state
        .latest
        .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state.latest_route == Some(route)
        && state_counts_are_consistent(state)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry;
    state.transition_count == expected_transition_count
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment
                .transition_count
        && state.latest.is_some_and(|latest| {
            snapshot_shape_is_exact(latest)
                && predecessor_route(cp417_shape(latest)).is_some_and(|route| {
                    completed_state_matches_validated_snapshot(state, latest, route)
                })
        })
        && completed_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment,
        )
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    if unit.system != system.id
        || snapshot.system != system.id
        || unit.controlled_zone != Some(snapshot.controlled_zone)
        || !classify_no_oa_sensible_subset(system).is_supported()
        || system.dehumidification_control_type != DehumidificationControlType::None
        || system.humidification_control_type != HumidificationControlType::None
        || !snapshot_shape_is_exact(snapshot)
        || !witness.is_some_and(|witness| snapshots_match_bit_exact(snapshot, witness))
    {
        return false;
    }
    let predecessor = cp417_shape(snapshot);
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry;
    if state.transition_count != unit.init_call_count
        || !completed_state_matches_validated_snapshot(state, snapshot, route)
        || !completed_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment,
        )
    {
        return false;
    }
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_latest_witness(system.id);
    let retained_predecessor = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment
        .latest;
    retained_predecessor.is_some_and(|retained| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(
            retained,
            predecessor,
        )
    }) && predecessor_witness.is_some_and(|witness| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(
            witness,
            predecessor,
        )
    })
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError::CoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshotMismatch { system }
}

fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment
            .transition_count,
        transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
            .transition_count,
    }
}
