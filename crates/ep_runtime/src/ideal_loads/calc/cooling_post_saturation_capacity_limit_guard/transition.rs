//! Pure CP379-to-CP380 post-saturation capacity-limit guard.

use ep_model::IdealLoadsLimit;

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_supply_enthalpy_post_saturation_assignment::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as PREDECESSOR_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE as PREDECESSOR_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER as PREDECESSOR_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRetainedRoute as PredecessorRoute,
};

/// Configured selector and its release-validated same-call corroboration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput
{
    /// The only source operand read at line 2264.
    pub cooling_limit: IdealLoadsLimit,
    /// True only after the public release validates the retained CP337 lineage.
    pub cp337_same_call_selector_lineage_corroborated: bool,
}

struct PreparedGuard {
    first_cooling_limit: Option<IdealLoadsLimit>,
    cooling_limit_capacity: Option<bool>,
    second_cooling_limit: Option<IdealLoadsLimit>,
    cooling_limit_flow_rate_and_capacity: Option<bool>,
    cooling_limit_condition_satisfied: Option<bool>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let active = predecessor_route_is_active(predecessor_route);
    let prepared = prepare_guard(active, input)?;
    let body_entered = prepared.cooling_limit_condition_satisfied == Some(true);
    let guard_false = prepared.cooling_limit_condition_satisfied == Some(false);
    let route = retained_route(predecessor_route, body_entered)?;
    if !next_transition_fits(state, predecessor_route, route, &prepared) {
        return None;
    }

    state.transition_count += 1;
    increment_inherited_route_count(state, predecessor_route);
    increment_route_count(state, route);
    if active {
        state.capacity_limit_guard_evaluation_count += 1;
        state.configured_cooling_limit_owned_read_count += 1;
        state.cp337_same_call_selector_lineage_corroboration_count += 1;
        state.first_cooling_limit_read_count += 1;
        state.cooling_limit_capacity_comparison_count += 1;
        state.source_site_execution_count += 2;
        if prepared.cooling_limit_capacity == Some(true) {
            state.cooling_limit_capacity_match_count += 1;
        } else {
            state.second_cooling_limit_read_count += 1;
            state.cooling_limit_flow_rate_and_capacity_comparison_count += 1;
            state.source_site_execution_count += 2;
            if prepared.cooling_limit_flow_rate_and_capacity == Some(true) {
                state.cooling_limit_flow_rate_and_capacity_match_count += 1;
            }
        }
        if body_entered {
            state.capacity_limit_body_entry_count += 1;
            state.source_site_execution_count += 1;
        } else {
            state.cooling_limit_rejected_count += 1;
            state.active_guard_false_fallthrough_count += 1;
        }
    }

