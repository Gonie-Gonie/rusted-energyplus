//! Release-bound CP406 latent-output capacity-guard else-branch entry.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_state as advance,
};
use crate::ideal_loads::calc::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshot_is_exact_direct_release,
};

mod private_characterization;
mod runtime_validation;
mod snapshot_validation;

pub(in crate::ideal_loads) use private_characterization::private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_characterization;
pub(in crate::ideal_loads) use runtime_validation::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_latest_metadata_is_consistent;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::counts_are_exact as test_counts_are_exact;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    pending_state_is_consistent, prepare_next_transition, prepared_completed_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    snapshot_is_exact as cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_is_exact,
    snapshot_route as cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_route,
    snapshots_match_bit_exact as cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshots_match_bit_exact,
};

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_latest_witness(system.id);
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(predecessor)
        && completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && snapshot_links_to_predecessor(snapshot, predecessor)
        && completed_state_is_consistent(unit, snapshot, witness)
}

/// Fail-closed CP406 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryError
{
    UnknownSystem {
        system: IdealLoadsAirSystemId,
    },
    InitializationNotReady {
        system: IdealLoadsAirSystemId,
    },
    SystemIdentityMismatch {
        expected: IdealLoadsAirSystemId,
        actual: IdealLoadsAirSystemId,
    },
    SystemOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    CoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshotMismatch
    {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        predecessor_transition_count: usize,
        else_branch_entry_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP406 for the exact direct no-OA release route.
///
/// The supplied CP405 maximum-capacity-assignment snapshot is admission evidence
/// only. CP406 evaluates no numeric expression: it records the guard-false
/// else-entry control point and preserves CP405 W/H/T payload bits unchanged.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp405: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryError>{
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryError as Error;
    let selected = predecessor_cp405.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let retained_predecessor = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment
        .latest;
    let predecessor_witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_latest_witness(selected);
    let else_entry_witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_latest_witness(selected);

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
    let controlled_zone = unit
        .controlled_zone
        .ok_or(Error::InitializationNotReady { system: selected })?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(Error::CoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshotMismatch { system: selected });
    };
    if predecessor_cp405.controlled_zone != controlled_zone
        || !crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshots_match_bit_exact(
            retained_predecessor,
            predecessor_cp405,
        )
        || !predecessor_witness.is_some_and(|witness| {
            crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshots_match_bit_exact(
                witness,
                predecessor_cp405,
            )
        })
    {
        return Err(Error::CoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshotMismatch { system: selected });
    }
    if !cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(predecessor_cp405) {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, retained_predecessor, else_entry_witness)
        || !completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let Some((prepared_state, prepared_snapshot)) = prepare_next_transition(
        &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry,
        retained_predecessor,
    ) else {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    };
    if !prepared_completed_state_is_consistent(unit, &prepared_state, prepared_snapshot) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let snapshot = {
        let unit = runtime
            .units
            .get_mut(&selected)
            .ok_or(Error::UnknownSystem { system: selected })?;
        advance(
            &mut unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry,
            retained_predecessor,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?
    };
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_latest_witness(
        selected,
        snapshot,
    );
    debug_assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_is_exact_direct_release(snapshot));
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn snapshot_links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor).is_some_and(|expected| {
        snapshot_validation::snapshots_match_bit_exact(expected, snapshot)
    })
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryError{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment.transition_count,
        else_branch_entry_transition_count: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry.transition_count,
    }
}
