use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
};

pub(in crate::ideal_loads) fn cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    snapshot_is_exact_source_characterization(snapshot)
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
        && snapshot.predecessor_ems_disabled_fallthrough == snapshot.cooling_body_entered
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    mut right: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    let float_fields_match = option_bits_match(
        left.supply_mass_flow_rate_before_limit_kg_per_s,
        right.supply_mass_flow_rate_before_limit_kg_per_s,
    ) && option_bits_match(
        left.maximum_cooling_air_mass_flow_rate_kg_per_s,
        right.maximum_cooling_air_mass_flow_rate_kg_per_s,
    ) && option_bits_match(
        left.minimum_supply_mass_flow_rate_kg_per_s,
        right.minimum_supply_mass_flow_rate_kg_per_s,
    ) && option_bits_match(
        left.assigned_supply_mass_flow_rate_kg_per_s,
        right.assigned_supply_mass_flow_rate_kg_per_s,
    ) && option_bits_match(
        left.resulting_supply_mass_flow_rate_kg_per_s,
        right.resulting_supply_mass_flow_rate_kg_per_s,
    );
    left.supply_mass_flow_rate_before_limit_kg_per_s = None;
    right.supply_mass_flow_rate_before_limit_kg_per_s = None;
    left.maximum_cooling_air_mass_flow_rate_kg_per_s = None;
    right.maximum_cooling_air_mass_flow_rate_kg_per_s = None;
    left.minimum_supply_mass_flow_rate_kg_per_s = None;
    right.minimum_supply_mass_flow_rate_kg_per_s = None;
    left.assigned_supply_mass_flow_rate_kg_per_s = None;
    right.assigned_supply_mass_flow_rate_kg_per_s = None;
    left.resulting_supply_mass_flow_rate_kg_per_s = None;
    right.resulting_supply_mass_flow_rate_kg_per_s = None;
    float_fields_match && left == right
}

pub(super) fn snapshot_is_exact_source_characterization(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered
        && !snapshot.supply_mass_flow_limit_body_entered
        && !snapshot.active_guard_false_fallthrough;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered
        && !snapshot.supply_mass_flow_limit_body_entered
        && !snapshot.active_guard_false_fallthrough;
    let active_applied = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.cooling_body_entered
        && snapshot.supply_mass_flow_limit_body_entered
        && !snapshot.active_guard_false_fallthrough;
    let active_fallthrough = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.cooling_body_entered
        && !snapshot.supply_mass_flow_limit_body_entered
        && snapshot.active_guard_false_fallthrough;
    let route_count = usize::from(unit_off)
        + usize::from(non_cooling)
        + usize::from(active_applied)
        + usize::from(active_fallthrough);

    provenance
        && route_count == 1
        && snapshot.body_skipped != active_applied
        && if active_applied {
            applied_fields_are_exact(snapshot)
        } else if active_fallthrough {
            fallthrough_fields_are_exact(snapshot)
        } else {
            skipped_fields_are_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> Option<Route> {
    if !snapshot_is_exact_source_characterization(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.supply_mass_flow_limit_body_entered {
        Some(Route::SupplyMassFlowLimitApplied)
    } else {
        Some(Route::ActiveGuardFalseFallthrough)
    }
}

fn applied_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    let Some(supply) = snapshot.supply_mass_flow_rate_before_limit_kg_per_s else {
        return false;
    };
    let Some(maximum) = snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(minimum) = snapshot.minimum_supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(assigned) = snapshot.assigned_supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(resulting) = snapshot.resulting_supply_mass_flow_rate_kg_per_s else {
        return false;
    };

    snapshot.supply_mass_flow_rate_for_minimum_read
        && snapshot.maximum_cooling_air_mass_flow_rate_for_minimum_read
        && snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot.supply_mass_flow_rate_assignment_performed
        && maximum > 0.0
        && minimum.to_bits() == source_min(supply, maximum).to_bits()
        && assigned.to_bits() == minimum.to_bits()
        && resulting.to_bits() == assigned.to_bits()
}

fn fallthrough_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    skipped_lexical_fields_are_exact(snapshot)
        && snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_some()
}

fn skipped_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    skipped_lexical_fields_are_exact(snapshot)
        && snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none()
}

fn skipped_lexical_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> bool {
    !snapshot.supply_mass_flow_rate_for_minimum_read
        && snapshot
            .supply_mass_flow_rate_before_limit_kg_per_s
            .is_none()
        && !snapshot.maximum_cooling_air_mass_flow_rate_for_minimum_read
        && snapshot
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot.minimum_supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.supply_mass_flow_rate_assignment_performed
        && snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none()
}

fn source_min(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
