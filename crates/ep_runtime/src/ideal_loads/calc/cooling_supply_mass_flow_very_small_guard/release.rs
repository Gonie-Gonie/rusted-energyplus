//! Release-bound CP327 cooling supply-mass-flow very-small guard.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardInput,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    advance_cooling_supply_mass_flow_very_small_guard_state,
};
use crate::ideal_loads::calc::cooling_supply_mass_flow_maximum::{
    completed_direct_cooling_supply_mass_flow_limit_body_is_consistent,
    cooling_supply_mass_flow_limit_body_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot, PurchasedAirRuntimeState,
    PurchasedAirSizedLimits, classify_no_oa_sensible_subset,
    cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::very_small_guard_links_to_limit_body;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_guard, completed_guard_state_is_consistent,
    pending_guard_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_mass_flow_very_small_guard_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    guard: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    guard_witness: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot>,
) -> bool {
    let Some(body) = unit.calc_cooling_supply_mass_flow_limit_body.latest else {
        return false;
    };

    completed_direct_cooling_supply_mass_flow_limit_body_is_consistent(
        runtime,
        unit,
        system,
        body,
        runtime.cooling_supply_mass_flow_limit_body_latest_witness(system.id),
    ) && very_small_guard_links_to_limit_body(guard, body)
        && completed_guard_state_is_consistent(unit, guard, guard_witness)
}

/// Fail-closed CP327 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError {
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
    CoolingSupplyMassFlowLimitBodySnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    SizedLimitsMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidMaximumCoolingMassFlowCache {
        system: IdealLoadsAirSystemId,
        value_kg_per_s: f64,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_supply_mass_flow_limit_body_transition_count: usize,
        cooling_supply_mass_flow_very_small_guard_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP327 for the exact direct no-OA/no-EMS route.
pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp326: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError,
> {
    let selected = predecessor_cp326.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::UnknownSystem {
            system: selected,
        },
    )?;
    let body_witness = runtime.cooling_supply_mass_flow_limit_body_latest_witness(selected);
    let guard_witness = runtime.cooling_supply_mass_flow_very_small_guard_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let sized_limits = unit.sized_limits.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::InitializationNotReady {
            system: selected,
        },
    )?;
    if sized_limits != PurchasedAirSizedLimits::from_system(system)
        || unit
            .sizing_outcome
            .is_none_or(|outcome| outcome.sized_limits != sized_limits)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::SizedLimitsMismatch {
                system: selected,
            },
        );
    }
    let maximum_cooling_air_mass_flow_rate_kg_per_s =
        unit.maximum_cooling_air_mass_flow_rate_kg_per_s;
    if !maximum_cooling_air_mass_flow_rate_kg_per_s.is_finite()
        || maximum_cooling_air_mass_flow_rate_kg_per_s < 0.0
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::
                InvalidMaximumCoolingMassFlowCache {
                    system: selected,
                    value_kg_per_s: maximum_cooling_air_mass_flow_rate_kg_per_s,
                },
        );
    }
    if !crate::ideal_loads::calc::cooling_economizer_condition::
        exact_direct_initialization_is_consistent(runtime, unit, system)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !unit
        .calc_cooling_supply_mass_flow_limit_body
        .latest
        .is_some_and(|latest| {
            cooling_supply_mass_flow_limit_body_snapshots_match_bit_exact(latest, predecessor_cp326)
        })
        || !body_witness.is_some_and(|witness| {
            cooling_supply_mass_flow_limit_body_snapshots_match_bit_exact(
                witness,
                predecessor_cp326,
            )
        })
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::
                CoolingSupplyMassFlowLimitBodySnapshotMismatch { system: selected },
        );
    }
    if !cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release(predecessor_cp326) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !call_order_is_pending_guard(unit, predecessor_cp326) {
        return Err(call_order_error(unit, selected));
    }
    if !completed_direct_cooling_supply_mass_flow_limit_body_is_consistent(
        runtime,
        unit,
        system,
        predecessor_cp326,
        body_witness,
    ) || !pending_guard_state_is_consistent(unit, predecessor_cp326, guard_witness)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if predecessor_cp326
        .resulting_supply_mass_flow_rate_kg_per_s
        .is_some()
        != predecessor_cp326.cooling_body_entered
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_supply_mass_flow_very_small_guard_state(
            &mut unit.calc_cooling_supply_mass_flow_very_small_guard,
            predecessor_cp326,
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardInput {
                supply_mass_flow_rate_kg_per_s: predecessor_cp326
                    .resulting_supply_mass_flow_rate_kg_per_s,
            },
        )
    };
    runtime.set_cooling_supply_mass_flow_very_small_guard_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(very_small_guard_links_to_limit_body(
        snapshot,
        predecessor_cp326
    ));
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_supply_mass_flow_very_small_guard_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_supply_mass_flow_very_small_guard_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError {
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_supply_mass_flow_limit_body_transition_count: unit
            .calc_cooling_supply_mass_flow_limit_body
            .transition_count,
        cooling_supply_mass_flow_very_small_guard_transition_count: unit
            .calc_cooling_supply_mass_flow_very_small_guard
            .transition_count,
    }
}
