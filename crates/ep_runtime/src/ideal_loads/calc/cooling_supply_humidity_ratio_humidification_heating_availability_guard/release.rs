//! Release-bound CP369 Cooling humidification heating-availability guard evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard_state,
};
use crate::ideal_loads::calc::cooling_default_supply_humidity_ratio_case_break::cooling_default_supply_humidity_ratio_case_break_snapshots_match_exact;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_default_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_lineage_is_exact, direct_predecessor_is_retained_and_complete,
    guard_links_to_predecessor, heating_on_provenance_is_exact,
};
pub(in crate::ideal_loads::calc) use private_counterfactual::{
    private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use runtime_validation::cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_exact as cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_humidity_ratio_humidification_heating_availability_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_default_supply_humidity_ratio_case_break
        .latest
    else {
        return false;
    };
    let predecessor_witness =
        runtime.cooling_default_supply_humidity_ratio_case_break_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            cooling_default_supply_humidity_ratio_case_break_snapshots_match_exact(
                predecessor,
                predecessor_witness,
            )
        })
        && cooling_default_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(
            predecessor,
        )
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && heating_on_provenance_is_exact(runtime, unit, predecessor, true)
        && guard_links_to_predecessor(snapshot, predecessor, true)
        && active_lineage_is_exact(runtime, unit, system, predecessor, snapshot)
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release(snapshot)
}

/// Fail-closed CP369 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError {
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
    HeatingAvailabilityOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: bool,
    },
    CoolingDefaultSupplyHumidityRatioCaseBreakSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    HeatingOnProvenanceMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_default_supply_humidity_ratio_case_break_transition_count: usize,
        cooling_supply_humidity_ratio_humidification_heating_availability_guard_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display
    for PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP369 Cooling humidification heating-availability guard release failed: {self:?}"
        )
    }
}

impl std::error::Error
    for PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError
{
}

/// Executes CP369 for the exact direct no-OA release route.
///
/// The source `HeatOn` operand comes only from retained same-call Calc-entry
/// state. The wrapper accepts no caller Boolean or numerical operand.
pub fn advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp368: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError,
> {
    let selected = predecessor_cp368.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_default_supply_humidity_ratio_case_break
        .latest;
    let predecessor_witness =
        runtime.cooling_default_supply_humidity_ratio_case_break_latest_witness(selected);
    let guard_witness = runtime
        .cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::SystemOutsideDirectSubset {
            system: selected,
        });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::InitializationNotReady {
            system: selected,
        });
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp368.controlled_zone != controlled_zone
        || !cooling_default_supply_humidity_ratio_case_break_snapshots_match_exact(
            retained_predecessor,
            predecessor_cp368,
        )
        || !predecessor_witness.is_some_and(|witness| {
            cooling_default_supply_humidity_ratio_case_break_snapshots_match_exact(
                witness,
                predecessor_cp368,
            )
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_default_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(
        predecessor_cp368,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::PredecessorOutsideDirectSubset {
            system: selected,
        });
    }
    let heating_on = unit
        .calc_entry
        .latest
        .ok_or(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::InitializationNotReady {
            system: selected,
        })?
        .heating_on;
    if !heating_on {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::HeatingAvailabilityOutsideDirectSubset {
            system: selected,
            actual: heating_on,
        });
    }
    if !heating_on_provenance_is_exact(runtime, unit, predecessor_cp368, heating_on) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::HeatingOnProvenanceMismatch {
            system: selected,
        });
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(unit, retained_predecessor, guard_witness, heating_on)
        || !direct_predecessor_is_retained_and_complete(
            runtime,
            unit,
            system,
            retained_predecessor,
        )
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    if !next_transition_fits(
        &unit
            .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard,
        retained_predecessor,
        heating_on,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard_state(
            &mut unit
                .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard,
            retained_predecessor,
            heating_on,
        )
    }
    .ok_or(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::RuntimeStateInvariantViolation {
        system: selected,
    })?;
    runtime
        .set_cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_supply_humidity_ratio_humidification_heating_availability_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError {
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::CoolingDefaultSupplyHumidityRatioCaseBreakSnapshotMismatch {
        system,
    }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError {
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_default_supply_humidity_ratio_case_break_transition_count: unit
            .calc_cooling_default_supply_humidity_ratio_case_break
            .transition_count,
        cooling_supply_humidity_ratio_humidification_heating_availability_guard_transition_count:
            unit
                .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
                .transition_count,
    }
}