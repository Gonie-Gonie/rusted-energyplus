//! Exact CP384 snapshot and raw binary64 assignment validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Predecessor;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    matches!(
        snapshot_route(snapshot),
        Some(
            Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
                | Route::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
        )
    )
}

pub(super) fn snapshot_links_to_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state(
        &mut state,
        predecessor,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_control_flow_shape_is_exact(snapshot)
    {
        return None;
    }
    if snapshot.unit_off_skipped {
        return complete_skip_is_exact(snapshot).then_some(Route::UnitOff);
    }
    if snapshot.non_cooling_skipped {
        return complete_skip_is_exact(snapshot).then_some(Route::NonCooling);
    }
    if snapshot.positive_guard_false_fallthrough_skipped {
        return complete_skip_is_exact(snapshot).then_some(Route::PositiveGuardFalseFallthrough);
    }
    if snapshot.predecessor_active_capacity_limit_guard_false_fallthrough {
        return capacity_guard_false_is_exact(snapshot)
            .then(|| inactive_route(snapshot, InactiveKind::Capacity));
    }
    if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        return dehumidification_guard_false_is_exact(snapshot)
            .then(|| inactive_route(snapshot, InactiveKind::Dehumidification));
    }
    if snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough {
        guard_false_is_exact(snapshot).then(|| active_route(snapshot, false))
    } else if snapshot
        .predecessor_dehumidification_total_output_capacity_adjustment_body_entered
    {
        assignment_is_exact(snapshot).then(|| active_route(snapshot, true))
    } else {
        None
    }
}

pub(in crate::ideal_loads::calc) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
}

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_control_flow_shape_is_exact(
    snapshot: Snapshot,
) -> bool {
    if base_flags(snapshot)
        .into_iter()
        .filter(|flag| *flag)
        .count()
        != 1
    {
        return false;
    }
    if snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
    {
        complete_skip_control_flow_is_exact(snapshot)
    } else if snapshot.predecessor_active_capacity_limit_guard_false_fallthrough {
        capacity_guard_false_control_flow_is_exact(snapshot)
    } else if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        dehumidification_guard_false_control_flow_is_exact(snapshot)
    } else if snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough {
        guard_false_control_flow_is_exact(snapshot)
    } else if snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered {
        assignment_control_flow_is_exact(snapshot)
    } else {
        false
    }
}

fn complete_skip_is_exact(snapshot: Snapshot) -> bool {
    complete_skip_control_flow_is_exact(snapshot) && line_fields_are_skipped(snapshot)
}

fn complete_skip_control_flow_is_exact(snapshot: Snapshot) -> bool {
    !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_guard_evaluated
        && !snapshot.predecessor_dehumidification_body_entered
        && !snapshot.predecessor_dehumidification_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_total_output_assignment_executed
        && !snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated
        && !snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered
        && !snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
        && !snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
        && !snapshot.dehumidification_total_output_maximum_capacity_assignment_executed
}

fn capacity_guard_false_is_exact(snapshot: Snapshot) -> bool {
    capacity_guard_false_control_flow_is_exact(snapshot) && line_fields_are_skipped(snapshot)
}

fn capacity_guard_false_control_flow_is_exact(snapshot: Snapshot) -> bool {
    snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_guard_evaluated
        && !snapshot.predecessor_dehumidification_body_entered
        && !snapshot.predecessor_dehumidification_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_total_output_assignment_executed
        && !snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated
        && !snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered
        && !snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
        && !snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
        && !snapshot.dehumidification_total_output_maximum_capacity_assignment_executed
}

fn dehumidification_guard_false_is_exact(snapshot: Snapshot) -> bool {
    dehumidification_guard_false_control_flow_is_exact(snapshot)
        && line_fields_are_skipped(snapshot)
}

fn dehumidification_guard_false_control_flow_is_exact(snapshot: Snapshot) -> bool {
    snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_guard_evaluated
        && !snapshot.predecessor_dehumidification_body_entered
        && snapshot.predecessor_dehumidification_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_total_output_assignment_executed
        && !snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated
        && !snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered
        && !snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
        && !snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
        && !snapshot.dehumidification_total_output_maximum_capacity_assignment_executed
}

fn active_prefix_is_exact(snapshot: Snapshot) -> bool {
    snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_guard_evaluated
        && snapshot.predecessor_dehumidification_body_entered
        && !snapshot.predecessor_dehumidification_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_total_output_assignment_executed
        && snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated
}

