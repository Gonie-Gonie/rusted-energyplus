//! Release-bound CP365 constant-supply-humidity-ratio assignment evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot as Snapshot,
    advance_cooling_constant_supply_humidity_ratio_assignment_state,
};
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_assignment::transition::predecessor_snapshots_match_exact;
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_case_entry::{
    completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent,
    cooling_constant_supply_humidity_ratio_case_entry_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

pub(in crate::ideal_loads) use prefix_validation::cooling_constant_supply_humidity_ratio_assignment_snapshot_links_to_predecessor;
use prefix_validation::{
    active_lineage_is_exact, assignment_links_to_predecessor,
    direct_predecessor_is_retained_and_complete,
};
pub(in crate::ideal_loads::calc) use private_counterfactual::{
    private_constant_supply_humidity_ratio_assignment_counterfactual_from_direct_release,
    private_constant_supply_humidity_ratio_assignment_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use runtime_validation::cooling_constant_supply_humidity_ratio_assignment_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as cooling_constant_supply_humidity_ratio_assignment_snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_constant_supply_humidity_ratio_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_constant_supply_humidity_ratio_case_entry
        .latest
    else {
        return false;
    };
    let predecessor_witness =
        runtime.cooling_constant_supply_humidity_ratio_case_entry_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            cooling_constant_supply_humidity_ratio_case_entry_snapshots_match_bit_exact(
                predecessor,
                predecessor_witness,
            )
        })
        && cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && assignment_links_to_predecessor(snapshot, predecessor)
        && active_lineage_is_exact(runtime, unit, system, predecessor, snapshot)
        && completed_state_is_consistent(
            unit,
            snapshot,
            witness,
            system.dehumidification_control_type,
        )
}

/// Fail-closed CP365 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError {
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
    DehumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: DehumidificationControlType,
    },
    CoolingConstantSupplyHumidityRatioCaseEntrySnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_constant_supply_humidity_ratio_case_entry_transition_count: usize,
        cooling_constant_supply_humidity_ratio_assignment_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP365 constant-supply-humidity-ratio assignment release failed: {self:?}"
        )
    }
}

impl std::error::Error for PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError {}

/// Executes CP365 for the exact direct no-OA release route.
///
/// Direct selector `None` has completed the switch before line 2235. Both
/// CP365 source sites and every numeric field therefore remain complete skips;
/// this wrapper deliberately accepts no numerical operand.
pub fn advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp364: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError> {
    let selected = predecessor_cp364.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_constant_supply_humidity_ratio_case_entry
        .latest;
    let predecessor_witness =
        runtime.cooling_constant_supply_humidity_ratio_case_entry_latest_witness(selected);
    let assignment_witness =
        runtime.cooling_constant_supply_humidity_ratio_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::DehumidificationControlTypeOutsideDirectSubset {
                system: selected,
                actual: system.dehumidification_control_type,
            },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp364.controlled_zone != controlled_zone
        || !predecessor_snapshots_match_exact(retained_predecessor, predecessor_cp364)
        || !predecessor_witness
            .is_some_and(|witness| predecessor_snapshots_match_exact(witness, predecessor_cp364))
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(
        predecessor_cp364,
    ) {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            assignment_witness,
            system.dehumidification_control_type,
        )
        || !direct_predecessor_is_retained_and_complete(runtime, unit, system, retained_predecessor)
    {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    if !next_transition_fits(
        &unit.calc_cooling_constant_supply_humidity_ratio_assignment,
        retained_predecessor,
    ) {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_constant_supply_humidity_ratio_assignment_state(
            &mut unit.calc_cooling_constant_supply_humidity_ratio_assignment,
            retained_predecessor,
            None,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime
        .set_cooling_constant_supply_humidity_ratio_assignment_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_constant_supply_humidity_ratio_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_constant_supply_humidity_ratio_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError {
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::
        CoolingConstantSupplyHumidityRatioCaseEntrySnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError {
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_constant_supply_humidity_ratio_case_entry_transition_count: unit
            .calc_cooling_constant_supply_humidity_ratio_case_entry
            .transition_count,
        cooling_constant_supply_humidity_ratio_assignment_transition_count: unit
            .calc_cooling_constant_supply_humidity_ratio_assignment
            .transition_count,
    }
}
