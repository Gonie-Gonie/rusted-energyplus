//! Release-bound CP346 dehumidification-control switch dispatch.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_cp319_corroborates_owner, active_input_from_owner, predecessor_is_active,
    predecessor_is_exact_direct, predecessor_snapshots_match_bit_exact,
    switch_links_to_predecessor,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release;

#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::next_transition_fits as next_transition_fits_for_test;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    >,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
        );
    let cp319 = predecessor_is_active(predecessor)
        .then_some(unit.calc_cooling_dehumidification_flow.latest)
        .flatten();
    let cp319_witness = predecessor_is_active(predecessor)
        .then(|| runtime.cooling_dehumidification_flow_latest_witness(system.id))
        .flatten();

    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            predecessor_snapshots_match_bit_exact(predecessor, predecessor_witness)
        })
        && predecessor_is_exact_direct(predecessor)
        && completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && active_cp319_corroborates_owner(
            unit,
            predecessor,
            system.dehumidification_control_type,
            cp319,
            cp319_witness,
        )
        && switch_links_to_predecessor(snapshot, predecessor)
        && completed_state_is_consistent(
            unit,
            snapshot,
            witness,
            system.dehumidification_control_type,
        )
}

/// Fail-closed CP346 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError {
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
    CoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    DehumidificationControlTypeLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_transition_count:
            usize,
        cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP346 for the exact direct no-OA release route.
///
/// CP345 is the immediate source-order predecessor and owns no selector
/// operand. The selector comes only from the immutable typed system retained
/// by the direct model binding. Same-call CP319 corroborates that value only
/// on CP346-active G/F/L routes; CP319 and CP346 aggregate reads are not equal
/// because the positive-guard-false route reads CP319 and skips CP346.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp345:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError,
> {
    let selected = predecessor_cp345.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest;
    let predecessor_witness = runtime
        .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            selected,
        );
    let switch_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::DehumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp345.controlled_zone != controlled_zone
        || !predecessor_snapshots_match_bit_exact(retained_predecessor, predecessor_cp345)
        || !predecessor_witness.is_some_and(|witness| {
            predecessor_snapshots_match_bit_exact(witness, predecessor_cp345)
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !predecessor_is_exact_direct(predecessor_cp345) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }

    let active = predecessor_is_active(retained_predecessor);
    let cp319 = active
        .then_some(unit.calc_cooling_dehumidification_flow.latest)
        .flatten();
    let cp319_witness = active
        .then(|| runtime.cooling_dehumidification_flow_latest_witness(selected))
        .flatten();
    if !active_cp319_corroborates_owner(
        unit,
        retained_predecessor,
        system.dehumidification_control_type,
        cp319,
        cp319_witness,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::DehumidificationControlTypeLineageMismatch {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            switch_witness,
            system.dehumidification_control_type,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let active_input = active_input_from_owner(retained_predecessor, system);
    if !next_transition_fits(unit, retained_predecessor, active_input) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state(
            &mut unit
                .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
            retained_predecessor,
            active_input,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError {
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::
        CoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshotMismatch {
            system,
        }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError {
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_transition_count:
                unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
                    .transition_count,
            cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_transition_count:
                unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
                    .transition_count,
        }
}
