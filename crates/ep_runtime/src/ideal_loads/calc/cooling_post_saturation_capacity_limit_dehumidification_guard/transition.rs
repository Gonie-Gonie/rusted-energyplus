//! Pure CP380-to-CP381 post-saturation capacity-limit dehumidification guard.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_guard::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE as PREDECESSOR_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE as PREDECESSOR_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER as PREDECESSOR_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRetainedRoute as PredecessorRoute,
};

mod accounting;

use accounting::{
    increment_inherited_route_count, increment_predecessor_route_count, increment_route_count,
};

/// Release-validated same-call numerical owners for the line-2266 comparison.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput
{
    pub supply_humidity_ratio: f64,
    pub mixed_air_humidity_ratio: f64,
    pub cp378_supply_humidity_ratio_saturation_limit_owned_read: bool,
    pub cp379_same_call_supply_humidity_ratio_bit_corroborated: bool,
    pub cp329_mixed_air_humidity_ratio_owned_read: bool,
}

struct PreparedGuard {
    supply_humidity_ratio: Option<f64>,
    mixed_air_humidity_ratio: Option<f64>,
    strictly_less: Option<bool>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput,
    >,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let active = predecessor_route_is_active(predecessor_route);
    let prepared = prepare_guard(active, input)?;
    let body_entered = prepared.strictly_less == Some(true);
    let guard_false = prepared.strictly_less == Some(false);
    let route = retained_route(predecessor_route, body_entered)?;
    if !next_transition_fits(state, predecessor_route, route, body_entered) {
        return None;
    }

    state.transition_count += 1;
    increment_inherited_route_count(state, predecessor_route);
    increment_predecessor_route_count(state, predecessor_route);
    increment_route_count(state, route);
    if active {
        state.dehumidification_guard_evaluation_count += 1;
        state.source_site_execution_count += 3 + usize::from(body_entered);
        state.cp378_supply_humidity_ratio_saturation_limit_owned_read_count += 1;
        state.cp379_same_call_supply_humidity_ratio_bit_corroboration_count += 1;
        state.purchased_air_supply_humidity_ratio_read_count += 1;
        state.cp329_mixed_air_humidity_ratio_owned_read_count += 1;
        state.purchased_air_mixed_air_humidity_ratio_read_count += 1;
        state.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count += 1;
        if body_entered {
            state.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count += 1;
            state.dehumidification_body_entry_count += 1;
        } else {
            state.dehumidification_guard_false_fallthrough_count += 1;
        }
    }