    let snapshot = Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor_route == PredecessorRoute::UnitOff,
        non_cooling_skipped: predecessor_route == PredecessorRoute::NonCooling,
        positive_guard_false_fallthrough_skipped: predecessor_route
            == PredecessorRoute::PositiveGuardFalseFallthrough,
        heating_availability_guard_false_fallthrough: predecessor_route
            == PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough,
        humidification_control_guard_false_fallthrough: predecessor_route
            == PredecessorRoute::HumidificationControlGuardFalseFallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor_route
            == PredecessorRoute::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
        dehumidification_control_none_maximum_assignment_executed: predecessor_route
            == PredecessorRoute::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
        dehumidification_control_guard_false_fallthrough: predecessor_route
            == PredecessorRoute::DehumidificationControlGuardFalseFallthrough,
        predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed:
            predecessor.local_supply_enthalpy_after_saturation_limit_assignment_performed,
        capacity_limit_guard_evaluated: active,
        configured_cooling_limit_owned_read: active,
        cp337_same_call_selector_lineage_corroborated: active,
        first_cooling_limit_read: active,
        first_cooling_limit: prepared.first_cooling_limit,
        cooling_limit_capacity_comparison_evaluated: active,
        cooling_limit_capacity: prepared.cooling_limit_capacity,
        second_cooling_limit_read: prepared.cooling_limit_capacity == Some(false),
        second_cooling_limit: prepared.second_cooling_limit,
        cooling_limit_flow_rate_and_capacity_comparison_evaluated:
            prepared.cooling_limit_capacity == Some(false),
        cooling_limit_flow_rate_and_capacity: prepared.cooling_limit_flow_rate_and_capacity,
        cooling_limit_condition_satisfied: prepared.cooling_limit_condition_satisfied,
        cooling_limit_rejected: guard_false,
        capacity_limit_body_entered: body_entered,
        active_guard_false_fallthrough: guard_false,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

fn prepare_guard(
    active: bool,
    input: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput>,
) -> Option<PreparedGuard> {
    if !active {
        return input.is_none().then_some(PreparedGuard {
            first_cooling_limit: None,
            cooling_limit_capacity: None,
            second_cooling_limit: None,
            cooling_limit_flow_rate_and_capacity: None,
            cooling_limit_condition_satisfied: None,
        });
    }
    let input = input?;
    if !input.cp337_same_call_selector_lineage_corroborated {
        return None;
    }
    let first = input.cooling_limit;
    let capacity = first == IdealLoadsLimit::LimitCapacity;
    let second = (!capacity).then_some(input.cooling_limit);
    let combined = second.map(|limit| limit == IdealLoadsLimit::LimitFlowRateAndCapacity);
    Some(PreparedGuard {
        first_cooling_limit: Some(first),
        cooling_limit_capacity: Some(capacity),
        second_cooling_limit: second,
        cooling_limit_flow_rate_and_capacity: combined,
        cooling_limit_condition_satisfied: Some(capacity || combined == Some(true)),
    })
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<PredecessorRoute> {
    if predecessor.source != PREDECESSOR_SOURCE
        || predecessor.first_excluded_source != PREDECESSOR_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order != PREDECESSOR_SOURCE_ORDER
    {
        return None;
    }
    let flags = [
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor.heating_availability_guard_false_fallthrough,
        predecessor.humidification_control_guard_false_fallthrough,
        predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        predecessor.dehumidification_control_none_maximum_assignment_executed,
        predecessor.dehumidification_control_guard_false_fallthrough,
    ];
    if flags.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    let route = if predecessor.unit_off_skipped {
        PredecessorRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        PredecessorRoute::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        PredecessorRoute::PositiveGuardFalseFallthrough
    } else if predecessor.heating_availability_guard_false_fallthrough {
        PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough
    } else if predecessor.humidification_control_guard_false_fallthrough {
        PredecessorRoute::HumidificationControlGuardFalseFallthrough
    } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
        PredecessorRoute::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
    } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
        PredecessorRoute::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
    } else {
        PredecessorRoute::DehumidificationControlGuardFalseFallthrough
    };
    (predecessor.local_supply_enthalpy_after_saturation_limit_assignment_performed
        == predecessor_route_is_active(route))
    .then_some(route)
}

pub(in crate::ideal_loads::calc) fn predecessor_route_is_active(route: PredecessorRoute) -> bool {
    !matches!(
        route,
        PredecessorRoute::UnitOff
            | PredecessorRoute::NonCooling
            | PredecessorRoute::PositiveGuardFalseFallthrough
    )
}

fn retained_route(predecessor: PredecessorRoute, body: bool) -> Option<Route> {
    Some(match (predecessor, body) {
        (PredecessorRoute::UnitOff, false) => Route::UnitOff,
        (PredecessorRoute::NonCooling, false) => Route::NonCooling,
        (PredecessorRoute::PositiveGuardFalseFallthrough, false) => {
            Route::PositiveGuardFalseFallthrough
        }
        (PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough, true) => {
            Route::HeatingAvailabilityGuardFalseFallthroughBodyEntered
        }
        (PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough, false) => {
            Route::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough
        }
        (PredecessorRoute::HumidificationControlGuardFalseFallthrough, true) => {
            Route::HumidificationControlGuardFalseFallthroughBodyEntered
        }
        (PredecessorRoute::HumidificationControlGuardFalseFallthrough, false) => {
            Route::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough
        }
        (
            PredecessorRoute::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
            true,
        ) => Route::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered,
        (
            PredecessorRoute::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
            false,
        ) => Route::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough,
        (
            PredecessorRoute::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
            true,
        ) => Route::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered,
        (
            PredecessorRoute::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
            false,
        ) => Route::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough,
        (PredecessorRoute::DehumidificationControlGuardFalseFallthrough, true) => {
            Route::DehumidificationControlGuardFalseFallthroughBodyEntered
        }
        (PredecessorRoute::DehumidificationControlGuardFalseFallthrough, false) => {
            Route::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough
        }
        _ => return None,
    })
}

