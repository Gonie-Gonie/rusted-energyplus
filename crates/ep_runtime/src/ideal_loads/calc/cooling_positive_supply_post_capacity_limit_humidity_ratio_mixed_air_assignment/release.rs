//! Release-bound CP345 post-capacity-limit humidity-ratio assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    advance_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_state,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_humidity_ratio_mixed_air_assignment::completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_input_from_owner, assignment_links_to_predecessor, corroboration_lineage_is_exact,
    owner_lineage_is_exact, predecessor_is_active, predecessor_snapshots_match_bit_exact,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release;

#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::next_transition_fits as next_transition_fits_for_test;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    >,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
            system.id,
        );
    let active = predecessor_is_active(predecessor);
    let owner = active
        .then_some(unit.calc_cooling_mixed_air_call.latest)
        .flatten();
    let owner_witness = active
        .then(|| runtime.cooling_mixed_air_call_latest_witness(system.id))
        .flatten();
    let corroboration = active
        .then_some(
            unit.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
                .latest,
        )
        .flatten();
    let corroboration_witness = active
        .then(|| {
            runtime.cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
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
    let corroboration_complete =
        match (active, corroboration, corroboration_witness) {
            (false, None, None) => true,
            (true, Some(corroboration), Some(corroboration_witness)) => {
                completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent(
                    runtime,
                    unit,
                    system,
                    corroboration,
                    Some(corroboration_witness),
                )
            }
            _ => false,
        };

    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
        predecessor,
    ) && predecessor_witness.is_some_and(|predecessor_witness| {
        predecessor_snapshots_match_bit_exact(predecessor, predecessor_witness)
    }) && completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        predecessor_witness,
    ) && owner_lineage_is_exact(predecessor, owner, owner_witness)
        && owner_complete
        && corroboration_lineage_is_exact(
            predecessor,
            owner,
            corroboration,
            corroboration_witness,
        )
        && corroboration_complete
        && assignment_links_to_predecessor(snapshot, predecessor)
        && completed_state_is_consistent(unit, snapshot, witness)
}

/// Fail-closed CP345 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError
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
    CoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    MixedAirHumidityRatioOwnerLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    CorroboratingHumidityRatioAssignmentLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_transition_count:
            usize,
        cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP345 for the exact direct no-OA release route.
///
/// The right-hand side is solely the same-call CP329 latest/private
/// `MixedAirHumRat`. CP335 is same-call bit-exact corroboration, while CP344 is
/// the immediate source-order predecessor. CP345 adds no numerical gate.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp344:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError,
> {
    let selected = predecessor_cp344.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let retained_predecessor = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
        .latest;
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
            selected,
        );
    let assignment_witness = runtime
        .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::InitializationNotReady {
                system: selected,
            },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(predecessor_mismatch(selected));
    };
    if predecessor_cp344.controlled_zone != controlled_zone
        || !predecessor_snapshots_match_bit_exact(retained_predecessor, predecessor_cp344)
        || !predecessor_witness.is_some_and(|witness| {
            predecessor_snapshots_match_bit_exact(witness, predecessor_cp344)
        })
    {
        return Err(predecessor_mismatch(selected));
    }
    if !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
        predecessor_cp344,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }

    let active = predecessor_is_active(retained_predecessor);
    let owner = active
        .then_some(unit.calc_cooling_mixed_air_call.latest)
        .flatten();
    let owner_witness = active
        .then(|| runtime.cooling_mixed_air_call_latest_witness(selected))
        .flatten();
    if !owner_lineage_is_exact(retained_predecessor, owner, owner_witness) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::MixedAirHumidityRatioOwnerLineageMismatch {
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
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::MixedAirHumidityRatioOwnerLineageMismatch {
                system: selected,
            },
        );
    }
    let corroboration = active
        .then_some(
            unit.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
                .latest,
        )
        .flatten();
    let corroboration_witness = active
        .then(|| {
            runtime.cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
                selected,
            )
        })
        .flatten();
    if !corroboration_lineage_is_exact(
        retained_predecessor,
        owner,
        corroboration,
        corroboration_witness,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::CorroboratingHumidityRatioAssignmentLineageMismatch {
                system: selected,
            },
        );
    }
    let corroboration_complete =
        match (active, corroboration, corroboration_witness) {
            (false, None, None) => true,
            (true, Some(corroboration), Some(corroboration_witness)) => {
                completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent(
                    runtime,
                    unit,
                    system,
                    corroboration,
                    Some(corroboration_witness),
                )
            }
            _ => false,
        };
    if !corroboration_complete {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::CorroboratingHumidityRatioAssignmentLineageMismatch {
                system: selected,
            },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            assignment_witness,
        )
        || !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::RuntimeStateInvariantViolation {
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
    if !next_transition_fits(unit, retained_predecessor, active_input) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_state(
            &mut unit
                .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
            retained_predecessor,
            active_input,
        )
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            selected,
            snapshot,
        );
    debug_assert!(
        cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError {
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::
        CoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshotMismatch {
            system,
        }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError {
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
                    .transition_count,
            cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_transition_count:
                unit.calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
                    .transition_count,
        }
}
