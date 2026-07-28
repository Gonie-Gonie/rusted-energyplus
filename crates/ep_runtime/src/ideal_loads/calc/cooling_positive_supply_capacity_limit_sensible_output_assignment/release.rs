//! Release-bound CP339 Cooling capacity-limit sensible-output assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    advance_cooling_positive_supply_capacity_limit_sensible_output_assignment_state,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_cp_air_assignment::completed_direct_cooling_positive_supply_capacity_limit_cp_air_assignment_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_enthalpy_assignment::completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent;
use crate::ideal_loads::calc::cooling_supply_mass_flow_positive_guard::completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release,
    cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

use prefix_validation::{
    active_operands_link_to_retained_prefix, cp_air_assignment_snapshots_match_bit_exact,
    sensible_output_assignment_links_to_cp_air_assignment,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_capacity_limit_sensible_output_assignment_state_is_consistent,
    next_capacity_limit_sensible_output_assignment_transition_fits,
    pending_capacity_limit_sensible_output_assignment_state_is_consistent,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::{
    next_capacity_limit_sensible_output_assignment_transition_fits as next_capacity_limit_sensible_output_assignment_transition_fits_for_test,
    pending_capacity_limit_sensible_output_assignment_state_is_consistent as pending_capacity_limit_sensible_output_assignment_state_is_consistent_for_test,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    >,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
        .latest
    else {
        return false;
    };
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(system.id);
    let active_lineage_is_exact =
        if snapshot.capacity_limit_sensible_output_assignment_executed {
            let Some(positive_guard) =
                unit.calc_cooling_supply_mass_flow_positive_guard.latest
            else {
                return false;
            };
            let Some(positive_guard_witness) =
                runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id)
            else {
                return false;
            };
            let Some(mixed_air) = unit.calc_cooling_mixed_air_call.latest else {
                return false;
            };
            let Some(mixed_air_witness) =
                runtime.cooling_mixed_air_call_latest_witness(system.id)
            else {
                return false;
            };
            let Some(supply_enthalpy) =
                unit.calc_cooling_positive_supply_enthalpy_assignment.latest
            else {
                return false;
            };
            let Some(supply_enthalpy_witness) = runtime
                .cooling_positive_supply_enthalpy_assignment_latest_witness(system.id)
            else {
                return false;
            };
            active_operands_link_to_retained_prefix(
                predecessor,
                positive_guard,
                positive_guard_witness,
                mixed_air,
                mixed_air_witness,
                supply_enthalpy,
                supply_enthalpy_witness,
                snapshot.supply_mass_flow_rate_kg_per_s,
                snapshot.mixed_air_enthalpy_j_per_kg,
                snapshot.supply_enthalpy_j_per_kg,
            ) && cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(
                positive_guard,
            ) && completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
                runtime,
                unit,
                system,
                positive_guard,
                Some(positive_guard_witness),
            ) && cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air)
                && completed_direct_cooling_mixed_air_call_is_consistent(
                    runtime,
                    unit,
                    system,
                    mixed_air,
                    Some(mixed_air_witness),
                )
                && cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
                    supply_enthalpy,
                )
                && completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
                    runtime,
                    unit,
                    system,
                    supply_enthalpy,
                    Some(supply_enthalpy_witness),
                )
        } else {
            true
        };

    cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
        predecessor,
    ) && completed_direct_cooling_positive_supply_capacity_limit_cp_air_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        predecessor_witness,
    ) && sensible_output_assignment_links_to_cp_air_assignment(snapshot, predecessor)
        && active_lineage_is_exact
        && completed_capacity_limit_sensible_output_assignment_state_is_consistent(
            unit, snapshot, witness,
        )
}

/// Active CP339 retained operand rejected before mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentInput {
    /// Retained CP330 supply mass flow rate.
    SupplyMassFlowRate,
    /// Retained CP329 mixed-air enthalpy projection.
    MixedAirEnthalpy,
    /// Retained CP336 supply enthalpy.
    SupplyEnthalpy,
}

