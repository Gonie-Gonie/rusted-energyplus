use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
};

pub(in crate::ideal_loads) fn cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    snapshot_is_exact_source_characterization(snapshot)
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
        && snapshot.predecessor_ems_disabled_fallthrough == snapshot.cooling_body_entered
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    mut right: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    let float_fields_match = option_bits_match(
        left.predecessor_supply_mass_flow_rate_kg_per_s,
        right.predecessor_supply_mass_flow_rate_kg_per_s,
    ) && option_bits_match(
        left.assigned_supply_mass_flow_rate_kg_per_s,
        right.assigned_supply_mass_flow_rate_kg_per_s,
    ) && option_bits_match(
        left.resulting_supply_mass_flow_rate_kg_per_s,
        right.resulting_supply_mass_flow_rate_kg_per_s,
    );
    left.predecessor_supply_mass_flow_rate_kg_per_s = None;
    right.predecessor_supply_mass_flow_rate_kg_per_s = None;
    left.assigned_supply_mass_flow_rate_kg_per_s = None;
    right.assigned_supply_mass_flow_rate_kg_per_s = None;
    left.resulting_supply_mass_flow_rate_kg_per_s = None;
    right.resulting_supply_mass_flow_rate_kg_per_s = None;
    float_fields_match && left == right
}

pub(super) fn snapshot_is_exact_source_characterization(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER;
    let predecessor_limit_route_is_exact = if snapshot.cooling_body_entered {
        snapshot.predecessor_supply_mass_flow_limit_body_skipped
            != snapshot.predecessor_supply_mass_flow_limit_body_entered
            && snapshot.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
                == snapshot.predecessor_supply_mass_flow_limit_body_skipped
    } else {
        !snapshot.predecessor_supply_mass_flow_limit_body_entered
            && snapshot.predecessor_supply_mass_flow_limit_body_skipped
            && !snapshot.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough
    };
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered
        && !snapshot.predecessor_zero_flow_reset_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.zero_flow_reset_body_entered
        && !snapshot.active_guard_false_fallthrough;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered
        && !snapshot.predecessor_zero_flow_reset_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.zero_flow_reset_body_entered
        && !snapshot.active_guard_false_fallthrough;
    let active_assigned = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.cooling_body_entered
        && snapshot.predecessor_zero_flow_reset_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && snapshot.zero_flow_reset_body_entered
        && !snapshot.active_guard_false_fallthrough;
    let active_fallthrough = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.cooling_body_entered
        && !snapshot.predecessor_zero_flow_reset_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.zero_flow_reset_body_entered
        && snapshot.active_guard_false_fallthrough;
    let route_count = usize::from(unit_off)
        + usize::from(non_cooling)
        + usize::from(active_assigned)
        + usize::from(active_fallthrough);

    provenance
        && predecessor_limit_route_is_exact
        && route_count == 1
        && snapshot.body_skipped != active_assigned
        && if active_assigned {
            assigned_fields_are_exact(snapshot)
        } else if active_fallthrough {
            fallthrough_fields_are_exact(snapshot)
        } else {
            skipped_fields_are_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> Option<Route> {
    if !snapshot_is_exact_source_characterization(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.zero_flow_reset_body_entered {
        Some(Route::PositiveZeroAssigned)
    } else {
        Some(Route::ActiveGuardFalseFallthrough)
    }
}

fn assigned_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    snapshot
        .predecessor_supply_mass_flow_rate_kg_per_s
        .is_some()
        && snapshot.supply_mass_flow_rate_positive_zero_assignment_performed
        && snapshot
            .assigned_supply_mass_flow_rate_kg_per_s
            .is_some_and(|assigned| assigned.to_bits() == 0)
        && snapshot
            .resulting_supply_mass_flow_rate_kg_per_s
            .is_some_and(|resulting| resulting.to_bits() == 0)
}

fn fallthrough_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    let Some(predecessor) = snapshot.predecessor_supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    !snapshot.supply_mass_flow_rate_positive_zero_assignment_performed
        && snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none()
        && snapshot
            .resulting_supply_mass_flow_rate_kg_per_s
            .is_some_and(|resulting| resulting.to_bits() == predecessor.to_bits())
}

fn skipped_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    snapshot
        .predecessor_supply_mass_flow_rate_kg_per_s
        .is_none()
        && !snapshot.supply_mass_flow_rate_positive_zero_assignment_performed
        && snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none()
        && snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none()
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
