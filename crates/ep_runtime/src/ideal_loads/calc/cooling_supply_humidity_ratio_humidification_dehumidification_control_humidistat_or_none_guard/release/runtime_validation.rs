//! Persistent CP371 runtime-state validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId};

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_humidification_flow.system == system
        && unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
            .system
            == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard
            .system
            == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_carried_counts_match(state, prior, predecessor)
}

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    control: DehumidificationControlType,
) -> bool {
    let mut probe = state.clone();
    advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state(
        &mut probe,
        predecessor,
        control,
    )
    .is_some()
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit
        .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && carried_counts_match(state, prior)
        && state.latest.is_some_and(|latest| snapshots_match_exact(latest, snapshot))
        && state.latest_transition_ordinal == Some(state.transition_count)
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_latest_metadata_is_consistent(
    state: &State,
    expected_transition_count: usize,
) -> bool {
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    system: IdealLoadsAirSystemId,
) -> bool {
    let Some(active) = checked_sum(&[
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ]) else {
        return false;
    };
    let Some(upstream_partition) = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        active,
    ]) else {
        return false;
    };
    let Some(final_partition) = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_on_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_type_humidistat_match_count,
        state.dehumidification_control_type_none_match_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]) else {
        return false;
    };
    let Some(heating_partition) = state
        .heating_on_body_entry_count
        .checked_add(state.heating_on_guard_false_fallthrough_count)
    else {
        return false;
    };
    let Some(humidification_partition) = state
        .humidification_control_body_entry_count
        .checked_add(state.humidification_control_guard_false_fallthrough_count)
    else {
        return false;
    };
    let Some(first_partition) = state
        .dehumidification_control_type_humidistat_match_count
        .checked_add(state.dehumidification_control_type_second_read_count)
    else {
        return false;
    };
    let Some(second_partition) = state
        .dehumidification_control_type_none_match_count
        .checked_add(state.dehumidification_control_guard_false_fallthrough_count)
    else {
        return false;
    };
    let Some(body) = state
        .dehumidification_control_type_humidistat_match_count
        .checked_add(state.dehumidification_control_type_none_match_count)
    else {
        return false;
    };
    let Some(source) = state
        .dehumidification_control_type_first_read_count
        .checked_mul(2)
        .and_then(|count| {
            state
                .dehumidification_control_type_second_read_count
                .checked_mul(2)
                .and_then(|second| count.checked_add(second))
        })
        .and_then(|count| count.checked_add(state.dehumidification_control_body_entry_count))
    else {
        return false;
    };
    state.system == system
        && upstream_partition == state.transition_count
        && final_partition == state.transition_count
        && state.heating_on_read_count == active
        && heating_partition == active
        && state.humidification_control_type_read_count == state.heating_on_body_entry_count
        && state.humidification_control_type_humidistat_comparison_count
            == state.humidification_control_type_read_count
        && humidification_partition == state.humidification_control_type_read_count
        && state.dehumidification_control_type_first_read_count
            == state.humidification_control_body_entry_count
        && state.dehumidification_control_type_humidistat_comparison_count
            == state.dehumidification_control_type_first_read_count
        && first_partition == state.dehumidification_control_type_first_read_count
        && state.dehumidification_control_type_none_comparison_count
            == state.dehumidification_control_type_second_read_count
        && second_partition == state.dehumidification_control_type_second_read_count
        && state.dehumidification_control_body_entry_count == body
        && state.source_site_execution_count == source
        && latest_metadata_is_consistent(state, witness)
}

fn latest_metadata_is_consistent(state: &State, witness: Option<Snapshot>) -> bool {
    if state.transition_count == 0 {
        return state.latest.is_none()
            && state.latest_route.is_none()
            && state.latest_transition_ordinal.is_none()
            && witness.is_none();
    }
    let (Some(latest), Some(route), Some(ordinal), Some(witness)) = (
        state.latest,
        state.latest_route,
        state.latest_transition_ordinal,
        witness,
    ) else {
        return false;
    };
    ordinal == state.transition_count
        && latest.system == state.system
        && snapshot_route(latest) == Some(route)
        && snapshots_match_exact(latest, witness)
        && latest_route_is_counted(state, route)
}

