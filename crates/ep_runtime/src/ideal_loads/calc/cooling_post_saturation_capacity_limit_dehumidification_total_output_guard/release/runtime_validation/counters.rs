//! CP383 exact counter arrays and route indexes.

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
            state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
            state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count,
            state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count,
            state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
        ],
    )
}

pub(super) fn active_counters(state: &State) -> [usize; 7] {
    [
        state.dehumidification_total_output_capacity_guard_evaluation_count,
        state.cp382_cooling_total_output_owned_read_count,
        state.cooling_total_output_read_count,
        state.cp321_maximum_total_cooling_capacity_owned_read_count,
        state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        state.maximum_total_cooling_capacity_read_count,
        state.cooling_total_output_maximum_total_cooling_capacity_comparison_count,
    ]
}

pub(super) fn inherited_index(route: PredecessorRoute) -> usize {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => 0,
        P::NonCooling => 1,
        P::PositiveGuardFalseFallthrough => 2,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned => 3,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => 4,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => 5,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => 6,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => 7,
    }
}

pub(super) fn partition_index(route: PredecessorRoute) -> Option<usize> {
    use PredecessorRoute as P;
    Some(match route {
        P::UnitOff | P::NonCooling | P::PositiveGuardFalseFallthrough => return None,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 0,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 1,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned => 2,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 3,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 4,
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => 5,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => 6,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 7,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => 8,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => 9,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 10,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => 11,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 12,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 13,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => 14,
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
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
        R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_capacity_adjustment_body_entry_count,
        R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_guard_false_fallthrough_count,
        R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_capacity_adjustment_body_entry_count,
    }
}

pub(super) fn checked_sum(values: &[usize]) -> Option<usize> {
    values
        .iter()
        .try_fold(0usize, |sum, value| sum.checked_add(*value))
}
