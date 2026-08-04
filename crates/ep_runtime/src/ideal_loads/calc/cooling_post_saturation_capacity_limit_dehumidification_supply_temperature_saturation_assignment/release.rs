//! Release-bound CP414 saturation-temperature assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::transition::predecessor_route;
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot as Snapshot,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER as ORDER,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
};

mod error;
mod runtime_validation;
pub use error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError;
use runtime_validation::{
    completed_predecessor_counts_match, pending_predecessor_counts_match,
    state_counts_are_consistent,
};

/// Executes CP414 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp413: Predecessor,
    barometric_pressure_pa: f64,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError,
> {
    use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError as Error;

    let selected = predecessor_cp413.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_latest_witness(selected);
    let predecessor_witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_witness(selected);
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
    if predecessor_cp413.controlled_zone != controlled_zone {
        return Err(predecessor_mismatch(selected));
    }
    let Some(retained_predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard
        .latest
    else {
        return Err(predecessor_mismatch(selected));
    };
    if !crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshots_match_bit_exact(
        predecessor_cp413,
        retained_predecessor,
    ) {
        return Err(predecessor_mismatch(selected));
    }
    let predecessor_metadata_is_consistent = crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_metadata_is_consistent(
        unit,
        unit.init_call_count,
    );
    let predecessor_witness_is_consistent = predecessor_witness.is_some_and(|witness| {
        crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshots_match_bit_exact(
            witness,
            predecessor_cp413,
        )
    });
    if !predecessor_metadata_is_consistent || !predecessor_witness_is_consistent {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let route = predecessor_route(predecessor_cp413)
        .ok_or(Error::PredecessorOutsideDirectSubset { system: selected })?;
    if route.assignment_executed {
        let enthalpy = predecessor_cp413
            .resulting_supply_enthalpy_j_per_kg
            .ok_or(predecessor_mismatch(selected))?;
        if !enthalpy.is_finite() {
            return Err(Error::SupplyEnthalpyOutsideDirectSubset {
                system: selected,
                bits: enthalpy.to_bits(),
            });
        }
        if !barometric_pressure_pa.is_finite() || barometric_pressure_pa <= 0.0 {
            return Err(Error::BarometricPressureOutsideDirectSubset {
                system: selected,
                bits: barometric_pressure_pa.to_bits(),
            });
        }
    }
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment;
    if !pending_state_is_consistent(unit, state, witness)
        || !pending_predecessor_counts_match(
            state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard,
            route,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    let Some(expected_predecessor_transition_count) = state.transition_count.checked_add(1) else {
        return Err(call_order_error(unit, selected));
    };
    if unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard
        .transition_count
        != expected_predecessor_transition_count
        || predecessor_cp413.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }

    let mut next_state = state.clone();
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_state(
        &mut next_state,
        predecessor_cp413,
        barometric_pressure_pa,
    )
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    if route.assignment_executed
        && !snapshot
            .resulting_supply_temperature_c
            .is_some_and(f64::is_finite)
    {
        return Err(Error::PsychrometricSaturationTemperatureOutsideDirectSubset {
            system: selected,
            bits: snapshot
                .resulting_supply_temperature_c
                .map_or(f64::NAN.to_bits(), f64::to_bits),
        });
    }
    if !snapshot_is_exact(snapshot)
        || !direct_subset_values_are_valid(snapshot)
        || !completed_state_is_consistent(&next_state, snapshot)
        || !completed_predecessor_counts_match(
            &next_state,
            &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let unit = runtime
        .units
        .get_mut(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment = next_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_latest_witness(selected, snapshot);
    Ok(snapshot)
}

pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization(
    predecessor: Predecessor,
    barometric_pressure_pa: f64,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_state(
        &mut state,
        predecessor,
        barometric_pressure_pa,
    )
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    let predecessor = cp413_shape(snapshot);
    crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(predecessor)
        && snapshot_is_exact(snapshot)
        && direct_subset_values_are_valid(snapshot)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact(
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
    let predecessor = cp413_shape(snapshot);
    let pressure = snapshot
        .outdoor_barometric_pressure_for_saturation_temperature_pa
        .unwrap_or(crate::psychrometrics::ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA);
    let Some(expected) = private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization(
        predecessor,
        pressure,
    ) else {
        return false;
    };
    snapshots_match_bit_exact(snapshot, expected)
}

fn direct_subset_values_are_valid(snapshot: Snapshot) -> bool {
    !snapshot.post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed
        || (snapshot
            .supply_enthalpy_for_saturation_temperature_j_per_kg
            .is_some_and(f64::is_finite)
            && snapshot
                .outdoor_barometric_pressure_for_saturation_temperature_pa
                .is_some_and(|pressure| pressure.is_finite() && pressure > 0.0)
            && snapshot
                .psychrometric_saturation_supply_temperature_result_c
                .is_some_and(f64::is_finite))
}

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact(
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
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard
                .transition_count
                .saturating_sub(1)
        && match (state.latest, witness) {
            (Some(latest), Some(witness)) => {
                snapshots_match_bit_exact(latest, witness)
                    && state.latest_transition_ordinal == Some(state.transition_count)
                    && state.latest_route == predecessor_route(cp413_shape(latest))
            }
            (None, None) => state.transition_count == 0,
            _ => false,
        }
}

fn completed_state_is_consistent(state: &State, snapshot: Snapshot) -> bool {
    state.latest.is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
        && state.latest_transition_ordinal == Some(state.transition_count)
        && state.latest_route == predecessor_route(cp413_shape(snapshot))
        && state_counts_are_consistent(state)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment;
    state.transition_count == expected_transition_count
        && state.transition_count
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard
                .transition_count
        && state.latest.is_some_and(|latest| completed_state_is_consistent(state, latest))
        && state.predecessor_route_counts
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard
                .predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard
                .guard_false_fallthrough_route_counts
        && state.predecessor_guard_body_entry_route_counts
            == unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard
                .guard_body_entry_route_counts
}

fn predecessor_mismatch(
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError::CoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshotMismatch { system }
}

fn call_order_error(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError {
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        predecessor_transition_count: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard.transition_count,
        transition_count: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment.transition_count,
    }
}

fn cp413_shape(snapshot: Snapshot) -> Predecessor {
    use crate::ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE as PREDECESSOR_EXCLUDED,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE as PREDECESSOR_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE_ORDER as PREDECESSOR_ORDER,
    };
    Predecessor {
        source: PREDECESSOR_SOURCE,
        first_excluded_source: PREDECESSOR_EXCLUDED,
        source_order: PREDECESSOR_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: snapshot.predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: snapshot.predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: snapshot.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: snapshot.predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break: snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        predecessor_cp409_resulting_supply_humidity_ratio: snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
        predecessor_cp409_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp409_resulting_supply_temperature_c: snapshot.predecessor_cp409_resulting_supply_temperature_c,
        predecessor_dehumidification_control_default_case_exited_via_break: snapshot.predecessor_dehumidification_control_default_case_exited_via_break,
        predecessor_cp410_resulting_supply_humidity_ratio: snapshot.predecessor_cp410_resulting_supply_humidity_ratio,
        predecessor_cp410_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp410_resulting_supply_temperature_c: snapshot.predecessor_cp410_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed: snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
        cp410_retained_supply_humidity_ratio_state_owned: snapshot.cp410_retained_supply_humidity_ratio_state_owned,
        cp410_retained_supply_enthalpy_state_owned: snapshot.cp410_retained_supply_enthalpy_state_owned,
        cp410_retained_supply_temperature_state_owned: snapshot.cp410_retained_supply_temperature_state_owned,
        cp410_retained_supply_humidity_ratio_owned_read: snapshot.cp410_retained_supply_humidity_ratio_owned_read,
        purchased_air_supply_humidity_ratio_read: snapshot.purchased_air_supply_humidity_ratio_read,
        purchased_air_supply_humidity_ratio_before_saturation_check: snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
        local_supply_humidity_ratio_original_assignment_performed: snapshot.local_supply_humidity_ratio_original_assignment_performed,
        assigned_supply_humidity_ratio_original: snapshot.assigned_supply_humidity_ratio_original,
        resulting_supply_humidity_ratio_original: snapshot.resulting_supply_humidity_ratio_original,
        predecessor_cp411_resulting_supply_humidity_ratio: snapshot.predecessor_cp411_resulting_supply_humidity_ratio,
        predecessor_cp411_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp411_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp411_resulting_supply_temperature_c: snapshot.predecessor_cp411_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed: snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed,
        cp411_retained_supply_humidity_ratio_state_owned: snapshot.cp411_retained_supply_humidity_ratio_state_owned,
        cp411_retained_supply_enthalpy_state_owned: snapshot.cp411_retained_supply_enthalpy_state_owned,
        cp411_retained_supply_temperature_state_owned: snapshot.cp411_retained_supply_temperature_state_owned,
        cp411_retained_supply_temperature_owned_read: snapshot.cp411_retained_supply_temperature_owned_read,
        purchased_air_supply_temperature_for_saturation_humidity_ratio_read: snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read,
        supply_temperature_for_saturation_humidity_ratio_c: snapshot.supply_temperature_for_saturation_humidity_ratio_c,
        environment_outdoor_barometric_pressure_owned_read: snapshot.environment_outdoor_barometric_pressure_owned_read,
        environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read,
        outdoor_barometric_pressure_pa: snapshot.outdoor_barometric_pressure_pa,
        psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated,
        saturation_supply_humidity_ratio: snapshot.saturation_supply_humidity_ratio,
        local_saturation_supply_humidity_ratio_assignment_performed: snapshot.local_saturation_supply_humidity_ratio_assignment_performed,
        assigned_saturation_supply_humidity_ratio: snapshot.assigned_saturation_supply_humidity_ratio,
        resulting_saturation_supply_humidity_ratio: snapshot.resulting_saturation_supply_humidity_ratio,
        predecessor_cp412_resulting_supply_humidity_ratio: snapshot.predecessor_cp412_resulting_supply_humidity_ratio,
        predecessor_cp412_resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp412_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp412_resulting_supply_temperature_c: snapshot.predecessor_cp412_resulting_supply_temperature_c,
        post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated: snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated,
        cp412_saturation_supply_humidity_ratio_owned_read: snapshot.cp412_saturation_supply_humidity_ratio_owned_read,
        saturation_supply_humidity_ratio_for_guard_read: snapshot.saturation_supply_humidity_ratio_for_guard_read,
        saturation_supply_humidity_ratio_for_guard: snapshot.saturation_supply_humidity_ratio_for_guard,
        cp411_original_supply_humidity_ratio_owned_read: snapshot.cp411_original_supply_humidity_ratio_owned_read,
        cp412_same_call_original_supply_humidity_ratio_bit_corroborated: snapshot.cp412_same_call_original_supply_humidity_ratio_bit_corroborated,
        original_supply_humidity_ratio_for_guard_read: snapshot.original_supply_humidity_ratio_for_guard_read,
        original_supply_humidity_ratio_for_guard: snapshot.original_supply_humidity_ratio_for_guard,
        saturation_original_supply_humidity_ratio_comparison_evaluated: snapshot.saturation_original_supply_humidity_ratio_comparison_evaluated,
        saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio: snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
        saturation_supply_humidity_ratio_guard_body_entered: snapshot.saturation_supply_humidity_ratio_guard_body_entered,
        saturation_supply_humidity_ratio_guard_false_fallthrough: snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough,
        cp412_retained_supply_humidity_ratio_state_owned: snapshot.cp412_retained_supply_humidity_ratio_state_owned,
        cp412_retained_supply_enthalpy_state_owned: snapshot.cp412_retained_supply_enthalpy_state_owned,
        cp412_retained_supply_temperature_state_owned: snapshot.cp412_retained_supply_temperature_state_owned,
        resulting_supply_humidity_ratio: snapshot.predecessor_cp413_resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: snapshot.predecessor_cp413_resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: snapshot.predecessor_cp413_resulting_supply_temperature_c,
    }
}

fn snapshots_match_bit_exact(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! compare_clear {
        ($field:ident) => {{
            let matches = option_bits_match(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }
    let values_match = compare_clear!(predecessor_cp409_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp409_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp409_resulting_supply_temperature_c)
        && compare_clear!(predecessor_cp410_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp410_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp410_resulting_supply_temperature_c)
        && compare_clear!(purchased_air_supply_humidity_ratio_before_saturation_check)
        && compare_clear!(assigned_supply_humidity_ratio_original)
        && compare_clear!(resulting_supply_humidity_ratio_original)
        && compare_clear!(predecessor_cp411_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp411_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp411_resulting_supply_temperature_c)
        && compare_clear!(supply_temperature_for_saturation_humidity_ratio_c)
        && compare_clear!(outdoor_barometric_pressure_pa)
        && compare_clear!(saturation_supply_humidity_ratio)
        && compare_clear!(assigned_saturation_supply_humidity_ratio)
        && compare_clear!(resulting_saturation_supply_humidity_ratio)
        && compare_clear!(predecessor_cp412_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp412_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp412_resulting_supply_temperature_c)
        && compare_clear!(saturation_supply_humidity_ratio_for_guard)
        && compare_clear!(original_supply_humidity_ratio_for_guard)
        && compare_clear!(predecessor_cp413_resulting_supply_humidity_ratio)
        && compare_clear!(predecessor_cp413_resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(predecessor_cp413_resulting_supply_temperature_c)
        && compare_clear!(supply_enthalpy_for_saturation_temperature_j_per_kg)
        && compare_clear!(outdoor_barometric_pressure_for_saturation_temperature_pa)
        && compare_clear!(psychrometric_saturation_supply_temperature_result_c)
        && compare_clear!(assigned_saturation_supply_temperature_c)
        && compare_clear!(resulting_supply_humidity_ratio)
        && compare_clear!(resulting_supply_enthalpy_j_per_kg)
        && compare_clear!(resulting_supply_temperature_c);
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
