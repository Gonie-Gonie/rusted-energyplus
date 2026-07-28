//! Release-bound CP328 cooling supply-mass-flow positive-zero reset body.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    advance_cooling_supply_mass_flow_very_small_guard_body_state,
};
use crate::ideal_loads::calc::cooling_supply_mass_flow_very_small_guard::{
    completed_direct_cooling_supply_mass_flow_very_small_guard_is_consistent,
    cooling_supply_mass_flow_very_small_guard_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot, PurchasedAirRuntimeState,
    PurchasedAirSizedLimits, classify_no_oa_sensible_subset,
    cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::very_small_guard_body_links_to_guard;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_body, completed_body_state_is_consistent,
    pending_body_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_mass_flow_very_small_guard_body_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    body: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    body_witness: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot>,
) -> bool {
    let Some(guard) = unit.calc_cooling_supply_mass_flow_very_small_guard.latest else {
        return false;
    };

    completed_direct_cooling_supply_mass_flow_very_small_guard_is_consistent(
        runtime,
        unit,
        system,
        guard,
        runtime.cooling_supply_mass_flow_very_small_guard_latest_witness(system.id),
    ) && very_small_guard_body_links_to_guard(body, guard)
        && completed_body_state_is_consistent(unit, body, body_witness)
}

/// Fail-closed CP328 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError {
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
    CoolingSupplyMassFlowVerySmallGuardSnapshotMismatch {
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
        cooling_supply_mass_flow_very_small_guard_transition_count: usize,
        cooling_supply_mass_flow_very_small_guard_body_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP328 for the exact direct no-OA/no-EMS route.
pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp327: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError,
> {
    let selected = predecessor_cp327.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::UnknownSystem {
            system: selected,
        },
    )?;
    let guard_witness = runtime.cooling_supply_mass_flow_very_small_guard_latest_witness(selected);
    let body_witness =
        runtime.cooling_supply_mass_flow_very_small_guard_body_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let sized_limits = unit.sized_limits.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::InitializationNotReady {
            system: selected,
        },
    )?;
    if sized_limits != PurchasedAirSizedLimits::from_system(system)
        || unit
            .sizing_outcome
            .is_none_or(|outcome| outcome.sized_limits != sized_limits)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::SizedLimitsMismatch {
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
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::
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
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !unit
        .calc_cooling_supply_mass_flow_very_small_guard
        .latest
        .is_some_and(|latest| {
            cooling_supply_mass_flow_very_small_guard_snapshots_match_bit_exact(
                latest,
                predecessor_cp327,
            )
        })
        || !guard_witness.is_some_and(|witness| {
            cooling_supply_mass_flow_very_small_guard_snapshots_match_bit_exact(
                witness,
                predecessor_cp327,
            )
        })
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::
                CoolingSupplyMassFlowVerySmallGuardSnapshotMismatch { system: selected },
        );
    }
    if !cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(
        predecessor_cp327,
    ) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !call_order_is_pending_body(unit, predecessor_cp327) {
        return Err(call_order_error(unit, selected));
    }
    if !completed_direct_cooling_supply_mass_flow_very_small_guard_is_consistent(
        runtime,
        unit,
        system,
        predecessor_cp327,
        guard_witness,
    ) || !pending_body_state_is_consistent(unit, predecessor_cp327, body_witness)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if predecessor_cp327.supply_mass_flow_rate_kg_per_s.is_some()
        != predecessor_cp327.cooling_body_entered
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_supply_mass_flow_very_small_guard_body_state(
            &mut unit.calc_cooling_supply_mass_flow_very_small_guard_body,
            predecessor_cp327,
        )
    };
    runtime.set_cooling_supply_mass_flow_very_small_guard_body_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(very_small_guard_body_links_to_guard(
        snapshot,
        predecessor_cp327
    ));
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_supply_mass_flow_very_small_guard_body_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_supply_mass_flow_very_small_guard_body_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError {
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_supply_mass_flow_very_small_guard_transition_count: unit
            .calc_cooling_supply_mass_flow_very_small_guard
            .transition_count,
        cooling_supply_mass_flow_very_small_guard_body_transition_count: unit
            .calc_cooling_supply_mass_flow_very_small_guard_body
            .transition_count,
    }
}
