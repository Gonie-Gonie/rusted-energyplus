//! Release-bound CP360 Humidistat local humidity-ratio assignment evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot as Snapshot,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_state,
};
use crate::ideal_loads::calc::cooling_humidistat_moisture_demand_assignment::{
    completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent,
    cooling_humidistat_moisture_demand_assignment_snapshots_match_bit_exact as cp359_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release,
};

mod error;
mod operand_validation;
mod prefix_validation;
// CP361 may consume this explicitly parameterized bridge. Keep it available
// while CP360 is the source-order frontier.
#[allow(dead_code)]
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

use error::{call_order_error, predecessor_mismatch};
use prefix_validation::{active_lineage_is_exact, assignment_links_to_predecessor};
pub(in crate::ideal_loads::calc) use private_counterfactual::{
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact as snapshots_match_bit_exact_for_test;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_humidistat_moisture_demand_assignment
        .latest
    else {
        return false;
    };
    let Some(predecessor_witness) =
        runtime.cooling_humidistat_moisture_demand_assignment_latest_witness(system.id)
    else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && cp359_snapshots_match_bit_exact(predecessor_witness, predecessor)
        && cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(predecessor_witness),
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

/// Fail-closed CP360 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError {
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
    CoolingHumidistatMoistureDemandAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_humidistat_moisture_demand_assignment_transition_count: usize,
        cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP360 for the exact direct no-OA release route.
///
/// Selector `None` completes the switch before line 2230. The six CP360
/// source sites and every numeric evidence field therefore remain complete
/// skips; this wrapper accepts no numerical operands.
pub fn advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp359: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError,
> {
    let selected = predecessor_cp359.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_humidistat_moisture_demand_assignment
        .latest;
    let predecessor_witness =
        runtime.cooling_humidistat_moisture_demand_assignment_latest_witness(selected);
    let assignment_witness = runtime
        .cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::DehumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp359.controlled_zone != controlled_zone
        || !cp359_snapshots_match_bit_exact(retained_predecessor, predecessor_cp359)
        || !predecessor_witness
            .is_some_and(|witness| cp359_snapshots_match_bit_exact(witness, predecessor_cp359))
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(
        predecessor_cp359,
    ) {
        return Err(
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::PredecessorOutsideDirectSubset {
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
        || !completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::RuntimeStateInvariantViolation {
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
        &unit.calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
        retained_predecessor,
    ) {
        return Err(
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_state(
            &mut unit
                .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
            retained_predecessor,
            None,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime
        .set_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
