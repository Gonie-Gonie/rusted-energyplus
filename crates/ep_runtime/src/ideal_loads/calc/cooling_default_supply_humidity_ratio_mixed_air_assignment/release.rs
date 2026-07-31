//! Release-bound CP367 default supply-humidity-ratio mixed-air assignment evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot as Snapshot,
    advance_cooling_default_supply_humidity_ratio_mixed_air_assignment_state,
};
use crate::ideal_loads::calc::cooling_default_supply_humidity_ratio_mixed_air_assignment::transition::predecessor_snapshots_match_bit_exact;
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_case_break::{
    completed_direct_cooling_constant_supply_humidity_ratio_case_break_is_consistent,
    cooling_constant_supply_humidity_ratio_case_break_snapshots_match_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_lineage_is_exact, assignment_links_to_predecessor,
    direct_predecessor_is_retained_and_complete,
};
pub(in crate::ideal_loads::calc) use private_counterfactual::{
    private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_from_direct_release,
    private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use runtime_validation::cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_exact as cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshots_match_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_default_supply_humidity_ratio_mixed_air_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_constant_supply_humidity_ratio_case_break
        .latest
    else {
        return false;
    };
    let predecessor_witness =
        runtime.cooling_constant_supply_humidity_ratio_case_break_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            cooling_constant_supply_humidity_ratio_case_break_snapshots_match_exact(
                predecessor,
                predecessor_witness,
            )
        })
        && cooling_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_constant_supply_humidity_ratio_case_break_is_consistent(
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

/// Fail-closed CP367 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError {
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
    CoolingConstantSupplyHumidityRatioCaseBreakSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_constant_supply_humidity_ratio_case_break_transition_count: usize,
        cooling_default_supply_humidity_ratio_mixed_air_assignment_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP367 default supply-humidity-ratio mixed-air assignment release failed: {self:?}"
        )
    }
}

impl std::error::Error for PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError {}

/// Executes CP367 for the exact direct no-OA release route.
///
/// Direct selector `None` exits its named case before line 2238. Both CP367
/// source sites are therefore complete skips. This wrapper accepts no numeric
/// operand and does not read mixed-air humidity.
pub fn advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp366: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError> {
    let selected = predecessor_cp366.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_constant_supply_humidity_ratio_case_break
        .latest;
    let predecessor_witness =
        runtime.cooling_constant_supply_humidity_ratio_case_break_latest_witness(selected);
    let assignment_witness =
        runtime.cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::DehumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp366.controlled_zone != controlled_zone
        || !predecessor_snapshots_match_bit_exact(retained_predecessor, predecessor_cp366)
        || !predecessor_witness.is_some_and(|witness| {
            predecessor_snapshots_match_bit_exact(witness, predecessor_cp366)
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(
        predecessor_cp366,
    ) {
        return Err(
            PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::PredecessorOutsideDirectSubset {
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
            PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::RuntimeStateInvariantViolation {
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
        &unit.calc_cooling_default_supply_humidity_ratio_mixed_air_assignment,
        retained_predecessor,
    ) {
        return Err(
            PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_default_supply_humidity_ratio_mixed_air_assignment_state(
            &mut unit.calc_cooling_default_supply_humidity_ratio_mixed_air_assignment,
            retained_predecessor,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime
        .set_cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_default_supply_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError {
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::
        CoolingConstantSupplyHumidityRatioCaseBreakSnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError {
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_constant_supply_humidity_ratio_case_break_transition_count: unit
            .calc_cooling_constant_supply_humidity_ratio_case_break
            .transition_count,
        cooling_default_supply_humidity_ratio_mixed_air_assignment_transition_count: unit
            .calc_cooling_default_supply_humidity_ratio_mixed_air_assignment
            .transition_count,
    }
}
