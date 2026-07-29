//! Persistent CP362 runtime-state validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Snapshot,
};
use super::snapshot_validation::{
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
    snapshot_route,
};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_mixed_air_limit::transition::{
    next_transition_fits as pure_next_transition_fits, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
            .system
            == system
        && unit
            .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
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
    let state = &unit.calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit;
    let prior =
        &unit.calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit;
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
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
            route
                == Route::DehumidificationControlHumidistatSupplyHumidityRatioMixedAirLimitExecuted,
        ) == Some(
            prior
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count,
        )
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
    let state = &unit.calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit;
    let prior =
        &unit.calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit;
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
        && state
            .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count
            == prior
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count
        && state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            == prior.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
        && state.latest.is_some_and(|latest| {
            cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
                latest, snapshot,
            )
        })
}

pub(in crate::ideal_loads) fn cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_metadata_is_consistent(
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
        state.dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ]) else {
        return false;
    };
    let Some(selected) = checked_sum(&[
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ]) else {
        return false;
    };
    let Some(witnessed_selected) = checked_sum(&[
        state.witnessed_dehumidification_control_none_case_completed_skip_count,
        state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state
            .witnessed_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
        state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ]) else {
        return false;
    };
    let h = state
        .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count;
    let Some(expected_source_sites) = h.checked_mul(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER
            .len(),
    ) else {
        return false;
    };
    let selector_partition = state.dehumidification_control_none_case_completed_skip_count
        == usize::from(selector == DehumidificationControlType::None) * selected
        && state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            == usize::from(selector == DehumidificationControlType::ConstantSensibleHeatRatio)
                * selected
        && h == usize::from(selector == DehumidificationControlType::Humidistat) * selected
        && state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            == usize::from(selector == DehumidificationControlType::ConstantSupplyHumidityRatio)
                * selected;
    if state.system != expected_system
        || route_partition != state.transition_count
        || selected != witnessed_selected
        || !selector_partition
        || state.source_site_execution_count != expected_source_sites
        || !site_counters_match_h(state, h)
        || state.witnessed_positive_guard_false_fallthrough_skip_count
            != state.positive_guard_false_fallthrough_skip_count
        || state.witnessed_dehumidification_control_none_case_completed_skip_count
            != state.dehumidification_control_none_case_completed_skip_count
        || state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            != state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
        || state
            .witnessed_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count
            != h
        || state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            != state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
    {
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
                && cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
                    latest, witness,
                )
                && (!latest.unit_body_entered
                    || latest.predecessor_dehumidification_control_type.is_none()
                    || latest.predecessor_dehumidification_control_type == Some(selector))
        }
        _ => false,
    }
}

fn site_counters_match_h(state: &State, h: usize) -> bool {
    [
        state.mixed_air_humidity_ratio_for_minimum_read_count,
        state.supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count,
        state.source_shaped_two_argument_minimum_evaluation_count,
        state.supply_humidity_ratio_assignment_count,
    ]
    .into_iter()
    .all(|count| count == h)
}

fn pending_count(count: usize, applies: bool) -> Option<usize> {
    count.checked_add(usize::from(applies))
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |total, value| total.checked_add(*value))
}