/// Fail-closed CP339 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError {
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
    CoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingActiveOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_capacity_limit_cp_air_assignment_transition_count: usize,
        cooling_positive_supply_capacity_limit_sensible_output_assignment_transition_count:
            usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    InvalidActiveInput {
        system: IdealLoadsAirSystemId,
        input:
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentInput,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP339 for the exact direct no-OA release route.
///
/// The active operands come only from the retained same-call CP330 supply
/// mass flow, CP329 mixed-air enthalpy projection, and CP336 supply enthalpy.
/// CP338's retained `CpAir` is lineage only and is not a scalar operand.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp338:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError,
> {
    let selected = predecessor_cp338.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness = runtime
        .cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness(selected);
    let assignment_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
            selected,
        );

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
            InitializationNotReady { system: selected },
    )?;
    if predecessor_cp338.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
            .latest
            .is_some_and(|latest| {
                cp_air_assignment_snapshots_match_bit_exact(latest, predecessor_cp338)
            })
        || !predecessor_witness.is_some_and(|witness| {
            cp_air_assignment_snapshots_match_bit_exact(witness, predecessor_cp338)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                CoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshotMismatch {
                    system: selected,
                },
        );
    }
    if !cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
        predecessor_cp338,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_capacity_limit_sensible_output_assignment_state_is_consistent(
            unit,
            predecessor_cp338,
            assignment_witness,
        )
        || !next_capacity_limit_sensible_output_assignment_transition_fits(
            unit,
            predecessor_cp338,
        )
        || !completed_direct_cooling_positive_supply_capacity_limit_cp_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp338,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp338)
        || predecessor_cp338.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input = if predecessor_cp338.capacity_limit_cp_air_assignment_executed {
        let positive_guard = unit
            .calc_cooling_supply_mass_flow_positive_guard
            .latest
            .ok_or(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let positive_guard_witness = runtime
            .cooling_supply_mass_flow_positive_guard_latest_witness(selected)
            .ok_or(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let mixed_air = unit.calc_cooling_mixed_air_call.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        let mixed_air_witness = runtime
            .cooling_mixed_air_call_latest_witness(selected)
            .ok_or(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let supply_enthalpy = unit
            .calc_cooling_positive_supply_enthalpy_assignment
            .latest
            .ok_or(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let supply_enthalpy_witness = runtime
            .cooling_positive_supply_enthalpy_assignment_latest_witness(selected)
            .ok_or(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let supply_mass_flow_rate_kg_per_s =
            positive_guard.supply_mass_flow_rate_kg_per_s.ok_or(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let mixed_air_enthalpy_j_per_kg =
            mixed_air.mixed_air_enthalpy_projection_j_per_kg.ok_or(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let supply_enthalpy_j_per_kg = supply_enthalpy.supply_enthalpy_j_per_kg.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        if supply_mass_flow_rate_kg_per_s <= 0.0
            || supply_mass_flow_rate_kg_per_s.is_nan()
        {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                    InvalidActiveInput {
                        system: selected,
                        input:
                            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentInput::
                                SupplyMassFlowRate,
                    },
            );
        }
        for (value, input) in [
            (
                mixed_air_enthalpy_j_per_kg,
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentInput::
                    MixedAirEnthalpy,
            ),
            (
                supply_enthalpy_j_per_kg,
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentInput::
                    SupplyEnthalpy,
            ),
        ] {
            if !value.is_finite() {
                return Err(
                    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                        InvalidActiveInput {
                            system: selected,
                            input,
                        },
                );
            }
        }
        if !active_operands_link_to_retained_prefix(
            predecessor_cp338,
            positive_guard,
            positive_guard_witness,
            mixed_air,
            mixed_air_witness,
            supply_enthalpy,
            supply_enthalpy_witness,
            Some(supply_mass_flow_rate_kg_per_s),
            Some(mixed_air_enthalpy_j_per_kg),
            Some(supply_enthalpy_j_per_kg),
        ) || !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(
            positive_guard,
        ) || !completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
            runtime,
            unit,
            system,
            positive_guard,
            Some(positive_guard_witness),
        ) || !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air)
            || !completed_direct_cooling_mixed_air_call_is_consistent(
                runtime,
                unit,
                system,
                mixed_air,
                Some(mixed_air_witness),
            )
            || !cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
                supply_enthalpy,
            )
            || !completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
                runtime,
                unit,
                system,
                supply_enthalpy,
                Some(supply_enthalpy_witness),
            )
        {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            );
        }
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentActiveInput {
                supply_mass_flow_rate_kg_per_s,
                mixed_air_enthalpy_j_per_kg,
                supply_enthalpy_j_per_kg,
            },
        )
    } else {
        None
    };

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
                UnknownSystem { system: selected },
        )?;
        advance_cooling_positive_supply_capacity_limit_sensible_output_assignment_state(
            &mut unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment,
            predecessor_cp338,
            active_input,
        )
    };
    runtime
        .set_cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
            selected, snapshot,
        );
    debug_assert!(
        cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
                    selected,
                ),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError {
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::
        PredecessorCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calculation_entry_call_count: unit.calc_entry.call_count,
            cooling_positive_supply_capacity_limit_cp_air_assignment_transition_count: unit
                .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
                .transition_count,
            cooling_positive_supply_capacity_limit_sensible_output_assignment_transition_count:
                unit.calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
                    .transition_count,
        }
}
