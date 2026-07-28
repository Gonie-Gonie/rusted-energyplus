//! Release-bound CP332 Cooling positive-supply temperature assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, ZoneId};

use super::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    advance_cooling_positive_supply_temperature_assignment_state,
};
use crate::heat_balance::state::ZoneHeatBalanceState;
use crate::ideal_loads::calc::cooling_positive_supply_cp_air_assignment::completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
    cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release,
};

mod prefix_validation;
mod runtime_validation;
mod snapshot_validation;

#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use prefix_validation::active_operands_link_to_retained_prefix as active_operands_link_to_retained_prefix_for_test;
use prefix_validation::{
    active_operands_link_to_retained_prefix, assigned_operands_match_sources,
    cp_air_assignment_snapshots_match_bit_exact, temperature_assignment_links_to_cp_air_assignment,
};
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use runtime_validation::next_supply_temperature_assignment_transition_fits as next_supply_temperature_assignment_transition_fits_for_test;
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use runtime_validation::pending_supply_temperature_assignment_state_is_consistent as pending_supply_temperature_assignment_state_is_consistent_for_test;
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending,
    completed_supply_temperature_assignment_state_is_consistent,
    next_supply_temperature_assignment_transition_fits,
    pending_supply_temperature_assignment_state_is_consistent,
};
pub(in crate::ideal_loads) use snapshot_validation::cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release;
#[cfg(test)]
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use snapshot_validation::snapshots_match_bit_exact;

pub(in crate::ideal_loads::calc) fn completed_direct_cooling_positive_supply_temperature_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot>,
) -> bool {
    let Some(predecessor) = unit.calc_cooling_positive_supply_cp_air_assignment.latest else {
        return false;
    };
    let Some(entry) = unit.calc_entry.latest else {
        return false;
    };
    let Some(sensible_flow) = unit.calc_cooling_sensible_flow.latest else {
        return false;
    };
    let Some(mixed_air) = unit.calc_cooling_mixed_air_call.latest else {
        return false;
    };
    let Some(positive_guard) = unit.calc_cooling_supply_mass_flow_positive_guard.latest else {
        return false;
    };
    let active_lineage_is_exact = if snapshot.supply_temperature_assignment_executed {
        let Some(zone_node_temperature_c) = snapshot.zone_node_temperature_c else {
            return false;
        };
        active_operands_link_to_retained_prefix(
            entry,
            sensible_flow,
            mixed_air,
            positive_guard,
            predecessor,
            zone_node_temperature_c,
        ) && assigned_operands_match_sources(snapshot, entry, positive_guard, predecessor)
    } else {
        true
    };

    completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_positive_supply_cp_air_assignment_latest_witness(system.id),
    ) && temperature_assignment_links_to_cp_air_assignment(snapshot, predecessor)
        && active_lineage_is_exact
        && completed_supply_temperature_assignment_state_is_consistent(unit, snapshot, witness)
}

/// Active CP332 release operand rejected before mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput {
    /// Retained CP310 `QZnCoolSP`.
    ZoneCoolingSetpointLoad,
    /// Retained CP331 local `CpAir`.
    CpAir,
    /// Retained CP330 `SupplyMassFlowRate`.
    SupplyMassFlowRate,
    /// Live controlled Zone-node temperature.
    ZoneNodeTemperature,
}

