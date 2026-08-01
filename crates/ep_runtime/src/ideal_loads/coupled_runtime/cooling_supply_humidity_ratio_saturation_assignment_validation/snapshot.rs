use crate::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

use super::*;

pub(super) fn matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_assignment;
    let predecessor =
        output.calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
    let cp334 = output.calculation_cooling_positive_supply_temperature_mixed_air_limit;
    let cp344 = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    [
        snapshot.system,
        predecessor.system,
        cp334.system,
        cp344.system,
    ]
    .into_iter()
    .all(|system| system == binding.ideal_loads_air_system)
        && [
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal,
            cp334.parent_call_ordinal,
            cp344.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == call_ordinal)
        && [
            snapshot.controlled_zone,
            predecessor.controlled_zone,
            cp334.controlled_zone,
            cp344.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == binding.zone)
        && cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            cp334,
        )
        && cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            cp344,
        )
        && links_exactly(snapshot, predecessor, cp334, cp344)
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
    let predecessor_matches = snapshot.predecessor_dehumidification_control_type
        == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed
            == predecessor.local_supply_humidity_ratio_original_assignment_performed
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_humidity_ratio_original,
            predecessor.resulting_supply_humidity_ratio_original,
        );
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

#[rustfmt::skip]
pub(super) fn snapshots_match_exact_bits(mut left: Snapshot, mut right: Snapshot) -> bool {
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
