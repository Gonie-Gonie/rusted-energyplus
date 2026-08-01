//! CP384 predecessor and CP385 retained-route classification.

use super::{Predecessor, State};
pub(in crate::ideal_loads::calc) use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRetainedRoute as PredecessorRoute;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedRoute as Route;

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<PredecessorRoute> {
    use PredecessorRoute as P;
    let base = [
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor.heating_availability_guard_false_fallthrough,
        predecessor.humidification_control_guard_false_fallthrough,
        predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        predecessor.dehumidification_control_none_maximum_assignment_executed,
        predecessor.dehumidification_control_guard_false_fallthrough,
    ];
    if base.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    if predecessor.unit_off_skipped {
        return Some(P::UnitOff);
    }
    if predecessor.non_cooling_skipped {
        return Some(P::NonCooling);
    }
    if predecessor.positive_guard_false_fallthrough_skipped {
        return Some(P::PositiveGuardFalseFallthrough);
    }
    let lineage = if predecessor.heating_availability_guard_false_fallthrough {
        0
    } else if predecessor.humidification_control_guard_false_fallthrough {
        1
    } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
        2
    } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
        3
    } else {
        4
    };
    let stages = [
        predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor.predecessor_dehumidification_guard_false_fallthrough,
        predecessor.dehumidification_total_output_capacity_guard_false_fallthrough,
        predecessor.dehumidification_total_output_maximum_capacity_assignment_executed,
    ];
    if stages.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    Some(match (lineage, stages) {
        (0, [true, false, false, false]) => P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (0, [false, true, false, false]) => P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (0, [false, false, true, false]) => P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (0, [false, false, false, true]) => P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
        (1, [true, false, false, false]) => P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (1, [false, true, false, false]) => P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (1, [false, false, true, false]) => P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (1, [false, false, false, true]) => P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
        (2, [true, false, false, false]) => P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (2, [false, true, false, false]) => P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        (2, [false, false, true, false]) => P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (2, [false, false, false, true]) => P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
        (3, [true, false, false, false]) => P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (3, [false, true, false, false]) => P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        (3, [false, false, true, false]) => P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (3, [false, false, false, true]) => P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
        (4, [true, false, false, false]) => P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (4, [false, true, false, false]) => P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (4, [false, false, true, false]) => P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (4, [false, false, false, true]) => P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
        _ => return None,
    })
}

pub(in crate::ideal_loads::calc) fn predecessor_route_is_assignment(
    route: PredecessorRoute,
) -> bool {
    use PredecessorRoute as P;
    matches!(
        route,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
            | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
            | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned
            | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned
            | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
    )
}

pub(super) fn predecessor_route_is_guard_evaluated(route: PredecessorRoute) -> bool {
    use PredecessorRoute as P;
    matches!(
        route,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
            | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
            | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned
            | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned
            | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
    )
}

pub(super) fn retained_route(predecessor: PredecessorRoute) -> Route {
    use PredecessorRoute as P;
    use Route as R;
    match predecessor {
        P::UnitOff => R::UnitOff,
        P::NonCooling => R::NonCooling,
        P::PositiveGuardFalseFallthrough => R::PositiveGuardFalseFallthrough,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => R::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
    }
}

pub(super) fn predecessor_route_count(state: &State, route: PredecessorRoute) -> usize {
    use PredecessorRoute as P;
    match route {
        P::UnitOff => state.unit_off_skip_count,
        P::NonCooling => state.non_cooling_skip_count,
        P::PositiveGuardFalseFallthrough => state.positive_guard_false_fallthrough_skip_count,
        P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.heating_availability_guard_false_fallthrough_dehumidification_guard_false_count,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.heating_availability_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.humidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.humidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_humidistat_maximum_assignment_dehumidification_total_output_assignment_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => state.dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_none_maximum_assignment_dehumidification_total_output_assignment_count,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => state.dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => state.dehumidification_control_guard_false_fallthrough_dehumidification_total_output_assignment_count,
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
