//! Release-bound validation for the CP313 cooling OA/max-flow gate.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit};

use super::{PurchasedAirCalcCoolingOaMaxFlowGateSnapshot, advance_cooling_oa_max_flow_gate_state};
use crate::ideal_loads::calc::cooling_entry_gate::{
    PurchasedAirCalcCoolingEntryGateSnapshot, cooling_entry_gate_snapshots_bitwise_equal,
};
use crate::ideal_loads::calc::minimum_oa_prefix::PurchasedAirCalcMinimumOaPrefixSnapshot;
use crate::ideal_loads::{
    PurchasedAirInitSnapshot, PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
};

mod validation;

use validation::*;

/// Fail-closed error before the bounded gate mutates CP313 state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingOaMaxFlowGateError {
    /// The selected unit is absent from the persistent arena.
    UnknownSystem {
        /// Missing typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The selected unit has not completed bounded initialization.
    InitializationNotReady {
        /// Unready typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The model object and predecessor snapshots disagree.
    SystemIdentityMismatch {
        /// Runtime-selected system.
        expected: IdealLoadsAirSystemId,
        /// Caller-supplied model system.
        actual: IdealLoadsAirSystemId,
    },
    /// The supplied same-step initialization snapshot has stale identities.
    InitializationSnapshotIdentityMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The supplied and retained maximum cooling mass-flow caches differ bitwise.
    InitializationMaximumCoolingMassFlowCacheMismatch {
        /// Runtime-retained cache.
        expected: f64,
        /// Caller-supplied initialization cache.
        actual: f64,
    },
    /// A maximum cooling mass-flow cache was nonfinite or negative.
    InvalidMaximumCoolingMassFlowCache {
        /// Rejected cache value.
        value: f64,
    },
    /// The supplied CP311 snapshot is not the retained latest snapshot.
    MinimumOaPrefixSnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The supplied CP312 snapshot is not the retained latest snapshot.
    CoolingEntryGateSnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310, CP311, and CP312 do not describe one source invocation.
    PredecessorLinkMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310 through CP313 are not in one-for-one source order.
    PredecessorCallOrder {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
        /// Completed initialization calls.
        init_call_count: usize,
        /// Completed CP310 calls.
        calculation_entry_call_count: usize,
        /// Completed CP311 transitions.
        minimum_oa_prefix_transition_count: usize,
        /// Completed CP312 transitions.
        cooling_entry_gate_transition_count: usize,
        /// Completed CP313 transitions.
        cooling_oa_max_flow_gate_transition_count: usize,
    },
    /// The current typed system lies outside the direct no-OA sensible subset.
    SystemOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The CP311 snapshot lies outside the direct no-OA/no-EMS shape.
    MinimumOaPrefixOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The CP312 snapshot lies outside the exact DualHeatCool release shape.
    CoolingEntryGateOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The retained CP313 counters violate their exact partitions.
    RuntimeStateInvariantViolation {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The direct no-OA route unexpectedly entered the excluded line-2058 body.
    ExactReleaseReductionViolated {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP313 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_oa_max_flow_gate(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    initialization: PurchasedAirInitSnapshot,
    minimum_oa_prefix: PurchasedAirCalcMinimumOaPrefixSnapshot,
    cooling_entry_gate: PurchasedAirCalcCoolingEntryGateSnapshot,
) -> Result<PurchasedAirCalcCoolingOaMaxFlowGateSnapshot, PurchasedAirCalcCoolingOaMaxFlowGateError>
{
    let selected = cooling_entry_gate.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(PurchasedAirCalcCoolingOaMaxFlowGateError::UnknownSystem { system: selected })?;
    if !unit.topology_completed || unit.topology_failure.is_some() {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::InitializationNotReady { system: selected },
        );
    }
    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }

    let initialization_ready = initialization.system == selected
        && unit.system == selected
        && unit.calc_entry.system == selected
        && unit.calc_minimum_oa_prefix.system == selected
        && unit.calc_cooling_entry_gate.system == selected
        && unit.calc_cooling_oa_max_flow_gate.system == selected
        && unit.controlled_zone == Some(initialization.controlled_zone)
        && unit.supply_node == Some(initialization.supply_node)
        && unit.recirculation_node == initialization.recirculation_node
        && initialization.flags == unit.flags(runtime.equipment_list_checked)
        && initialization.flags.state_machine_used
        && initialization.flags.one_time_checked
        && initialization.flags.topology_ready
        && initialization.flags.environment_initialized
        && initialization.flags.sizing_checked
        && initialization.flags.equipment_list_checked
        && initialization.flags.return_plenum_inactive;
    if !initialization_ready {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::InitializationSnapshotIdentityMismatch {
                system: selected,
            },
        );
    }

    let supplied_maximum = initialization.maximum_cooling_air_mass_flow_rate_kg_per_s;
    let retained_maximum = unit.maximum_cooling_air_mass_flow_rate_kg_per_s;
    if !supplied_maximum.is_finite() || supplied_maximum < 0.0 {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::InvalidMaximumCoolingMassFlowCache {
                value: supplied_maximum,
            },
        );
    }
    if !retained_maximum.is_finite() || retained_maximum < 0.0 {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::InvalidMaximumCoolingMassFlowCache {
                value: retained_maximum,
            },
        );
    }
    if supplied_maximum.to_bits() != retained_maximum.to_bits() {
        return Err(PurchasedAirCalcCoolingOaMaxFlowGateError::
            InitializationMaximumCoolingMassFlowCacheMismatch {
                expected: retained_maximum,
                actual: supplied_maximum,
            });
    }

    if !unit
        .calc_minimum_oa_prefix
        .latest
        .is_some_and(|latest| minimum_oa_snapshots_bitwise_equal(latest, minimum_oa_prefix))
    {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::MinimumOaPrefixSnapshotMismatch {
                system: selected,
            },
        );
    }
    if !unit.calc_cooling_entry_gate.latest.is_some_and(|latest| {
        cooling_entry_gate_snapshots_bitwise_equal(latest, cooling_entry_gate)
    }) {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::CoolingEntryGateSnapshotMismatch {
                system: selected,
            },
        );
    }
    let calculation_entry = unit.calc_entry.latest.ok_or(
        PurchasedAirCalcCoolingOaMaxFlowGateError::PredecessorLinkMismatch { system: selected },
    )?;
    let linked = minimum_oa_prefix.system == selected
        && minimum_oa_prefix.parent_call_ordinal == calculation_entry.call_ordinal
        && minimum_oa_prefix.controlled_zone == calculation_entry.controlled_zone
        && minimum_oa_prefix.unit_body_entered == calculation_entry.unit_body_entered
        && cooling_entry_gate.parent_call_ordinal == minimum_oa_prefix.parent_call_ordinal
        && cooling_entry_gate.controlled_zone == minimum_oa_prefix.controlled_zone
        && cooling_entry_gate.unit_body_entered == minimum_oa_prefix.unit_body_entered
        && option_f64_bits_equal(
            cooling_entry_gate.minimum_outdoor_air_sensible_output_w,
            minimum_oa_prefix.minimum_outdoor_air_sensible_output_w,
        )
        && if calculation_entry.unit_body_entered {
            option_f64_has_bits(
                cooling_entry_gate.cooling_setpoint_demand_w,
                calculation_entry.demand.remaining_output_req_to_cool_sp_w,
            )
        } else {
            cooling_entry_gate.cooling_setpoint_demand_w.is_none()
        }
        && unit.controlled_zone == Some(calculation_entry.controlled_zone);
    if !linked {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::PredecessorLinkMismatch { system: selected },
        );
    }

    let ordered = unit.init_call_count == unit.calc_entry.call_count
        && unit.calc_entry.call_count == unit.calc_minimum_oa_prefix.transition_count
        && unit.calc_minimum_oa_prefix.transition_count
            == unit.calc_cooling_entry_gate.transition_count
        && unit
            .calc_cooling_oa_max_flow_gate
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_entry_gate.transition_count)
        && calculation_entry.call_ordinal == unit.calc_entry.call_count
        && minimum_oa_prefix.parent_call_ordinal == unit.calc_minimum_oa_prefix.transition_count
        && cooling_entry_gate.parent_call_ordinal == unit.calc_cooling_entry_gate.transition_count;
    if !ordered {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::PredecessorCallOrder {
                system: selected,
                init_call_count: unit.init_call_count,
                calculation_entry_call_count: unit.calc_entry.call_count,
                minimum_oa_prefix_transition_count: unit.calc_minimum_oa_prefix.transition_count,
                cooling_entry_gate_transition_count: unit.calc_cooling_entry_gate.transition_count,
                cooling_oa_max_flow_gate_transition_count: unit
                    .calc_cooling_oa_max_flow_gate
                    .transition_count,
            },
        );
    }
    if !minimum_oa_snapshot_is_direct_release(minimum_oa_prefix) {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::MinimumOaPrefixOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !cooling_entry_snapshot_is_direct_release(
        cooling_entry_gate,
        calculation_entry.demand.remaining_output_req_to_cool_sp_w,
    ) {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::CoolingEntryGateOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !cooling_oa_max_flow_runtime_state_is_consistent(
        &unit.calc_cooling_oa_max_flow_gate,
        system.cooling_limit,
    ) {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let outdoor_air_mass_flow_rate_kg_per_s = if cooling_entry_gate.cooling_body_entered {
        minimum_oa_prefix
            .working_outdoor_air_mass_flow_rate_kg_per_s
            .ok_or(
                PurchasedAirCalcCoolingOaMaxFlowGateError::MinimumOaPrefixOutsideDirectSubset {
                    system: selected,
                },
            )?
    } else {
        0.0
    };
    let would_enter_excluded_body = cooling_entry_gate.cooling_body_entered
        && matches!(
            system.cooling_limit,
            IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
        )
        && outdoor_air_mass_flow_rate_kg_per_s > retained_maximum;
    if would_enter_excluded_body {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowGateError::ExactReleaseReductionViolated {
                system: selected,
            },
        );
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(PurchasedAirCalcCoolingOaMaxFlowGateError::UnknownSystem { system: selected })?;
    let snapshot = advance_cooling_oa_max_flow_gate_state(
        &mut unit.calc_cooling_oa_max_flow_gate,
        cooling_entry_gate,
        system.cooling_limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        retained_maximum,
    );
    debug_assert!(!snapshot.maximum_cooling_flow_body_entered);
    Ok(snapshot)
}
