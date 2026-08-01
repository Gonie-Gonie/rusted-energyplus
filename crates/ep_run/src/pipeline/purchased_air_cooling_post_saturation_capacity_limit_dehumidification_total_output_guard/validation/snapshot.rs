//! CP383 snapshot lineage and raw-comparison validation.

use super::*;

pub(super) fn metadata_is_exact(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    capacity: CapacitySnapshot,
    corroborator: CapacityCorroboratorSnapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER
        && [
            snapshot.system,
            predecessor.system,
            capacity.system,
            corroborator.system,
        ]
        .into_iter()
        .all(|system| system == expected_system)
        && [
            snapshot.controlled_zone,
            predecessor.controlled_zone,
            capacity.controlled_zone,
            corroborator.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == expected_zone)
        && [
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal,
            capacity.parent_call_ordinal,
            corroborator.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == calls)
}

pub(super) fn links_exactly(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    capacity: CapacitySnapshot,
    corroborator: CapacityCorroboratorSnapshot,
) -> bool {
    inherited_lineage_is_exact(snapshot, predecessor)
        && guard_shape_is_exact(snapshot, predecessor, capacity, corroborator)
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
            == predecessor.dehumidification_total_output_assignment_executed
}

fn guard_shape_is_exact(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    capacity: CapacitySnapshot,
    corroborator: CapacityCorroboratorSnapshot,
) -> bool {
    let active = predecessor.dehumidification_total_output_assignment_executed;
    if snapshot.dehumidification_total_output_capacity_guard_evaluated != active
        || active_flags(snapshot)
            .into_iter()
            .any(|flag| flag != active)
    {
        return false;
    }
    if !active {
        return snapshot.cooling_total_output_w.is_none()
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && snapshot
                .cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity
                .is_none()
            && !snapshot.dehumidification_total_output_capacity_adjustment_body_entered
            && !snapshot.dehumidification_total_output_capacity_guard_false_fallthrough;
    }

    let (Some(output), Some(maximum_capacity)) = (
        predecessor.cooling_total_output_w,
        capacity.maximum_total_cooling_capacity_w,
    ) else {
        return false;
    };
    let corroborated_capacity = corroborator.maximum_total_cooling_capacity_w;
    let comparison = output > maximum_capacity;
    predecessor.cooling_total_output_assigned
        && capacity.maximum_total_cooling_capacity_read
        && capacity.maximum_total_cooling_capacity_comparison_evaluated
        && corroborator.maximum_total_cooling_capacity_read
        && option_bits_equal(corroborated_capacity, Some(maximum_capacity))
        && option_bits_equal(snapshot.cooling_total_output_w, Some(output))
        && option_bits_equal(
            snapshot.maximum_total_cooling_capacity_w,
            Some(maximum_capacity),
        )
        && snapshot.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity
            == Some(comparison)
        && snapshot.dehumidification_total_output_capacity_adjustment_body_entered == comparison
        && snapshot.dehumidification_total_output_capacity_guard_false_fallthrough != comparison
}

fn active_flags(snapshot: Snapshot) -> [bool; 7] {
    [
        snapshot.cp382_cooling_total_output_owned_read,
        snapshot.cooling_total_output_read,
        snapshot.cp321_maximum_total_cooling_capacity_owned_read,
        snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated,
        snapshot.maximum_total_cooling_capacity_read,
        snapshot.cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated,
        snapshot.dehumidification_total_output_capacity_guard_evaluated,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_strict_comparison_handles_nonfinite_left_without_normalization() {
        for (left, expected) in [
            (f64::INFINITY, true),
            (f64::NEG_INFINITY, false),
            (f64::from_bits(0x7ff8_0000_0000_0383), false),
        ] {
            assert_eq!(left > 100.0, expected);
        }
    }

    #[test]
    fn exact_bit_comparison_distinguishes_signed_zero() {
        assert!(!option_bits_equal(Some(0.0), Some(-0.0)));
    }
}
