//! Persistent CP376 runtime-state validation.

use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner as Owner,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state,
};
use super::snapshot_validation::{snapshot_owner, snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_pre_saturation_original_assignment::transition::{
    predecessor_route, route_count,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn calc_state_identities_match(
    unit: &PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> bool {
    unit.system == system
        && unit.calc_entry.system == system
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment
            .system
            == system
        && unit
            .calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
            .system
            == system
}

pub(super) fn call_order_is_pending(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
) -> bool {
    let ordinal = predecessor.parent_call_ordinal;
    unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
        .transition_count
        .checked_add(1)
        == Some(ordinal)
        && unit.calc_entry.call_count == ordinal
        && unit
            .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment
            .transition_count
            == ordinal
}

pub(super) fn pending_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    witness: Option<Snapshot>,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let state = &unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment;
    state_is_consistent(state, witness, predecessor.system)
        && direct_owner_distribution_is_consistent(state)
        && state.transition_count.checked_add(1) == Some(predecessor.parent_call_ordinal)
        && pending_route_counts_match(state, prior, route)
}

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> bool {
    let mut probe = state.clone();
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state(
        &mut probe,
        predecessor,
        input,
    )
    .is_some()
}

#[allow(dead_code)]
pub(super) fn completed_state_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let state = &unit.calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
    let prior = &unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment;
    state_is_consistent(state, witness, snapshot.system)
        && direct_owner_distribution_is_consistent(state)
        && state.transition_count == prior.transition_count
        && completed_route_counts_match(state, prior)
        && state
            .latest
            .is_some_and(|latest| snapshots_match_bit_exact(latest, snapshot))
}

#[allow(dead_code)]
pub(in crate::ideal_loads) fn cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_metadata_is_consistent(
    state: &State,
    expected_transition_count: usize,
) -> bool {
    state.transition_count == expected_transition_count
        && state_is_consistent(state, state.latest, state.system)
        && direct_owner_distribution_is_consistent(state)
}

fn direct_owner_distribution_is_consistent(state: &State) -> bool {
    state
        .heating_availability_guard_false_fallthrough_count
        .checked_add(state.humidification_control_guard_false_fallthrough_count)
        == Some(state.cp347_none_case_owner_count)
        && state.cp375_maximum_assignment_owner_count == 0
        && state.cp356_constant_shr_owner_count == 0
        && state.cp362_humidistat_owner_count == 0
        && state.cp365_constant_supply_humidity_ratio_owner_count == 0
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
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER.len(),
    ) else {
        return false;
    };
    let Some(owner_total) = checked_sum(&[
        state.cp375_maximum_assignment_owner_count,
        state.cp347_none_case_owner_count,
        state.cp356_constant_shr_owner_count,
        state.cp362_humidistat_owner_count,
        state.cp365_constant_supply_humidity_ratio_owner_count,
    ]) else {
        return false;
    };
    let cp375_owned = state
        .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count
        .checked_add(
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        );
    state.system == system
        && partition == state.transition_count
        && state.source_site_execution_count == source_sites
        && state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count == active
        && state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count
            == active
        && owner_total == active
        && cp375_owned == Some(state.cp375_maximum_assignment_owner_count)
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
        && snapshots_match_bit_exact(latest, witness)
        && route_count(state, route) > 0
        && snapshot_owner(latest).is_none_or(|owner| owner_count(state, owner) > 0)
}

fn pending_route_counts_match(state: &State, prior: &PredecessorState, route: Route) -> bool {
    route_count_pairs(state, prior)
        .into_iter()
        .all(|(current, expected, applies)| {
            current.checked_add(usize::from(applies == route)) == Some(expected)
        })
}

#[allow(dead_code)]
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

fn owner_count(state: &State, owner: Owner) -> usize {
    match owner {
        Owner::Cp375MaximumAssignment => state.cp375_maximum_assignment_owner_count,
        Owner::Cp347NoneCase => state.cp347_none_case_owner_count,
        Owner::Cp356ConstantShr => state.cp356_constant_shr_owner_count,
        Owner::Cp362Humidistat => state.cp362_humidistat_owner_count,
        Owner::Cp365ConstantSupplyHumidityRatio => {
            state.cp365_constant_supply_humidity_ratio_owner_count
        }
    }
}

fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
