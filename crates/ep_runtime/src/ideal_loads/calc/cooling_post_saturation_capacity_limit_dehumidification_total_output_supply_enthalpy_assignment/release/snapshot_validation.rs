//! Exact CP385 snapshot and raw binary64 expression validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedInput as RetainedInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Predecessor;

pub(in crate::ideal_loads) fn cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
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
    retained_input: Option<RetainedInput>,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state(
        &mut state,
        predecessor,
        retained_input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(in crate::ideal_loads::calc) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::transition::{
        predecessor_route, predecessor_route_is_assignment,
    };

    if snapshot.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let predecessor = predecessor_shape(snapshot);
    if !crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_control_flow_shape_is_exact(predecessor)
    {
        return None;
    }
    let predecessor_route = predecessor_route(predecessor)?;
    let route = route_from_predecessor_route(predecessor_route);
    let assignment = predecessor_route_is_assignment(predecessor_route);
    let active = predecessor.predecessor_dehumidification_total_output_capacity_guard_evaluated;
    if assignment {
        assignment_fields_are_exact(snapshot).then_some(route)
    } else if active {
        guard_false_fields_are_exact(snapshot).then_some(route)
    } else {
        skipped_fields_are_exact(snapshot).then_some(route)
    }
}

pub(in crate::ideal_loads::calc) fn snapshot_is_exact(snapshot: Snapshot) -> bool {
    snapshot_route(snapshot).is_some()
}

fn predecessor_shape(snapshot: Snapshot) -> Predecessor {
    Predecessor {
        source: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        first_excluded_source: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        preexisting_cooling_total_output_w: None,
        cp383_retained_maximum_total_cooling_capacity_owned_read: false,
        maximum_total_cooling_capacity_read: false,
        maximum_total_cooling_capacity_w: None,
        cooling_total_output_assigned: false,
        assigned_cooling_total_output_w: None,
        resulting_cooling_total_output_w: None,
    }
}

fn route_from_predecessor_route(
    predecessor: crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRetainedRoute,
) -> Route {
    use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRetainedRoute as P;
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

fn skipped_fields_are_exact(snapshot: Snapshot) -> bool {
    !snapshot.supply_enthalpy_assignment_executed
        && snapshot.preexisting_supply_enthalpy_j_per_kg.is_none()
        && !snapshot.cp379_retained_supply_enthalpy_owned_read
        && active_fields_are_absent(snapshot)
        && snapshot.resulting_supply_enthalpy_j_per_kg.is_none()
}

fn guard_false_fields_are_exact(snapshot: Snapshot) -> bool {
    let (Some(preexisting), Some(resulting)) = (
        snapshot.preexisting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) else {
        return false;
    };
    !snapshot.supply_enthalpy_assignment_executed
        && snapshot.cp379_retained_supply_enthalpy_owned_read
        && active_fields_are_absent(snapshot)
        && preexisting.to_bits() == resulting.to_bits()
}

fn active_fields_are_absent(snapshot: Snapshot) -> bool {
    !snapshot.cp329_retained_mixed_air_enthalpy_owned_read
        && !snapshot.mixed_air_enthalpy_read
        && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
        && !snapshot.cp384_retained_cooling_total_output_owned_read
        && !snapshot.cooling_total_output_read
        && snapshot.cooling_total_output_w.is_none()
        && !snapshot.cp330_retained_supply_mass_flow_rate_owned_read
        && !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.specific_cooling_output_calculated
        && snapshot.specific_cooling_output_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_difference_calculated
        && snapshot.calculated_supply_enthalpy_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_assigned
        && snapshot.assigned_supply_enthalpy_j_per_kg.is_none()
}

fn assignment_fields_are_exact(snapshot: Snapshot) -> bool {
    let (
        Some(_preexisting),
        Some(mixed_air),
        Some(output),
        Some(flow),
        Some(specific),
        Some(calculated),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.preexisting_supply_enthalpy_j_per_kg,
        snapshot.mixed_air_enthalpy_j_per_kg,
        snapshot.cooling_total_output_w,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.specific_cooling_output_j_per_kg,
        snapshot.calculated_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    )
    else {
        return false;
    };
    let expected_specific = output / flow;
    let expected_enthalpy = mixed_air - expected_specific;
    snapshot.supply_enthalpy_assignment_executed
        && snapshot.cp379_retained_supply_enthalpy_owned_read
        && snapshot.cp329_retained_mixed_air_enthalpy_owned_read
        && snapshot.mixed_air_enthalpy_read
        && snapshot.cp384_retained_cooling_total_output_owned_read
        && snapshot.cooling_total_output_read
        && snapshot.cp330_retained_supply_mass_flow_rate_owned_read
        && snapshot.supply_mass_flow_rate_read
        && snapshot.specific_cooling_output_calculated
        && snapshot.supply_enthalpy_difference_calculated
        && snapshot.supply_enthalpy_assigned
        && specific.to_bits() == expected_specific.to_bits()
        && calculated.to_bits() == expected_enthalpy.to_bits()
        && assigned.to_bits() == calculated.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_supply_enthalpy_j_per_kg,
            right.preexisting_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.mixed_air_enthalpy_j_per_kg,
            right.mixed_air_enthalpy_j_per_kg,
        ),
        option_bits_match(left.cooling_total_output_w, right.cooling_total_output_w),
        option_bits_match(
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        option_bits_match(
            left.specific_cooling_output_j_per_kg,
            right.specific_cooling_output_j_per_kg,
        ),
        option_bits_match(
            left.calculated_supply_enthalpy_j_per_kg,
            right.calculated_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.assigned_supply_enthalpy_j_per_kg,
            right.assigned_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_supply_enthalpy_j_per_kg = None;
        snapshot.mixed_air_enthalpy_j_per_kg = None;
        snapshot.cooling_total_output_w = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.specific_cooling_output_j_per_kg = None;
        snapshot.calculated_supply_enthalpy_j_per_kg = None;
        snapshot.assigned_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
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
