//! Release-bound CP416 supply-humidity-ratio assignment.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem,
    IdealLoadsAirSystemId,
};

use super::transition::{RetainedRoute, predecessor_route};
use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_state as advance,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

mod error;
mod runtime_validation;
mod snapshot;
pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentError;
use runtime_validation::{
    completed_predecessor_counts_match, pending_predecessor_counts_match,
    state_counts_are_consistent,
};
use snapshot::{cp415_shape, option_bits_match, snapshots_match_bit_exact};

/// Executes CP416 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp415: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentError,
>{
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentError as Error;

    let selected = predecessor_cp415.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_latest_witness(selected);
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witness(selected);
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
    if predecessor_cp415.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    let Some(retained_predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit
        .latest
    else {
        return Err(predecessor_mismatch(selected));
    };
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact(
        predecessor_cp415,
        retained_predecessor,
    ) || !predecessor_witness.is_some_and(|predecessor_witness| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact(
            predecessor_cp415,
            predecessor_witness,
        )
    }) || !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact_direct_release(
        predecessor_cp415,
    ) || !crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_metadata_is_consistent(
        unit,
        unit.init_call_count,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let route = predecessor_route(predecessor_cp415)
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    if route.active {
        let supply_temperature = predecessor_cp415
            .resulting_supply_temperature_c
            .ok_or(predecessor_mismatch(selected))?;
        if !supply_temperature.is_finite() {
            return Err(Error::SupplyTemperatureOutsideDirectSubset {
                system: selected,
                bits: supply_temperature.to_bits(),
            });
        }
        let supply_enthalpy = predecessor_cp415
            .resulting_supply_enthalpy_j_per_kg
            .ok_or(predecessor_mismatch(selected))?;
        if !supply_enthalpy.is_finite() {
            return Err(Error::SupplyEnthalpyOutsideDirectSubset {
                system: selected,
                bits: supply_enthalpy.to_bits(),
            });
        }
        let humidity_ratio = energyplus_psy_w_fn_tdb_h(supply_temperature, supply_enthalpy);
        if !humidity_ratio.is_finite() {
            return Err(Error::PsychrometricSupplyHumidityRatioOutsideDirectSubset {
                system: selected,
                bits: humidity_ratio.to_bits(),
            });
        }
    }
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment;
    if !pending_state_is_consistent(unit, state, witness)
        || !pending_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit,
            route,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let Some(expected_predecessor_transition_count) = state.transition_count.checked_add(1) else {
        return Err(call_order_error(unit, selected));
    };
    if unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit
        .transition_count
        != expected_predecessor_transition_count
        || predecessor_cp415.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = state.clone();
    let snapshot = advance(&mut next_state, predecessor_cp415)
        .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if !snapshot_is_exact(snapshot)
        || !direct_subset_values_are_valid(snapshot)
        || !completed_state_is_consistent(&next_state, snapshot)
        || !completed_predecessor_counts_match(
            &next_state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_latest_witness(
        selected,
        snapshot,
    );
    Ok(snapshot)
}

pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_characterization(
    predecessor: Predecessor,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor)
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact_direct_release(
        cp415_shape(snapshot),
    ) && snapshot_is_exact(snapshot)
        && direct_subset_values_are_valid(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_is_exact(snapshot)
}

fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    if snapshot.source != SOURCE
        || snapshot.first_excluded_source != EXCLUDED
        || snapshot.source_order != ORDER
    {
        return false;
    }
    let predecessor = cp415_shape(snapshot);
    if !cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact(predecessor)
    {
        return false;
    }
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let terminal_prefix_matches = option_bits_match(
        snapshot.predecessor_cp415_resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_match(
        snapshot.predecessor_cp415_resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        snapshot.predecessor_cp415_resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && option_bits_match(
        snapshot.resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_match(
        snapshot.resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    );
    if !terminal_prefix_matches
        || snapshot
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed
            != route.active
        || snapshot.cp415_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp415_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp415_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
    {
        return false;
    }
    if !route.active {
        return !snapshot.cp415_retained_supply_temperature_owned_read
            && !snapshot.supply_temperature_for_humidity_ratio_inversion_read
            && snapshot.supply_temperature_c.is_none()
            && !snapshot.cp415_retained_supply_enthalpy_owned_read
            && !snapshot.supply_enthalpy_for_humidity_ratio_inversion_read
            && snapshot.supply_enthalpy_j_per_kg.is_none()
            && !snapshot.psychrometric_supply_humidity_ratio_evaluated
            && snapshot.psychrometric_supply_humidity_ratio.is_none()
            && !snapshot.supply_humidity_ratio_assignment_performed
            && snapshot.assigned_supply_humidity_ratio.is_none()
            && option_bits_match(
                snapshot.resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            );
    }
    let (Some(temperature), Some(enthalpy), Some(psychrometric), Some(assigned), Some(resulting)) = (
        snapshot.supply_temperature_c,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.psychrometric_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ) else {
        return false;
    };
    let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);
    snapshot.cp415_retained_supply_temperature_owned_read
        && snapshot.supply_temperature_for_humidity_ratio_inversion_read
        && snapshot.cp415_retained_supply_enthalpy_owned_read
        && snapshot.supply_enthalpy_for_humidity_ratio_inversion_read
        && snapshot.psychrometric_supply_humidity_ratio_evaluated
        && snapshot.supply_humidity_ratio_assignment_performed
        && predecessor
            .resulting_supply_temperature_c
            .is_some_and(|value| value.to_bits() == temperature.to_bits())
        && predecessor
            .resulting_supply_enthalpy_j_per_kg
            .is_some_and(|value| value.to_bits() == enthalpy.to_bits())
        && psychrometric.to_bits() == expected.to_bits()
        && assigned.to_bits() == expected.to_bits()
        && resulting.to_bits() == expected.to_bits()
}

fn direct_subset_values_are_valid(snapshot: Snapshot) -> bool {
    !snapshot
        .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed
        || (snapshot.supply_temperature_c.is_some_and(f64::is_finite)
            && snapshot
                .supply_enthalpy_j_per_kg
                .is_some_and(f64::is_finite)
            && snapshot
                .psychrometric_supply_humidity_ratio
                .is_some_and(f64::is_finite))
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot_route(
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    snapshot_is_exact(snapshot)
        .then(|| predecessor_route(cp415_shape(snapshot)))
        .flatten()
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
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
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit
                .transition_count
                .saturating_sub(1)
        && match (state.latest, witness) {
            (Some(latest), Some(witness)) => {
                snapshots_match_bit_exact(latest, witness)
                    && state.latest_transition_ordinal == Some(state.transition_count)
                    && state.latest_route == predecessor_route(cp415_shape(latest))
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
        && state.latest_route == predecessor_route(cp415_shape(snapshot))
        && state_counts_are_consistent(state)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment;
    state.transition_count == expected_transition_count
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit
                .transition_count
        && state
            .latest
            .is_some_and(|latest| completed_state_is_consistent(state, latest))
        && completed_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit,
        )
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_is_consistent(
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
        || !cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_latest_metadata_is_consistent(
            unit,
            unit.init_call_count,
        )
    {
        return false;
    }
    let predecessor = cp415_shape(snapshot);
    let predecessor_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witness(system.id);
    let retained_predecessor = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit
        .latest;
    retained_predecessor.is_some_and(|retained| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact(
            retained,
            predecessor,
        )
    }) && predecessor_witness.is_some_and(|witness| {
        cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshots_match_bit_exact(
            witness,
            predecessor,
        )
    })
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentError
{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentError::CoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshotMismatch { system }
}

fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentError
{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit
            .transition_count,
        transition_count: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment
            .transition_count,
    }
}
