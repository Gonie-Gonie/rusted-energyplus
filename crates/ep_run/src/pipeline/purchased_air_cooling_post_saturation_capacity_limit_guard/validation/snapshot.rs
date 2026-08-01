use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot as SelectorSnapshot,
};

use super::*;

pub(super) fn metadata_is_exact(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> bool {
    snapshot.source == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && [snapshot.system, predecessor.system]
            .into_iter()
            .all(|system| system == expected_system)
        && [snapshot.controlled_zone, predecessor.controlled_zone]
            .into_iter()
            .all(|zone| zone == expected_zone)
        && [snapshot.parent_call_ordinal, predecessor.parent_call_ordinal]
            .into_iter()
            .all(|ordinal| ordinal == calls)
}

pub(super) fn links_exactly(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    selector: SelectorSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    let active = predecessor.local_supply_enthalpy_after_saturation_limit_assignment_performed;
    route_flags(snapshot) == predecessor_route_flags(predecessor)
        && route_flags(snapshot)
            .into_iter()
            .filter(|route| *route)
            .count()
            == 1
        && snapshot.predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed
            == active
        && snapshot.capacity_limit_guard_evaluated == active
        && active_or_null_shape(snapshot, cooling_limit, active)
        && selector_corroborates(selector, snapshot, cooling_limit, active)
}

fn active_or_null_shape(snapshot: Snapshot, cooling_limit: IdealLoadsLimit, active: bool) -> bool {
    if !active {
        return !snapshot.configured_cooling_limit_owned_read
            && !snapshot.cp337_same_call_selector_lineage_corroborated
            && !snapshot.first_cooling_limit_read
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
            && !snapshot.active_guard_false_fallthrough;
    }
    let capacity_match = cooling_limit == IdealLoadsLimit::LimitCapacity;
    let second = !capacity_match;
    let combined = second && cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let selected = capacity_match || combined;
    snapshot.configured_cooling_limit_owned_read
        && snapshot.cp337_same_call_selector_lineage_corroborated
        && snapshot.first_cooling_limit_read
        && snapshot.first_cooling_limit == Some(cooling_limit)
        && snapshot.cooling_limit_capacity_comparison_evaluated
        && snapshot.cooling_limit_capacity == Some(capacity_match)
        && snapshot.second_cooling_limit_read == second
        && snapshot.second_cooling_limit == second.then_some(cooling_limit)
        && snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated == second
        && snapshot.cooling_limit_flow_rate_and_capacity == second.then_some(combined)
        && snapshot.cooling_limit_condition_satisfied == Some(selected)
        && snapshot.cooling_limit_rejected != selected
        && snapshot.capacity_limit_body_entered == selected
        && snapshot.active_guard_false_fallthrough != selected
}

fn selector_corroborates(
    selector: SelectorSnapshot,
    snapshot: Snapshot,
    cooling_limit: IdealLoadsLimit,
    active: bool,
) -> bool {
    if !active {
        return true;
    }
    selector.source == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        && selector.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        && selector.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER
        && selector.system == snapshot.system
        && selector.parent_call_ordinal == snapshot.parent_call_ordinal
        && selector.controlled_zone == snapshot.controlled_zone
        && selector.capacity_limit_guard_evaluated
        && selector.first_cooling_limit == Some(cooling_limit)
        && selector.second_cooling_limit
            == (cooling_limit != IdealLoadsLimit::LimitCapacity).then_some(cooling_limit)
}

fn route_flags(snapshot: Snapshot) -> [bool; 8] {
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

fn predecessor_route_flags(snapshot: PredecessorSnapshot) -> [bool; 8] {
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
