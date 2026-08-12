//! Release-bound CP419 not-dehumidifying `CpAir` assignment.

use super::transition::{
    RetainedRoute,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_state_with_validated_route as advance_with_validated_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_route_from_validated_predecessor,
    predecessor_route,
};
use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_state as advance,
};
use crate::ideal_loads::calc::{
    cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_committed_latest_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;
use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};
mod committed;
mod error;
mod runtime_validation;
mod snapshot;
pub(in crate::ideal_loads::calc) use committed::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route;
pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentError;
use runtime_validation::{
    committed_predecessor_counts_match, pending_predecessor_counts_match,
    state_counts_are_consistent,
};
use snapshot::{
    cp418_shape, direct_subset_values_are_valid, option_bits_match, snapshots_match_bit_exact,
};

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn cp418_shape_for_test(snapshot: Snapshot) -> Predecessor {
    cp418_shape(snapshot)
}

/// Executes CP419 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp418: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentError,
>{
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentError as Error;

    let selected = predecessor_cp418.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_latest_witness(selected);
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_latest_witness(selected);
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
    if predecessor_cp418.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    let Some(retained_predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
        .latest
    else {
        return Err(predecessor_mismatch(selected));
    };
    if !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact(
        predecessor_cp418,
        retained_predecessor,
    ) || !predecessor_witness.is_some_and(|predecessor_witness| {
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact(
            predecessor_cp418,
            predecessor_witness,
        )
    }) {
        return Err(predecessor_mismatch(selected));
    }
    let committed_route =
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_committed_latest_route(
            unit,
            predecessor_cp418,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route = cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_route_from_validated_predecessor(
        predecessor_cp418,
        committed_route,
    )
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    let active_input = if route.active {
        let owner_witness = runtime
            .cooling_mixed_air_call_latest_witness(selected)
            .ok_or(Error::CoolingMixedAirCallHumidityOwnerMismatch { system: selected })?;
        let mixed_air_humidity_ratio =
            cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio(unit, owner_witness)
                .ok_or(Error::CoolingMixedAirCallHumidityOwnerMismatch { system: selected })?;
        if !mixed_air_humidity_ratio.is_finite() || mixed_air_humidity_ratio < 0.0 {
            return Err(Error::MixedAirHumidityRatioOutsideDirectSubset {
                system: selected,
                bits: mixed_air_humidity_ratio.to_bits(),
            });
        }
        let cp_air = energyplus_psy_cp_air_fn_w(mixed_air_humidity_ratio);
        if !cp_air.is_finite() {
            return Err(Error::PsychrometricCpAirOutsideDirectSubset {
                system: selected,
                bits: cp_air.to_bits(),
            });
        }
        Some(ActiveInput {
            mixed_air_humidity_ratio,
        })
    } else {
        None
    };
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment;
    if !pending_state_is_consistent(unit, state, witness)
        || !pending_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
            route,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let Some(expected_predecessor_transition_count) = state.transition_count.checked_add(1) else {
        return Err(call_order_error(unit, selected));
    };
    if unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
        .transition_count
        != expected_predecessor_transition_count
        || predecessor_cp418.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = state.clone();
    let snapshot =
        advance_with_validated_route(&mut next_state, predecessor_cp418, route, active_input)
            .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !snapshot_matches_validated_predecessor(snapshot, predecessor_cp418, route)
        || !direct_subset_values_are_valid(snapshot)
        || !completed_state_matches_validated_snapshot(&next_state, snapshot, route)
        || !committed_predecessor_counts_match(
            &next_state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =
        next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_latest_witness(
        selected,
        Some(snapshot),
    );
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_characterization(
    predecessor: Predecessor,
    owner_mixed_air_humidity_ratio: Option<f64>,
) -> Option<Snapshot> {
    let active_input =
        if predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered {
            Some(ActiveInput {
                mixed_air_humidity_ratio: owner_mixed_air_humidity_ratio?,
            })
        } else {
            None
        };
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor, active_input)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release(
        cp418_shape(snapshot),
    ) && snapshot_shape_is_exact(snapshot)
        && direct_subset_values_are_valid(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    if !snapshot_shape_is_exact(snapshot) {
        return false;
    }
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact(
        cp418_shape(snapshot),
    )
}

fn snapshot_shape_is_exact(snapshot: Snapshot) -> bool {
    let active = snapshot
        .predecessor_post_saturation_capacity_limit_dehumidification_guard_else_branch_entered;
    let local = match (
        snapshot.mixed_air_humidity_ratio_for_cp_air,
        snapshot.psychrometric_cp_air_result_j_per_kg_k,
        snapshot.cp_air_j_per_kg_k,
    ) {
        (Some(humidity_ratio), Some(cp_air), Some(assigned)) if active => {
            cp_air.to_bits() == energyplus_psy_cp_air_fn_w(humidity_ratio).to_bits()
                && assigned.to_bits() == cp_air.to_bits()
        }
        (None, None, None) if !active => true,
        _ => false,
    };
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed == active
        && snapshot.cp329_retained_mixed_air_humidity_ratio_owned_read == active
        && snapshot.mixed_air_humidity_ratio_for_cp_air_read == active
        && snapshot.psychrometric_cp_air_evaluated == active
        && snapshot.cp_air_assigned == active
        && local
        && option_bits_match(
            snapshot.predecessor_cp418_resulting_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        )
        && option_bits_match(
            snapshot.predecessor_cp418_resulting_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_match(
            snapshot.predecessor_cp418_resulting_supply_temperature_c,
            snapshot.resulting_supply_temperature_c,
        )
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_has_exact_cp418_prefix_and_local_assignment(
    snapshot: Snapshot,
    predecessor: Predecessor,
    owner_mixed_air_humidity_ratio: Option<f64>,
) -> bool {
    snapshot_shape_is_exact(snapshot)
        && direct_subset_values_are_valid(snapshot)
        && cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact(
            cp418_shape(snapshot),
            predecessor,
        )
        && match owner_mixed_air_humidity_ratio {
            Some(owner) => snapshot
                .mixed_air_humidity_ratio_for_cp_air
                .is_some_and(|value| value.to_bits() == owner.to_bits()),
            None => snapshot.mixed_air_humidity_ratio_for_cp_air.is_none(),
        }
}

fn snapshot_matches_validated_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: RetainedRoute,
) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_has_exact_cp418_prefix_and_local_assignment(
        snapshot,
        predecessor,
        snapshot.mixed_air_humidity_ratio_for_cp_air,
    )
        && snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
            == route.active
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_route(
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    snapshot_shape_is_exact(snapshot).then_some(())?;
    predecessor_route(cp418_shape(snapshot))
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshots_match_bit_exact(
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
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
                .transition_count
                .saturating_sub(1)
        && match (state.latest, witness) {
            (Some(latest), Some(witness)) => {
                snapshots_match_bit_exact(latest, witness)
                    && snapshot_shape_is_exact(latest)
                    && state.latest_transition_ordinal == Some(state.transition_count)
                    && state
                        .latest_route
                        .is_some_and(|route| {
                            route.active
                                == latest.post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
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
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment;
    state.transition_count == expected_transition_count
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
                .transition_count
        && state.latest.is_some_and(|latest| {
            snapshot_shape_is_exact(latest)
                && predecessor_route(cp418_shape(latest)).is_some_and(|route| {
                    completed_state_matches_validated_snapshot(state, latest, route)
                })
        })
        && committed_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
        )
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_is_consistent(
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
        || !direct_subset_values_are_valid(snapshot)
        || !witness.is_some_and(|witness| snapshots_match_bit_exact(snapshot, witness))
    {
        return false;
    }
    let predecessor = cp418_shape(snapshot);
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let owner_matches = match (
        route.active,
        runtime.cooling_mixed_air_call_latest_witness(system.id),
    ) {
        (true, Some(witness)) => {
            cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio(unit, witness)
                .zip(snapshot.mixed_air_humidity_ratio_for_cp_air)
                .is_some_and(|(owner, retained)| owner.to_bits() == retained.to_bits())
        }
        (false, _) => snapshot.mixed_air_humidity_ratio_for_cp_air.is_none(),
        (true, None) => false,
    };
    if !owner_matches {
        return false;
    }
    let state =
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment;
    if state.transition_count != unit.init_call_count
        || !completed_state_matches_validated_snapshot(state, snapshot, route)
        || !committed_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
        )
    {
        return false;
    }
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_latest_witness(system.id);
    let retained_predecessor = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
        .latest;
    retained_predecessor.is_some_and(|retained| {
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact(
            retained,
            predecessor,
        )
    }) && predecessor_witness.is_some_and(|witness| {
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact(
            witness,
            predecessor,
        )
    })
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentError{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentError::CoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshotMismatch { system }
}

fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentError{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
            .transition_count,
        transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
            .transition_count,
    }
}
