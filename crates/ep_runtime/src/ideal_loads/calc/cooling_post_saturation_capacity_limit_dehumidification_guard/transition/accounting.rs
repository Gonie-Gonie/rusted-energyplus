//! CP381 inherited, predecessor, and retained-route counter increments.

use super::{PredecessorRoute, Route, State};

pub(super) fn increment_inherited_route_count(state: &mut State, route: PredecessorRoute) {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => state.unit_off_skip_count += 1,
        P::NonCooling => state.non_cooling_skip_count += 1,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count += 1,
        P::HeatingAvailabilityGuardFalseFallthroughBodyEntered
        | P::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_count += 1;
        }
        P::HumidificationControlGuardFalseFallthroughBodyEntered
        | P::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_count += 1;
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered
        | P::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count += 1;
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered
        | P::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count += 1;
        }
        P::DehumidificationControlGuardFalseFallthroughBodyEntered
        | P::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_count += 1;
        }
    }
}

pub(super) fn increment_predecessor_route_count(state: &mut State, route: PredecessorRoute) {
    use PredecessorRoute as P;
    match route {
        P::UnitOff | P::NonCooling | P::PositiveGuardFalseFallthrough => {}
        P::HeatingAvailabilityGuardFalseFallthroughBodyEntered => {
            state.heating_availability_guard_false_fallthrough_body_entry_count += 1;
        }
        P::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count += 1;
        }
        P::HumidificationControlGuardFalseFallthroughBodyEntered => {
            state.humidification_control_guard_false_fallthrough_body_entry_count += 1;
        }
        P::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count += 1;
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered => {
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count += 1;
        }
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count += 1;
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered => {
            state.dehumidification_control_none_maximum_assignment_body_entry_count += 1;
        }
        P::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough => {
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count += 1;
        }
        P::DehumidificationControlGuardFalseFallthroughBodyEntered => {
            state.dehumidification_control_guard_false_fallthrough_body_entry_count += 1;
        }
        P::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count += 1;
        }
    }
}

pub(super) fn increment_route_count(state: &mut State, route: Route) {
    use Route as R;
    match route {
        R::UnitOff
        | R::NonCooling
        | R::PositiveGuardFalseFallthrough
        | R::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        | R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        | R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => {}
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered => {
            state.heating_availability_guard_false_fallthrough_dehumidification_body_entry_count += 1;
        }
        R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count += 1;
        }
        R::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => {
            state.humidification_control_guard_false_fallthrough_dehumidification_body_entry_count += 1;
        }
        R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count += 1;
        }
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count += 1;
        }
        R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count += 1;
        }
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count += 1;
        }
        R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count += 1;
        }
        R::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count += 1;
        }
        R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => {
            state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count += 1;
        }
    }
}
