//! Release-bound CP326 cooling supply-mass-flow limit body.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyInput,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    advance_cooling_supply_mass_flow_limit_body_state,
};
use crate::ideal_loads::calc::cooling_supply_mass_flow_maximum::flow_limit_guard::completed_direct_cooling_supply_mass_flow_limit_guard_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot, PurchasedAirRuntimeState,
    PurchasedAirSizedLimits, classify_no_oa_sensible_subset,
    cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    limit_body_inputs_link_to_supply_maximum_and_cache, limit_body_links_to_guard,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_body, completed_body_state_is_consistent,
    pending_body_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_supply_mass_flow_limit_body_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    body: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    body_witness: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot>,
) -> bool {
    let Some(guard) = unit.calc_cooling_supply_mass_flow_limit_guard.latest else {
        return false;
    };
    let Some(maximum) = unit.calc_cooling_supply_mass_flow_maximum.latest else {
        return false;
    };

    completed_direct_cooling_supply_mass_flow_limit_guard_is_consistent(
        runtime,
        unit,
        system,
        guard,
        runtime.cooling_supply_mass_flow_limit_guard_latest_witness(system.id),
    ) && limit_body_links_to_guard(body, guard)
        && limit_body_inputs_link_to_supply_maximum_and_cache(
            body,
            maximum,
            unit.maximum_cooling_air_mass_flow_rate_kg_per_s,
        )
        && completed_body_state_is_consistent(unit, body, body_witness)
}

/// Fail-closed CP326 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError {
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
    CoolingSupplyMassFlowLimitGuardSnapshotMismatch {
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
        cooling_supply_mass_flow_limit_guard_transition_count: usize,
        cooling_supply_mass_flow_limit_body_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP326 for the exact direct no-OA/no-EMS route.
pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp325: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError,
> {
    let selected = predecessor_cp325.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::UnknownSystem { system: selected },
    )?;
    let guard_witness = runtime.cooling_supply_mass_flow_limit_guard_latest_witness(selected);
    let body_witness = runtime.cooling_supply_mass_flow_limit_body_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let sized_limits = unit.sized_limits.ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::InitializationNotReady {
            system: selected,
        },
    )?;
    if sized_limits != PurchasedAirSizedLimits::from_system(system)
        || unit
            .sizing_outcome
            .is_none_or(|outcome| outcome.sized_limits != sized_limits)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::SizedLimitsMismatch {
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
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::
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
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if unit.calc_cooling_supply_mass_flow_limit_guard.latest != Some(predecessor_cp325)
        || guard_witness != Some(predecessor_cp325)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::
                CoolingSupplyMassFlowLimitGuardSnapshotMismatch { system: selected },
        );
    }
    if !cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(predecessor_cp325) {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !call_order_is_pending_body(unit, predecessor_cp325) {
        return Err(call_order_error(unit, selected));
    }
    if !completed_direct_cooling_supply_mass_flow_limit_guard_is_consistent(
        runtime,
        unit,
        system,
        predecessor_cp325,
        guard_witness,
    ) || !pending_body_state_is_consistent(unit, predecessor_cp325, body_witness)
    {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    let Some(maximum_snapshot) = unit.calc_cooling_supply_mass_flow_maximum.latest else {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    };
    let retained_supply_mass_flow_rate_kg_per_s =
        maximum_snapshot.resulting_supply_mass_flow_rate_kg_per_s;
    if retained_supply_mass_flow_rate_kg_per_s.is_some() != predecessor_cp325.cooling_body_entered {
        return Err(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::UnknownSystem { system: selected },
        )?;
        advance_cooling_supply_mass_flow_limit_body_state(
            &mut unit.calc_cooling_supply_mass_flow_limit_body,
            predecessor_cp325,
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyInput {
                supply_mass_flow_rate_before_limit_kg_per_s:
                    retained_supply_mass_flow_rate_kg_per_s,
                maximum_cooling_air_mass_flow_rate_kg_per_s,
            },
        )
    };
    runtime.set_cooling_supply_mass_flow_limit_body_latest_witness(selected, snapshot);
    debug_assert!(cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release(snapshot));
    debug_assert!(limit_body_links_to_guard(snapshot, predecessor_cp325));
    debug_assert!(limit_body_inputs_link_to_supply_maximum_and_cache(
        snapshot,
        maximum_snapshot,
        maximum_cooling_air_mass_flow_rate_kg_per_s,
    ));
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_supply_mass_flow_limit_body_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_supply_mass_flow_limit_body_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError {
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_supply_mass_flow_limit_guard_transition_count: unit
            .calc_cooling_supply_mass_flow_limit_guard
            .transition_count,
        cooling_supply_mass_flow_limit_body_transition_count: unit
            .calc_cooling_supply_mass_flow_limit_body
            .transition_count,
    }
}
