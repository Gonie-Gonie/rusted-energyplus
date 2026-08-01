//! CP384 snapshot lineage and three-shape assignment validation.

use super::*;

pub(super) fn metadata_is_exact(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == expected_system
        && predecessor.system == expected_system
        && snapshot.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && snapshot.parent_call_ordinal == calls
        && predecessor.parent_call_ordinal == calls
}

pub(super) fn links_exactly(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    inherited_lineage_is_exact(snapshot, predecessor)
        && assignment_shape_is_exact(snapshot, predecessor)
}

fn inherited_lineage_is_exact(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot.heating_availability_guard_false_fallthrough
            == predecessor.heating_availability_guard_false_fallthrough
        && snapshot.humidification_control_guard_false_fallthrough
            == predecessor.humidification_control_guard_false_fallthrough
        && snapshot.dehumidification_control_humidistat_maximum_assignment_executed
            == predecessor.dehumidification_control_humidistat_maximum_assignment_executed
        && snapshot.dehumidification_control_none_maximum_assignment_executed
            == predecessor.dehumidification_control_none_maximum_assignment_executed
        && snapshot.dehumidification_control_guard_false_fallthrough
            == predecessor.dehumidification_control_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_guard_evaluated
            == predecessor.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
            == predecessor.predecessor_capacity_limit_body_entered
        && snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_guard_evaluated
            == predecessor.predecessor_dehumidification_guard_evaluated
        && snapshot.predecessor_dehumidification_body_entered
            == predecessor.predecessor_dehumidification_body_entered
        && snapshot.predecessor_dehumidification_guard_false_fallthrough
            == predecessor.predecessor_dehumidification_guard_false_fallthrough
        && snapshot.predecessor_dehumidification_total_output_assignment_executed
            == predecessor.predecessor_dehumidification_total_output_assignment_executed
        && snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated
            == predecessor.dehumidification_total_output_capacity_guard_evaluated
        && snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered
            == predecessor.dehumidification_total_output_capacity_adjustment_body_entered
        && snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough
            == predecessor.dehumidification_total_output_capacity_guard_false_fallthrough
}

fn assignment_shape_is_exact(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    let evaluated = predecessor.dehumidification_total_output_capacity_guard_evaluated;
    let guard_false = predecessor.dehumidification_total_output_capacity_guard_false_fallthrough;
    let assignment = predecessor.dehumidification_total_output_capacity_adjustment_body_entered;
    if snapshot.dehumidification_total_output_capacity_guard_false_fallthrough != guard_false
        || snapshot.dehumidification_total_output_maximum_capacity_assignment_executed != assignment
        || evaluated != (guard_false || assignment)
        || (evaluated && guard_false == assignment)
    {
        return false;
    }
    if !evaluated {
        return numeric_values(snapshot)
            .into_iter()
            .all(|value| value.is_none())
            && !snapshot.cp383_retained_maximum_total_cooling_capacity_owned_read
            && !snapshot.maximum_total_cooling_capacity_read
            && !snapshot.cooling_total_output_assigned;
    }

    let Some(preexisting) = predecessor.cooling_total_output_w else {
        return false;
    };
    if !option_bits_equal(
        snapshot.preexisting_cooling_total_output_w,
        Some(preexisting),
    ) {
        return false;
    }
    if guard_false {
        return !snapshot.cp383_retained_maximum_total_cooling_capacity_owned_read
            && !snapshot.maximum_total_cooling_capacity_read
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && !snapshot.cooling_total_output_assigned
            && snapshot.assigned_cooling_total_output_w.is_none()
            && option_bits_equal(snapshot.resulting_cooling_total_output_w, Some(preexisting));
    }

    let Some(maximum) = predecessor.maximum_total_cooling_capacity_w else {
        return false;
    };
    predecessor.cp382_cooling_total_output_owned_read
        && predecessor.cp321_maximum_total_cooling_capacity_owned_read
        && predecessor.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated
        && predecessor.maximum_total_cooling_capacity_read
        && snapshot.cp383_retained_maximum_total_cooling_capacity_owned_read
        && snapshot.maximum_total_cooling_capacity_read
        && option_bits_equal(snapshot.maximum_total_cooling_capacity_w, Some(maximum))
        && snapshot.cooling_total_output_assigned
        && option_bits_equal(snapshot.assigned_cooling_total_output_w, Some(maximum))
        && option_bits_equal(snapshot.resulting_cooling_total_output_w, Some(maximum))
}

fn numeric_values(snapshot: Snapshot) -> [Option<f64>; 4] {
    [
        snapshot.preexisting_cooling_total_output_w,
        snapshot.maximum_total_cooling_capacity_w,
        snapshot.assigned_cooling_total_output_w,
        snapshot.resulting_cooling_total_output_w,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_bit_copy_preserves_nan_payload_and_signed_zero() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0384);
        assert!(option_bits_equal(Some(nan), Some(nan)));
        assert!(!option_bits_equal(Some(0.0), Some(-0.0)));
    }
}
