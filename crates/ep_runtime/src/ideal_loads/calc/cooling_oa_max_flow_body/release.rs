//! Release-bound validation for the CP314 warning-and-clamp body.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{PurchasedAirCalcCoolingOaMaxFlowBodySnapshot, advance_cooling_oa_max_flow_body_state};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingOaMaxFlowGateSnapshot, PurchasedAirInitSnapshot,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
};

mod validation;

use validation::*;

/// Fail-closed error before the bounded body mutates CP314 state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingOaMaxFlowBodyError {
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
    /// The current model system disagrees with CP313.
    SystemIdentityMismatch {
        /// Runtime-selected system.
        expected: IdealLoadsAirSystemId,
        /// Caller-supplied model system.
        actual: IdealLoadsAirSystemId,
    },
    /// The supplied initialization identities or density are stale.
    InitializationSnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The supplied and retained maximum cooling mass-flow caches differ.
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
    /// The supplied CP313 snapshot is not the retained latest snapshot.
    CoolingOaMaxFlowGateSnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// Initialization and CP313 do not describe one source invocation.
    PredecessorLinkMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310 through CP314 are not in one-for-one source order.
    PredecessorCallOrder {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
        /// Completed initialization calls.
        init_call_count: usize,
        /// Completed CP313 transitions.
        cooling_oa_max_flow_gate_transition_count: usize,
        /// Completed CP314 transitions.
        cooling_oa_max_flow_body_transition_count: usize,
    },
    /// The current system lies outside the direct no-OA sensible subset.
    SystemOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP313 does not have the exact direct-release fallthrough shape.
    PredecessorOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// Retained CP314 state violates the exact direct-release partitions.
    RuntimeStateInvariantViolation {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP314 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_oa_max_flow_body(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    initialization: PurchasedAirInitSnapshot,
    predecessor: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
) -> Result<PurchasedAirCalcCoolingOaMaxFlowBodySnapshot, PurchasedAirCalcCoolingOaMaxFlowBodyError>
{
    let selected = predecessor.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(PurchasedAirCalcCoolingOaMaxFlowBodyError::UnknownSystem { system: selected })?;
    if !unit.topology_completed || unit.topology_failure.is_some() {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::InitializationNotReady { system: selected },
        );
    }
    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    let retained_density = unit.standard_air_density_kg_per_m3;
    let supplied_density = initialization.standard_air_density_kg_per_m3;
    let initialization_matches = initialization.system == selected
        && unit.system == selected
        && unit.calc_cooling_oa_max_flow_gate.system == selected
        && unit.calc_cooling_oa_max_flow_body.system == selected
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
        && initialization.flags.return_plenum_inactive
        && option_f64_bits_equal(retained_density, supplied_density)
        && supplied_density.is_some_and(|density| density.is_finite() && density > 0.0);
    if !initialization_matches {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::InitializationSnapshotMismatch {
                system: selected,
            },
        );
    }
    let supplied_maximum = initialization.maximum_cooling_air_mass_flow_rate_kg_per_s;
    let retained_maximum = unit.maximum_cooling_air_mass_flow_rate_kg_per_s;
    if !supplied_maximum.is_finite() || supplied_maximum < 0.0 {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::InvalidMaximumCoolingMassFlowCache {
                value: supplied_maximum,
            },
        );
    }
    if !retained_maximum.is_finite() || retained_maximum < 0.0 {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::InvalidMaximumCoolingMassFlowCache {
                value: retained_maximum,
            },
        );
    }
    if supplied_maximum.to_bits() != retained_maximum.to_bits() {
        return Err(PurchasedAirCalcCoolingOaMaxFlowBodyError::
            InitializationMaximumCoolingMassFlowCacheMismatch {
                expected: retained_maximum,
                actual: supplied_maximum,
            });
    }
    if !unit
        .calc_cooling_oa_max_flow_gate
        .latest
        .is_some_and(|latest| gate_snapshots_bitwise_equal(latest, predecessor))
    {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::CoolingOaMaxFlowGateSnapshotMismatch {
                system: selected,
            },
        );
    }
    let cooling_entry = unit.calc_cooling_entry_gate.latest.ok_or(
        PurchasedAirCalcCoolingOaMaxFlowBodyError::PredecessorLinkMismatch { system: selected },
    )?;
    let linked = predecessor.controlled_zone == initialization.controlled_zone
        && predecessor.parent_call_ordinal == unit.calc_cooling_oa_max_flow_gate.transition_count
        && predecessor.parent_call_ordinal == cooling_entry.parent_call_ordinal
        && predecessor.controlled_zone == cooling_entry.controlled_zone
        && predecessor.unit_body_entered == cooling_entry.unit_body_entered
        && predecessor.predecessor_cooling_body_entered == cooling_entry.cooling_body_entered;
    if !linked {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::PredecessorLinkMismatch { system: selected },
        );
    }
    let ordered = unit.init_call_count == unit.calc_entry.call_count
        && unit.calc_entry.call_count == unit.calc_minimum_oa_prefix.transition_count
        && unit.calc_minimum_oa_prefix.transition_count
            == unit.calc_cooling_entry_gate.transition_count
        && unit.calc_cooling_entry_gate.transition_count
            == unit.calc_cooling_oa_max_flow_gate.transition_count
        && unit
            .calc_cooling_oa_max_flow_body
            .transition_count
            .checked_add(1)
            == Some(unit.calc_cooling_oa_max_flow_gate.transition_count);
    if !ordered {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::PredecessorCallOrder {
                system: selected,
                init_call_count: unit.init_call_count,
                cooling_oa_max_flow_gate_transition_count: unit
                    .calc_cooling_oa_max_flow_gate
                    .transition_count,
                cooling_oa_max_flow_body_transition_count: unit
                    .calc_cooling_oa_max_flow_body
                    .transition_count,
            },
        );
    }
    if !predecessor_is_exact_direct_fallthrough(predecessor, system.cooling_limit, retained_maximum)
    {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !direct_runtime_states_are_consistent(
        &unit.calc_cooling_oa_max_flow_body,
        &unit.calc_cooling_oa_max_flow_gate,
        predecessor,
        system.cooling_limit,
    ) {
        return Err(
            PurchasedAirCalcCoolingOaMaxFlowBodyError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(PurchasedAirCalcCoolingOaMaxFlowBodyError::UnknownSystem { system: selected })?;
    let snapshot = advance_cooling_oa_max_flow_body_state(
        &mut unit.calc_cooling_oa_max_flow_body,
        predecessor,
        0.0,
        1.0,
        0.0,
        0.0,
    );
    debug_assert!(snapshot.body_skipped);
    debug_assert!(!snapshot.outdoor_air_mass_flow_rate_read);
    debug_assert!(!snapshot.standard_air_density_read);
    debug_assert!(!snapshot.outdoor_air_mass_flow_clamp_assignment_performed);
    Ok(snapshot)
}
