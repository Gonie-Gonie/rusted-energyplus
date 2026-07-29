//! Release-bound CP363 Humidistat case-break evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot as Snapshot,
    advance_cooling_humidistat_case_break_state,
};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_mixed_air_limit::{
    completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::case_break_links_to_predecessor;
pub(in crate::ideal_loads::calc) use private_counterfactual::{
    private_constant_supply_humidity_ratio_counterfactual_from_direct_release,
    private_constant_supply_humidity_ratio_counterfactual_links_to_direct_release,
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use runtime_validation::cooling_humidistat_case_break_latest_metadata_is_consistent;
pub(in crate::ideal_loads) use snapshot_validation::cooling_humidistat_case_break_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_exact as cooling_humidistat_case_break_snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_humidistat_case_break_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
                predecessor,
                predecessor_witness,
            )
        })
        && cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && case_break_links_to_predecessor(snapshot, predecessor)
        && completed_state_is_consistent(
            unit,
            snapshot,
            witness,
            system.dehumidification_control_type,
        )
}

/// Fail-closed CP363 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingHumidistatCaseBreakError {
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
    CoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_humidistat_supply_humidity_ratio_mixed_air_limit_transition_count: usize,
        cooling_humidistat_case_break_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcCoolingHumidistatCaseBreakError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "CP363 Humidistat case-break release failed: {self:?}")
    }
}

impl std::error::Error for PurchasedAirCalcCoolingHumidistatCaseBreakError {}

/// Executes CP363 for the exact direct no-OA release route.
///
/// CP362 latest, its witness, and CP362's recursive proof are the sole
/// predecessor evidence owners. CP363 independently reads no earlier state and carries
/// no numerical operand, result, arithmetic, comparison, finite, or range
/// gate. Direct `None` completes its case before line 2233, so the break site
/// is a complete control-flow skip.
pub fn advance_direct_no_oa_calc_cooling_humidistat_case_break(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp362: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcCoolingHumidistatCaseBreakError> {
    let selected = predecessor_cp362.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingHumidistatCaseBreakError::UnknownSystem { system: selected },
    )?;
    let retained_predecessor = unit
        .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .latest;
    let predecessor_witness =
        runtime.cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(selected);
    let case_break_witness = runtime.cooling_humidistat_case_break_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingHumidistatCaseBreakError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingHumidistatCaseBreakError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingHumidistatCaseBreakError::DehumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingHumidistatCaseBreakError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingHumidistatCaseBreakError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp362.controlled_zone != controlled_zone
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
            retained_predecessor,
            predecessor_cp362,
        )
        || !predecessor_witness.is_some_and(|witness| {
            cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
                witness,
                predecessor_cp362,
            )
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
        predecessor_cp362,
    ) {
        return Err(
            PurchasedAirCalcCoolingHumidistatCaseBreakError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            case_break_witness,
            system.dehumidification_control_type,
        )
        || !completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingHumidistatCaseBreakError::RuntimeStateInvariantViolation {
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
        &unit.calc_cooling_humidistat_case_break,
        retained_predecessor,
    ) {
        return Err(
            PurchasedAirCalcCoolingHumidistatCaseBreakError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingHumidistatCaseBreakError::UnknownSystem { system: selected },
        )?;
        advance_cooling_humidistat_case_break_state(
            &mut unit.calc_cooling_humidistat_case_break,
            retained_predecessor,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingHumidistatCaseBreakError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime.set_cooling_humidistat_case_break_latest_witness(selected, snapshot);
    debug_assert!(cooling_humidistat_case_break_snapshot_is_exact_direct_release(snapshot));
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_humidistat_case_break_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_humidistat_case_break_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingHumidistatCaseBreakError {
    PurchasedAirCalcCoolingHumidistatCaseBreakError::
        CoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshotMismatch { system }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingHumidistatCaseBreakError {
    PurchasedAirCalcCoolingHumidistatCaseBreakError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_humidistat_supply_humidity_ratio_mixed_air_limit_transition_count: unit
            .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
            .transition_count,
        cooling_humidistat_case_break_transition_count: unit
            .calc_cooling_humidistat_case_break
            .transition_count,
    }
}
