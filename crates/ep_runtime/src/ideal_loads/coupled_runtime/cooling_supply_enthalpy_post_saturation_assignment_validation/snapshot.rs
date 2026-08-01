use super::*;

pub(super) fn matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let snapshot = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
    let humidity = output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let temperature = output.calculation_cooling_supply_humidity_ratio_saturation_assignment;
    [snapshot.system, humidity.system, temperature.system]
        .into_iter()
        .all(|system| system == binding.ideal_loads_air_system)
        && [
            snapshot.parent_call_ordinal,
            humidity.parent_call_ordinal,
            temperature.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == call_ordinal)
        && [
            snapshot.controlled_zone,
            humidity.controlled_zone,
            temperature.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == binding.zone)
        && cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
            humidity,
        )
        && cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            temperature,
        )
        && links_exactly(snapshot, humidity, temperature)
}

fn links_exactly(
    snapshot: Snapshot,
    humidity: HumiditySnapshot,
    temperature: TemperatureSnapshot,
) -> bool {
    let snapshot_routes = route_flags(snapshot);
    let humidity_routes = humidity_route_flags(humidity);
    let temperature_routes = temperature_route_flags(temperature);
    let active = !(snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped);
    snapshot_routes == humidity_routes
        && snapshot_routes == temperature_routes
        && snapshot_routes.into_iter().filter(|route| *route).count() == 1
        && snapshot.predecessor_dehumidification_control_type
            == humidity.predecessor_dehumidification_control_type
        && snapshot.predecessor_supply_humidity_ratio_saturation_limit_assignment_performed
            == humidity.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_humidity_ratio,
            humidity.resulting_supply_humidity_ratio,
        )
        && snapshot.cp334_supply_temperature_mixed_air_limit_owned_read
            == temperature.cp334_supply_temperature_mixed_air_limit_owned_read
        && snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read
            == temperature.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read
        && snapshot.cp377_supply_temperature_owned_read == active
        && snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read == active
        && snapshot.purchased_air_supply_temperature_for_post_saturation_enthalpy_read == active
        && snapshot.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read == active
        && option_bits_equal(
            snapshot.supply_temperature_c,
            temperature.supply_temperature_for_saturation_humidity_ratio_c,
        )
        && option_bits_equal(
            snapshot.supply_humidity_ratio,
            humidity.resulting_supply_humidity_ratio,
        )
        && snapshot.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated == active
        && snapshot.local_supply_enthalpy_after_saturation_limit_assignment_performed == active
        && option_bits_equal(
            snapshot.psychrometric_supply_enthalpy_j_per_kg,
            snapshot.assigned_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.assigned_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
        )
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

#[rustfmt::skip]
pub(super) fn snapshots_match_exact_bits(mut left: Snapshot, mut right: Snapshot) -> bool {
    let values_match = [
        (left.predecessor_resulting_supply_humidity_ratio, right.predecessor_resulting_supply_humidity_ratio),
        (left.supply_temperature_c, right.supply_temperature_c),
        (left.supply_humidity_ratio, right.supply_humidity_ratio),
        (left.psychrometric_supply_enthalpy_j_per_kg, right.psychrometric_supply_enthalpy_j_per_kg),
        (left.assigned_supply_enthalpy_j_per_kg, right.assigned_supply_enthalpy_j_per_kg),
        (left.resulting_supply_enthalpy_j_per_kg, right.resulting_supply_enthalpy_j_per_kg),
    ].into_iter().all(|(left, right)| option_bits_equal(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_supply_humidity_ratio = None;
        snapshot.supply_temperature_c = None;
        snapshot.supply_humidity_ratio = None;
        snapshot.psychrometric_supply_enthalpy_j_per_kg = None;
        snapshot.assigned_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
    }
    values_match && left == right
}
