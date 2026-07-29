//! Persistent CP364 runtime-state validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId};

use super::super::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState as State,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Snapshot,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_case_entry::transition::{
    next_transition_fits as pure_next_transition_fits, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot as Predecessor, PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_humidistat_case_break.system == system
        && unit
            .calc_cooling_constant_supply_humidity_ratio_case_entry
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_constant_supply_humidity_ratio_case_entry
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_cooling_humidistat_case_break.transition_count == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
    selector: DehumidificationControlType,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let state = &unit.calc_cooling_constant_supply_humidity_ratio_case_entry;
    let prior = &unit.calc_cooling_humidistat_case_break;
    state_is_consistent(state, witness, predecessor.system, selector)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_count(state.unit_off_skip_count, route == Route::UnitOff)
            == Some(prior.unit_off_skip_count)
        && pending_count(state.non_cooling_skip_count, route == Route::NonCooling)
            == Some(prior.non_cooling_skip_count)
        && pending_count(
            state.positive_guard_false_fallthrough_skip_count,
            route == Route::PositiveGuardFalseFallthrough,
        ) == Some(prior.positive_guard_false_fallthrough_skip_count)
        && pending_count(
            state.dehumidification_control_none_case_completed_skip_count,
            route == Route::DehumidificationControlNoneCaseCompletedSkip,
        ) == Some(prior.dehumidification_control_none_case_completed_skip_count)
        && pending_count(
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            route == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,
        ) == Some(
            prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        )
        && pending_count(
            state.dehumidification_control_humidistat_case_completed_skip_count,
            route == Route::DehumidificationControlHumidistatCaseCompletedSkip,
        ) == Some(prior.dehumidification_control_humidistat_case_break_count)
        && pending_count(
            state.dehumidification_control_constant_supply_humidity_ratio_case_entry_count,
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseEntered,
        ) == Some(
            prior.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        )
}

pub(super) fn next_transition_fits(state: &State, predecessor: Predecessor) -> bool {
    predecessor_route(predecessor).is_some_and(|route| pure_next_transition_fits(state, route))
}

pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
    selector: DehumidificationControlType,
) -> bool {
    let state = &unit.calc_cooling_constant_supply_humidity_ratio_case_entry;
    let prior = &unit.calc_cooling_humidistat_case_break;
    state_is_consistent(state, witness, snapshot.system, selector)
        && state.transition_count == prior.transition_count
        && state.unit_off_skip_count == prior.unit_off_skip_count
        && state.non_cooling_skip_count == prior.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == prior.positive_guard_false_fallthrough_skip_count
        && state.dehumidification_control_none_case_completed_skip_count
            == prior.dehumidification_control_none_case_completed_skip_count
        && state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            == prior.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
        && state.dehumidification_control_humidistat_case_completed_skip_count
            == prior.dehumidification_control_humidistat_case_break_count
        && state.dehumidification_control_constant_supply_humidity_ratio_case_entry_count
            == prior
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_exact(latest, snapshot))
}

pub(in crate::ideal_loads) fn cooling_constant_supply_humidity_ratio_case_entry_latest_metadata_is_consistent(
    state: &State,
    expected_ordinal: usize,
) -> bool {
    state.transition_count == expected_ordinal
        && state.latest_transition_ordinal == Some(expected_ordinal)
        && state
            .latest
            .is_some_and(|latest| snapshot_route(latest) == state.latest_route)
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    expected_system: IdealLoadsAirSystemId,
    selector: DehumidificationControlType,
) -> bool {
    let Some(route_partition) = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_entry_count,
    ]) else {
        return false;
    };
    let Some(selected) = checked_sum(&[
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_entry_count,
    ]) else {
        return false;
    };
    let Some(recursively_witnessed) = checked_sum(&[
        state.witnessed_dehumidification_control_none_case_completed_skip_count,
        state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.witnessed_dehumidification_control_humidistat_case_completed_skip_count,
        state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_entry_count,
    ]) else {
        return false;
    };
    let completed_none = state.dehumidification_control_none_case_completed_skip_count;
    let completed_constant_sensible =
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count;
    let completed_humidistat = state.dehumidification_control_humidistat_case_completed_skip_count;
    let entered_constant_supply =
        state.dehumidification_control_constant_supply_humidity_ratio_case_entry_count;
    let Some(completed_before_constant_supply) = checked_sum(&[
        completed_none,
        completed_constant_sensible,
        completed_humidistat,
    ]) else {
        return false;
    };
    let Some(control_flow_partition) =
        completed_before_constant_supply.checked_add(entered_constant_supply)
    else {
        return false;
    };
    let selected_counts_match = completed_none
        == usize::from(selector == DehumidificationControlType::None) * selected
        && completed_constant_sensible
            == usize::from(selector == DehumidificationControlType::ConstantSensibleHeatRatio)
                * selected
        && completed_humidistat
            == usize::from(selector == DehumidificationControlType::Humidistat) * selected
        && entered_constant_supply
            == usize::from(selector == DehumidificationControlType::ConstantSupplyHumidityRatio)
                * selected;
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && selected == recursively_witnessed
        && selected == control_flow_partition
        && selected_counts_match
        && state.source_site_execution_count == entered_constant_supply
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_dehumidification_control_none_case_completed_skip_count
            == completed_none
        && state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            == completed_constant_sensible
        && state.witnessed_dehumidification_control_humidistat_case_completed_skip_count
            == completed_humidistat
        && state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_entry_count
            == entered_constant_supply;
    if !counters_match {
        return false;
    }
    match (state.transition_count, state.latest, witness) {
        (0, None, None) => {
            state.latest_route.is_none() && state.latest_transition_ordinal.is_none()
        }
        (count, Some(latest), Some(witness)) => {
            count > 0
                && state.latest_transition_ordinal == Some(count)
                && snapshot_route(latest) == state.latest_route
                && latest.system == expected_system
                && latest.parent_call_ordinal == count
                && snapshots_match_exact(latest, witness)
                && (!latest.unit_body_entered
                    || latest.predecessor_dehumidification_control_type.is_none()
                    || latest.predecessor_dehumidification_control_type == Some(selector))
        }
        _ => false,
    }
}

fn pending_count(count: usize, applies: bool) -> Option<usize> {
    count.checked_add(usize::from(applies))
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |total, value| total.checked_add(*value))
}
