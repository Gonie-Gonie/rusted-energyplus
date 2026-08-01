//! Exact CP382 snapshot and grouped IEEE arithmetic validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Predecessor;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    matches!(
        snapshot_route(snapshot),
        Some(
            Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
                | Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned
                | Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned
                | Route::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
        )
    )
}

pub(super) fn snapshot_links_to_predecessor(snapshot: Snapshot, predecessor: Predecessor) -> bool {
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
            != predecessor.dehumidification_guard_evaluated
        || snapshot.predecessor_dehumidification_body_entered
            != predecessor.dehumidification_body_entered
        || snapshot.predecessor_dehumidification_guard_false_fallthrough
            != predecessor.dehumidification_guard_false_fallthrough
    {
        return false;
    }
    let input = if predecessor.dehumidification_body_entered {
        let (Some(flow), Some(mixed), Some(supply)) = (
            snapshot.supply_mass_flow_rate_kg_per_s,
            snapshot.mixed_air_enthalpy_j_per_kg,
            snapshot.supply_enthalpy_j_per_kg,
        ) else {
            return false;
        };
        Some(ActiveInput {
            supply_mass_flow_rate_kg_per_s: flow,
            mixed_air_enthalpy_j_per_kg: mixed,
            supply_enthalpy_j_per_kg: supply,
            cp330_supply_mass_flow_rate_owned_read: snapshot.cp330_supply_mass_flow_rate_owned_read,
            cp329_same_call_supply_mass_flow_rate_bit_corroborated: snapshot
                .cp329_same_call_supply_mass_flow_rate_bit_corroborated,
            cp339_same_call_supply_mass_flow_rate_bit_corroborated: snapshot
                .cp339_same_call_supply_mass_flow_rate_bit_corroborated,
            cp329_mixed_air_enthalpy_owned_read: snapshot.cp329_mixed_air_enthalpy_owned_read,
            cp329_same_call_recirculation_enthalpy_bit_corroborated: snapshot
                .cp329_same_call_recirculation_enthalpy_bit_corroborated,
            cp339_same_call_mixed_air_enthalpy_bit_corroborated: snapshot
                .cp339_same_call_mixed_air_enthalpy_bit_corroborated,
            cp379_post_saturation_supply_enthalpy_owned_read: snapshot
                .cp379_post_saturation_supply_enthalpy_owned_read,
            cp379_same_call_supply_enthalpy_bits_corroborated: snapshot
                .cp379_same_call_supply_enthalpy_bits_corroborated,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_state(
        &mut state,
        predecessor,
        input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER
        || base_flags(snapshot)
            .into_iter()
            .filter(|selected| *selected)
            .count()
            != 1
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
        if !capacity_guard_false_is_exact(snapshot) {
            return None;
        }
        return Some(capacity_guard_false_route(snapshot));
    }
    if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        if !dehumidification_guard_false_is_exact(snapshot) {
            return None;
        }
        return Some(dehumidification_guard_false_route(snapshot));
    }
    if !active_assignment_is_exact(snapshot) {
        return None;
    }
    Some(assignment_route(snapshot))
}

fn complete_skip_is_exact(snapshot: Snapshot) -> bool {
    !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.predecessor_dehumidification_guard_evaluated
        && !snapshot.predecessor_dehumidification_body_entered
        && !snapshot.predecessor_dehumidification_guard_false_fallthrough
        && line_fields_are_skipped(snapshot)
}

fn capacity_guard_false_is_exact(snapshot: Snapshot) -> bool {
    snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_dehumidification_guard_evaluated
        && !snapshot.predecessor_dehumidification_body_entered
        && !snapshot.predecessor_dehumidification_guard_false_fallthrough
        && line_fields_are_skipped(snapshot)
}

fn dehumidification_guard_false_is_exact(snapshot: Snapshot) -> bool {
    snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_guard_evaluated
        && !snapshot.predecessor_dehumidification_body_entered
        && line_fields_are_skipped(snapshot)
}

fn active_assignment_is_exact(snapshot: Snapshot) -> bool {
    let (Some(flow), Some(mixed), Some(supply), Some(difference), Some(calculated), Some(assigned)) = (
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.mixed_air_enthalpy_j_per_kg,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.mixed_air_minus_supply_enthalpy_j_per_kg,
        snapshot.calculated_cooling_total_output_w,
        snapshot.cooling_total_output_w,
    ) else {
        return false;
    };
    let expected_difference = mixed - supply;
    let expected_output = flow * expected_difference;
    snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_guard_evaluated
        && snapshot.predecessor_dehumidification_body_entered
        && !snapshot.predecessor_dehumidification_guard_false_fallthrough
        && snapshot.dehumidification_total_output_assignment_executed
        && snapshot.cp330_supply_mass_flow_rate_owned_read
        && snapshot.cp329_same_call_supply_mass_flow_rate_bit_corroborated
        && snapshot.cp339_same_call_supply_mass_flow_rate_bit_corroborated
        && snapshot.supply_mass_flow_rate_read
        && snapshot.cp329_mixed_air_enthalpy_owned_read
        && snapshot.cp329_same_call_recirculation_enthalpy_bit_corroborated
        && snapshot.cp339_same_call_mixed_air_enthalpy_bit_corroborated
        && snapshot.mixed_air_enthalpy_read
        && snapshot.cp379_post_saturation_supply_enthalpy_owned_read
        && snapshot.cp379_same_call_supply_enthalpy_bits_corroborated
        && snapshot.supply_enthalpy_read
        && snapshot.enthalpy_difference_calculated
        && difference.to_bits() == expected_difference.to_bits()
        && snapshot.cooling_total_output_calculated
        && calculated.to_bits() == expected_output.to_bits()
        && snapshot.cooling_total_output_assigned
        && assigned.to_bits() == expected_output.to_bits()
}

fn line_fields_are_skipped(snapshot: Snapshot) -> bool {
    !snapshot.dehumidification_total_output_assignment_executed
        && !snapshot.cp330_supply_mass_flow_rate_owned_read
        && !snapshot.cp329_same_call_supply_mass_flow_rate_bit_corroborated
        && !snapshot.cp339_same_call_supply_mass_flow_rate_bit_corroborated
        && !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.cp329_mixed_air_enthalpy_owned_read
        && !snapshot.cp329_same_call_recirculation_enthalpy_bit_corroborated
        && !snapshot.cp339_same_call_mixed_air_enthalpy_bit_corroborated
        && !snapshot.mixed_air_enthalpy_read
        && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
        && !snapshot.cp379_post_saturation_supply_enthalpy_owned_read
        && !snapshot.cp379_same_call_supply_enthalpy_bits_corroborated
        && !snapshot.supply_enthalpy_read
        && snapshot.supply_enthalpy_j_per_kg.is_none()
        && !snapshot.enthalpy_difference_calculated
        && snapshot.mixed_air_minus_supply_enthalpy_j_per_kg.is_none()
        && !snapshot.cooling_total_output_calculated
        && snapshot.calculated_cooling_total_output_w.is_none()
        && !snapshot.cooling_total_output_assigned
        && snapshot.cooling_total_output_w.is_none()
}

fn capacity_guard_false_route(snapshot: Snapshot) -> Route {
    if snapshot.heating_availability_guard_false_fallthrough {
        Route::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
    } else if snapshot.humidification_control_guard_false_fallthrough {
        Route::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
    } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
        Route::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
    } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
        Route::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough
    } else {
        Route::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
    }
}

