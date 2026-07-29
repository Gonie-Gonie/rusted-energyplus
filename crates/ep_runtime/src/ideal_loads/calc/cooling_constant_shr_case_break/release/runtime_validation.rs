//! Persistent CP357 runtime-state validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId};

use super::super::{
    PurchasedAirCalcCoolingConstantShrCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState as State,
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot as Snapshot,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_constant_shr_case_break::transition::{
    next_transition_fits as pure_next_transition_fits, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit
            .system
            == system
        && unit.calc_cooling_constant_shr_case_break.system == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_constant_shr_case_break
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit
            .transition_count
            == ordinal
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
    let state = &unit.calc_cooling_constant_shr_case_break;
    let prior = &unit.calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit;
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
            state.dehumidification_control_constant_sensible_heat_ratio_case_break_count,
            route == Route::DehumidificationControlConstantSensibleHeatRatioCaseBreak,
        ) == Some(
            prior
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count,
        )
        && pending_count(
            state.dehumidification_control_humidistat_case_selected_skip_count,
            route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
        ) == Some(prior.dehumidification_control_humidistat_case_selected_skip_count)
        && pending_count(
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
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
    let state = &unit.calc_cooling_constant_shr_case_break;
    let prior = &unit.calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit;
    state_is_consistent(state, witness, snapshot.system, selector)
        && state.transition_count == prior.transition_count
        && state.unit_off_skip_count == prior.unit_off_skip_count
        && state.non_cooling_skip_count == prior.non_cooling_skip_count
        && state.positive_guard_false_fallthrough_skip_count
            == prior.positive_guard_false_fallthrough_skip_count
        && state.dehumidification_control_none_case_completed_skip_count
            == prior.dehumidification_control_none_case_completed_skip_count
        && state.dehumidification_control_constant_sensible_heat_ratio_case_break_count
            == prior
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count
        && state.dehumidification_control_humidistat_case_selected_skip_count
            == prior.dehumidification_control_humidistat_case_selected_skip_count
        && state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            == prior
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_exact(latest, snapshot))
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
        state.dehumidification_control_constant_sensible_heat_ratio_case_break_count,
        state.dehumidification_control_humidistat_case_selected_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ]) else {
        return false;
    };
    let Some(selected) = checked_sum(&[
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_break_count,
        state.dehumidification_control_humidistat_case_selected_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ]) else {
        return false;
    };
    let Some(recursively_witnessed) = checked_sum(&[
        state.witnessed_dehumidification_control_none_case_completed_skip_count,
        state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_break_count,
        state.witnessed_dehumidification_control_humidistat_case_selected_skip_count,
        state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ]) else {
        return false;
    };
    let completed_skip = state.dehumidification_control_none_case_completed_skip_count;
    let break_flow = state.dehumidification_control_constant_sensible_heat_ratio_case_break_count;
    let Some(later_case_skip) = state
        .dehumidification_control_humidistat_case_selected_skip_count
        .checked_add(
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        )
    else {
        return false;
    };
    let Some(after_none_case) = break_flow.checked_add(later_case_skip) else {
        return false;
    };
    let Some(control_flow_partition) = completed_skip
        .checked_add(break_flow)
        .and_then(|count| count.checked_add(later_case_skip))
    else {
        return false;
    };
    let selected_counts_match = state.dehumidification_control_none_case_completed_skip_count
        == usize::from(selector == DehumidificationControlType::None) * selected
        && break_flow
            == usize::from(selector == DehumidificationControlType::ConstantSensibleHeatRatio)
                * selected
        && state.dehumidification_control_humidistat_case_selected_skip_count
            == usize::from(selector == DehumidificationControlType::Humidistat) * selected
        && state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            == usize::from(selector == DehumidificationControlType::ConstantSupplyHumidityRatio)
                * selected;
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && selected == recursively_witnessed
        && selected == control_flow_partition
        && after_none_case == break_flow + later_case_skip
        && selected_counts_match
        && state.source_site_execution_count == break_flow
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_dehumidification_control_none_case_completed_skip_count
            == state.dehumidification_control_none_case_completed_skip_count
        && state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_break_count
            == state.dehumidification_control_constant_sensible_heat_ratio_case_break_count
        && state.witnessed_dehumidification_control_humidistat_case_selected_skip_count
            == state.dehumidification_control_humidistat_case_selected_skip_count
        && state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            == state
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count;
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
