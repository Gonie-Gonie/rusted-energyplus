use ep_model::IdealLoadsLimit;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
};

pub(in crate::ideal_loads) fn cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
) -> bool {
    snapshot_is_exact_source_characterization(snapshot)
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
}

pub(super) fn snapshot_is_exact_source_characterization(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
        && !snapshot.predecessor_ems_disabled_fallthrough;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.cooling_body_entered
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
        && !snapshot.predecessor_ems_disabled_fallthrough;
    let active_disabled = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.cooling_body_entered
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
        && snapshot.predecessor_ems_disabled_fallthrough;
    let active_override = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.cooling_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
        && !snapshot.predecessor_ems_disabled_fallthrough;
    let route_count = usize::from(unit_off)
        + usize::from(non_cooling)
        + usize::from(active_disabled)
        + usize::from(active_override);

    provenance
        && route_count == 1
        && if snapshot.cooling_body_entered {
            active_fields_are_exact(snapshot)
        } else {
            skipped_fields_are_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
) -> Option<Route> {
    if !snapshot_is_exact_source_characterization(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.cooling_limit_condition_satisfied != Some(true) {
        Some(Route::CoolingLimitRejected)
    } else if snapshot.supply_mass_flow_limit_body_entered {
        Some(Route::FlowLimitBodyEntered)
    } else {
        Some(Route::MaximumCoolingMassFlowNotPositive)
    }
}

fn active_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
) -> bool {
    let Some(first_limit) = snapshot.first_cooling_limit else {
        return false;
    };
    let first_match = first_limit == IdealLoadsLimit::LimitFlowRate;
    let second_expected = !first_match;
    let second_match = snapshot
        .second_cooling_limit
        .map(|limit| limit == IdealLoadsLimit::LimitFlowRateAndCapacity);
    let limit_satisfied = first_match || second_match == Some(true);
    let maximum_expected = limit_satisfied;
    let maximum_positive = snapshot
        .maximum_cooling_air_mass_flow_rate_kg_per_s
        .map(|maximum| maximum > 0.0);
    let body_entered = maximum_positive == Some(true);

    snapshot.first_cooling_limit_read
        && snapshot.cooling_limit_flow_rate_comparison_evaluated
        && snapshot.cooling_limit_flow_rate == Some(first_match)
        && snapshot.second_cooling_limit_read == second_expected
        && snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated == second_expected
        && if second_expected {
            snapshot.second_cooling_limit == Some(first_limit)
                && snapshot.cooling_limit_flow_rate_and_capacity == second_match
        } else {
            snapshot.second_cooling_limit.is_none()
                && snapshot.cooling_limit_flow_rate_and_capacity.is_none()
        }
        && snapshot.cooling_limit_condition_satisfied == Some(limit_satisfied)
        && snapshot.maximum_cooling_air_mass_flow_rate_read == maximum_expected
        && snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated
            == maximum_expected
        && if maximum_expected {
            snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_some()
                && snapshot.maximum_cooling_air_mass_flow_rate_strictly_positive == maximum_positive
        } else {
            snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_none()
                && snapshot
                    .maximum_cooling_air_mass_flow_rate_strictly_positive
                    .is_none()
        }
        && snapshot.supply_mass_flow_limit_body_entered == body_entered
        && snapshot.active_guard_false_fallthrough != body_entered
}

fn skipped_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
) -> bool {
    !snapshot.first_cooling_limit_read
        && snapshot.first_cooling_limit.is_none()
        && !snapshot.cooling_limit_flow_rate_comparison_evaluated
        && snapshot.cooling_limit_flow_rate.is_none()
        && !snapshot.second_cooling_limit_read
        && snapshot.second_cooling_limit.is_none()
        && !snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated
        && snapshot.cooling_limit_flow_rate_and_capacity.is_none()
        && snapshot.cooling_limit_condition_satisfied.is_none()
        && !snapshot.maximum_cooling_air_mass_flow_rate_read
        && snapshot
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated
        && snapshot
            .maximum_cooling_air_mass_flow_rate_strictly_positive
            .is_none()
        && !snapshot.supply_mass_flow_limit_body_entered
        && !snapshot.active_guard_false_fallthrough
}