fn dehumidification_guard_false_route(snapshot: Snapshot) -> Route {
    if snapshot.heating_availability_guard_false_fallthrough {
        Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
    } else if snapshot.humidification_control_guard_false_fallthrough {
        Route::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
    } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
        Route::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
    } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
        Route::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough
    } else {
        Route::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
    }
}

fn assignment_route(snapshot: Snapshot) -> Route {
    if snapshot.heating_availability_guard_false_fallthrough {
        Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned
    } else if snapshot.humidification_control_guard_false_fallthrough {
        Route::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned
    } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
        Route::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned
    } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
        Route::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned
    } else {
        Route::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned
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
    let values_match =
        option_bits_match(
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ) && option_bits_match(
            left.mixed_air_enthalpy_j_per_kg,
            right.mixed_air_enthalpy_j_per_kg,
        ) && option_bits_match(
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        ) && option_bits_match(
            left.mixed_air_minus_supply_enthalpy_j_per_kg,
            right.mixed_air_minus_supply_enthalpy_j_per_kg,
        ) && option_bits_match(
            left.calculated_cooling_total_output_w,
            right.calculated_cooling_total_output_w,
        ) && option_bits_match(left.cooling_total_output_w, right.cooling_total_output_w);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.mixed_air_enthalpy_j_per_kg = None;
        snapshot.supply_enthalpy_j_per_kg = None;
        snapshot.mixed_air_minus_supply_enthalpy_j_per_kg = None;
        snapshot.calculated_cooling_total_output_w = None;
        snapshot.cooling_total_output_w = None;
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
