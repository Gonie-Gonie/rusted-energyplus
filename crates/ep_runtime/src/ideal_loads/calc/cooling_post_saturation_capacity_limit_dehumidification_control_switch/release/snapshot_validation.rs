//! Exact CP386 snapshot, route, and bit-preservation validation.

use ep_model::DehumidificationControlType;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Snapshot,
};
use super::super::transition::routes::{PredecessorRoute as P, RetainedRoute};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Cp384Snapshot;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    let Some(route) = snapshot_route(snapshot) else {
        return false;
    };
    matches!(
        route.predecessor,
        P::UnitOff
            | P::NonCooling
            | P::PositiveGuardFalseFallthrough
            | P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
            | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough
            | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
            | P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough
            | P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough
            | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
    ) && route.selected_case.is_none_or(|selector| selector == DehumidificationControlType::None)
}

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact(
    snapshot: Snapshot,
) -> bool {
    snapshot_route(snapshot).is_some()
}

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshots_match_bit_exact(
    left: Snapshot,
    right: Snapshot,
) -> bool {
    snapshots_match_bit_exact(left, right)
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<RetainedRoute> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE
        || snapshot.first_excluded_lexical_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER
    {
        return None;
    }
    if !crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_control_flow_shape_is_exact(
        snapshot_control_shape(snapshot),
    ) {
        return None;
    }
    let predecessor = predecessor_route_from_snapshot(snapshot)?;
    let active = predecessor.is_assignment();
    if snapshot.dehumidification_control_type_read != active
        || snapshot.dehumidification_control_switch_dispatched != active
        || snapshot.dehumidification_control_type.is_some() != active
        || !super::super::transition::routes::selector_is_allowed(
            predecessor,
            snapshot.dehumidification_control_type,
        )
    {
        return None;
    }
    let enthalpy_matches = match (
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) {
        (Some(predecessor), Some(resulting)) => predecessor.to_bits() == resulting.to_bits(),
        (None, None) => true,
        _ => false,
    };
    if !enthalpy_matches
        || snapshot.predecessor_supply_enthalpy_assignment_executed != active
        || (predecessor_has_enthalpy(predecessor)
            != snapshot.resulting_supply_enthalpy_j_per_kg.is_some())
    {
        return None;
    }
    Some(RetainedRoute {
        predecessor,
        selected_case: snapshot.dehumidification_control_type,
    })
}

fn snapshot_control_shape(snapshot: Snapshot) -> Cp384Snapshot {
    Cp384Snapshot {
        source: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        first_excluded_source: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot
            .predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot
            .dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot
            .dehumidification_total_output_maximum_capacity_assignment_executed,
        preexisting_cooling_total_output_w: None,
        cp383_retained_maximum_total_cooling_capacity_owned_read: false,
        maximum_total_cooling_capacity_read: false,
        maximum_total_cooling_capacity_w: None,
        cooling_total_output_assigned: false,
        assigned_cooling_total_output_w: None,
        resulting_cooling_total_output_w: None,
    }
}

fn predecessor_route_from_snapshot(snapshot: Snapshot) -> Option<P> {
    let base = [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ];
    if base.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    if snapshot.unit_off_skipped {
        return Some(P::UnitOff);
    }
    if snapshot.non_cooling_skipped {
        return Some(P::NonCooling);
    }
    if snapshot.positive_guard_false_fallthrough_skipped {
        return Some(P::PositiveGuardFalseFallthrough);
    }
    let lineage = if snapshot.heating_availability_guard_false_fallthrough {
        0
    } else if snapshot.humidification_control_guard_false_fallthrough {
        1
    } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
        2
    } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
        3
    } else {
        4
    };
    let stages = [
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
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

const fn predecessor_has_enthalpy(route: P) -> bool {
    use P as Route;
    matches!(
        route,
        Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | Route::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
            | Route::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | Route::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
            | Route::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | Route::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned
            | Route::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | Route::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned
            | Route::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
            | Route::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
    )
}

pub(super) fn snapshots_match_bit_exact(mut left: Snapshot, mut right: Snapshot) -> bool {
    let predecessor_matches = option_bits_match(
        left.predecessor_resulting_supply_enthalpy_j_per_kg,
        right.predecessor_resulting_supply_enthalpy_j_per_kg,
    );
    let resulting_matches = option_bits_match(
        left.resulting_supply_enthalpy_j_per_kg,
        right.resulting_supply_enthalpy_j_per_kg,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
    }
    predecessor_matches && resulting_matches && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
