//! Exact CP337 snapshot validation.

use ep_model::IdealLoadsLimit;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
};

pub(in crate::ideal_loads) fn cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.capacity_limit_guard_evaluated;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.capacity_limit_guard_evaluated;
    let positive_guard_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
        && snapshot.positive_guard_false_fallthrough_skipped
        && !snapshot.capacity_limit_guard_evaluated;
    let active = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.capacity_limit_guard_evaluated;

    provenance
        && usize::from(unit_off)
            + usize::from(non_cooling)
            + usize::from(positive_guard_false)
            + usize::from(active)
            == 1
        && if active {
            active_fields_are_exact(snapshot)
        } else {
            skipped_fields_are_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> Option<Route> {
    if !cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.positive_guard_false_fallthrough_skipped {
        Some(Route::PositiveGuardFalseFallthrough)
    } else if snapshot.capacity_limit_body_entered {
        Some(Route::CapacityLimitBodyEntered)
    } else {
        Some(Route::ActiveCapacityLimitGuardFalseFallthrough)
    }
}

fn active_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> bool {
    let Some(first_limit) = snapshot.first_cooling_limit else {
        return false;
    };
    let capacity = first_limit == IdealLoadsLimit::LimitCapacity;
    let second_expected = !capacity;
    let combined = snapshot
        .second_cooling_limit
        .map(|limit| limit == IdealLoadsLimit::LimitFlowRateAndCapacity);
    let satisfied = capacity || combined == Some(true);

    snapshot.first_cooling_limit_read
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

fn skipped_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
) -> bool {
    !snapshot.first_cooling_limit_read
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
