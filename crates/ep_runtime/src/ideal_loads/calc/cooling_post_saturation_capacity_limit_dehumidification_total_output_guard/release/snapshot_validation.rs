//! Exact CP383 snapshot and raw IEEE comparison validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Predecessor;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release(
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
                | Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered
                | Route::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered
        )
    )
}

pub(super) fn snapshot_links_to_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
) -> bool {
    if snapshot.system != predecessor.system
        || snapshot.parent_call_ordinal != predecessor.parent_call_ordinal
        || snapshot.controlled_zone != predecessor.controlled_zone
        || base_flags(snapshot) != predecessor_base_flags(predecessor)
        || snapshot.predecessor_capacity_limit_guard_evaluated
            != predecessor.predecessor_capacity_limit_guard_evaluated
        || snapshot.predecessor_capacity_limit_body_entered
            != predecessor.predecessor_capacity_limit_body_entered
        || snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            != predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        || snapshot.predecessor_dehumidification_guard_evaluated
            != predecessor.predecessor_dehumidification_guard_evaluated
        || snapshot.predecessor_dehumidification_body_entered
            != predecessor.predecessor_dehumidification_body_entered
        || snapshot.predecessor_dehumidification_guard_false_fallthrough
            != predecessor.predecessor_dehumidification_guard_false_fallthrough
        || snapshot.predecessor_dehumidification_total_output_assignment_executed
            != predecessor.dehumidification_total_output_assignment_executed
    {
        return false;
    }
    let input = if predecessor.dehumidification_total_output_assignment_executed {
        let (Some(output), Some(capacity)) = (
            snapshot.cooling_total_output_w,
            snapshot.maximum_total_cooling_capacity_w,
        ) else {
            return false;
        };
        Some(ActiveInput {
            cooling_total_output_w: output,
            maximum_total_cooling_capacity_w: capacity,
            cp382_cooling_total_output_owned_read: snapshot.cp382_cooling_total_output_owned_read,
            cp321_maximum_total_cooling_capacity_owned_read: snapshot
                .cp321_maximum_total_cooling_capacity_owned_read,
            cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: snapshot
                .cp340_same_call_maximum_total_cooling_capacity_bit_corroborated,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state(
        &mut state,
        predecessor,
        input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER
        || base_flags(snapshot).into_iter().filter(|flag| *flag).count() != 1
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
        return (capacity_guard_false_is_exact(snapshot))
            .then(|| inactive_route(snapshot, InactiveKind::Capacity));
    }
    if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        return (dehumidification_guard_false_is_exact(snapshot))
            .then(|| inactive_route(snapshot, InactiveKind::Dehumidification));
    }
    if !active_guard_is_exact(snapshot) {
        return None;
    }
    Some(active_route(
        snapshot,
        snapshot.dehumidification_total_output_capacity_adjustment_body_entered,
    ))
}

fn complete_skip_is_exact(snapshot: Snapshot) -> bool {
    !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_guard_evaluated
        && !snapshot.predecessor_dehumidification_body_entered
        && !snapshot.predecessor_dehumidification_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_total_output_assignment_executed
        && line_fields_are_skipped(snapshot)
}

fn capacity_guard_false_is_exact(snapshot: Snapshot) -> bool {
    snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_guard_evaluated
        && !snapshot.predecessor_dehumidification_body_entered
        && !snapshot.predecessor_dehumidification_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_total_output_assignment_executed
        && line_fields_are_skipped(snapshot)
}

fn dehumidification_guard_false_is_exact(snapshot: Snapshot) -> bool {
    snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_guard_evaluated
        && !snapshot.predecessor_dehumidification_body_entered
        && snapshot.predecessor_dehumidification_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_total_output_assignment_executed
        && line_fields_are_skipped(snapshot)
}

fn active_guard_is_exact(snapshot: Snapshot) -> bool {
    let (Some(output), Some(capacity), Some(greater)) = (
        snapshot.cooling_total_output_w,
        snapshot.maximum_total_cooling_capacity_w,
        snapshot.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity,
    ) else {
        return false;
    };
    snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_guard_evaluated
        && snapshot.predecessor_dehumidification_body_entered
        && !snapshot.predecessor_dehumidification_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_total_output_assignment_executed
        && snapshot.dehumidification_total_output_capacity_guard_evaluated
        && snapshot.cp382_cooling_total_output_owned_read
        && snapshot.cooling_total_output_read
        && snapshot.cp321_maximum_total_cooling_capacity_owned_read
        && snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
        && snapshot.maximum_total_cooling_capacity_read
        && snapshot.cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated
        && greater == (output > capacity)
        && snapshot.dehumidification_total_output_capacity_adjustment_body_entered == greater
        && snapshot.dehumidification_total_output_capacity_guard_false_fallthrough != greater
}

fn line_fields_are_skipped(snapshot: Snapshot) -> bool {
    !snapshot.dehumidification_total_output_capacity_guard_evaluated
        && !snapshot.cp382_cooling_total_output_owned_read
        && !snapshot.cooling_total_output_read
        && snapshot.cooling_total_output_w.is_none()
        && !snapshot.cp321_maximum_total_cooling_capacity_owned_read
        && !snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
        && !snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_w.is_none()
        && !snapshot.cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated
        && snapshot
            .cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity
            .is_none()
        && !snapshot.dehumidification_total_output_capacity_adjustment_body_entered
        && !snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
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

fn active_route(snapshot: Snapshot, body: bool) -> Route {
    use Route as R;
    match (lineage(snapshot), body) {
        (0, false) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (0, true) => R::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
        (1, false) => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (1, true) => R::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
        (2, false) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (2, true) => R::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
        (3, false) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (3, true) => R::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
        (4, false) => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
        (4, true) => R::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered,
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

fn predecessor_base_flags(predecessor: Predecessor) -> [bool; 8] {
    [
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor.heating_availability_guard_false_fallthrough,
        predecessor.humidification_control_guard_false_fallthrough,
        predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        predecessor.dehumidification_control_none_maximum_assignment_executed,
        predecessor.dehumidification_control_guard_false_fallthrough,
    ]
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = option_bits_match(left.cooling_total_output_w, right.cooling_total_output_w)
        && option_bits_match(
            left.maximum_total_cooling_capacity_w,
            right.maximum_total_cooling_capacity_w,
        );
    for snapshot in [&mut left, &mut right] {
        snapshot.cooling_total_output_w = None;
        snapshot.maximum_total_cooling_capacity_w = None;
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
