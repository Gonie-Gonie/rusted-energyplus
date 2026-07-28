//! Exact CP342 snapshot validation.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
};

pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let positive_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
        && snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let capacity_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_capacity_limit_guard_evaluated
        && !snapshot.predecessor_capacity_limit_body_entered
        && snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.capacity_limit_guard_false_fallthrough_skipped
        && !snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let active_prefix = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
        && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
        && !snapshot.capacity_limit_guard_false_fallthrough_skipped
        && snapshot.predecessor_capacity_limit_cp_air_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated;
    let guard_false = active_prefix
        && snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && !snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let assigned = active_prefix
        && !snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot
            .capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(positive_false)
            + usize::from(capacity_false)
            + usize::from(guard_false)
            + usize::from(assigned)
            == 1
        && if guard_false {
            false_fallthrough_snapshot_is_exact(snapshot)
        } else if assigned {
            assigned_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> Option<Route> {
    if !cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
        snapshot,
    ) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.positive_guard_false_fallthrough_skipped {
        Some(Route::PositiveGuardFalseFallthrough)
    } else if snapshot.capacity_limit_guard_false_fallthrough_skipped {
        Some(Route::ActiveCapacityLimitGuardFalseFallthrough)
    } else if snapshot
        .capacity_limit_sensible_output_supply_enthalpy_assignment_executed
    {
        Some(Route::CapacityLimitSensibleOutputSupplyEnthalpyAssigned)
    } else {
        Some(Route::CapacityLimitSensibleOutputGuardFalseFallthrough)
    }
}

fn false_fallthrough_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let (Some(preexisting), Some(resulting)) = (
        snapshot.preexisting_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ) else {
        return false;
    };
    preexisting.is_finite()
        && preexisting.to_bits() == resulting.to_bits()
        && source_values_are_none(snapshot)
}

fn assigned_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let (
        Some(preexisting),
        Some(mixed_air),
        Some(output),
        Some(flow),
        Some(quotient),
        Some(calculated),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.preexisting_supply_enthalpy_j_per_kg,
        snapshot.mixed_air_enthalpy_j_per_kg,
        snapshot.cooling_sensible_output_w,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.specific_cooling_output_j_per_kg,
        snapshot.calculated_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    )
    else {
        return false;
    };
    let expected_quotient = output / flow;
    let expected_enthalpy = mixed_air - expected_quotient;
    preexisting.is_finite()
        && snapshot.mixed_air_enthalpy_read
        && mixed_air.is_finite()
        && snapshot.cooling_sensible_output_read
        && output.is_finite()
        && output > 0.0
        && snapshot.supply_mass_flow_rate_read
        && flow > 0.0
        && snapshot.specific_cooling_output_calculated
        && quotient.to_bits() == expected_quotient.to_bits()
        && snapshot.supply_enthalpy_calculated
        && calculated.to_bits() == expected_enthalpy.to_bits()
        && snapshot.supply_enthalpy_assigned
        && assigned.to_bits() == calculated.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

fn skipped_snapshot_is_exact(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
        && !snapshot
            .capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && snapshot.preexisting_supply_enthalpy_j_per_kg.is_none()
        && snapshot.resulting_supply_enthalpy_j_per_kg.is_none()
        && source_values_are_none(snapshot)
}

fn source_values_are_none(
    snapshot:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    !snapshot.mixed_air_enthalpy_read
        && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
        && !snapshot.cooling_sensible_output_read
        && snapshot.cooling_sensible_output_w.is_none()
        && !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.specific_cooling_output_calculated
        && snapshot.specific_cooling_output_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_calculated
        && snapshot.calculated_supply_enthalpy_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_assigned
        && snapshot.assigned_supply_enthalpy_j_per_kg.is_none()
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
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
        option_bits_match(
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
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
        snapshot.cooling_sensible_output_w = None;
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
