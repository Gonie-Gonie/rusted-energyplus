use super::*;

pub(super) fn matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let predecessor = output.calculation_cooling_supply_humidity_ratio_saturation_assignment;
    [snapshot.system, predecessor.system]
        .into_iter()
        .all(|system| system == binding.ideal_loads_air_system)
        && [snapshot.parent_call_ordinal, predecessor.parent_call_ordinal]
            .into_iter()
            .all(|ordinal| ordinal == call_ordinal)
        && [snapshot.controlled_zone, predecessor.controlled_zone]
            .into_iter()
            .all(|zone| zone == binding.zone)
        && cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && links_exactly(snapshot, predecessor)
        && reconciles_unchanged_numerical_output(snapshot, output)
}

fn links_exactly(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    let routes_match = snapshot.unit_off_skipped == predecessor.unit_off_skipped
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
            == predecessor.dehumidification_control_guard_false_fallthrough;
    let routes = [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ];
    routes.into_iter().filter(|route| *route).count() == 1
        && routes_match
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed
            == predecessor.predecessor_local_supply_humidity_ratio_original_assignment_performed
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_humidity_ratio_original,
            predecessor.predecessor_resulting_supply_humidity_ratio_original,
        )
        && snapshot.predecessor_local_saturation_supply_humidity_ratio_assignment_performed
            == predecessor.local_saturation_supply_humidity_ratio_assignment_performed
        && option_bits_equal(
            snapshot.predecessor_resulting_saturation_supply_humidity_ratio,
            predecessor.resulting_saturation_supply_humidity_ratio,
        )
}

fn reconciles_unchanged_numerical_output(
    snapshot: Snapshot,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
) -> bool {
    if snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
    {
        return true;
    }
    let Some(minimum) = snapshot.minimum_supply_humidity_ratio_after_saturation_limit else {
        return false;
    };
    let Some(assigned) = snapshot.assigned_supply_humidity_ratio else {
        return false;
    };
    let Some(resulting) = snapshot.resulting_supply_humidity_ratio else {
        return false;
    };
    let expected = resulting.to_bits();
    minimum.to_bits() == expected
        && assigned.to_bits() == expected
        && output
            .coupling
            .purchased_air
            .calculation
            .supply_humidity_ratio
            .to_bits()
            == expected
        && output
            .coupling
            .purchased_air
            .supply_node_update
            .humidity_ratio
            .to_bits()
            == expected
        && output
            .coupling
            .purchased_air
            .report
            .supply_humidity_ratio
            .to_bits()
            == expected
}

#[rustfmt::skip]
pub(super) fn snapshots_match_exact_bits(mut left: Snapshot, mut right: Snapshot) -> bool {
    let values_match = [
        (left.predecessor_resulting_supply_humidity_ratio_original, right.predecessor_resulting_supply_humidity_ratio_original),
        (left.predecessor_resulting_saturation_supply_humidity_ratio, right.predecessor_resulting_saturation_supply_humidity_ratio),
        (left.original_supply_humidity_ratio_before_saturation_limit, right.original_supply_humidity_ratio_before_saturation_limit),
        (left.saturation_supply_humidity_ratio_for_limit, right.saturation_supply_humidity_ratio_for_limit),
        (left.minimum_supply_humidity_ratio_after_saturation_limit, right.minimum_supply_humidity_ratio_after_saturation_limit),
        (left.assigned_supply_humidity_ratio, right.assigned_supply_humidity_ratio),
        (left.resulting_supply_humidity_ratio, right.resulting_supply_humidity_ratio),
    ].into_iter().all(|(left, right)| option_bits_equal(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio_original = None;
        snapshot.predecessor_resulting_saturation_supply_humidity_ratio = None;
        snapshot.original_supply_humidity_ratio_before_saturation_limit = None;
        snapshot.saturation_supply_humidity_ratio_for_limit = None;
        snapshot.minimum_supply_humidity_ratio_after_saturation_limit = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left == right
}

#[rustfmt::skip]
pub(super) fn predecessor_snapshots_match_exact_bits(
    mut left: PredecessorSnapshot,
    mut right: PredecessorSnapshot,
) -> bool {
    let values_match = [
        (left.predecessor_resulting_supply_humidity_ratio_original, right.predecessor_resulting_supply_humidity_ratio_original),
        (left.supply_temperature_for_saturation_humidity_ratio_c, right.supply_temperature_for_saturation_humidity_ratio_c),
        (left.outdoor_barometric_pressure_pa, right.outdoor_barometric_pressure_pa),
        (left.saturation_supply_humidity_ratio, right.saturation_supply_humidity_ratio),
        (left.assigned_saturation_supply_humidity_ratio, right.assigned_saturation_supply_humidity_ratio),
        (left.resulting_saturation_supply_humidity_ratio, right.resulting_saturation_supply_humidity_ratio),
    ].into_iter().all(|(left, right)| option_bits_equal(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio_original = None;
        snapshot.supply_temperature_for_saturation_humidity_ratio_c = None;
        snapshot.outdoor_barometric_pressure_pa = None;
        snapshot.saturation_supply_humidity_ratio = None;
        snapshot.assigned_saturation_supply_humidity_ratio = None;
        snapshot.resulting_saturation_supply_humidity_ratio = None;
    }
    values_match && left == right
}
