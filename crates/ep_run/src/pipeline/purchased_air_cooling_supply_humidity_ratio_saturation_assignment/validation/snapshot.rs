use ep_runtime::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

use super::*;

pub(super) fn metadata_is_exact(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    cp334: Cp334Snapshot,
    cp344: Cp344Snapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER
        && cp334.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        && cp334.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        && cp334.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        && cp344.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        && cp344.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        && cp344.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        && [
            snapshot.system,
            predecessor.system,
            cp334.system,
            cp344.system,
        ]
        .into_iter()
        .all(|system| system == expected_system)
        && [
            snapshot.controlled_zone,
            predecessor.controlled_zone,
            cp334.controlled_zone,
            cp344.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == expected_zone)
        && [
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal,
            cp334.parent_call_ordinal,
            cp344.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == calls)
}

pub(super) fn links_exactly(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    cp334: Cp334Snapshot,
    cp344: Cp344Snapshot,
) -> bool {
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
    let predecessor_matches = snapshot.predecessor_dehumidification_control_type
        == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed
            == predecessor.local_supply_humidity_ratio_original_assignment_performed
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_humidity_ratio_original,
            predecessor.resulting_supply_humidity_ratio_original,
        );
    routes.into_iter().filter(|route| *route).count() == 1
        && routes_match
        && predecessor_matches
        && active_or_null_values_match(snapshot, cp334, cp344)
}

fn active_or_null_values_match(
    snapshot: Snapshot,
    cp334: Cp334Snapshot,
    cp344: Cp344Snapshot,
) -> bool {
    let active = !(snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped);
    if !active {
        return [
            snapshot.cp334_supply_temperature_mixed_air_limit_owned_read,
            snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
            snapshot.environment_outdoor_barometric_pressure_owned_read,
            snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read,
            snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read,
            snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated,
            snapshot.local_saturation_supply_humidity_ratio_assignment_performed,
        ]
        .into_iter()
        .all(|flag| !flag)
            && [
                snapshot.supply_temperature_for_saturation_humidity_ratio_c,
                snapshot.outdoor_barometric_pressure_pa,
                snapshot.saturation_supply_humidity_ratio,
                snapshot.assigned_saturation_supply_humidity_ratio,
                snapshot.resulting_saturation_supply_humidity_ratio,
            ]
            .into_iter()
            .all(|value| value.is_none());
    }

    let cp344_owned =
        cp344.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    let owner_temperature = if cp344_owned {
        cp344.resulting_supply_temperature_c
    } else {
        cp334.assigned_supply_temperature_c
    };
    let Some(temperature) = owner_temperature else {
        return false;
    };
    let Some(pressure) = snapshot.outdoor_barometric_pressure_pa else {
        return false;
    };
    let saturation = energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure);
    snapshot.cp334_supply_temperature_mixed_air_limit_owned_read != cp344_owned
        && snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read
            == cp344_owned
        && snapshot.environment_outdoor_barometric_pressure_owned_read
        && snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read
        && snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
        && snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated
        && snapshot.local_saturation_supply_humidity_ratio_assignment_performed
        && temperature.is_finite()
        && pressure.is_finite()
        && pressure > 0.0
        && saturation.is_finite()
        && option_bits_equal(
            snapshot.supply_temperature_for_saturation_humidity_ratio_c,
            Some(temperature),
        )
        && [
            snapshot.saturation_supply_humidity_ratio,
            snapshot.assigned_saturation_supply_humidity_ratio,
            snapshot.resulting_saturation_supply_humidity_ratio,
        ]
        .into_iter()
        .all(|value| option_bits_equal(value, Some(saturation)))
}
