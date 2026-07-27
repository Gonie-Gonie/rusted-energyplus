//! Release-bound validation for the CP312 cooling-entry gate.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingEntryGateSnapshot, PurchasedAirTemperatureControlType,
    advance_cooling_entry_gate_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcEntrySnapshot, PurchasedAirCalcMinimumOaPrefixSnapshot,
    PurchasedAirRuntimeState,
};

use crate::ideal_loads::calc::minimum_oa_prefix::calculation_entry_snapshots_bitwise_equal;

/// Predicate scalar rejected by the finite direct release boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingEntryGatePredicateInput {
    /// CP310 `QZnCoolSP`.
    CoolingSetpointDemand,
}

/// Fail-closed error before the bounded gate mutates CP312 state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingEntryGateError {
    /// The selected unit is absent from the persistent arena.
    UnknownSystem {
        /// Missing typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The selected unit has not completed bounded topology initialization.
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
    /// The supplied CP310 snapshot is not the retained latest snapshot.
    CalculationEntrySnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The supplied CP311 snapshot is not the retained latest snapshot.
    MinimumOaPrefixSnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The two predecessor snapshots do not describe the same source call.
    PredecessorLinkMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310, CP311, and CP312 are not in one-for-one source order.
    PredecessorCallOrder {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
        /// Completed CP310 calls.
        calculation_entry_call_count: usize,
        /// Completed CP311 transitions.
        minimum_oa_prefix_transition_count: usize,
        /// Completed CP312 transitions.
        cooling_entry_gate_transition_count: usize,
    },
    /// A predecessor snapshot lies outside the no-OA/no-EMS release shape.
    MinimumOaPrefixOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// Outdoor air lies outside the direct no-OA release subset.
    OutdoorAirOutsideDirectSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The caller did not supply the prevalidated DualHeatCool release type.
    TemperatureControlTypeOutsideDirectSubset {
        /// Rejected source control type.
        actual: PurchasedAirTemperatureControlType,
    },
    /// An active release predicate input was not finite.
    NonFinitePredicateInput {
        /// Rejected predicate scalar.
        input: PurchasedAirCalcCoolingEntryGatePredicateInput,
    },
}

/// Executes CP312 for the exact no-OA, no-EMS, DualHeatCool release route.
pub fn advance_direct_no_oa_calc_cooling_entry_gate(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    calculation_entry: PurchasedAirCalcEntrySnapshot,
    minimum_oa_prefix: PurchasedAirCalcMinimumOaPrefixSnapshot,
    temperature_control_type: PurchasedAirTemperatureControlType,
) -> Result<PurchasedAirCalcCoolingEntryGateSnapshot, PurchasedAirCalcCoolingEntryGateError> {
    let unit = runtime.units.get_mut(&calculation_entry.system).ok_or(
        PurchasedAirCalcCoolingEntryGateError::UnknownSystem {
            system: calculation_entry.system,
        },
    )?;
    if !unit.topology_completed || unit.topology_failure.is_some() {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::InitializationNotReady {
                system: calculation_entry.system,
            },
        );
    }
    if system.id != calculation_entry.system {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::SystemIdentityMismatch {
                expected: calculation_entry.system,
                actual: system.id,
            },
        );
    }
    if !unit
        .calc_entry
        .latest
        .is_some_and(|latest| calculation_entry_snapshots_bitwise_equal(latest, calculation_entry))
    {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::CalculationEntrySnapshotMismatch {
                system: calculation_entry.system,
            },
        );
    }
    if !unit
        .calc_minimum_oa_prefix
        .latest
        .is_some_and(|latest| minimum_oa_snapshots_bitwise_equal(latest, minimum_oa_prefix))
    {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::MinimumOaPrefixSnapshotMismatch {
                system: calculation_entry.system,
            },
        );
    }
    let predecessors_linked = minimum_oa_prefix.system == calculation_entry.system
        && minimum_oa_prefix.parent_call_ordinal == calculation_entry.call_ordinal
        && minimum_oa_prefix.controlled_zone == calculation_entry.controlled_zone
        && minimum_oa_prefix.unit_body_entered == calculation_entry.unit_body_entered
        && unit.controlled_zone == Some(calculation_entry.controlled_zone);
    if !predecessors_linked {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::PredecessorLinkMismatch {
                system: calculation_entry.system,
            },
        );
    }
    let call_order_ready = unit.calc_cooling_entry_gate.transition_count.checked_add(1)
        == Some(unit.calc_minimum_oa_prefix.transition_count)
        && unit.calc_minimum_oa_prefix.transition_count == unit.calc_entry.call_count
        && minimum_oa_prefix.parent_call_ordinal == unit.calc_minimum_oa_prefix.transition_count;
    if !call_order_ready {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::PredecessorCallOrder {
                system: calculation_entry.system,
                calculation_entry_call_count: unit.calc_entry.call_count,
                minimum_oa_prefix_transition_count: unit.calc_minimum_oa_prefix.transition_count,
                cooling_entry_gate_transition_count: unit.calc_cooling_entry_gate.transition_count,
            },
        );
    }
    if !minimum_oa_snapshot_is_direct_release(minimum_oa_prefix) {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::MinimumOaPrefixOutsideDirectSubset {
                system: calculation_entry.system,
            },
        );
    }
    let outdoor_air_absent = calculation_entry.outdoor_air_node.is_none()
        && system
            .design_specification_outdoor_air_object_name
            .is_none()
        && system.outdoor_air_inlet_node_name.is_none();
    if !outdoor_air_absent {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::OutdoorAirOutsideDirectSubset {
                system: calculation_entry.system,
            },
        );
    }
    if temperature_control_type != PurchasedAirTemperatureControlType::DualHeatCool {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::TemperatureControlTypeOutsideDirectSubset {
                actual: temperature_control_type,
            },
        );
    }
    if calculation_entry.unit_body_entered
        && !calculation_entry
            .demand
            .remaining_output_req_to_cool_sp_w
            .is_finite()
    {
        return Err(
            PurchasedAirCalcCoolingEntryGateError::NonFinitePredicateInput {
                input: PurchasedAirCalcCoolingEntryGatePredicateInput::CoolingSetpointDemand,
            },
        );
    }

    Ok(advance_cooling_entry_gate_state(
        &mut unit.calc_cooling_entry_gate,
        calculation_entry,
        minimum_oa_prefix,
        temperature_control_type,
    ))
}

