//! Release-bound CP436 heating outdoor-air volume-flow assignment.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::transition::{
    advance_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_state as advance,
    advance_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_state_with_validated_route as advance_with_route,
    heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor,
};
use super::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::calc::heating_outdoor_air_maximum_flow_guard_committed_latest_route;
use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact,
};

mod error;
mod prefix;
mod runtime_validation;
mod snapshot_validation;

pub use error::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError;
use runtime_validation::{pending_state_is_consistent, post_transition_state_is_consistent};
use snapshot_validation::{prefix_and_local_shape_match, snapshot_is_exact, snapshots_match_bit_exact};
pub(in crate::ideal_loads::calc) use snapshot_validation::{
    retained_route_matches_snapshot_bounded as heating_outdoor_air_maximum_flow_body_volume_flow_assignment_retained_route_matches_snapshot_bounded,
    snapshot_route as heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_route,
};

/// Losslessly reconstructs the CP435 predecessor retained in CP436.
#[must_use]
pub fn heating_outdoor_air_maximum_flow_body_volume_flow_assignment_predecessor_cp435_snapshot(
    snapshot: Snapshot,
) -> Predecessor {
    prefix::predecessor_cp435_snapshot(snapshot)
}

/// Executes CP436 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp435: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError,
> {
    use PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError as Error;

    let selected = predecessor_cp435.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .heating_outdoor_air_maximum_flow_body_volume_flow_assignment_latest_witness(selected);
    if system.id != selected {
        return Err(Error::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(Error::InitializationNotReady { system: selected });
    }
    if unit.controlled_zone != Some(predecessor_cp435.controlled_zone)
        || unit
            .calc_heating_outdoor_air_maximum_flow_guard
            .latest
            .is_none_or(|latest| {
                !heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact(
                    latest,
                    predecessor_cp435,
                )
            })
    {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_witness = runtime
        .heating_outdoor_air_maximum_flow_guard_latest_witness(selected)
        .ok_or_else(|| predecessor_mismatch(selected))?;
    if !heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact(
        predecessor_cp435,
        predecessor_witness,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let mixed_air_witness = if predecessor_cp435
        .cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_executed
    {
        Some(runtime.cooling_mixed_air_call_latest_witness(selected).ok_or(
            Error::RuntimeStateInvariantViolation { system: selected },
        )?)
    } else {
        None
    };
    let predecessor_route = heating_outdoor_air_maximum_flow_guard_committed_latest_route(
        unit,
        predecessor_witness,
        mixed_air_witness,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let route =
        heating_outdoor_air_maximum_flow_body_volume_flow_assignment_route_from_committed_predecessor(
            predecessor_cp435,
            predecessor_route,
        )
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let standard_air_density_kg_per_m3 = if route.assignment_executed {
        begin_environment_standard_air_density_is_bit_exact(unit, system).ok_or(
            Error::StandardAirDensityUnavailableOrInconsistent { system: selected },
        )?
    } else {
        1.0
    };
    if route.assignment_executed {
        return Err(Error::ExactReleaseReductionViolated { system: selected });
    }
    if !pending_state_is_consistent(unit, witness) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if predecessor_cp435.parent_call_ordinal != unit.init_call_count {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = unit
        .calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment
        .clone();
    let snapshot = advance_with_route(
        &mut next_state,
        predecessor_cp435,
        predecessor_route,
        standard_air_density_kg_per_m3,
        route,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    let post_route = next_state
        .latest_route
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !prefix_and_local_shape_match(
        snapshot,
        predecessor_cp435,
        predecessor_route,
        post_route,
    ) || !post_transition_state_is_consistent(
        &next_state,
        snapshot,
        post_route,
        &unit.calc_heating_outdoor_air_maximum_flow_guard,
    ) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment = next_state;
    runtime.set_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_latest_witness(
        selected,
        Some(snapshot),
    );
    Ok(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_characterization(
    predecessor: Predecessor,
    standard_air_density_kg_per_m3: f64,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor, standard_air_density_kg_per_m3)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError {
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError::HeatingOutdoorAirMaximumFlowGuardSnapshotMismatch {
        system,
    }
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError {
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_heating_outdoor_air_maximum_flow_guard
            .transition_count,
        transition_count: unit
            .calc_heating_outdoor_air_maximum_flow_body_volume_flow_assignment
            .transition_count,
    }
}

fn begin_environment_standard_air_density_is_bit_exact(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
) -> Option<f64> {
    let density = unit.standard_air_density_kg_per_m3?;
    let sized = unit.sized_limits?;
    let Some(ep_model::AutosizeOrNumber::Value(volume)) =
        sized.maximum_heating_air_flow_rate_m3_per_s
    else {
        return None;
    };
    let expected_maximum_mass_flow = volume * density;
    (unit.environment_initialization_count > 0
        && !unit.sizing_needed
        && density.is_finite()
        && density > 0.0
        && volume.is_finite()
        && volume >= 0.0
        && expected_maximum_mass_flow.is_finite()
        && expected_maximum_mass_flow >= 0.0
        && system.maximum_heating_air_flow_rate_m3_per_s
            == sized.maximum_heating_air_flow_rate_m3_per_s
        && unit.maximum_heating_air_mass_flow_rate_kg_per_s.to_bits()
            == expected_maximum_mass_flow.to_bits())
    .then_some(density)
}

#[cfg(test)]
pub(super) fn state_counts_are_consistent_for_test(state: &State) -> bool {
    runtime_validation::state_counts_are_consistent(state)
}
