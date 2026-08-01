//! Release-bound CP370 Cooling humidification-control Humidistat-guard evidence.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_state,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_heating_availability_guard::cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    direct_predecessor_is_retained_and_complete, guard_links_to_predecessor,
    humidification_control_type_provenance_is_exact,
};
pub(in crate::ideal_loads::calc) use private_counterfactual::{
    private_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use runtime_validation::cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_metadata_is_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    snapshot_route as cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_route,
    snapshots_match_exact as cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshots_match_exact,
};

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
        .latest
    else {
        return false;
    };
    let predecessor_witness =
        runtime.cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_witness(system.id);
    system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact(
                predecessor,
                predecessor_witness,
            )
        })
        && cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release(
            predecessor,
        )
        && direct_predecessor_is_retained_and_complete(runtime, unit, system, predecessor)
        && humidification_control_type_provenance_is_exact(runtime, unit, system, predecessor)
        && guard_links_to_predecessor(
            snapshot,
            predecessor,
            HumidificationControlType::None,
        )
        && completed_state_is_consistent(unit, snapshot, witness)
        && cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release(snapshot)
}

/// Fail-closed CP370 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError {
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
    HumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: HumidificationControlType,
    },
    CoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    HumidificationControlTypeProvenanceMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_supply_humidity_ratio_humidification_heating_availability_guard_transition_count: usize,
        cooling_supply_humidity_ratio_humidification_control_humidistat_guard_transition_count:
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
    for PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP370 Cooling humidification-control Humidistat guard release failed: {self:?}"
        )
    }
}

impl std::error::Error
    for PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError
{
}

/// Executes CP370 for the exact direct no-OA release route.
///
/// The source `HumidCtrlType` operand comes only from the immutable selected
/// system. The wrapper accepts no caller enum or numerical operand.
pub fn advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp369: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError,
> {
    let selected = predecessor_cp369.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
        .latest;
    let predecessor_witness =
        runtime.cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_witness(selected);
    let guard_witness = runtime
        .cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::SystemOutsideDirectSubset {
            system: selected,
        });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::InitializationNotReady {
            system: selected,
        });
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp369.controlled_zone != controlled_zone
        || !cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact(
            retained_predecessor,
            predecessor_cp369,
        )
        || !predecessor_witness.is_some_and(|witness| {
            cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact(
                witness,
                predecessor_cp369,
            )
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release(
        predecessor_cp369,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::PredecessorOutsideDirectSubset {
            system: selected,
        });
    }
    let humidification_control_type = system.humidification_control_type;
    if !humidification_control_type_provenance_is_exact(
        runtime,
        unit,
        system,
        predecessor_cp369,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::HumidificationControlTypeProvenanceMismatch {
            system: selected,
        });
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            guard_witness,
            humidification_control_type,
        )
        || !direct_predecessor_is_retained_and_complete(
            runtime,
            unit,
            system,
            retained_predecessor,
        )
    {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::RuntimeStateInvariantViolation {
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
            .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard,
        retained_predecessor,
        humidification_control_type,
    ) {
        return Err(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::RuntimeStateInvariantViolation {
            system: selected,
        });
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_state(
            &mut unit
                .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard,
            retained_predecessor,
            humidification_control_type,
        )
    }
    .ok_or(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::RuntimeStateInvariantViolation {
        system: selected,
    })?;
    runtime
        .set_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError {
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::CoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshotMismatch {
        system,
    }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError {
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_supply_humidity_ratio_humidification_heating_availability_guard_transition_count: unit
            .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
            .transition_count,
        cooling_supply_humidity_ratio_humidification_control_humidistat_guard_transition_count:
            unit
                .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard
                .transition_count,
    }
}