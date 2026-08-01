use super::*;

pub(super) fn matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
    let capacity_owner = output.calculation_cooling_capacity_zero_flow_reset;
    let capacity_corroborator =
        output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard;

    metadata_matches(
        snapshot,
        predecessor,
        capacity_owner,
        capacity_corroborator,
        call_ordinal,
        binding,
    ) && cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release(snapshot)
        && expected_snapshot(predecessor, capacity_owner, capacity_corroborator)
            .is_some_and(|expected| snapshots_match_exact_bits(snapshot, expected))
}

fn metadata_matches(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    capacity_owner: CapacitySnapshot,
    capacity_corroborator: CapacityCorroboratorSnapshot,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && [
            predecessor.system,
            capacity_owner.system,
            capacity_corroborator.system,
        ]
        .into_iter()
        .all(|system| system == snapshot.system)
        && [
            predecessor.parent_call_ordinal,
            capacity_owner.parent_call_ordinal,
            capacity_corroborator.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == snapshot.parent_call_ordinal)
        && [
            predecessor.controlled_zone,
            capacity_owner.controlled_zone,
            capacity_corroborator.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == snapshot.controlled_zone)
}

pub(super) fn expected_snapshot(
    predecessor: PredecessorSnapshot,
    capacity_owner: CapacitySnapshot,
    capacity_corroborator: CapacityCorroboratorSnapshot,
) -> Option<Snapshot> {
    let evaluated = predecessor.dehumidification_total_output_assignment_executed;
    let (cooling_total_output_w, maximum_total_cooling_capacity_w) = if evaluated {
        if !predecessor.cooling_total_output_assigned
            || !capacity_owner.maximum_total_cooling_capacity_read
            || !capacity_owner.maximum_total_cooling_capacity_comparison_evaluated
            || capacity_owner.maximum_total_cooling_capacity_equal_to_zero != Some(false)
            || capacity_owner.zero_cooling_capacity_body_entered
            || !capacity_corroborator.capacity_limit_sensible_output_guard_evaluated
            || !capacity_corroborator.maximum_total_cooling_capacity_read
        {
            return None;
        }
        let cooling_total_output_w = predecessor.cooling_total_output_w?;
        let maximum_total_cooling_capacity_w = capacity_owner.maximum_total_cooling_capacity_w?;
        let corroborated = capacity_corroborator.maximum_total_cooling_capacity_w?;
        if corroborated.to_bits() != maximum_total_cooling_capacity_w.to_bits() {
            return None;
        }
        (
            Some(cooling_total_output_w),
            Some(maximum_total_cooling_capacity_w),
        )
    } else {
        (None, None)
    };
    let comparison = match (cooling_total_output_w, maximum_total_cooling_capacity_w) {
        (Some(output), Some(maximum)) => Some(output > maximum),
        (None, None) => None,
        _ => return None,
    };

    Some(Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER,
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
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: evaluated,
        dehumidification_total_output_capacity_guard_evaluated: evaluated,
        cp382_cooling_total_output_owned_read: evaluated,
        cooling_total_output_read: evaluated,
        cooling_total_output_w,
        cp321_maximum_total_cooling_capacity_owned_read: evaluated,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: evaluated,
        maximum_total_cooling_capacity_read: evaluated,
        maximum_total_cooling_capacity_w,
        cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated: evaluated,
        cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity: comparison,
        dehumidification_total_output_capacity_adjustment_body_entered: comparison == Some(true),
        dehumidification_total_output_capacity_guard_false_fallthrough: comparison == Some(false),
    })
}

pub(super) fn snapshots_match_exact_bits(left: Snapshot, right: Snapshot) -> bool {
    let values_match =
        exact_optional_f64(left.cooling_total_output_w, right.cooling_total_output_w)
            && exact_optional_f64(
                left.maximum_total_cooling_capacity_w,
                right.maximum_total_cooling_capacity_w,
            );
    let mut left_without_values = left;
    let mut right_without_values = right;
    left_without_values.cooling_total_output_w = None;
    right_without_values.cooling_total_output_w = None;
    left_without_values.maximum_total_cooling_capacity_w = None;
    right_without_values.maximum_total_cooling_capacity_w = None;
    values_match && left_without_values == right_without_values
}
