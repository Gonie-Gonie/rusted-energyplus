//! CP382 predecessor and CP383 retained-route classification.

use super::{Predecessor, State};
pub(in crate::ideal_loads::calc) use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRetainedRoute as PredecessorRoute;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_route as snapshot_route;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<PredecessorRoute> {
    snapshot_route(predecessor)
}

pub(in crate::ideal_loads::calc) fn predecessor_route_is_active(
    route: PredecessorRoute,
) -> bool {
    use PredecessorRoute as P;
    matches!(
        route,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned
            | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned
            | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned
            | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned
            | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned
    )
}

pub(super) fn retained_route(
    predecessor: PredecessorRoute,
    body: bool,
) -> Option<Route> {
    use PredecessorRoute as P;
    use Route as R;
    Some(match (predecessor, body) {
        (P::UnitOff, false) => R::UnitOff,
        (P::NonCooling, false) => R::NonCooling,
        (P::PositiveGuardFalseFallthrough, false) => R::PositiveGuardFalseFallthrough,
        (P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough, false) => R::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough, false) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned, false) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned, true) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
        (P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough, false) => R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough, false) => R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned, false) => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned, true) => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
        (P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough, false) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough, false) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        (P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned, false) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned, true) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
        (P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough, false) => R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough, false) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        (P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned, false) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned, true) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
        (P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough, false) => R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough, false) => R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned, false) => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned, true) => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
        _ => return None,
    })
}

pub(super) fn predecessor_route_count(state: &State, route: PredecessorRoute) -> usize {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => state.unit_off_skip_count,
        P::NonCooling => state.non_cooling_skip_count,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
    }
}

pub(super) fn successor_route_count(state: &State, route: Route) -> usize {
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