fn next_transition_fits(
    state: &State,
    predecessor: PredecessorRoute,
    route: Route,
    prepared: &PreparedGuard,
) -> bool {
    if state.transition_count.checked_add(1).is_none()
        || inherited_route_count(state, predecessor)
            .checked_add(1)
            .is_none()
        || route_count(state, route).checked_add(1).is_none()
    {
        return false;
    }
    if !predecessor_route_is_active(predecessor) {
        return true;
    }
    let capacity = prepared.cooling_limit_capacity == Some(true);
    let second = prepared.cooling_limit_capacity == Some(false);
    let combined = prepared.cooling_limit_flow_rate_and_capacity == Some(true);
    let body = prepared.cooling_limit_condition_satisfied == Some(true);
    let source_sites = 2 + 2 * usize::from(second) + usize::from(body);
    state
        .source_site_execution_count
        .checked_add(source_sites)
        .is_some()
        && [
            state.capacity_limit_guard_evaluation_count,
            state.configured_cooling_limit_owned_read_count,
            state.cp337_same_call_selector_lineage_corroboration_count,
            state.first_cooling_limit_read_count,
            state.cooling_limit_capacity_comparison_count,
        ]
        .into_iter()
        .all(|count| count.checked_add(1).is_some())
        && (!capacity
            || state
                .cooling_limit_capacity_match_count
                .checked_add(1)
                .is_some())
        && (!second
            || (state
                .second_cooling_limit_read_count
                .checked_add(1)
                .is_some()
                && state
                    .cooling_limit_flow_rate_and_capacity_comparison_count
                    .checked_add(1)
                    .is_some()))
        && (!combined
            || state
                .cooling_limit_flow_rate_and_capacity_match_count
                .checked_add(1)
                .is_some())
        && if body {
            state
                .capacity_limit_body_entry_count
                .checked_add(1)
                .is_some()
        } else {
            state.cooling_limit_rejected_count.checked_add(1).is_some()
                && state
                    .active_guard_false_fallthrough_count
                    .checked_add(1)
                    .is_some()
        }
}

pub(in crate::ideal_loads::calc) fn inherited_route_count(
    state: &State,
    route: PredecessorRoute,
) -> usize {
    match route {
        PredecessorRoute::UnitOff => state.unit_off_skip_count,
        PredecessorRoute::NonCooling => state.non_cooling_skip_count,
        PredecessorRoute::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count
        }
        PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count
        }
        PredecessorRoute::HumidificationControlGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count
        }
        PredecessorRoute::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count
        }
        PredecessorRoute::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count
        }
        PredecessorRoute::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count
        }
    }
}

pub(in crate::ideal_loads::calc) fn route_count(state: &State, route: Route) -> usize {
    match route {
        Route::UnitOff => state.unit_off_skip_count,
        Route::NonCooling => state.non_cooling_skip_count,
        Route::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        Route::HeatingAvailabilityGuardFalseFallthroughBodyEntered => {
            state.heating_availability_guard_false_fallthrough_body_entry_count
        }
        Route::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count
        }
        Route::HumidificationControlGuardFalseFallthroughBodyEntered => {
            state.humidification_control_guard_false_fallthrough_body_entry_count
        }
        Route::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count
        }
        Route::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered => {
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count
        }
        Route::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count
        }
        Route::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered => {
            state.dehumidification_control_none_maximum_assignment_body_entry_count
        }
        Route::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count
        }
        Route::DehumidificationControlGuardFalseFallthroughBodyEntered => {
            state.dehumidification_control_guard_false_fallthrough_body_entry_count
        }
        Route::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count
        }
    }
}

fn increment_inherited_route_count(state: &mut State, route: PredecessorRoute) {
    match route {
        PredecessorRoute::UnitOff => state.unit_off_skip_count += 1,
        PredecessorRoute::NonCooling => state.non_cooling_skip_count += 1,
        PredecessorRoute::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count += 1;
        }
        PredecessorRoute::HeatingAvailabilityGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count += 1;
        }
        PredecessorRoute::HumidificationControlGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count += 1;
        }
        PredecessorRoute::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count += 1;
        }
        PredecessorRoute::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count += 1;
        }
        PredecessorRoute::DehumidificationControlGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count += 1;
        }
    }
}

fn increment_route_count(state: &mut State, route: Route) {
    match route {
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalseFallthrough => {}
        Route::HeatingAvailabilityGuardFalseFallthroughBodyEntered => {
            state.heating_availability_guard_false_fallthrough_body_entry_count += 1;
        }
        Route::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count += 1;
        }
        Route::HumidificationControlGuardFalseFallthroughBodyEntered => {
            state.humidification_control_guard_false_fallthrough_body_entry_count += 1;
        }
        Route::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count += 1;
        }
        Route::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered => {
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count += 1;
        }
        Route::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count += 1;
        }
        Route::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered => {
            state.dehumidification_control_none_maximum_assignment_body_entry_count += 1;
        }
        Route::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count += 1;
        }
        Route::DehumidificationControlGuardFalseFallthroughBodyEntered => {
            state.dehumidification_control_guard_false_fallthrough_body_entry_count += 1;
        }
        Route::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count += 1;
        }
    }
}
