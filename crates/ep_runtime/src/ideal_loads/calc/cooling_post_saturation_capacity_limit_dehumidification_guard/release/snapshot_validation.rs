//! Exact CP381 snapshot and raw binary64 comparison validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Predecessor;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    matches!(
        snapshot_route(snapshot),
        Some(
            Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered
                | Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        )
    )
}

pub(super) fn snapshot_links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
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
        || snapshot.predecessor_capacity_limit_guard_evaluated
            != predecessor.capacity_limit_guard_evaluated
        || snapshot.predecessor_capacity_limit_body_entered
            != predecessor.capacity_limit_body_entered
        || snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            != predecessor.active_guard_false_fallthrough
    {
        return false;
    }
    let input = if predecessor.capacity_limit_body_entered {
        let (Some(supply), Some(mixed)) = (
            snapshot.supply_humidity_ratio,
            snapshot.mixed_air_humidity_ratio,
        ) else {
            return false;
        };
        Some(ActiveInput {
            supply_humidity_ratio: supply,
            mixed_air_humidity_ratio: mixed,
            cp378_supply_humidity_ratio_saturation_limit_owned_read: snapshot
                .cp378_supply_humidity_ratio_saturation_limit_owned_read,
            cp379_same_call_supply_humidity_ratio_bit_corroborated: snapshot
                .cp379_same_call_supply_humidity_ratio_bit_corroborated,
            cp329_mixed_air_humidity_ratio_owned_read: snapshot
                .cp329_mixed_air_humidity_ratio_owned_read,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state(
        &mut state,
        predecessor,
        input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER
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
        return predecessor_complete_skip_is_exact(snapshot).then_some(Route::UnitOff);
    }
    if snapshot.non_cooling_skipped {
        return predecessor_complete_skip_is_exact(snapshot).then_some(Route::NonCooling);
    }
    if snapshot.positive_guard_false_fallthrough_skipped {
        return predecessor_complete_skip_is_exact(snapshot)
            .then_some(Route::PositiveGuardFalseFallthrough);
    }
    if !snapshot.predecessor_capacity_limit_guard_evaluated
        || (snapshot.predecessor_capacity_limit_body_entered
            == snapshot.predecessor_active_capacity_limit_guard_false_fallthrough)
    {
        return None;
    }
    if snapshot.predecessor_active_capacity_limit_guard_false_fallthrough {
        if !skipped_fields_are_exact(snapshot) {
            return None;
        }
        return Some(if snapshot.heating_availability_guard_false_fallthrough {
            Route::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        } else if snapshot.humidification_control_guard_false_fallthrough {
            Route::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
            Route::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
            Route::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
        } else {
            Route::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
        });
    }
    if !active_fields_are_exact(snapshot) {
        return None;
    }
    let body = snapshot.dehumidification_body_entered;
    Some(if snapshot.heating_availability_guard_false_fallthrough {
        if body {
            Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered
        } else {
            Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
    } else if snapshot.humidification_control_guard_false_fallthrough {
        if body {
            Route::HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
        } else {
            Route::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        }
    } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
        if body {
            Route::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered
        } else {
            Route::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        }
    } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
        if body {
            Route::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered
        } else {
            Route::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
        }
    } else if body {
        Route::DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered
    } else {
        Route::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
    })
}

fn predecessor_complete_skip_is_exact(snapshot: Snapshot) -> bool {
    !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && skipped_fields_are_exact(snapshot)
}

fn active_fields_are_exact(snapshot: Snapshot) -> bool {
    let (Some(supply), Some(mixed)) = (
        snapshot.supply_humidity_ratio,
        snapshot.mixed_air_humidity_ratio,
    ) else {
        return false;
    };
    let less = supply < mixed;
    snapshot.dehumidification_guard_evaluated
        && snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read
        && snapshot.cp379_same_call_supply_humidity_ratio_bit_corroborated
        && snapshot.purchased_air_supply_humidity_ratio_read
        && snapshot.cp329_mixed_air_humidity_ratio_owned_read
        && snapshot.purchased_air_mixed_air_humidity_ratio_read
        && snapshot.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated
        && snapshot.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio == Some(less)
        && snapshot.dehumidification_body_entered == less
        && snapshot.dehumidification_guard_false_fallthrough != less
}

fn skipped_fields_are_exact(snapshot: Snapshot) -> bool {
    !snapshot.dehumidification_guard_evaluated
        && !snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read
        && !snapshot.cp379_same_call_supply_humidity_ratio_bit_corroborated
        && !snapshot.purchased_air_supply_humidity_ratio_read
        && snapshot.supply_humidity_ratio.is_none()
        && !snapshot.cp329_mixed_air_humidity_ratio_owned_read
        && !snapshot.purchased_air_mixed_air_humidity_ratio_read
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated
        && snapshot
            .supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio
            .is_none()
        && !snapshot.dehumidification_body_entered
        && !snapshot.dehumidification_guard_false_fallthrough
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = option_bits_match(left.supply_humidity_ratio, right.supply_humidity_ratio)
        && option_bits_match(
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        );
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_humidity_ratio = None;
        snapshot.mixed_air_humidity_ratio = None;
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
