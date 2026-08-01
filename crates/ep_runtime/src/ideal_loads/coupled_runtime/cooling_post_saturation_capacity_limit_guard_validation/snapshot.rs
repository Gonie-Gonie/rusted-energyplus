use super::*;

pub(super) fn matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
    let selector = output.calculation_cooling_positive_supply_capacity_limit_guard;
    let snapshot = output.calculation_cooling_post_saturation_capacity_limit_guard;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(snapshot)
        && selector_corroborates(selector, binding.system.cooling_limit, snapshot)
        && snapshot == expected_snapshot(predecessor, binding.system.cooling_limit)
}

pub(super) fn expected_snapshot(
    predecessor: PredecessorSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> Snapshot {
    let active = predecessor.local_supply_enthalpy_after_saturation_limit_assignment_performed;
    let capacity_match = active && cooling_limit == IdealLoadsLimit::LimitCapacity;
    let second_comparison = active && !capacity_match;
    let combined_match =
        second_comparison && cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let selected = capacity_match || combined_match;

    Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed: active,
        capacity_limit_guard_evaluated: active,
        configured_cooling_limit_owned_read: active,
        cp337_same_call_selector_lineage_corroborated: active,
        first_cooling_limit_read: active,
        first_cooling_limit: active.then_some(cooling_limit),
        cooling_limit_capacity_comparison_evaluated: active,
        cooling_limit_capacity: active.then_some(capacity_match),
        second_cooling_limit_read: second_comparison,
        second_cooling_limit: second_comparison.then_some(cooling_limit),
        cooling_limit_flow_rate_and_capacity_comparison_evaluated: second_comparison,
        cooling_limit_flow_rate_and_capacity: second_comparison.then_some(combined_match),
        cooling_limit_condition_satisfied: active.then_some(selected),
        cooling_limit_rejected: active && !selected,
        capacity_limit_body_entered: active && selected,
        active_guard_false_fallthrough: active && !selected,
    }
}

fn selector_corroborates(
    selector: SelectorSnapshot,
    cooling_limit: IdealLoadsLimit,
    snapshot: Snapshot,
) -> bool {
    if !snapshot.capacity_limit_guard_evaluated {
        return !snapshot.cp337_same_call_selector_lineage_corroborated;
    }
    selector.system == snapshot.system
        && selector.parent_call_ordinal == snapshot.parent_call_ordinal
        && selector.controlled_zone == snapshot.controlled_zone
        && selector.capacity_limit_guard_evaluated
        && selector.first_cooling_limit == Some(cooling_limit)
        && selector.second_cooling_limit
            == (cooling_limit != IdealLoadsLimit::LimitCapacity).then_some(cooling_limit)
        && snapshot.cp337_same_call_selector_lineage_corroborated
}
