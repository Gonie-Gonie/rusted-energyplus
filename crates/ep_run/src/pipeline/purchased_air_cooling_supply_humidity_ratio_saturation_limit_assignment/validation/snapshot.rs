use super::*;

pub(super) fn metadata_is_exact(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
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

pub(super) fn links_exactly(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
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
        && active_or_null_values_match(snapshot)
}

fn active_or_null_values_match(snapshot: Snapshot) -> bool {
    let active = !(snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped);
    if !active {
        return [
            snapshot.cp376_original_supply_humidity_ratio_owned_read,
            snapshot.cp377_saturation_supply_humidity_ratio_owned_read,
            snapshot.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read,
            snapshot.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read,
            snapshot.source_shaped_two_argument_minimum_evaluated,
            snapshot.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed,
        ]
        .into_iter()
        .all(|flag| !flag)
            && [
                snapshot.original_supply_humidity_ratio_before_saturation_limit,
                snapshot.saturation_supply_humidity_ratio_for_limit,
                snapshot.minimum_supply_humidity_ratio_after_saturation_limit,
                snapshot.assigned_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ]
            .into_iter()
            .all(|value| value.is_none());
    }
    let Some(original) = snapshot.original_supply_humidity_ratio_before_saturation_limit else {
        return false;
    };
    let Some(saturation) = snapshot.saturation_supply_humidity_ratio_for_limit else {
        return false;
    };
    let minimum = if original < saturation {
        original
    } else {
        saturation
    };
    snapshot.cp376_original_supply_humidity_ratio_owned_read
        && snapshot.cp377_saturation_supply_humidity_ratio_owned_read
        && snapshot.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read
        && snapshot.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read
        && snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed
        && option_bits_equal(
            snapshot.original_supply_humidity_ratio_before_saturation_limit,
            snapshot.predecessor_resulting_supply_humidity_ratio_original,
        )
        && option_bits_equal(
            snapshot.saturation_supply_humidity_ratio_for_limit,
            snapshot.predecessor_resulting_saturation_supply_humidity_ratio,
        )
        && [
            snapshot.minimum_supply_humidity_ratio_after_saturation_limit,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ]
        .into_iter()
        .all(|value| option_bits_equal(value, Some(minimum)))
}