fn minimum_oa_snapshot_is_direct_release(
    snapshot: PurchasedAirCalcMinimumOaPrefixSnapshot,
) -> bool {
    let common = !snapshot.ems_override_applied && snapshot.psychrometric_call_count == 0;
    if !common {
        return false;
    }
    if snapshot.unit_body_entered {
        snapshot.zone_heat_balance_reference_bound
            && snapshot.minimum_oa_child_called
            && snapshot.minimum_oa_child_no_outdoor_air_route
            && option_f64_has_bits(
                snapshot.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
                0.0,
            )
            && snapshot.retained_minimum_outdoor_air_write_performed
            && snapshot.ems_override_flag_read
            && snapshot.ems_override_enabled == Some(false)
            && option_f64_has_bits(snapshot.working_outdoor_air_mass_flow_rate_kg_per_s, 0.0)
            && snapshot.outdoor_air_flag_read
            && snapshot.outdoor_air_enabled == Some(false)
            && snapshot.no_outdoor_air_zero_branch_entered
            && option_f64_has_bits(snapshot.minimum_outdoor_air_sensible_output_w, 0.0)
            && option_f64_has_bits(snapshot.minimum_outdoor_air_moisture_output_kg_per_s, 0.0)
    } else {
        !snapshot.zone_heat_balance_reference_bound
            && !snapshot.minimum_oa_child_called
            && !snapshot.minimum_oa_child_no_outdoor_air_route
            && snapshot
                .retained_minimum_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.retained_minimum_outdoor_air_write_performed
            && !snapshot.ems_override_flag_read
            && snapshot.ems_override_enabled.is_none()
            && snapshot
                .working_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.outdoor_air_flag_read
            && snapshot.outdoor_air_enabled.is_none()
            && !snapshot.no_outdoor_air_zero_branch_entered
            && snapshot.minimum_outdoor_air_sensible_output_w.is_none()
            && snapshot
                .minimum_outdoor_air_moisture_output_kg_per_s
                .is_none()
    }
}

fn minimum_oa_snapshots_bitwise_equal(
    retained: PurchasedAirCalcMinimumOaPrefixSnapshot,
    supplied: PurchasedAirCalcMinimumOaPrefixSnapshot,
) -> bool {
    let floats_match = [
        (
            retained.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
            supplied.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
        ),
        (
            retained.working_outdoor_air_mass_flow_rate_kg_per_s,
            supplied.working_outdoor_air_mass_flow_rate_kg_per_s,
        ),
        (
            retained.minimum_outdoor_air_sensible_output_w,
            supplied.minimum_outdoor_air_sensible_output_w,
        ),
        (
            retained.minimum_outdoor_air_moisture_output_kg_per_s,
            supplied.minimum_outdoor_air_moisture_output_kg_per_s,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_f64_bits_equal(left, right));
    if !floats_match {
        return false;
    }
    let mut retained_without_floats = retained;
    let mut supplied_without_floats = supplied;
    retained_without_floats.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s = None;
    retained_without_floats.working_outdoor_air_mass_flow_rate_kg_per_s = None;
    retained_without_floats.minimum_outdoor_air_sensible_output_w = None;
    retained_without_floats.minimum_outdoor_air_moisture_output_kg_per_s = None;
    supplied_without_floats.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s = None;
    supplied_without_floats.working_outdoor_air_mass_flow_rate_kg_per_s = None;
    supplied_without_floats.minimum_outdoor_air_sensible_output_w = None;
    supplied_without_floats.minimum_outdoor_air_moisture_output_kg_per_s = None;
    retained_without_floats == supplied_without_floats
}

fn option_f64_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn option_f64_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
