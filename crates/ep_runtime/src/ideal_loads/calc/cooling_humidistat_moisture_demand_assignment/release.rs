//! Release-bound CP359 Humidistat moisture-demand assignment evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Snapshot,
    advance_cooling_humidistat_moisture_demand_assignment_state,
};
use crate::ideal_loads::calc::cooling_humidistat_case_entry::completed_direct_cooling_humidistat_case_entry_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot as Predecessor, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset, cooling_humidistat_case_entry_snapshot_is_exact_direct_release,
};

mod prefix_validation;
// CP360 may consume this explicitly parameterized bridge. Keep it available
// while CP359 is the source-order frontier.
#[allow(dead_code)]
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{active_lineage_is_exact, assignment_links_to_predecessor};
pub(in crate::ideal_loads::calc) use private_counterfactual::{
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as snapshots_match_bit_exact_for_test;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit.calc_cooling_humidistat_case_entry.latest else {
        return false;
    };
    let predecessor_witness = runtime.cooling_humidistat_case_entry_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness == Some(predecessor)
        && cooling_humidistat_case_entry_snapshot_is_exact_direct_release(predecessor)
        && completed_direct_cooling_humidistat_case_entry_is_consistent(
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

/// Fail-closed CP359 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError {
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
    CoolingHumidistatCaseEntrySnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_humidistat_case_entry_transition_count: usize,
        cooling_humidistat_moisture_demand_assignment_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP359 for the exact direct no-OA release route.
///
/// Public direct release accepts no moisture-demand operand. Selector `None`
/// completes the switch before line 2229, so both CP359 source sites and all
/// numerical fields are complete skips.
pub fn advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp358: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError> {
    let selected = predecessor_cp358.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit.calc_cooling_humidistat_case_entry.latest;
    let predecessor_witness = runtime.cooling_humidistat_case_entry_latest_witness(selected);
    let assignment_witness =
        runtime.cooling_humidistat_moisture_demand_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::DehumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp358.controlled_zone != controlled_zone
        || retained_predecessor != predecessor_cp358
        || predecessor_witness != Some(predecessor_cp358)
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_humidistat_case_entry_snapshot_is_exact_direct_release(predecessor_cp358) {
        return Err(
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::PredecessorOutsideDirectSubset {
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
        || !completed_direct_cooling_humidistat_case_entry_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::RuntimeStateInvariantViolation {
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
        &unit.calc_cooling_humidistat_moisture_demand_assignment,
        retained_predecessor,
    ) {
        return Err(
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_humidistat_moisture_demand_assignment_state(
            &mut unit.calc_cooling_humidistat_moisture_demand_assignment,
            retained_predecessor,
            None,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime.set_cooling_humidistat_moisture_demand_assignment_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_humidistat_moisture_demand_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError {
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::CoolingHumidistatCaseEntrySnapshotMismatch {
        system,
    }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError {
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_humidistat_case_entry_transition_count: unit
            .calc_cooling_humidistat_case_entry
            .transition_count,
        cooling_humidistat_moisture_demand_assignment_transition_count: unit
            .calc_cooling_humidistat_moisture_demand_assignment
            .transition_count,
    }
}
