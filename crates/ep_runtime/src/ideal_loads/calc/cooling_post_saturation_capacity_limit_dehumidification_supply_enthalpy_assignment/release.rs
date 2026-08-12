//! Release-bound CP417 supply-enthalpy assignment.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::transition::{RetainedRoute, predecessor_route};
use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_state as advance,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshots_match_bit_exact,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

mod error;
mod runtime_validation;
mod snapshot;
pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError;
pub(super) use runtime_validation::committed_route_counts_match;
use runtime_validation::{
    completed_predecessor_counts_match, pending_predecessor_counts_match,
    state_counts_are_consistent,
};
use snapshot::{cp416_shape, option_bits_match, snapshots_match_bit_exact};

/// Executes CP417 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp416: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError,
>{
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError as Error;

    let selected = predecessor_cp416.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_latest_witness(selected);
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_latest_witness(selected);
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
    let controlled_zone = unit
        .controlled_zone
        .ok_or(Error::InitializationNotReady { system: selected })?;
    if predecessor_cp416.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    let Some(retained_predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment
        .latest
    else {
        return Err(predecessor_mismatch(selected));
    };
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshots_match_bit_exact(
        predecessor_cp416,
        retained_predecessor,
    ) || !predecessor_witness.is_some_and(|predecessor_witness| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshots_match_bit_exact(
            predecessor_cp416,
            predecessor_witness,
        )
    }) || !cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
        predecessor_cp416,
    ) || !crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_latest_metadata_is_consistent(
        unit,
        unit.init_call_count,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let route = predecessor_route(predecessor_cp416)
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    if route.active {
        let supply_temperature = predecessor_cp416
            .resulting_supply_temperature_c
            .ok_or(predecessor_mismatch(selected))?;
        if !supply_temperature.is_finite() {
            return Err(Error::SupplyTemperatureOutsideDirectSubset {
                system: selected,
                bits: supply_temperature.to_bits(),
            });
        }
        let supply_humidity_ratio = predecessor_cp416
            .resulting_supply_humidity_ratio
            .ok_or(predecessor_mismatch(selected))?;
        if !supply_humidity_ratio.is_finite() || supply_humidity_ratio < 0.0 {
            return Err(Error::SupplyHumidityRatioOutsideDirectSubset {
                system: selected,
                bits: supply_humidity_ratio.to_bits(),
            });
        }
        let supply_enthalpy =
            energyplus_psy_h_fn_tdb_w(supply_temperature, supply_humidity_ratio);
        if !supply_enthalpy.is_finite() {
            return Err(Error::PsychrometricSupplyEnthalpyOutsideDirectSubset {
                system: selected,
                bits: supply_enthalpy.to_bits(),
            });
        }
    }
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment;
    if !pending_state_is_consistent(unit, state, witness)
        || !pending_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment,
            route,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let Some(expected_predecessor_transition_count) = state.transition_count.checked_add(1) else {
        return Err(call_order_error(unit, selected));
    };
    if unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment
        .transition_count
        != expected_predecessor_transition_count
        || predecessor_cp416.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = state.clone();
    let snapshot = advance(&mut next_state, predecessor_cp416)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !snapshot_is_exact(snapshot)
        || !direct_subset_values_are_valid(snapshot)
        || !completed_state_is_consistent(&next_state, snapshot)
        || !completed_predecessor_counts_match(
            &next_state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_latest_witness(
        selected,
        snapshot,
    );
    Ok(snapshot)
}

pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor)
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
        cp416_shape(snapshot),
    ) && snapshot_is_exact(snapshot)
        && direct_subset_values_are_valid(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    let predecessor = cp416_shape(snapshot);
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact(predecessor)
    {
        return false;
    }
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    snapshot_matches_validated_predecessor(snapshot, predecessor, route)
}

