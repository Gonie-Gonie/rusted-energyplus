//! Release-bound CP435 heating outdoor-air maximum-flow guard.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId, IdealLoadsLimit,
};

use super::transition::{
    advance_heating_outdoor_air_maximum_flow_guard_state as advance,
    advance_heating_outdoor_air_maximum_flow_guard_state_with_validated_route as advance_with_route,
    heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor,
};
use super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Snapshot,
};
use crate::ideal_loads::calc::heating_operating_mode_deadband_assignment_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    heating_operating_mode_deadband_assignment_snapshots_match_bit_exact,
};

mod committed;
mod error;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardError;
pub(in crate::ideal_loads::calc) use committed::heating_outdoor_air_maximum_flow_guard_committed_latest_route;
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{
    prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact,
};
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    retained_route_matches_snapshot_bounded as heating_outdoor_air_maximum_flow_guard_retained_route_matches_snapshot_bounded,
    snapshot_route as heating_outdoor_air_maximum_flow_guard_snapshot_route,
};

/// Losslessly reconstructs the CP434 predecessor retained in CP435.
#[must_use]
pub fn heating_outdoor_air_maximum_flow_guard_predecessor_cp434_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp434_snapshot(snapshot)
}

/// Executes CP435 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_guard(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp434: Predecessor,
) -> Result<Snapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardError> {
    use PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardError as Error;

    let selected = predecessor_cp434.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.heating_outdoor_air_maximum_flow_guard_latest_witness(selected);
    if system.id != selected {
        return Err(Error::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(Error::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if system.humidification_control_type != HumidificationControlType::None {
        return Err(Error::HumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.humidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(Error::InitializationNotReady { system: selected });
    }
    if unit.controlled_zone != Some(predecessor_cp434.controlled_zone)
        || unit
            .calc_heating_operating_mode_deadband_assignment
            .latest
            .is_none_or(|latest| {
                !heating_operating_mode_deadband_assignment_snapshots_match_bit_exact(
                    latest,
                    predecessor_cp434,
                )
            })
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .heating_operating_mode_deadband_assignment_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !heating_operating_mode_deadband_assignment_snapshots_match_bit_exact(
        predecessor_cp434,
        predecessor_witness,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let mixed_air_witness = if predecessor_cp434
        .cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_executed
    {
        Some(
            runtime
                .cooling_mixed_air_call_latest_witness(selected)
                .ok_or(Error::PredecessorMixedAirTemperatureWitnessUnavailableOrInconsistent {
                    system: selected,
                })?,
        )
    } else {
        None
    };
    let predecessor_route = heating_operating_mode_deadband_assignment_committed_latest_route(
        unit,
        predecessor_witness,
        mixed_air_witness,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let flow_limit_active = predecessor_cp434.heating_or_no_load_case_entered
        && matches!(
            system.heating_limit,
            IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
        );
    let (outdoor_air_mass_flow_rate_kg_per_s, maximum_heating_air_mass_flow_rate_kg_per_s) =
        if flow_limit_active {
            let minimum_oa = unit.calc_minimum_oa_prefix.latest.ok_or(
                Error::MinimumOutdoorAirMassFlowUnavailableOrInconsistent { system: selected },
            )?;
            let outdoor = minimum_oa
                .working_outdoor_air_mass_flow_rate_kg_per_s
                .ok_or(Error::MinimumOutdoorAirMassFlowUnavailableOrInconsistent {
                    system: selected,
                })?;
            let maximum = unit.maximum_heating_air_mass_flow_rate_kg_per_s;
            if !minimum_oa_is_sealed_same_call_no_oa(unit, predecessor_cp434, minimum_oa)
                || outdoor.to_bits() != 0.0f64.to_bits()
                || !maximum_heating_cache_is_bit_exact(unit, system)
            {
                return Err(Error::MaximumFlowInputsOutsideDirectSubset { system: selected });
            }
            (outdoor, maximum)
        } else {
            (0.0, 0.0)
        };
    let route = heating_outdoor_air_maximum_flow_guard_route_from_committed_predecessor(
        predecessor_cp434,
        predecessor_route,
        system.heating_limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        maximum_heating_air_mass_flow_rate_kg_per_s,
    )
    .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    if route.body_entered {
        return Err(Error::ExactReleaseReductionViolated { system: selected });
    }
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp434.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = unit.calc_heating_outdoor_air_maximum_flow_guard.clone();
    let snapshot = advance_with_route(
        &mut next_state,
        predecessor_cp434,
        predecessor_route,
        system.heating_limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        maximum_heating_air_mass_flow_rate_kg_per_s,
        route,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state
        .latest_route
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(snapshot, predecessor_cp434, post_route)
        || !post_transition_state_is_consistent(
            &next_state,
            snapshot,
            post_route,
            &unit.calc_heating_operating_mode_deadband_assignment,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_heating_outdoor_air_maximum_flow_guard = next_state;
    runtime.set_heating_outdoor_air_maximum_flow_guard_latest_witness(selected, Some(snapshot));
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_heating_outdoor_air_maximum_flow_guard_characterization(
    predecessor: Predecessor,
    heating_limit: IdealLoadsLimit,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    maximum_heating_air_mass_flow_rate_kg_per_s: f64,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(
        &mut state,
        predecessor,
        heating_limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        maximum_heating_air_mass_flow_rate_kg_per_s,
    )
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_guard_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardError {
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardError::HeatingOperatingModeDeadbandAssignmentSnapshotMismatch {
        system,
    }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardError {
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_heating_operating_mode_deadband_assignment
            .transition_count,
        transition_count: unit.calc_heating_outdoor_air_maximum_flow_guard.transition_count,
    }
}

fn minimum_oa_is_sealed_same_call_no_oa(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    snapshot: crate::ideal_loads::PurchasedAirCalcMinimumOaPrefixSnapshot,
) -> bool {
    let entry = unit.calc_entry.latest;
    snapshot.source == crate::ideal_loads::PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
        && snapshot.minimum_oa_child_source
            == crate::ideal_loads::PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
        && snapshot.source_order == crate::ideal_loads::PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER
        && snapshot.system == unit.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.parent_call_ordinal == unit.calc_minimum_oa_prefix.transition_count
        && unit.calc_minimum_oa_prefix.transition_count == unit.calc_entry.call_count
        && unit.calc_entry.call_count == unit.init_call_count
        && entry.is_some_and(|entry| {
            entry.call_ordinal == snapshot.parent_call_ordinal
                && entry.controlled_zone == snapshot.controlled_zone
                && entry.unit_body_entered == snapshot.unit_body_entered
        })
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.unit_body_entered
        && snapshot.zone_heat_balance_reference_bound
        && snapshot.minimum_oa_child_called
        && snapshot.minimum_oa_child_no_outdoor_air_route
        && has_positive_zero(snapshot.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s)
        && snapshot.retained_minimum_outdoor_air_write_performed
        && snapshot.ems_override_flag_read
        && snapshot.ems_override_enabled == Some(false)
        && !snapshot.ems_override_applied
        && has_positive_zero(snapshot.working_outdoor_air_mass_flow_rate_kg_per_s)
        && snapshot.outdoor_air_flag_read
        && snapshot.outdoor_air_enabled == Some(false)
        && snapshot.no_outdoor_air_zero_branch_entered
        && snapshot.psychrometric_call_count == 0
        && has_positive_zero(snapshot.minimum_outdoor_air_sensible_output_w)
        && has_positive_zero(snapshot.minimum_outdoor_air_moisture_output_kg_per_s)
}

fn maximum_heating_cache_is_bit_exact(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
) -> bool {
    let Some(density) = unit.standard_air_density_kg_per_m3 else {
        return false;
    };
    let Some(sized) = unit.sized_limits else {
        return false;
    };
    let Some(ep_model::AutosizeOrNumber::Value(volume)) =
        sized.maximum_heating_air_flow_rate_m3_per_s
    else {
        return false;
    };
    let expected = volume * density;
    // This latch is rearmed after BeginEnvironment for the next environment;
    // it does not invalidate the density-derived cache in the current one.
    unit.environment_initialization_count > 0
        && !unit.sizing_needed
        && density.is_finite()
        && density > 0.0
        && volume.is_finite()
        && volume >= 0.0
        && expected.is_finite()
        && expected >= 0.0
        && system.maximum_heating_air_flow_rate_m3_per_s
            == sized.maximum_heating_air_flow_rate_m3_per_s
        && unit.maximum_heating_air_mass_flow_rate_kg_per_s.to_bits() == expected.to_bits()
}

fn has_positive_zero(value: Option<f64>) -> bool {
    value.is_some_and(|value| value.to_bits() == 0.0f64.to_bits())
}

#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
