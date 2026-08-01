//! Exact CP380 snapshot validation without numerical operands.

use ep_model::IdealLoadsLimit;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_guard_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Predecessor;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    matches!(
        snapshot_route(snapshot),
        Some(
            Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthroughBodyEntered
                | Route::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughBodyEntered
                | Route::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough
        )
    )
}

pub(super) fn snapshot_links_to_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    if snapshot.system != predecessor.system
        || snapshot.parent_call_ordinal != predecessor.parent_call_ordinal
        || snapshot.controlled_zone != predecessor.controlled_zone
        || snapshot.unit_off_skipped != predecessor.unit_off_skipped
        || snapshot.non_cooling_skipped != predecessor.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
        || snapshot.heating_availability_guard_false_fallthrough
            != predecessor.heating_availability_guard_false_fallthrough
        || snapshot.humidification_control_guard_false_fallthrough
            != predecessor.humidification_control_guard_false_fallthrough
        || snapshot.dehumidification_control_humidistat_maximum_assignment_executed
            != predecessor.dehumidification_control_humidistat_maximum_assignment_executed
        || snapshot.dehumidification_control_none_maximum_assignment_executed
            != predecessor.dehumidification_control_none_maximum_assignment_executed
        || snapshot.dehumidification_control_guard_false_fallthrough
            != predecessor.dehumidification_control_guard_false_fallthrough
        || snapshot.predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed
            != predecessor.local_supply_enthalpy_after_saturation_limit_assignment_performed
    {
        return false;
    }
    let active = predecessor.local_supply_enthalpy_after_saturation_limit_assignment_performed;
    let input = active.then_some(ActiveInput {
        cooling_limit,
        cp337_same_call_selector_lineage_corroborated: true,
    });
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_guard_state(&mut state, predecessor, input)
        .is_some_and(|expected| expected == snapshot)
}

pub(in crate::ideal_loads::calc) fn predecessor_route_is_active(
    predecessor: Predecessor,
) -> Option<bool> {
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
    (flags.into_iter().filter(|flag| *flag).count() == 1).then_some(
        !(predecessor.unit_off_skipped
            || predecessor.non_cooling_skipped
            || predecessor.positive_guard_false_fallthrough_skipped),
    )
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER
    {
        return None;
    }
    let flags = [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ];
    if flags.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    if snapshot.unit_off_skipped {
        return skipped_fields_are_exact(snapshot).then_some(Route::UnitOff);
    }
    if snapshot.non_cooling_skipped {
        return skipped_fields_are_exact(snapshot).then_some(Route::NonCooling);
    }
    if snapshot.positive_guard_false_fallthrough_skipped {
        return skipped_fields_are_exact(snapshot).then_some(Route::PositiveGuardFalseFallthrough);
    }
    if !active_fields_are_exact(snapshot) {
        return None;
    }
    let body = snapshot.capacity_limit_body_entered;
    Some(if snapshot.heating_availability_guard_false_fallthrough {
        if body {
            Route::HeatingAvailabilityGuardFalseFallthroughBodyEntered
        } else {
            Route::HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough
        }
    } else if snapshot.humidification_control_guard_false_fallthrough {
        if body {
            Route::HumidificationControlGuardFalseFallthroughBodyEntered
        } else {
            Route::HumidificationControlGuardFalseFallthroughGuardFalseFallthrough
        }
    } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
        if body {
            Route::DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered
        } else {
            Route::DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough
        }
    } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
        if body {
            Route::DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered
        } else {
            Route::DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough
        }
    } else if body {
        Route::DehumidificationControlGuardFalseFallthroughBodyEntered
    } else {
        Route::DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough
    })
}

fn active_fields_are_exact(snapshot: Snapshot) -> bool {
    let Some(first_limit) = snapshot.first_cooling_limit else {
        return false;
    };
    let capacity = first_limit == IdealLoadsLimit::LimitCapacity;
    let second_expected = !capacity;
    let combined = snapshot
        .second_cooling_limit
        .map(|limit| limit == IdealLoadsLimit::LimitFlowRateAndCapacity);
    let satisfied = capacity || combined == Some(true);

    snapshot.predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed
        && snapshot.capacity_limit_guard_evaluated
        && snapshot.configured_cooling_limit_owned_read
        && snapshot.cp337_same_call_selector_lineage_corroborated
        && snapshot.first_cooling_limit_read
        && snapshot.cooling_limit_capacity_comparison_evaluated
        && snapshot.cooling_limit_capacity == Some(capacity)
        && snapshot.second_cooling_limit_read == second_expected
        && snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated == second_expected
        && if second_expected {
            snapshot.second_cooling_limit == Some(first_limit)
                && snapshot.cooling_limit_flow_rate_and_capacity == combined
        } else {
            snapshot.second_cooling_limit.is_none()
                && snapshot.cooling_limit_flow_rate_and_capacity.is_none()
        }
        && snapshot.cooling_limit_condition_satisfied == Some(satisfied)
        && snapshot.cooling_limit_rejected != satisfied
        && snapshot.capacity_limit_body_entered == satisfied
        && snapshot.active_guard_false_fallthrough != satisfied
        && snapshot.cooling_limit_rejected == snapshot.active_guard_false_fallthrough
}

fn skipped_fields_are_exact(snapshot: Snapshot) -> bool {
    !snapshot.predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed
        && !snapshot.capacity_limit_guard_evaluated
        && !snapshot.configured_cooling_limit_owned_read
        && !snapshot.cp337_same_call_selector_lineage_corroborated
        && !snapshot.first_cooling_limit_read
        && snapshot.first_cooling_limit.is_none()
        && !snapshot.cooling_limit_capacity_comparison_evaluated
        && snapshot.cooling_limit_capacity.is_none()
        && !snapshot.second_cooling_limit_read
        && snapshot.second_cooling_limit.is_none()
        && !snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated
        && snapshot.cooling_limit_flow_rate_and_capacity.is_none()
        && snapshot.cooling_limit_condition_satisfied.is_none()
        && !snapshot.cooling_limit_rejected
        && !snapshot.capacity_limit_body_entered
        && !snapshot.active_guard_false_fallthrough
}

pub(in crate::ideal_loads::calc) fn snapshots_match_exact(left: Snapshot, right: Snapshot) -> bool {
    left == right
}
