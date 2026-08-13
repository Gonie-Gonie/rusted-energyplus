//! Release-bound CP321 cooling-capacity-zero candidate reset.

use ep_model::{AutosizeOrNumber, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit};

use super::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetInput,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    advance_cooling_capacity_zero_flow_reset_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidificationFlowSnapshot, PurchasedAirRuntimeState,
    PurchasedAirSizedLimits, classify_no_oa_sensible_subset,
};

mod predecessor_validation;
mod runtime_validation;
mod snapshot_validation;
mod committed;

pub(in crate::ideal_loads::calc) use committed::cooling_capacity_zero_flow_reset_committed_latest_maximum_total_cooling_capacity;

use predecessor_validation::predecessor_chain_and_candidates_are_consistent;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending_capacity_zero_reset,
    pending_capacity_zero_reset_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release;

/// Fail-closed CP321 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingCapacityZeroFlowResetError {
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
    CoolingHumidificationFlowSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorLinkMismatch {
        system: IdealLoadsAirSystemId,
    },
    SizedLimitsMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_humidification_flow_transition_count: usize,
        cooling_capacity_zero_flow_reset_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP321 without live service arguments.
pub fn advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp320: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) -> Result<
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingCapacityZeroFlowResetError,
> {
    let selected = predecessor_cp320.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingCapacityZeroFlowResetError::UnknownSystem { system: selected },
    )?;
    let condition_witness = runtime.cooling_economizer_condition_latest_witness(selected);
    let body_witness = runtime.cooling_economizer_body_latest_witness(selected);
    let sensible_witness = runtime.cooling_sensible_flow_latest_witness(selected);
    let dehumidification_witness = runtime.cooling_dehumidification_flow_latest_witness(selected);
    let humidification_witness = runtime.cooling_humidification_flow_latest_witness(selected);
    let reset_witness = runtime.cooling_capacity_zero_flow_reset_latest_witness(selected);
    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if unit.init_call_count == 0 || unit.calc_entry.latest.is_none() {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let sized_limits = unit.sized_limits.ok_or(
        PurchasedAirCalcCoolingCapacityZeroFlowResetError::InitializationNotReady {
            system: selected,
        },
    )?;
    let expected_sized_limits = PurchasedAirSizedLimits::from_system(system);
    if sized_limits != expected_sized_limits
        || unit
            .sizing_outcome
            .is_none_or(|outcome| outcome.sized_limits != sized_limits)
    {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::SizedLimitsMismatch {
                system: selected,
            },
        );
    }
    if !super::super::cooling_economizer_condition::exact_direct_initialization_is_consistent(
        runtime, unit, system,
    ) {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::InitializationNotReady {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected) {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if unit.calc_cooling_humidification_flow.latest != Some(predecessor_cp320)
        || humidification_witness != Some(predecessor_cp320)
    {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::
                CoolingHumidificationFlowSnapshotMismatch { system: selected },
        );
    }
    let dehumidification = unit.calc_cooling_dehumidification_flow.latest.ok_or(
        PurchasedAirCalcCoolingCapacityZeroFlowResetError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let sensible = unit.calc_cooling_sensible_flow.latest.ok_or(
        PurchasedAirCalcCoolingCapacityZeroFlowResetError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let body = unit.calc_cooling_economizer_body.latest.ok_or(
        PurchasedAirCalcCoolingCapacityZeroFlowResetError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let condition = unit.calc_cooling_economizer_condition.latest.ok_or(
        PurchasedAirCalcCoolingCapacityZeroFlowResetError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    let guard = unit.calc_cooling_economizer_guard.latest.ok_or(
        PurchasedAirCalcCoolingCapacityZeroFlowResetError::PredecessorLinkMismatch {
            system: selected,
        },
    )?;
    if !predecessor_chain_and_candidates_are_consistent(
        sensible,
        dehumidification,
        predecessor_cp320,
    ) {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::PredecessorLinkMismatch {
                system: selected,
            },
        );
    }
    if !super::super::cooling_humidification_flow::
        cooling_humidification_flow_snapshot_is_exact_direct_release(predecessor_cp320)
    {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !call_order_is_pending_capacity_zero_reset(unit, predecessor_cp320) {
        return Err(call_order_error(unit, selected));
    }
    if !super::super::cooling_economizer_condition::
        completed_direct_prefix_through_economizer_guard_is_consistent(unit, system, guard)
        || !super::super::cooling_economizer_condition::
            completed_direct_economizer_condition_is_consistent(unit, condition, condition_witness)
        || !super::super::cooling_economizer_body::
            completed_direct_cooling_economizer_body_is_consistent(
                unit,
                condition,
                body,
                body_witness,
            )
        || !super::super::cooling_sensible_flow::
            completed_direct_cooling_sensible_flow_is_consistent(
                unit,
                body,
                sensible,
                sensible_witness,
            )
        || !super::super::cooling_dehumidification_flow::
        completed_direct_cooling_dehumidification_flow_is_consistent(
            unit,
            sensible,
            dehumidification,
            dehumidification_witness,
        )
        || !super::super::cooling_humidification_flow::
            completed_direct_cooling_humidification_flow_is_consistent(
                unit,
                dehumidification,
                predecessor_cp320,
                humidification_witness,
            )
        || !pending_capacity_zero_reset_state_is_consistent(
            unit,
            predecessor_cp320,
            reset_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let cooling = predecessor_cp320.cooling_body_entered;
    let prior_cool = sensible
        .resulting_supply_mass_flow_rate_for_cool_kg_per_s
        .filter(|_| cooling)
        .unwrap_or(f64::NAN);
    let prior_dehumidification = dehumidification
        .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s
        .filter(|_| cooling)
        .unwrap_or(f64::NAN);
    let maximum_capacity = maximum_capacity_input(system.cooling_limit, sized_limits);
    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingCapacityZeroFlowResetError::UnknownSystem { system: selected },
        )?;
        advance_cooling_capacity_zero_flow_reset_state(
            &mut unit.calc_cooling_capacity_zero_flow_reset,
            predecessor_cp320,
            PurchasedAirCalcCoolingCapacityZeroFlowResetInput {
                cooling_limit: system.cooling_limit,
                maximum_total_cooling_capacity_w: maximum_capacity,
                supply_mass_flow_rate_for_cool_kg_per_s: prior_cool,
                supply_mass_flow_rate_for_dehumidification_kg_per_s: prior_dehumidification,
            },
        )
    };
    runtime.set_cooling_capacity_zero_flow_reset_latest_witness(selected, snapshot);
    debug_assert!(cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(snapshot));
    Ok(snapshot)
}

fn maximum_capacity_input(
    cooling_limit: IdealLoadsLimit,
    sized_limits: PurchasedAirSizedLimits,
) -> f64 {
    if matches!(
        cooling_limit,
        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
    ) {
        match sized_limits.maximum_total_cooling_capacity_w {
            Some(AutosizeOrNumber::Value(value)) => value,
            Some(AutosizeOrNumber::Autosize) | None => f64::NAN,
        }
    } else {
        f64::NAN
    }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingCapacityZeroFlowResetError {
    PurchasedAirCalcCoolingCapacityZeroFlowResetError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_humidification_flow_transition_count: unit
            .calc_cooling_humidification_flow
            .transition_count,
        cooling_capacity_zero_flow_reset_transition_count: unit
            .calc_cooling_capacity_zero_flow_reset
            .transition_count,
    }
}
