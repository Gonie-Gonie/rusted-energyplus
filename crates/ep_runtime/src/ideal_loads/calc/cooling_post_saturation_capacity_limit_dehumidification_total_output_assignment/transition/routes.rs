//! CP381 predecessor and CP382 retained-route classification.

use super::{Predecessor, Route};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE as PREDECESSOR_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE as PREDECESSOR_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER as PREDECESSOR_SOURCE_ORDER,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PredecessorRoute {
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered,
    HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered,
    HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
    DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
    DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered,
    DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
    DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
    DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered,
    DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
    DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
    DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered,
    DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
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
        return complete_skip_is_exact(predecessor).then_some(PredecessorRoute::UnitOff);
    }
    if predecessor.non_cooling_skipped {
        return complete_skip_is_exact(predecessor).then_some(PredecessorRoute::NonCooling);
    }
    if predecessor.positive_guard_false_fallthrough_skipped {
        return complete_skip_is_exact(predecessor)
            .then_some(PredecessorRoute::PositiveGuardFalseFallthrough);
    }
    if predecessor.predecessor_active_capacity_limit_guard_false_fallthrough {
        if !capacity_guard_false_is_exact(predecessor) {
            return None;
        }
        return Some(capacity_guard_false_route(predecessor));
    }
    if !dehumidification_guard_is_exact(predecessor) {
        return None;
    }
    Some(dehumidification_route(predecessor))
}

fn complete_skip_is_exact(predecessor: Predecessor) -> bool {
    !predecessor.predecessor_capacity_limit_guard_evaluated
        && !predecessor.predecessor_capacity_limit_body_entered
        && !predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && line_fields_are_skipped(predecessor)
}

fn capacity_guard_false_is_exact(predecessor: Predecessor) -> bool {
    predecessor.predecessor_capacity_limit_guard_evaluated
        && !predecessor.predecessor_capacity_limit_body_entered
        && line_fields_are_skipped(predecessor)
}

fn dehumidification_guard_is_exact(predecessor: Predecessor) -> bool {
    if !predecessor.predecessor_capacity_limit_guard_evaluated
        || !predecessor.predecessor_capacity_limit_body_entered
        || predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
    {
        return false;
    }
    let (Some(supply), Some(mixed), Some(less)) = (
        predecessor.supply_humidity_ratio,
        predecessor.mixed_air_humidity_ratio,
        predecessor.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio,
    ) else {
        return false;
    };
    predecessor.dehumidification_guard_evaluated
        && predecessor.cp378_supply_humidity_ratio_saturation_limit_owned_read
        && predecessor.cp379_same_call_supply_humidity_ratio_bit_corroborated
        && predecessor.purchased_air_supply_humidity_ratio_read
        && predecessor.cp329_mixed_air_humidity_ratio_owned_read
        && predecessor.purchased_air_mixed_air_humidity_ratio_read
        && predecessor.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated
        && less == (supply < mixed)
        && predecessor.dehumidification_body_entered == less
        && predecessor.dehumidification_guard_false_fallthrough != less
}

fn line_fields_are_skipped(predecessor: Predecessor) -> bool {
    !predecessor.dehumidification_guard_evaluated
        && !predecessor.cp378_supply_humidity_ratio_saturation_limit_owned_read
        && !predecessor.cp379_same_call_supply_humidity_ratio_bit_corroborated
        && !predecessor.purchased_air_supply_humidity_ratio_read
        && predecessor.supply_humidity_ratio.is_none()
        && !predecessor.cp329_mixed_air_humidity_ratio_owned_read
        && !predecessor.purchased_air_mixed_air_humidity_ratio_read
        && predecessor.mixed_air_humidity_ratio.is_none()
        && !predecessor.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated
        && predecessor
            .supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio
            .is_none()
        && !predecessor.dehumidification_body_entered
        && !predecessor.dehumidification_guard_false_fallthrough
}

fn capacity_guard_false_route(predecessor: Predecessor) -> PredecessorRoute {
    if predecessor.heating_availability_guard_false_fallthrough {
        PredecessorRoute::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
    } else if predecessor.humidification_control_guard_false_fallthrough {
        PredecessorRoute::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
    } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
        PredecessorRoute::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
    } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
        PredecessorRoute::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
    } else {
        PredecessorRoute::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
    }
}

fn dehumidification_route(predecessor: Predecessor) -> PredecessorRoute {
    use PredecessorRoute as P;
    let body = predecessor.dehumidification_body_entered;
    if predecessor.heating_availability_guard_false_fallthrough {
        if body {
            P::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered
        } else {
            P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
    } else if predecessor.humidification_control_guard_false_fallthrough {
        if body {
            P::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        } else {
            P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
    } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
        if body {
            P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered
        } else {
            P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        }
    } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
        if body {
            P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered
        } else {
            P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        }
    } else if body {
        P::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
    } else {
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
    }
}

pub(in crate::ideal_loads::calc) fn predecessor_route_is_assignment(
    route: PredecessorRoute,
) -> bool {
    matches!(
        route,
        PredecessorRoute::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered
            | PredecessorRoute::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
            | PredecessorRoute::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered
            | PredecessorRoute::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered
            | PredecessorRoute::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
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
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned,
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        P::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned,
        P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned,
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned,
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned,
        P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
    }
}
