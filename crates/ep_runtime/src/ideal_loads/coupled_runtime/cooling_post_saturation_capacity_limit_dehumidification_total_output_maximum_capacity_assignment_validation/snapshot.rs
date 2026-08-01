use super::*;

pub(super) fn matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment;

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && predecessor.system == snapshot.system
        && predecessor.parent_call_ordinal == snapshot.parent_call_ordinal
        && predecessor.controlled_zone == snapshot.controlled_zone
        && cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(snapshot)
        && expected_snapshot(predecessor)
            .is_some_and(|expected| snapshots_match_exact_bits(snapshot, expected))
}

pub(super) fn expected_snapshot(predecessor: PredecessorSnapshot) -> Option<Snapshot> {
    let evaluated = predecessor.dehumidification_total_output_capacity_guard_evaluated;
    let guard_false = predecessor.dehumidification_total_output_capacity_guard_false_fallthrough;
    let assignment = predecessor.dehumidification_total_output_capacity_adjustment_body_entered;
    if evaluated != (guard_false || assignment) || (evaluated && guard_false == assignment) {
        return None;
    }
    let preexisting = if evaluated {
        Some(predecessor.cooling_total_output_w?)
    } else {
        None
    };
    let maximum = if assignment {
        Some(predecessor.maximum_total_cooling_capacity_w?)
    } else {
        None
    };
    let resulting = if assignment { maximum } else { preexisting };

    Some(Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_total_output_assignment_executed: predecessor
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: assignment,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: guard_false,
        dehumidification_total_output_capacity_guard_false_fallthrough: guard_false,
        dehumidification_total_output_maximum_capacity_assignment_executed: assignment,
        preexisting_cooling_total_output_w: preexisting,
        cp383_retained_maximum_total_cooling_capacity_owned_read: assignment,
        maximum_total_cooling_capacity_read: assignment,
        maximum_total_cooling_capacity_w: maximum,
        cooling_total_output_assigned: assignment,
        assigned_cooling_total_output_w: maximum,
        resulting_cooling_total_output_w: resulting,
    })
}

pub(super) fn snapshots_match_exact_bits(left: Snapshot, right: Snapshot) -> bool {
    let values_match = [
        (
            left.preexisting_cooling_total_output_w,
            right.preexisting_cooling_total_output_w,
        ),
        (
            left.maximum_total_cooling_capacity_w,
            right.maximum_total_cooling_capacity_w,
        ),
        (
            left.assigned_cooling_total_output_w,
            right.assigned_cooling_total_output_w,
        ),
        (
            left.resulting_cooling_total_output_w,
            right.resulting_cooling_total_output_w,
        ),
    ]
    .into_iter()
    .all(|(left, right)| exact_optional_f64(left, right));
    let mut left_without_values = left;
    let mut right_without_values = right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.preexisting_cooling_total_output_w = None;
        snapshot.maximum_total_cooling_capacity_w = None;
        snapshot.assigned_cooling_total_output_w = None;
        snapshot.resulting_cooling_total_output_w = None;
    }
    values_match && left_without_values == right_without_values
}
