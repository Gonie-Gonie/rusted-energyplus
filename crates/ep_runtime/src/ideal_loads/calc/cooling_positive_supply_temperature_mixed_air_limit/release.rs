//! Release-bound CP334 Cooling positive-supply mixed-air temperature limit.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    advance_cooling_positive_supply_temperature_mixed_air_limit_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::completed_direct_cooling_positive_supply_temperature_minimum_limit_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
    cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use prefix_validation::active_operands_link_to_retained_prefix as active_operands_link_to_retained_prefix_for_test;
use prefix_validation::{
    active_operands_link_to_retained_prefix, minimum_limit_snapshots_match_bit_exact,
    mixed_air_limit_links_to_minimum_limit,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_supply_temperature_mixed_air_limit_state_is_consistent,
    next_supply_temperature_mixed_air_limit_transition_fits,
    pending_supply_temperature_mixed_air_limit_state_is_consistent,
};
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use runtime_validation::{
    next_supply_temperature_mixed_air_limit_transition_fits as next_supply_temperature_mixed_air_limit_transition_fits_for_test,
    pending_supply_temperature_mixed_air_limit_state_is_consistent as pending_supply_temperature_mixed_air_limit_state_is_consistent_for_test,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release;
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_temperature_minimum_limit
        .latest
    else {
        return false;
    };
    let active_lineage_is_exact = if snapshot.supply_temperature_mixed_air_limit_executed {
        let Some(mixed_air) = unit.calc_cooling_mixed_air_call.latest else {
            return false;
        };
        active_operands_link_to_retained_prefix(
            predecessor,
            mixed_air,
            snapshot.supply_temperature_before_mixed_air_limit_c,
            snapshot.mixed_air_temperature_c,
        )
    } else {
        true
    };

    completed_direct_cooling_positive_supply_temperature_minimum_limit_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_positive_supply_temperature_minimum_limit_latest_witness(system.id),
    ) && mixed_air_limit_links_to_minimum_limit(snapshot, predecessor)
        && active_lineage_is_exact
        && completed_supply_temperature_mixed_air_limit_state_is_consistent(unit, snapshot, witness)
}

/// Fail-closed CP334 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError {
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
    CoolingPositiveSupplyTemperatureMinimumLimitSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingActiveOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    InvalidMixedAirTemperature {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_temperature_minimum_limit_transition_count: usize,
        cooling_positive_supply_temperature_mixed_air_limit_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP334 for the exact direct no-OA release route.
///
/// The left minimum operand is CP333's exact assigned result. The right
/// operand is the same-call CP329 mixed-air output, not a live Zone-state
/// reread. The source-shaped minimum intentionally selects its right operand
/// on equality and unordered comparisons.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp333: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError,
> {
    let selected = predecessor_cp333.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness =
        runtime.cooling_positive_supply_temperature_minimum_limit_latest_witness(selected);
    let limit_witness =
        runtime.cooling_positive_supply_temperature_mixed_air_limit_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
            InitializationNotReady { system: selected },
    )?;
    if predecessor_cp333.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_positive_supply_temperature_minimum_limit
            .latest
            .is_some_and(|latest| {
                minimum_limit_snapshots_match_bit_exact(latest, predecessor_cp333)
            })
        || !predecessor_witness.is_some_and(|witness| {
            minimum_limit_snapshots_match_bit_exact(witness, predecessor_cp333)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                CoolingPositiveSupplyTemperatureMinimumLimitSnapshotMismatch {
                    system: selected,
                },
        );
    }
    if !cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
        predecessor_cp333,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_supply_temperature_mixed_air_limit_state_is_consistent(
            unit,
            predecessor_cp333,
            limit_witness,
        )
        || !next_supply_temperature_mixed_air_limit_transition_fits(unit, predecessor_cp333)
        || !completed_direct_cooling_positive_supply_temperature_minimum_limit_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp333,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp333)
        || predecessor_cp333.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input = if predecessor_cp333.supply_temperature_minimum_limit_executed {
        let supply_temperature_before_mixed_air_limit_c = predecessor_cp333
            .assigned_supply_temperature_c
            .ok_or(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let mixed_air = unit.calc_cooling_mixed_air_call.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        let mixed_air_temperature_c = mixed_air.mixed_air_temperature_c.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        if !mixed_air_temperature_c.is_finite() {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                    InvalidMixedAirTemperature { system: selected },
            );
        }
        if !active_operands_link_to_retained_prefix(
            predecessor_cp333,
            mixed_air,
            Some(supply_temperature_before_mixed_air_limit_c),
            Some(mixed_air_temperature_c),
        ) {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            );
        }
        Some(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitActiveInput {
                supply_temperature_before_mixed_air_limit_c,
                mixed_air_temperature_c,
            },
        )
    } else {
        None
    };

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_positive_supply_temperature_mixed_air_limit_state(
            &mut unit.calc_cooling_positive_supply_temperature_mixed_air_limit,
            predecessor_cp333,
            active_input,
        )
    };
    runtime
        .set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_positive_supply_temperature_mixed_air_limit_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError {
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_positive_supply_temperature_minimum_limit_transition_count: unit
            .calc_cooling_positive_supply_temperature_minimum_limit
            .transition_count,
        cooling_positive_supply_temperature_mixed_air_limit_transition_count: unit
            .calc_cooling_positive_supply_temperature_mixed_air_limit
            .transition_count,
    }
}