    let snapshot = Snapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor.capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .active_guard_false_fallthrough,
        dehumidification_guard_evaluated: active,
        cp378_supply_humidity_ratio_saturation_limit_owned_read: active,
        cp379_same_call_supply_humidity_ratio_bit_corroborated: active,
        purchased_air_supply_humidity_ratio_read: active,
        supply_humidity_ratio: prepared.supply_humidity_ratio,
        cp329_mixed_air_humidity_ratio_owned_read: active,
        purchased_air_mixed_air_humidity_ratio_read: active,
        mixed_air_humidity_ratio: prepared.mixed_air_humidity_ratio,
        supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated: active,
        supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio: prepared.strictly_less,
        dehumidification_body_entered: body_entered,
        dehumidification_guard_false_fallthrough: guard_false,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

fn prepare_guard(
    active: bool,
    input: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput,
    >,
) -> Option<PreparedGuard> {
    if !active {
        return input.is_none().then_some(PreparedGuard {
            supply_humidity_ratio: None,
            mixed_air_humidity_ratio: None,
            strictly_less: None,
        });
    }
    let input = input?;
    if !input.cp378_supply_humidity_ratio_saturation_limit_owned_read
        || !input.cp379_same_call_supply_humidity_ratio_bit_corroborated
        || !input.cp329_mixed_air_humidity_ratio_owned_read
    {
        return None;
    }
    Some(PreparedGuard {
        supply_humidity_ratio: Some(input.supply_humidity_ratio),
        mixed_air_humidity_ratio: Some(input.mixed_air_humidity_ratio),
        strictly_less: Some(input.supply_humidity_ratio < input.mixed_air_humidity_ratio),
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
    if predecessor.unit_off_skipped {
        return predecessor_skip_shape_is_exact(predecessor).then_some(PredecessorRoute::UnitOff);
    }
    if predecessor.non_cooling_skipped {
        return predecessor_skip_shape_is_exact(predecessor)
            .then_some(PredecessorRoute::NonCooling);
    }
    if predecessor.positive_guard_false_fallthrough_skipped {
        return predecessor_skip_shape_is_exact(predecessor)
            .then_some(PredecessorRoute::PositiveGuardFalseFallthrough);
    }
    if !predecessor_active_shape_is_exact(predecessor) {
        return None;
    }
    Some(
        if predecessor.heating_availability_guard_false_fallthrough {
            if predecessor.capacity_limit_body_entered {
                PredecessorRoute::HeatingAvailabilityGuardFalseFallthroughBodyEntered
            } else {
                PredecessorRoute::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough
            }
        } else if predecessor.humidification_control_guard_false_fallthrough {
            if predecessor.capacity_limit_body_entered {
                PredecessorRoute::HumidificationControlGuardFalseFallthroughBodyEntered
            } else {
                PredecessorRoute::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough
            }
        } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
            if predecessor.capacity_limit_body_entered {
                PredecessorRoute::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered
            } else {
                PredecessorRoute::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough
            }
        } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
            if predecessor.capacity_limit_body_entered {
                PredecessorRoute::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered
            } else {
                PredecessorRoute::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough
            }
        } else if predecessor.capacity_limit_body_entered {
            PredecessorRoute::DehumidificationControlGuardFalseFallthroughBodyEntered
        } else {
            PredecessorRoute::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough
        },
    )
}

fn predecessor_skip_shape_is_exact(predecessor: Predecessor) -> bool {
    !predecessor.capacity_limit_guard_evaluated
        && !predecessor.capacity_limit_body_entered
        && !predecessor.active_guard_false_fallthrough
}

fn predecessor_active_shape_is_exact(predecessor: Predecessor) -> bool {
    predecessor.capacity_limit_guard_evaluated
        && predecessor.predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed
        && (predecessor.capacity_limit_body_entered != predecessor.active_guard_false_fallthrough)
        && predecessor.cooling_limit_condition_satisfied
            == Some(predecessor.capacity_limit_body_entered)
        && predecessor.cooling_limit_rejected == predecessor.active_guard_false_fallthrough
}

pub(in crate::ideal_loads::calc) fn predecessor_route_is_active(route: PredecessorRoute) -> bool {
    matches!(
        route,
        PredecessorRoute::HeatingAvailabilityGuardFalseFallthroughBodyEntered
            | PredecessorRoute::HumidificationControlGuardFalseFallthroughBodyEntered
            | PredecessorRoute::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered
            | PredecessorRoute::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered
            | PredecessorRoute::DehumidificationControlGuardFalseFallthroughBodyEntered
    )
}

fn retained_route(predecessor: PredecessorRoute, body: bool) -> Option<Route> {
    use PredecessorRoute as P;
    Some(match (predecessor, body) {
        (P::UnitOff, false) => Route::UnitOff,
        (P::NonCooling, false) => Route::NonCooling,
        (P::PositiveGuardFalseFallthrough, false) => Route::PositiveGuardFalseFallthrough,
        (P::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough, false) => {
            Route::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        }
        (P::HeatingAvailabilityGuardFalseFallthroughBodyEntered, true) => {
            Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered
        }
        (P::HeatingAvailabilityGuardFalseFallthroughBodyEntered, false) => {
            Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
        (P::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough, false) => {
            Route::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        }
        (P::HumidificationControlGuardFalseFallthroughBodyEntered, true) => {
            Route::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        }
        (P::HumidificationControlGuardFalseFallthroughBodyEntered, false) => {
            Route::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
        (
            P::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough,
            false,
        ) => Route::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (P::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered, true) => {
            Route::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered
        }
        (P::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered, false) => {
            Route::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        }
        (
            P::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough,
            false,
        ) => Route::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (P::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered, true) => {
            Route::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered
        }
        (P::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered, false) => {
            Route::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        }
        (P::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough, false) => {
            Route::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        }
        (P::DehumidificationControlGuardFalseFallthroughBodyEntered, true) => {
            Route::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        }
        (P::DehumidificationControlGuardFalseFallthroughBodyEntered, false) => {
            Route::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
        _ => return None,
    })
}

fn next_transition_fits(
    state: &State,
    predecessor: PredecessorRoute,
    route: Route,
    body: bool,
) -> bool {
    if state.transition_count.checked_add(1).is_none()
        || inherited_route_count(state, predecessor)
            .checked_add(1)
            .is_none()
        || predecessor_route_count(state, predecessor)
            .checked_add(1)
            .is_none()
        || route_count(state, route).checked_add(1).is_none()
    {
        return false;
    }
    if !predecessor_route_is_active(predecessor) {
        return true;
    }
    state
        .source_site_execution_count
        .checked_add(3 + usize::from(body))
        .is_some()
        && [
            state.dehumidification_guard_evaluation_count,
            state.cp378_supply_humidity_ratio_saturation_limit_owned_read_count,
            state.cp379_same_call_supply_humidity_ratio_bit_corroboration_count,
            state.purchased_air_supply_humidity_ratio_read_count,
            state.cp329_mixed_air_humidity_ratio_owned_read_count,
            state.purchased_air_mixed_air_humidity_ratio_read_count,
            state.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count,
        ]
        .into_iter()
        .all(|count| count.checked_add(1).is_some())
        && if body {
            state
                .supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count
                .checked_add(1)
                .is_some()
                && state
                    .dehumidification_body_entry_count
                    .checked_add(1)
                    .is_some()
        } else {
            state
                .dehumidification_guard_false_fallthrough_count
                .checked_add(1)
                .is_some()
        }
}

pub(in crate::ideal_loads::calc) fn inherited_route_count(
    state: &State,
    route: PredecessorRoute,
) -> usize {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => state.unit_off_skip_count,
        P::NonCooling => state.non_cooling_skip_count,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        P::HeatingAvailabilityGuardFalseFallthroughBodyEntered
        | P::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count
        }
        P::HumidificationControlGuardFalseFallthroughBodyEntered
        | P::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered
        | P::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count
        }
        P::DehumidificationControlGuardFalseFallthroughBodyEntered
        | P::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count
        }
    }
}

