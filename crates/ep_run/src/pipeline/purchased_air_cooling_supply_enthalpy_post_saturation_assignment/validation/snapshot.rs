use ep_runtime::psychrometrics::energyplus_psy_h_fn_tdb_w;

use super::*;

pub(super) fn metadata_is_exact(
    snapshot: Snapshot,
    humidity: HumiditySnapshot,
    temperature: TemperatureSnapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER
        && humidity.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        && humidity.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && humidity.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER
        && temperature.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        && temperature.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && temperature.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
        && [snapshot.system, humidity.system, temperature.system]
            .into_iter()
            .all(|system| system == expected_system)
        && [
            snapshot.controlled_zone,
            humidity.controlled_zone,
            temperature.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == expected_zone)
        && [
            snapshot.parent_call_ordinal,
            humidity.parent_call_ordinal,
            temperature.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == calls)
}

pub(super) fn links_exactly(
    snapshot: Snapshot,
    humidity: HumiditySnapshot,
    temperature: TemperatureSnapshot,
) -> bool {
    let routes = route_flags(snapshot);
    let active = !(snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped);
    let direct_route_is_supported = if !active {
        snapshot.predecessor_dehumidification_control_type.is_none()
    } else {
        (snapshot.heating_availability_guard_false_fallthrough
            || snapshot.humidification_control_guard_false_fallthrough)
            && snapshot.predecessor_dehumidification_control_type
                == Some(ep_model::DehumidificationControlType::None)
    };
    routes.into_iter().filter(|route| *route).count() == 1
        && direct_route_is_supported
        && routes == humidity_route_flags(humidity)
        && routes == temperature_route_flags(temperature)
        && snapshot.predecessor_dehumidification_control_type
            == humidity.predecessor_dehumidification_control_type
        && snapshot.predecessor_supply_humidity_ratio_saturation_limit_assignment_performed
            == humidity.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_humidity_ratio,
            humidity.resulting_supply_humidity_ratio,
        )
        && active_or_null_values_match(snapshot, humidity, temperature, active)
}

fn active_or_null_values_match(
    snapshot: Snapshot,
    humidity: HumiditySnapshot,
    temperature: TemperatureSnapshot,
    active: bool,
) -> bool {
    if !active {
        return [
            snapshot.cp377_supply_temperature_owned_read,
            snapshot.cp334_supply_temperature_mixed_air_limit_owned_read,
            snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
            snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read,
            snapshot.purchased_air_supply_temperature_for_post_saturation_enthalpy_read,
            snapshot.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read,
            snapshot.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated,
            snapshot.local_supply_enthalpy_after_saturation_limit_assignment_performed,
        ]
        .into_iter()
        .all(|flag| !flag)
            && [
                snapshot.supply_temperature_c,
                snapshot.supply_humidity_ratio,
                snapshot.psychrometric_supply_enthalpy_j_per_kg,
                snapshot.assigned_supply_enthalpy_j_per_kg,
                snapshot.resulting_supply_enthalpy_j_per_kg,
            ]
            .into_iter()
            .all(|value| value.is_none());
    }

    let Some(temperature_c) = snapshot.supply_temperature_c else {
        return false;
    };
    let Some(humidity_ratio) = snapshot.supply_humidity_ratio else {
        return false;
    };
    let enthalpy = energyplus_psy_h_fn_tdb_w(temperature_c, humidity_ratio);
    let temperature_owner_count = [
        snapshot.cp334_supply_temperature_mixed_air_limit_owned_read,
        snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
    ]
    .into_iter()
    .filter(|owned| *owned)
    .count();
    temperature_c.is_finite()
        && humidity_ratio.is_finite()
        && humidity_ratio >= 0.0
        && enthalpy.is_finite()
        && snapshot.predecessor_supply_humidity_ratio_saturation_limit_assignment_performed
        && snapshot.cp377_supply_temperature_owned_read
        && temperature_owner_count == 1
        && snapshot.cp334_supply_temperature_mixed_air_limit_owned_read
            == temperature.cp334_supply_temperature_mixed_air_limit_owned_read
        && snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read
            == temperature.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read
        && snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read
        && snapshot.purchased_air_supply_temperature_for_post_saturation_enthalpy_read
        && option_bits_equal(
            snapshot.supply_temperature_c,
            temperature.supply_temperature_for_saturation_humidity_ratio_c,
        )
        && snapshot.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read
        && option_bits_equal(
            snapshot.supply_humidity_ratio,
            humidity.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.supply_humidity_ratio,
            snapshot.predecessor_resulting_supply_humidity_ratio,
        )
        && snapshot.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated
        && option_bits_equal(
            snapshot.psychrometric_supply_enthalpy_j_per_kg,
            Some(enthalpy),
        )
        && snapshot.local_supply_enthalpy_after_saturation_limit_assignment_performed
        && option_bits_equal(snapshot.assigned_supply_enthalpy_j_per_kg, Some(enthalpy))
        && option_bits_equal(snapshot.resulting_supply_enthalpy_j_per_kg, Some(enthalpy))
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

fn humidity_route_flags(snapshot: HumiditySnapshot) -> [bool; 8] {
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

fn temperature_route_flags(snapshot: TemperatureSnapshot) -> [bool; 8] {
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
