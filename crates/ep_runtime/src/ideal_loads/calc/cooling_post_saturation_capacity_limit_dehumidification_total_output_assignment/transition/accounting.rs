//! CP382 inherited, predecessor-route, and assignment-route accounting.

use super::{PredecessorRoute as P, Route as R, State};

pub(super) fn increment_route_counts(state: &mut State, predecessor: P, route: R) {
    increment_inherited_count(state, predecessor);
    increment_predecessor_partition_count(state, predecessor);
    increment_assignment_route_count(state, route);
}

pub(super) fn next_route_counters_fit(state: &State, predecessor: P, route: R) -> bool {
    inherited_count(state, predecessor).checked_add(1).is_some()
        && predecessor_partition_count(state, predecessor)
            .is_none_or(|count| count.checked_add(1).is_some())
        && assignment_route_count(state, route).is_none_or(|count| count.checked_add(1).is_some())
}

fn increment_inherited_count(state: &mut State, route: P) {
    match route {
        P::UnitOff => state.unit_off_skip_count += 1,
        P::NonCooling => state.non_cooling_skip_count += 1,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count += 1,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count += 1;
        }
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count += 1;
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count += 1;
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count += 1;
        }
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count += 1;
        }
    }
}

fn increment_predecessor_partition_count(state: &mut State, route: P) {
    match route {
        P::UnitOff | P::NonCooling | P::PositiveGuardFalseFallthrough => {}
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count += 1;
        }
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered => {
            state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count += 1;
        }
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count += 1;
        }
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count += 1;
        }
        P::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => {
            state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count += 1;
        }
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count += 1;
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count += 1;
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count += 1;
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count += 1;
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => {
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count += 1;
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count += 1;
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count += 1;
        }
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count += 1;
        }
        P::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count += 1;
        }
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count += 1;
        }
    }
}

fn increment_assignment_route_count(state: &mut State, route: R) {
    match route {
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned => {
            state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count += 1;
        }
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => {
            state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count += 1;
        }
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count += 1;
        }
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count += 1;
        }
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count += 1;
        }
        _ => {}
    }
}

fn inherited_count(state: &State, route: P) -> usize {
    match route {
        P::UnitOff => state.unit_off_skip_count,
        P::NonCooling => state.non_cooling_skip_count,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count
        }
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count
        }
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count
        }
    }
}

fn predecessor_partition_count(state: &State, route: P) -> Option<usize> {
    Some(match route {
        P::UnitOff | P::NonCooling | P::PositiveGuardFalseFallthrough => return None,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered => state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        P::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered => state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
    })
}

fn assignment_route_count(state: &State, route: R) -> Option<usize> {
    Some(match route {
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        _ => return None,
    })
}
