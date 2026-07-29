//! Release-bound CP364 constant-supply-humidity-ratio case-entry evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Snapshot,
    advance_cooling_constant_supply_humidity_ratio_case_entry_state,
};
use crate::ideal_loads::calc::cooling_humidistat_case_break::{
    completed_direct_cooling_humidistat_case_break_is_consistent,
    cooling_humidistat_case_break_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot as Predecessor, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset, cooling_humidistat_case_break_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::case_entry_links_to_predecessor;
pub(in crate::ideal_loads::calc) use private_counterfactual::{
    private_constant_supply_humidity_ratio_case_entry_counterfactual_from_direct_release,
    private_constant_supply_humidity_ratio_case_entry_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use runtime_validation::cooling_constant_supply_humidity_ratio_case_entry_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_exact as cooling_constant_supply_humidity_ratio_case_entry_snapshots_match_bit_exact;

pub(in crate::ideal_loads) use prefix_validation::cooling_constant_supply_humidity_ratio_case_entry_snapshot_links_to_predecessor;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit.calc_cooling_humidistat_case_break.latest else {
        return false;
    };
    let predecessor_witness = runtime.cooling_humidistat_case_break_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            cooling_humidistat_case_break_snapshots_match_bit_exact(
                predecessor,
                predecessor_witness,
            )
        })
        && cooling_humidistat_case_break_snapshot_is_exact_direct_release(predecessor)
        && completed_direct_cooling_humidistat_case_break_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && case_entry_links_to_predecessor(snapshot, predecessor)
        && completed_state_is_consistent(
            unit,
            snapshot,
            witness,
            system.dehumidification_control_type,
        )
}

/// Fail-closed CP364 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError {
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
    CoolingHumidistatCaseBreakSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_humidistat_case_break_transition_count: usize,
        cooling_constant_supply_humidity_ratio_case_entry_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP364 constant-supply-humidity-ratio case-entry release failed: {self:?}"
        )
    }
}

impl std::error::Error for PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError {}

/// Executes CP364 for the exact direct no-OA release route.
///
/// CP363 latest, its witness, and CP363's recursive proof are the sole
/// predecessor evidence owners. Direct `None` has already completed its case,
/// so line 2234 is skipped. CP364 carries no numerical operand or result.
pub fn advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp363: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError> {
    let selected = predecessor_cp363.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit.calc_cooling_humidistat_case_break.latest;
    let predecessor_witness = runtime.cooling_humidistat_case_break_latest_witness(selected);
    let case_entry_witness =
        runtime.cooling_constant_supply_humidity_ratio_case_entry_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::DehumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp363.controlled_zone != controlled_zone
        || !cooling_humidistat_case_break_snapshots_match_bit_exact(
            retained_predecessor,
            predecessor_cp363,
        )
        || !predecessor_witness.is_some_and(|witness| {
            cooling_humidistat_case_break_snapshots_match_bit_exact(witness, predecessor_cp363)
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_humidistat_case_break_snapshot_is_exact_direct_release(predecessor_cp363) {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            case_entry_witness,
            system.dehumidification_control_type,
        )
        || !completed_direct_cooling_humidistat_case_break_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::RuntimeStateInvariantViolation {
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
        &unit.calc_cooling_constant_supply_humidity_ratio_case_entry,
        retained_predecessor,
    ) {
        return Err(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_constant_supply_humidity_ratio_case_entry_state(
            &mut unit.calc_cooling_constant_supply_humidity_ratio_case_entry,
            retained_predecessor,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime
        .set_cooling_constant_supply_humidity_ratio_case_entry_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_constant_supply_humidity_ratio_case_entry_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError {
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::
        CoolingHumidistatCaseBreakSnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError {
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_humidistat_case_break_transition_count: unit
            .calc_cooling_humidistat_case_break
            .transition_count,
        cooling_constant_supply_humidity_ratio_case_entry_transition_count: unit
            .calc_cooling_constant_supply_humidity_ratio_case_entry
            .transition_count,
    }
}
