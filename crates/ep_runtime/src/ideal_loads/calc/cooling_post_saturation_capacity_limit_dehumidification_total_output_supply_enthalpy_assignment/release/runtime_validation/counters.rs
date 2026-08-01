//! CP385 exact counter arrays and route indexes.

use super::{PredecessorRoute, PredecessorState, Route, State};

pub(super) fn inherited_counts(state: &State) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

pub(super) fn prior_inherited_counts(state: &PredecessorState) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

pub(super) fn predecessor_partitions(state: &State) -> [usize; 15] {
    [
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
    ]
}

pub(super) fn prior_partitions(state: &PredecessorState) -> [usize; 15] {
    [
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
    ]
}

pub(super) fn successor_partitions(state: &State) -> ([usize; 5], [usize; 5]) {
    (
        [
            state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
            state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
            state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        ],
        [
            state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
            state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
            state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        ],
    )
}

pub(super) fn prior_successor_partitions(
    state: &PredecessorState,
) -> ([usize; 5], [usize; 5]) {
    (
        [
            state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
            state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
            state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        ],
        [
            state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
            state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
            state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        ],
    )
}

pub(super) fn inherited_index(route: PredecessorRoute) -> usize {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => 0,
        P::NonCooling => 1,
        P::PositiveGuardFalseFallthrough => 2,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => 3,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => 4,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => 5,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => 6,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => 7,
    }
}

pub(super) fn partition_index(route: PredecessorRoute) -> Option<usize> {
    use PredecessorRoute as P;
    Some(match route {
        P::UnitOff | P::NonCooling | P::PositiveGuardFalseFallthrough => return None,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 0,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 1,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => 2,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 3,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 4,
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => 5,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => 6,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 7,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => 8,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => 9,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 10,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => 11,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 12,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 13,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => 14,
    })
}

pub(super) fn successor_index(route: PredecessorRoute) -> Option<(bool, usize)> {
    use PredecessorRoute as P;
    Some(match route {
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => (false, 0),
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => (true, 0),
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => (false, 1),
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => (true, 1),
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => (false, 2),
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => (true, 2),
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => (false, 3),
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => (true, 3),
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => (false, 4),
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => (true, 4),
        _ => return None,
    })
}

pub(super) fn route_count(state: &State, route: Route) -> usize {
    use Route as R;
    match route {
        R::UnitOff => state.unit_off_skip_count,
        R::NonCooling => state.non_cooling_skip_count,
        R::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        R::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_maximum_capacity_assignment_count,
        R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_maximum_capacity_assignment_count,
    }
}

pub(super) fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
