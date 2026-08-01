//! CP383 checked route and source-site accounting.

use super::routes::{predecessor_route_count, successor_route_count};
use super::{PredecessorRoute, State};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRetainedRoute as Route;

pub(super) fn increment_counts(
    state: &mut State,
    predecessor: PredecessorRoute,
    route: Route,
    active: bool,
    body: bool,
) {
    increment_inherited(state, predecessor);
    increment_predecessor(state, predecessor);
    if active {
        increment_successor(state, route);
        state.dehumidification_total_output_capacity_guard_evaluation_count += 1;
        state.source_site_execution_count += 3 + usize::from(body);
        state.cp382_cooling_total_output_owned_read_count += 1;
        state.cooling_total_output_read_count += 1;
        state.cp321_maximum_total_cooling_capacity_owned_read_count += 1;
        state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count += 1;
        state.maximum_total_cooling_capacity_read_count += 1;
        state.cooling_total_output_maximum_total_cooling_capacity_comparison_count += 1;
        if body {
            state.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count +=
                1;
            state.dehumidification_total_output_capacity_adjustment_body_entry_count += 1;
        } else {
            state.dehumidification_total_output_capacity_guard_false_fallthrough_count += 1;
        }
    }
}

pub(super) fn next_transition_fits(
    state: &State,
    predecessor: PredecessorRoute,
    route: Route,
    body: bool,
) -> bool {
    let active = super::predecessor_route_is_active(predecessor);
    if state.transition_count.checked_add(1).is_none()
        || inherited_count(state, predecessor).checked_add(1).is_none()
        || predecessor_route_count(state, predecessor)
            .checked_add(1)
            .is_none()
        || successor_route_count(state, route).checked_add(1).is_none()
    {
        return false;
    }
    if !active {
        return true;
    }
    state
        .source_site_execution_count
        .checked_add(3 + usize::from(body))
        .is_some()
        && [
            state.dehumidification_total_output_capacity_guard_evaluation_count,
            state.cp382_cooling_total_output_owned_read_count,
            state.cooling_total_output_read_count,
            state.cp321_maximum_total_cooling_capacity_owned_read_count,
            state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
            state.maximum_total_cooling_capacity_read_count,
            state.cooling_total_output_maximum_total_cooling_capacity_comparison_count,
        ]
        .into_iter()
        .all(|count| count.checked_add(1).is_some())
        && if body {
            state
                .cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count
                .checked_add(1)
                .is_some()
                && state
                    .dehumidification_total_output_capacity_adjustment_body_entry_count
                    .checked_add(1)
                    .is_some()
        } else {
            state
                .dehumidification_total_output_capacity_guard_false_fallthrough_count
                .checked_add(1)
                .is_some()
        }
}

pub(super) fn inherited_count(state: &State, route: PredecessorRoute) -> usize {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => state.unit_off_skip_count,
        P::NonCooling => state.non_cooling_skip_count,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned => {
            state.heating_availability_guard_false_fallthrough_count
        }
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => {
            state.humidification_control_guard_false_fallthrough_count
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count
        }
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => {
            state.dehumidification_control_guard_false_fallthrough_count
        }
    }
}

fn increment_inherited(state: &mut State, route: PredecessorRoute) {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => state.unit_off_skip_count += 1,
        P::NonCooling => state.non_cooling_skip_count += 1,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count += 1,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.heating_availability_guard_false_fallthrough_count += 1,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.humidification_control_guard_false_fallthrough_count += 1,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count += 1,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count += 1,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.dehumidification_control_guard_false_fallthrough_count += 1,
    }
}

fn increment_predecessor(state: &mut State, route: PredecessorRoute) {
    use PredecessorRoute as P;
    match route {
        P::UnitOff | P::NonCooling | P::PositiveGuardFalseFallthrough => {}
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_capacity_guard_false_count += 1,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count += 1,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count += 1,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_capacity_guard_false_count += 1,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count += 1,
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count += 1,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count += 1,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count += 1,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count += 1,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count += 1,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count += 1,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count += 1,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count += 1,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count += 1,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count += 1,
    }
}

fn increment_successor(state: &mut State, route: Route) {
    use Route as R;
    match route {
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count += 1,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count += 1,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count += 1,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count += 1,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count += 1,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count += 1,
        _ => {}
    }
}
