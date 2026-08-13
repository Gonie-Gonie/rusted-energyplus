//! Release-bound CP420 not-dehumidifying sensible-output assignment.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::transition::{
    RetainedRoute,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_state as advance,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_state_with_validated_route as advance_with_validated_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_route_from_validated_predecessor,
    predecessor_route,
};
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::calc::{
    cooling_mixed_air_call_committed_latest_sensible_output_inputs,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route,
    cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshots_match_bit_exact,
};

mod error;
mod runtime_validation;
mod snapshot;
mod committed;
mod route_commitment;
pub(in crate::ideal_loads::calc) use committed::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_latest_route_and_cooling_sensible_output;
pub(in crate::ideal_loads::calc) use route_commitment::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_route_matches_snapshot;
pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentError;
use runtime_validation::{
    completed_predecessor_counts_match, pending_predecessor_counts_match,
    state_counts_are_consistent,
};
use snapshot::{cp419_shape, snapshot_shape_is_exact, snapshots_match_bit_exact};

/// Executes CP420 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp419: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentError>{
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentError as Error;

    let selected = predecessor_cp419.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_latest_witness(selected);
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_latest_witness(selected);

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
    let retained_predecessor = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .latest
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if predecessor_cp419.controlled_zone != controlled_zone
        || !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshots_match_bit_exact(
            predecessor_cp419,
            retained_predecessor,
        )
        || !predecessor_witness.is_some_and(|witness| {
            cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshots_match_bit_exact(
                predecessor_cp419,
                witness,
            )
        })
    {
        return Err(predecessor_mismatch(selected));
    }

    let committed_route =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route(
            unit,
            predecessor_cp419,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_route_from_validated_predecessor(
            predecessor_cp419,
            committed_route,
        )
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    let active_input = if route.active {
        let mixed_witness = runtime
            .cooling_mixed_air_call_latest_witness(selected)
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let cp329 =
            cooling_mixed_air_call_committed_latest_sensible_output_inputs(unit, mixed_witness)
                .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let flow_witness = runtime
            .cooling_supply_mass_flow_positive_guard_latest_witness(selected)
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        let supply_mass_flow_rate_kg_per_s =
            cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate(
                unit,
                flow_witness,
                cp329,
            )
            .ok_or(Error::FormulaOperandOwnersUnavailableOrInconsistent { system: selected })?;
        Some(ActiveInput {
            supply_mass_flow_rate_kg_per_s,
            mixed_air_temperature_c: cp329.mixed_air_temperature_c,
        })
    } else {
        None
    };

    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment;
    if !pending_state_is_consistent(unit, state, witness)
        || !pending_predecessor_counts_match(
            state,
            &unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment,
            route,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let expected = state
        .transition_count
        .checked_add(1)
        .ok_or_else(|| call_order_error(unit, selected))?;
    if unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .transition_count
        != expected
        || predecessor_cp419.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = state.clone();
    let snapshot =
        advance_with_validated_route(&mut next_state, predecessor_cp419, route, active_input)
            .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_has_exact_cp419_prefix_and_local_assignment(
        snapshot,
        predecessor_cp419,
        active_input,
    )
        || !completed_state_matches_validated_snapshot(&next_state, snapshot, route)
        || !completed_predecessor_counts_match(
            &next_state,
            &unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_latest_witness(selected, Some(snapshot));
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_characterization(
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor, active_input)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_shape_is_exact(snapshot)
        && crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact(
            cp419_shape(snapshot),
        )
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_has_exact_cp419_prefix_and_local_assignment(
    snapshot: Snapshot,
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> bool {
    snapshot_shape_is_exact(snapshot)
        && cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshots_match_bit_exact(
            cp419_shape(snapshot),
            predecessor,
        )
        && operands_match(snapshot, active_input)
}

fn operands_match(snapshot: Snapshot, active_input: Option<ActiveInput>) -> bool {
    match active_input {
        Some(input) => {
            snapshot
                .supply_mass_flow_rate_kg_per_s
                .is_some_and(|value| {
                    value.to_bits() == input.supply_mass_flow_rate_kg_per_s.to_bits()
                })
                && snapshot
                    .mixed_air_temperature_for_sensible_output_c
                    .is_some_and(|value| value.to_bits() == input.mixed_air_temperature_c.to_bits())
        }
        None => {
            snapshot.supply_mass_flow_rate_kg_per_s.is_none()
                && snapshot
                    .mixed_air_temperature_for_sensible_output_c
                    .is_none()
        }
    }
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_route(
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    snapshot_shape_is_exact(snapshot).then_some(())?;
    predecessor_route(cp419_shape(snapshot))
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshots_match_bit_exact(
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
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
                .transition_count
                .saturating_sub(1)
        && match (state.latest, witness) {
            (Some(latest), Some(witness)) => {
                snapshots_match_bit_exact(latest, witness)
                    && snapshot_shape_is_exact(latest)
                    && state.latest_transition_ordinal == Some(state.transition_count)
                    && state.latest_route.is_some_and(|route| {
                        route.active
                            == latest
                                .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed
                    })
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
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment;
    state.transition_count == expected_transition_count
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
                .transition_count
        && state.latest.is_some_and(|latest| {
            snapshot_shape_is_exact(latest)
                && predecessor_route(cp419_shape(latest))
                    .is_some_and(|route| completed_state_matches_validated_snapshot(state, latest, route))
        })
        && completed_predecessor_counts_match(
            state,
            &unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment,
        )
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_is_consistent(
    _runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    unit.system == system.id
        && snapshot.system == system.id
        && unit.controlled_zone == Some(snapshot.controlled_zone)
        && classify_no_oa_sensible_subset(system).is_supported()
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && snapshot_is_exact(snapshot)
        && witness.is_some_and(|witness| snapshots_match_bit_exact(snapshot, witness))
        && unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentError{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentError::CoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshotMismatch { system }
}

fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentError{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
            .transition_count,
        transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment
            .transition_count,
    }
}