/// Fail-closed CP332 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError {
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
    ZoneIdentityMismatch {
        expected: ZoneId,
        actual: ZoneId,
    },
    SystemOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    CoolingPositiveSupplyCpAirAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    CoolingActiveOperandLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        cooling_positive_supply_cp_air_assignment_transition_count: usize,
        cooling_positive_supply_temperature_assignment_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    InvalidActiveInput {
        system: IdealLoadsAirSystemId,
        input: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP332 for the exact direct no-OA release route.
///
/// The load, `CpAir`, and flow operands come from retained same-call stages.
/// Only the Zone-node temperature is read live, and CP318/CP329 values are
/// checked solely as bit-exact lineage evidence. This stage retains the raw
/// product, quotient, sum, and assignment without applying line 2187's clamp.
pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp331: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    zone_state: &ZoneHeatBalanceState,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError,
> {
    let selected = predecessor_cp331.system;
    let unit = runtime.units.get(&selected).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::UnknownSystem {
            system: selected,
        },
    )?;
    let predecessor_witness =
        runtime.cooling_positive_supply_cp_air_assignment_latest_witness(selected);
    let assignment_witness =
        runtime.cooling_positive_supply_temperature_assignment_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                SystemIdentityMismatch {
                    expected: selected,
                    actual: system.id,
                },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                SystemOutsideDirectSubset { system: selected },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                InitializationNotReady { system: selected },
        );
    }
    let controlled_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::InitializationNotReady {
            system: selected,
        },
    )?;
    if zone_state.zone_id != controlled_zone {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::ZoneIdentityMismatch {
                expected: controlled_zone,
                actual: zone_state.zone_id,
            },
        );
    }
    if predecessor_cp331.controlled_zone != controlled_zone
        || !unit
            .calc_cooling_positive_supply_cp_air_assignment
            .latest
            .is_some_and(|latest| {
                cp_air_assignment_snapshots_match_bit_exact(latest, predecessor_cp331)
            })
        || !predecessor_witness.is_some_and(|witness| {
            cp_air_assignment_snapshots_match_bit_exact(witness, predecessor_cp331)
        })
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                CoolingPositiveSupplyCpAirAssignmentSnapshotMismatch { system: selected },
        );
    }
    if !cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(
        predecessor_cp331,
    ) {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                PredecessorOutsideDirectSubset { system: selected },
        );
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_supply_temperature_assignment_state_is_consistent(
            unit,
            predecessor_cp331,
            assignment_witness,
        )
        || !next_supply_temperature_assignment_transition_fits(unit, predecessor_cp331)
        || !completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor_cp331,
            predecessor_witness,
        )
    {
        return Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp331)
        || predecessor_cp331.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let active_input = if predecessor_cp331.cp_air_assignment_executed {
        let entry = unit.calc_entry.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                RuntimeStateInvariantViolation { system: selected },
        )?;
        let sensible_flow = unit.calc_cooling_sensible_flow.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        let mixed_air = unit.calc_cooling_mixed_air_call.latest.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                CoolingActiveOperandLineageMismatch { system: selected },
        )?;
        let positive_guard = unit
            .calc_cooling_supply_mass_flow_positive_guard
            .latest
            .ok_or(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            )?;
        let zone_cooling_setpoint_load_w = entry.demand.remaining_output_req_to_cool_sp_w;
        let cp_air_j_per_kg_k = predecessor_cp331.cp_air_j_per_kg_k.ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::InvalidActiveInput {
                system: selected,
                input: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput::CpAir,
            },
        )?;
        let supply_mass_flow_rate_kg_per_s =
            positive_guard.supply_mass_flow_rate_kg_per_s.ok_or(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    InvalidActiveInput {
                        system: selected,
                        input: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput::
                            SupplyMassFlowRate,
                    },
            )?;
        let zone_node_temperature_c = zone_state.mean_air_temperature_c;

        if !zone_cooling_setpoint_load_w.is_finite() {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    InvalidActiveInput {
                        system: selected,
                        input: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput::
                            ZoneCoolingSetpointLoad,
                    },
            );
        }
        if !cp_air_j_per_kg_k.is_finite() || cp_air_j_per_kg_k <= 0.0 {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    InvalidActiveInput {
                        system: selected,
                        input: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput::
                            CpAir,
                    },
            );
        }
        if supply_mass_flow_rate_kg_per_s <= 0.0 || supply_mass_flow_rate_kg_per_s.is_nan() {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    InvalidActiveInput {
                        system: selected,
                        input: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput::
                            SupplyMassFlowRate,
                    },
            );
        }
        if !zone_node_temperature_c.is_finite() {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    InvalidActiveInput {
                        system: selected,
                        input: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput::
                            ZoneNodeTemperature,
                    },
            );
        }
        if !active_operands_link_to_retained_prefix(
            entry,
            sensible_flow,
            mixed_air,
            positive_guard,
            predecessor_cp331,
            zone_node_temperature_c,
        ) {
            return Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    CoolingActiveOperandLineageMismatch { system: selected },
            );
        }
        Some(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput {
                zone_cooling_setpoint_load_w,
                cp_air_j_per_kg_k,
                supply_mass_flow_rate_kg_per_s,
                zone_node_temperature_c,
            },
        )
    } else {
        None
    };

    let snapshot = {
        let unit = runtime.units.get_mut(&selected).ok_or(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::UnknownSystem {
                system: selected,
            },
        )?;
        advance_cooling_positive_supply_temperature_assignment_state(
            &mut unit.calc_cooling_positive_supply_temperature_assignment,
            predecessor_cp331,
            active_input,
        )
    };
    runtime.set_cooling_positive_supply_temperature_assignment_latest_witness(selected, snapshot);
    debug_assert!(
        cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(snapshot)
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_positive_supply_temperature_assignment_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_positive_supply_temperature_assignment_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError {
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        cooling_positive_supply_cp_air_assignment_transition_count: unit
            .calc_cooling_positive_supply_cp_air_assignment
            .transition_count,
        cooling_positive_supply_temperature_assignment_transition_count: unit
            .calc_cooling_positive_supply_temperature_assignment
            .transition_count,
    }
}
