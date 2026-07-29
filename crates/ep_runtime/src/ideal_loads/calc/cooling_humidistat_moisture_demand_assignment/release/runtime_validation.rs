//! Persistent CP359 runtime-state validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_humidistat_moisture_demand_assignment::transition::{
    next_transition_fits as pure_next_transition_fits, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot as Predecessor, PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit.calc_cooling_humidistat_case_entry.system == system
        && unit
            .calc_cooling_humidistat_moisture_demand_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_humidistat_moisture_demand_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit.calc_cooling_humidistat_case_entry.transition_count == ordinal
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
    let state = &unit.calc_cooling_humidistat_moisture_demand_assignment;
    let prior = &unit.calc_cooling_humidistat_case_entry;
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
            state.dehumidification_control_humidistat_moisture_demand_assignment_count,
            route == Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted,
        ) == Some(prior.dehumidification_control_humidistat_case_entry_count)
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
    let state = &unit.calc_cooling_humidistat_moisture_demand_assignment;
    let prior = &unit.calc_cooling_humidistat_case_entry;
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
        && state.dehumidification_control_humidistat_moisture_demand_assignment_count
            == prior.dehumidification_control_humidistat_case_entry_count
        && state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
            == prior
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
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
        state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ]) else {
        return false;
    };
    let Some(selected) = checked_sum(&[
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ]) else {
        return false;
    };
    let Some(witnessed_selected) = checked_sum(&[
        state.witnessed_dehumidification_control_none_case_completed_skip_count,
        state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.witnessed_dehumidification_control_humidistat_moisture_demand_assignment_count,
        state
            .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ]) else {
        return false;
    };
    let h = state.dehumidification_control_humidistat_moisture_demand_assignment_count;
    let Some(expected_source_sites) = h.checked_mul(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len(),
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
    let counters_match = state.system == expected_system
        && route_partition == state.transition_count
        && selected == witnessed_selected
        && selector_partition
        && state.source_site_execution_count == expected_source_sites
        && state.zone_dehumidifying_setpoint_moisture_demand_read_count == h
        && state.zone_dehumidifying_setpoint_moisture_demand_assignment_count == h
        && state.witnessed_positive_guard_false_fallthrough_skip_count
            == state.positive_guard_false_fallthrough_skip_count
        && state.witnessed_dehumidification_control_none_case_completed_skip_count
            == state.dehumidification_control_none_case_completed_skip_count
        && state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
            == state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count
        && state.witnessed_dehumidification_control_humidistat_moisture_demand_assignment_count
            == h
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
                && snapshots_match_bit_exact(latest, witness)
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
