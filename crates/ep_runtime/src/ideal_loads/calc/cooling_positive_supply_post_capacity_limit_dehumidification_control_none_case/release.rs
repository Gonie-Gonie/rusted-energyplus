//! Release-bound CP347 dehumidification-control `None` case.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_switch::completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_input_from_owner, humidity_ratio_lineage_is_exact, none_case_links_to_predecessor,
    owner_lineage_is_exact, predecessor_selects_none_case, predecessor_snapshots_match_bit_exact,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release;

#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::next_transition_fits as next_transition_fits_for_test;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    >,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            system.id,
        );
    let active = predecessor_selects_none_case(predecessor);
    let owner = active
        .then_some(unit.calc_cooling_mixed_air_call.latest)
        .flatten();
    let owner_witness = active
        .then(|| runtime.cooling_mixed_air_call_latest_witness(system.id))
        .flatten();
    let cp345 = active
        .then_some(
            unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
                .latest,
        )
        .flatten();
    let cp345_witness = active
        .then(|| {
            runtime
                .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
                    system.id,
                )
        })
        .flatten();
    let owner_complete = match (active, owner, owner_witness) {
        (false, None, None) => true,
        (true, Some(owner), Some(owner_witness)) => {
            completed_direct_cooling_mixed_air_call_is_consistent(
                runtime,
                unit,
                system,
                owner,
                Some(owner_witness),
            )
        }
        _ => false,
    };

    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_witness.is_some_and(|predecessor_witness| {
            predecessor_snapshots_match_bit_exact(predecessor, predecessor_witness)
        })
        && cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            predecessor_witness,
        )
        && owner_lineage_is_exact(predecessor, owner, owner_witness)
        && owner_complete
        && humidity_ratio_lineage_is_exact(
            predecessor,
            owner,
            cp345,
            cp345_witness,
        )
        && none_case_links_to_predecessor(snapshot, predecessor)
        && completed_state_is_consistent(
            unit,
            snapshot,
            witness,
            system.dehumidification_control_type,
        )
}

/// Fail-closed CP347 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError
{
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
    CoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    MixedAirHumidityRatioOwnerLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    HumidityRatioLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_transition_count:
            usize,
        cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP347 for the exact direct no-OA release route.
///
/// CP346 is the immediate source-order predecessor. On its selected `None`
/// route the right-hand side is solely same-call CP329 `MixedAirHumRat`;
/// CP345 and CP346 retain bit-exact corroboration of that owner value.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp346:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError,
> {
    let selected = predecessor_cp346.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
        .latest;
    let predecessor_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            selected,
        );
    let none_case_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::DehumidificationControlTypeOutsideDirectSubset {
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
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp346.controlled_zone != controlled_zone
        || !predecessor_snapshots_match_bit_exact(retained_predecessor, predecessor_cp346)
        || !predecessor_witness.is_some_and(|witness| {
            predecessor_snapshots_match_bit_exact(witness, predecessor_cp346)
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(
        predecessor_cp346,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }

    let active = predecessor_selects_none_case(retained_predecessor);
    let owner = active
        .then_some(unit.calc_cooling_mixed_air_call.latest)
        .flatten();
    let owner_witness = active
        .then(|| runtime.cooling_mixed_air_call_latest_witness(selected))
        .flatten();
    if !owner_lineage_is_exact(retained_predecessor, owner, owner_witness) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::MixedAirHumidityRatioOwnerLineageMismatch {
                system: selected,
            },
        );
    }
    let owner_complete = match (active, owner, owner_witness) {
        (false, None, None) => true,
        (true, Some(owner), Some(owner_witness)) => {
            completed_direct_cooling_mixed_air_call_is_consistent(
                runtime,
                unit,
                system,
                owner,
                Some(owner_witness),
            )
        }
        _ => false,
    };
    if !owner_complete {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::MixedAirHumidityRatioOwnerLineageMismatch {
                system: selected,
            },
        );
    }
    let cp345 = active
        .then_some(
            unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
                .latest,
        )
        .flatten();
    let cp345_witness = active
        .then(|| {
            runtime
                .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
                    selected,
                )
        })
        .flatten();
    if !humidity_ratio_lineage_is_exact(retained_predecessor, owner, cp345, cp345_witness) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::HumidityRatioLineageMismatch {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            none_case_witness,
            system.dehumidification_control_type,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    let active_input = active_input_from_owner(retained_predecessor, owner);
    if active != active_input.is_some()
        || !next_transition_fits(unit, retained_predecessor, active_input)
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state(
            &mut unit
                .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
            retained_predecessor,
            active_input,
        )
    }
    .ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::RuntimeStateInvariantViolation {
            system: selected,
        },
    )?;
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError {
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::
        CoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshotMismatch {
            system,
        }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError {
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_transition_count:
                unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
                    .transition_count,
            cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_transition_count:
                unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
                    .transition_count,
        }
}