pub(in crate::ideal_loads::calc) fn predecessor_route_count(
    state: &State,
    route: PredecessorRoute,
) -> usize {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => state.unit_off_skip_count,
        P::NonCooling => state.non_cooling_skip_count,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        P::HeatingAvailabilityGuardFalseFallthroughBodyEntered => {
            state.heating_availability_guard_false_fallthrough_body_entry_count
        }
        P::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count
        }
        P::HumidificationControlGuardFalseFallthroughBodyEntered => {
            state.humidification_control_guard_false_fallthrough_body_entry_count
        }
        P::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered => {
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered => {
            state.dehumidification_control_none_maximum_assignment_body_entry_count
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count
        }
        P::DehumidificationControlGuardFalseFallthroughBodyEntered => {
            state.dehumidification_control_guard_false_fallthrough_body_entry_count
        }
        P::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count
        }
    }
}

pub(in crate::ideal_loads::calc) fn route_count(state: &State, route: Route) -> usize {
    use Route as R;
    match route {
        R::UnitOff => state.unit_off_skip_count,
        R::NonCooling => state.non_cooling_skip_count,
        R::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        R::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count
        }
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered => {
            state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count
        }
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count
        }
        R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count
        }
        R::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => {
            state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count
        }
        R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count
        }
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count
        }
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count
        }
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count
        }
        R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => {
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count
        }
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count
        }
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count
        }
        R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count
        }
        R::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count
        }
        R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count
        }
    }
}