fn latest_route_is_counted(state: &State, route: Route) -> bool {
    match route {
        Route::UnitOff => state.unit_off_skip_count > 0,
        Route::NonCooling => state.non_cooling_skip_count > 0,
        Route::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count > 0
        }
        Route::HeatingAvailabilityGuardFalseFallthrough => {
            state.heating_on_guard_false_fallthrough_count > 0
        }
        Route::HumidificationControlGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count > 0
        }
        Route::DehumidificationControlHumidistatBodyEntered => {
            state.dehumidification_control_type_humidistat_match_count > 0
        }
        Route::DehumidificationControlNoneBodyEntered => {
            state.dehumidification_control_type_none_match_count > 0
        }
        Route::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count > 0
        }
    }
}

fn pending_carried_counts_match(
    state: &State,
    prior: &PredecessorState,
    latest: Predecessor,
) -> bool {
    let pairs = [
        (state.unit_off_skip_count, prior.unit_off_skip_count, latest.unit_off_skipped),
        (state.non_cooling_skip_count, prior.non_cooling_skip_count, latest.non_cooling_skipped),
        (state.positive_guard_false_fallthrough_skip_count, prior.positive_guard_false_fallthrough_skip_count, latest.positive_guard_false_fallthrough_skipped),
        (state.dehumidification_control_none_case_completed_skip_count, prior.dehumidification_control_none_case_completed_skip_count, latest.dehumidification_control_none_case_completed_skip),
        (state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count, prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count, latest.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip),
        (state.dehumidification_control_humidistat_case_completed_skip_count, prior.dehumidification_control_humidistat_case_completed_skip_count, latest.dehumidification_control_humidistat_case_completed_skip),
        (state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count, prior.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count, latest.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip),
        (state.heating_on_read_count, prior.heating_on_read_count, latest.predecessor_heating_on_read),
        (state.heating_on_body_entry_count, prior.heating_on_body_entry_count, latest.predecessor_cooling_supply_humidity_ratio_humidification_body_entered),
        (state.heating_on_guard_false_fallthrough_count, prior.heating_on_guard_false_fallthrough_count, latest.predecessor_heating_on_guard_false_fallthrough),
        (state.humidification_control_type_read_count, prior.humidification_control_type_read_count, latest.humidification_control_type_read),
        (state.humidification_control_type_humidistat_comparison_count, prior.humidification_control_type_humidistat_comparison_count, latest.humidification_control_type_humidistat.is_some()),
        (state.humidification_control_body_entry_count, prior.humidification_control_body_entry_count, latest.humidification_control_body_entered),
        (state.humidification_control_guard_false_fallthrough_count, prior.humidification_control_guard_false_fallthrough_count, latest.humidification_control_guard_false_fallthrough),
    ];
    pairs.into_iter().all(|(current, expected, increment)| {
        current.checked_add(usize::from(increment)) == Some(expected)
    })
}

fn carried_counts_match(state: &State, prior: &PredecessorState) -> bool {
    state.unit_off_skip_count == prior.unit_off_skip_count
        && state.non_cooling_skip_count == prior.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count == prior.positive_guard_false_fallthrough_skip_count
        && state.dehumidification_control_none_case_completed_skip_count == prior.dehumidification_control_none_case_completed_skip_count
        && state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count == prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
        && state.dehumidification_control_humidistat_case_completed_skip_count == prior.dehumidification_control_humidistat_case_completed_skip_count
        && state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count == prior.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count
        && state.heating_on_read_count == prior.heating_on_read_count
        && state.heating_on_body_entry_count == prior.heating_on_body_entry_count
        && state.heating_on_guard_false_fallthrough_count == prior.heating_on_guard_false_fallthrough_count
        && state.humidification_control_type_read_count == prior.humidification_control_type_read_count
        && state.humidification_control_type_humidistat_comparison_count == prior.humidification_control_type_humidistat_comparison_count
        && state.humidification_control_body_entry_count == prior.humidification_control_body_entry_count
        && state.humidification_control_guard_false_fallthrough_count == prior.humidification_control_guard_false_fallthrough_count
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}