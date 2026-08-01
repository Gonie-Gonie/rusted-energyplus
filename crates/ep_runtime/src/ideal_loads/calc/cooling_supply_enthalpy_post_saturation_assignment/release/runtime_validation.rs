//! Persistent CP379 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Snapshot,
    advance_cooling_supply_enthalpy_post_saturation_assignment_state,
};
use super::snapshot_validation::{
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
    snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_supply_enthalpy_post_saturation_assignment::transition::{
    predecessor_route, route_count, route_is_active,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as TemperatureOwner;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as TemperatureState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_supply_humidity_ratio_saturation_assignment
            .system
            == system
        && unit
            .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
            .system
            == system
        && unit
            .calc_cooling_supply_enthalpy_post_saturation_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_supply_enthalpy_post_saturation_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
    input: Option<ActiveInput>,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let state = &unit.calc_cooling_supply_enthalpy_post_saturation_assignment;
    let prior = &unit.calc_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let temperature = &unit.calc_cooling_supply_humidity_ratio_saturation_assignment;
    state_is_consistent(state, witness, predecessor.system)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_route_counts_match(state, prior, route)
        && pending_owner_counts_match(state, prior, temperature, route, input)
}

pub(super) fn prepare_next_transition(
    state: &State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<(State, Snapshot)> {
    let mut next = state.clone();
    let snapshot = advance_cooling_supply_enthalpy_post_saturation_assignment_state(
        &mut next,
        predecessor,
        input,
    )?;
    Some((next, snapshot))
}

pub(super) fn prepared_completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
) -> bool {
    completed_state_parts(unit, state, snapshot, Some(snapshot))
}

pub(in crate::ideal_loads::calc) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    completed_state_parts(
        unit,
        &unit.calc_cooling_supply_enthalpy_post_saturation_assignment,
        snapshot,
        witness,
    )
}

fn completed_state_parts(
    unit: &PurchasedAirUnitRuntimeState,
    state: &State,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let prior = &unit.calc_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let temperature = &unit.calc_cooling_supply_humidity_ratio_saturation_assignment;
    state_is_consistent(state, witness, snapshot.system)
        && state.transition_count == prior.transition_count
        && completed_route_counts_match(state, prior)
        && completed_owner_counts_match(state, prior, temperature)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_supply_enthalpy_post_saturation_assignment_latest_metadata_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    expected_transition_count: usize,
) -> bool {
    let state = &unit.calc_cooling_supply_enthalpy_post_saturation_assignment;
    let prior = &unit.calc_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let temperature = &unit.calc_cooling_supply_humidity_ratio_saturation_assignment;
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && completed_owner_counts_match(state, prior, temperature)
}

fn state_is_consistent(
    state: &State,
    witness: Option<Snapshot>,
    system: IdealLoadsAirSystemId,
) -> bool {
    let Some(partition) = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]) else {
        return false;
    };
    let Some(active) = checked_sum(&[
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]) else {
        return false;
    };
    let Some(source_sites) = active.checked_mul(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER.len(),
    ) else {
        return false;
    };
    let Some(temperature_owner_total) = state
        .cp334_supply_temperature_mixed_air_limit_owner_count
        .checked_add(state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count)
    else {
        return false;
    };
    state.system == system
        && partition == state.transition_count
        && state.source_site_execution_count == source_sites
        && active_counters(state)
            .into_iter()
            .all(|count| count == active)
        && temperature_owner_total == active
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
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == state.system
        && snapshot_route(latest) == Some(route)
        && cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            latest,
        )
        && snapshots_match_bit_exact(latest, witness)
        && route_count(state, route) > 0
}

fn pending_route_counts_match(state: &State, prior: &PredecessorState, route: Route) -> bool {
    route_count_pairs(state, prior)
        .into_iter()
        .all(|(current, expected, applies)| {
            current.checked_add(usize::from(applies == route)) == Some(expected)
        })
}

fn completed_route_counts_match(state: &State, prior: &PredecessorState) -> bool {
    route_count_pairs(state, prior)
        .into_iter()
        .all(|(current, expected, _)| current == expected)
}

fn route_count_pairs(state: &State, prior: &PredecessorState) -> [(usize, usize, Route); 8] {
    [
        (
            state.unit_off_skip_count,
            prior.unit_off_skip_count,
            Route::UnitOff,
        ),
        (
            state.non_cooling_skip_count,
            prior.non_cooling_skip_count,
            Route::NonCooling,
        ),
        (
            state.positive_guard_false_fallthrough_skip_count,
            prior.positive_guard_false_fallthrough_skip_count,
            Route::PositiveGuardFalseFallthrough,
        ),
        (
            state.heating_availability_guard_false_fallthrough_count,
            prior.heating_availability_guard_false_fallthrough_count,
            Route::HeatingAvailabilityGuardFalseFallthrough,
        ),
        (
            state.humidification_control_guard_false_fallthrough_count,
            prior.humidification_control_guard_false_fallthrough_count,
            Route::HumidificationControlGuardFalseFallthrough,
        ),
        (
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            prior
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
        ),
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            prior.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
        ),
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            prior.dehumidification_control_guard_false_fallthrough_count,
            Route::DehumidificationControlGuardFalseFallthrough,
        ),
    ]
}

fn pending_owner_counts_match(
    state: &State,
    prior: &PredecessorState,
    temperature: &TemperatureState,
    route: Route,
    input: Option<ActiveInput>,
) -> bool {
    let active = route_is_active(route);
    let cp334_pending = active
        && input.map(|input| input.temperature_owner)
            == Some(TemperatureOwner::Cp334MixedAirLimit);
    let cp344_pending = active
        && input.map(|input| input.temperature_owner)
            == Some(TemperatureOwner::Cp344CapacityMixedAirLimit);
    state
        .cp334_supply_temperature_mixed_air_limit_owner_count
        .checked_add(usize::from(cp334_pending))
        == Some(temperature.cp334_supply_temperature_mixed_air_limit_owner_count)
        && state
            .cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count
            .checked_add(usize::from(cp344_pending))
            == Some(
                temperature.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
            )
        && state
            .cp378_supply_humidity_ratio_saturation_limit_owner_count
            .checked_add(usize::from(active))
            == Some(prior.purchased_air_supply_humidity_ratio_saturation_limit_assignment_count)
}

fn completed_owner_counts_match(
    state: &State,
    prior: &PredecessorState,
    temperature: &TemperatureState,
) -> bool {
    state.cp334_supply_temperature_mixed_air_limit_owner_count
        == temperature.cp334_supply_temperature_mixed_air_limit_owner_count
        && state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count
            == temperature.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count
        && state.cp378_supply_humidity_ratio_saturation_limit_owner_count
            == prior.purchased_air_supply_humidity_ratio_saturation_limit_assignment_count
}

fn active_counters(state: &State) -> [usize; 5] {
    [
        state.purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count,
        state.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count,
        state.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count,
        state.local_supply_enthalpy_after_saturation_limit_assignment_count,
        state.cp378_supply_humidity_ratio_saturation_limit_owner_count,
    ]
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