fn snapshot_matches_validated_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
    route: RetainedRoute,
) -> bool {
    if snapshot.source != SOURCE
        || snapshot.first_excluded_source != EXCLUDED
        || snapshot.source_order != ORDER
        || route.predecessor_guard_false_fallthrough
            != predecessor.saturation_supply_humidity_ratio_guard_false_fallthrough
        || route.predecessor_guard_body_entered
            != predecessor.saturation_supply_humidity_ratio_guard_body_entered
        || route.predecessor_saturation_temperature_assignment_executed
            != predecessor
                .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed
        || route.predecessor_saturation_temperature_mixed_air_limit_executed
            != predecessor
                .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed
        || route.predecessor_supply_humidity_ratio_assignment_executed
            != predecessor
                .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed
    {
        return false;
    }
    let terminal_prefix_matches = option_bits_match(
        snapshot.predecessor_cp416_resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_match(
        snapshot.predecessor_cp416_resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        snapshot.predecessor_cp416_resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && option_bits_match(
        snapshot.resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_match(
        snapshot.resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    );
    if !terminal_prefix_matches
        || snapshot
            .post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed
            != route.active
        || snapshot.cp416_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp416_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp416_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
    {
        return false;
    }
    if !route.active {
        return !snapshot.cp416_retained_supply_temperature_owned_read
            && !snapshot.supply_temperature_for_enthalpy_read
            && snapshot.supply_temperature_for_enthalpy_c.is_none()
            && !snapshot.cp416_retained_supply_humidity_ratio_owned_read
            && !snapshot.supply_humidity_ratio_for_enthalpy_read
            && snapshot.supply_humidity_ratio_for_enthalpy.is_none()
            && !snapshot.psychrometric_supply_enthalpy_evaluated
            && snapshot.psychrometric_supply_enthalpy_j_per_kg.is_none()
            && !snapshot.supply_enthalpy_assignment_performed
            && snapshot.assigned_supply_enthalpy_j_per_kg.is_none()
            && option_bits_match(
                snapshot.resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            );
    }
    let (
        Some(temperature),
        Some(humidity_ratio),
        Some(psychrometric),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.supply_temperature_for_enthalpy_c,
        snapshot.supply_humidity_ratio_for_enthalpy,
        snapshot.psychrometric_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) else {
        return false;
    };
    let expected = energyplus_psy_h_fn_tdb_w(temperature, humidity_ratio);
    snapshot.cp416_retained_supply_temperature_owned_read
        && snapshot.supply_temperature_for_enthalpy_read
        && snapshot.cp416_retained_supply_humidity_ratio_owned_read
        && snapshot.supply_humidity_ratio_for_enthalpy_read
        && snapshot.psychrometric_supply_enthalpy_evaluated
        && snapshot.supply_enthalpy_assignment_performed
        && predecessor
            .resulting_supply_temperature_c
            .is_some_and(|value| value.to_bits() == temperature.to_bits())
        && predecessor
            .resulting_supply_humidity_ratio
            .is_some_and(|value| value.to_bits() == humidity_ratio.to_bits())
        && psychrometric.to_bits() == expected.to_bits()
        && assigned.to_bits() == expected.to_bits()
        && resulting.to_bits() == expected.to_bits()
}

fn direct_subset_values_are_valid(snapshot: Snapshot) -> bool {
    !snapshot
        .post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed
        || (snapshot
            .supply_temperature_for_enthalpy_c
            .is_some_and(f64::is_finite)
            && snapshot
                .supply_humidity_ratio_for_enthalpy
                .is_some_and(|value| value.is_finite() && value >= 0.0)
            && snapshot
                .psychrometric_supply_enthalpy_j_per_kg
                .is_some_and(f64::is_finite))
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_route(
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    snapshot_is_exact(snapshot)
        .then(|| predecessor_route(cp416_shape(snapshot)))
        .flatten()
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

/// Returns the sealed route attached to the committed CP417 latest snapshot.
///
/// This is deliberately calc-private: CP418 may trust the route only after the CP417 public
/// release committed the snapshot, counters, ordinal, and predecessor accounting together.
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_committed_latest_route(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment;
    let route = state.latest_route?;
    let predecessor = cp416_shape(snapshot);
    (unit.system == snapshot.system
        && state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment
                .transition_count
        && snapshot.parent_call_ordinal == unit.init_call_count
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
        && state_counts_are_consistent(state)
        && completed_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment,
        )
        && committed_route_counts_match(state, route)
        && snapshot_matches_validated_predecessor(snapshot, predecessor, route))
    .then_some(route)
}

fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    witness: Option<Snapshot>,
) -> bool {
    state.system == unit.system
        && state_counts_are_consistent(state)
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment
                .transition_count
                .saturating_sub(1)
        && match (state.latest, witness) {
            (Some(latest), Some(witness)) => {
                snapshots_match_bit_exact(latest, witness)
                    && state.latest_transition_ordinal == Some(state.transition_count)
                    && state.latest_route == predecessor_route(cp416_shape(latest))
            }
            (None, None) => state.transition_count == 0,
            _ => false,
        }
}

fn completed_state_is_consistent(state: &State, snapshot: Snapshot) -> bool {
    state
        .latest
        .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state.latest_route == predecessor_route(cp416_shape(snapshot))
        && state_counts_are_consistent(state)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment;
    state.transition_count == expected_transition_count
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment
                .transition_count
        && state
            .latest
            .is_some_and(|latest| completed_state_is_consistent(state, latest))
        && completed_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment,
        )
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    if unit.system != system.id
        || snapshot.system != system.id
        || unit.controlled_zone != Some(snapshot.controlled_zone)
        || !snapshot_is_exact(snapshot)
        || !direct_subset_values_are_valid(snapshot)
        || !witness.is_some_and(|witness| snapshots_match_bit_exact(snapshot, witness))
        || !cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_latest_metadata_is_consistent(
            unit,
            unit.init_call_count,
        )
    {
        return false;
    }
    let predecessor = cp416_shape(snapshot);
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_latest_witness(system.id);
    let retained_predecessor = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment
        .latest;
    retained_predecessor.is_some_and(|retained| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshots_match_bit_exact(
            retained,
            predecessor,
        )
    }) && predecessor_witness.is_some_and(|witness| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshots_match_bit_exact(
            witness,
            predecessor,
        )
    })
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError
{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError::CoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshotMismatch { system }
}

fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError
{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment
            .transition_count,
        transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment
            .transition_count,
    }
}