fn line_fields_are_skipped(snapshot: Snapshot) -> bool {
    !snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
        && !snapshot.dehumidification_total_output_maximum_capacity_assignment_executed
        && snapshot.preexisting_cooling_total_output_w.is_none()
        && !snapshot.cp383_retained_maximum_total_cooling_capacity_owned_read
        && !snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_w.is_none()
        && !snapshot.cooling_total_output_assigned
        && snapshot.assigned_cooling_total_output_w.is_none()
        && snapshot.resulting_cooling_total_output_w.is_none()
}

fn guard_false_is_exact(snapshot: Snapshot) -> bool {
    let (Some(preexisting), Some(resulting)) = (
        snapshot.preexisting_cooling_total_output_w,
        snapshot.resulting_cooling_total_output_w,
    ) else {
        return false;
    };
    guard_false_control_flow_is_exact(snapshot)
        && !snapshot.cp383_retained_maximum_total_cooling_capacity_owned_read
        && !snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_w.is_none()
        && !snapshot.cooling_total_output_assigned
        && snapshot.assigned_cooling_total_output_w.is_none()
        && preexisting.to_bits() == resulting.to_bits()
}

fn guard_false_control_flow_is_exact(snapshot: Snapshot) -> bool {
    active_prefix_is_exact(snapshot)
        && !snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered
        && snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
        && snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
        && !snapshot.dehumidification_total_output_maximum_capacity_assignment_executed
}

fn assignment_is_exact(snapshot: Snapshot) -> bool {
    let (Some(preexisting), Some(maximum), Some(assigned), Some(resulting)) = (
        snapshot.preexisting_cooling_total_output_w,
        snapshot.maximum_total_cooling_capacity_w,
        snapshot.assigned_cooling_total_output_w,
        snapshot.resulting_cooling_total_output_w,
    ) else {
        return false;
    };
    assignment_control_flow_is_exact(snapshot)
        && snapshot.cp383_retained_maximum_total_cooling_capacity_owned_read
        && snapshot.maximum_total_cooling_capacity_read
        && snapshot.cooling_total_output_assigned
        && preexisting > maximum
        && maximum.to_bits() == assigned.to_bits()
        && maximum.to_bits() == resulting.to_bits()
}

fn assignment_control_flow_is_exact(snapshot: Snapshot) -> bool {
    active_prefix_is_exact(snapshot)
        && snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered
        && !snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
        && !snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
        && snapshot.dehumidification_total_output_maximum_capacity_assignment_executed
}

enum InactiveKind {
    Capacity,
    Dehumidification,
}

fn inactive_route(snapshot: Snapshot, kind: InactiveKind) -> Route {
    use InactiveKind::{Capacity, Dehumidification};
    use Route as R;
    match (lineage(snapshot), kind) {
        (0, Capacity) => R::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (0, Dehumidification) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (1, Capacity) => R::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (1, Dehumidification) => R::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        (2, Capacity) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (2, Dehumidification) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        (3, Capacity) => R::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
        (3, Dehumidification) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
        (4, Capacity) => R::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
        (4, Dehumidification) => R::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
        _ => unreachable!(),
    }
}

fn active_route(snapshot: Snapshot, assignment: bool) -> Route {
    use Route as R;
    match (lineage(snapshot), assignment) {
        (0, false) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (0, true) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
        (1, false) => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (1, true) => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
        (2, false) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (2, true) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
        (3, false) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (3, true) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
        (4, false) => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (4, true) => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
        _ => unreachable!(),
    }
}

fn lineage(snapshot: Snapshot) -> usize {
    if snapshot.heating_availability_guard_false_fallthrough {
        0
    } else if snapshot.humidification_control_guard_false_fallthrough {
        1
    } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
        2
    } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
        3
    } else {
        4
    }
}

fn base_flags(snapshot: Snapshot) -> [bool; 8] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ]
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        (left.preexisting_cooling_total_output_w, right.preexisting_cooling_total_output_w),
        (left.maximum_total_cooling_capacity_w, right.maximum_total_cooling_capacity_w),
        (left.assigned_cooling_total_output_w, right.assigned_cooling_total_output_w),
        (left.resulting_cooling_total_output_w, right.resulting_cooling_total_output_w),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_cooling_total_output_w = None;
        snapshot.maximum_total_cooling_capacity_w = None;
        snapshot.assigned_cooling_total_output_w = None;
        snapshot.resulting_cooling_total_output_w = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
