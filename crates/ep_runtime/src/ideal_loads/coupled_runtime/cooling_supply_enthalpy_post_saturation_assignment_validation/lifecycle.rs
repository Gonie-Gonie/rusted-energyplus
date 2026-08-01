use super::*;

pub(super) fn validate(
    lifecycle: &Lifecycle,
    humidity: &HumidityLifecycle,
    temperature: &TemperatureLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || humidity.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        || humidity.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || temperature.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || temperature.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || [
            lifecycle.state.system,
            humidity.state.system,
            temperature.state.system,
        ]
        .into_iter()
        .any(|system| system != binding.ideal_loads_air_system)
    {
        return Err(violation("source_and_system_identity", 1, 0));
    }
    validate_counts(
        &lifecycle.state,
        &humidity.state,
        &temperature.state,
        timestep_count,
    )?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !snapshot::snapshots_match_exact_bits(
        latest,
        latest_output.calculation_cooling_supply_enthalpy_post_saturation_assignment,
    ) || !snapshot::matches_release(latest_output, timestep_count, binding)
        || !latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &humidity.state,
            &temperature.state,
            latest,
        )
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    humidity: &HumidityState,
    temperature: &TemperatureState,
    timestep_count: usize,
) -> Result<(), Error> {
    let carried = route_counts(state);
    let humidity_counts = humidity_route_counts(humidity);
    let temperature_counts = temperature_route_counts(temperature);
    if carried != humidity_counts || carried != temperature_counts {
        return Err(violation("predecessor_route_counters", 1, 0));
    }
    let partition = checked_sum(&carried)?;
    let active = checked_sum(&carried[3..])?;
    let source_sites = active
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "cp378_transition_count",
            humidity.transition_count,
            state.transition_count,
        ),
        (
            "cp377_transition_count",
            temperature.transition_count,
            state.transition_count,
        ),
        ("transition_partition", state.transition_count, partition),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_temperature_read_count",
            active,
            state.purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count,
        ),
        (
            "supply_humidity_ratio_read_count",
            active,
            state.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count,
        ),
        (
            "psy_h_fn_tdb_w_evaluation_count",
            active,
            state.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count,
        ),
        (
            "local_supply_enthalpy_assignment_count",
            active,
            state.local_supply_enthalpy_after_saturation_limit_assignment_count,
        ),
        (
            "cp334_temperature_owner_count",
            temperature.cp334_supply_temperature_mixed_air_limit_owner_count,
            state.cp334_supply_temperature_mixed_air_limit_owner_count,
        ),
        (
            "cp344_temperature_owner_count",
            temperature.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
            state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        ),
        (
            "cp378_humidity_owner_count",
            active,
            state.cp378_supply_humidity_ratio_saturation_limit_owner_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    let temperature_owner_count = state
        .cp334_supply_temperature_mixed_air_limit_owner_count
        .checked_add(state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count)
        .ok_or_else(|| violation("temperature_owner_count_overflow", 0, usize::MAX))?;
    ensure_count(
        temperature_owner_count,
        active,
        "temperature_owner_partition",
    )
}

fn route_counts(state: &State) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn humidity_route_counts(state: &HumidityState) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn temperature_route_counts(state: &TemperatureState) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    humidity: &HumidityState,
    temperature: &TemperatureState,
    latest: Snapshot,
) -> bool {
    let index = if latest.unit_off_skipped {
        0
    } else if latest.non_cooling_skipped {
        1
    } else if latest.positive_guard_false_fallthrough_skipped {
        2
    } else if latest.heating_availability_guard_false_fallthrough {
        3
    } else if latest.humidification_control_guard_false_fallthrough {
        4
    } else if latest.dehumidification_control_humidistat_maximum_assignment_executed {
        5
    } else if latest.dehumidification_control_none_maximum_assignment_executed {
        6
    } else if latest.dehumidification_control_guard_false_fallthrough {
        7
    } else {
        return false;
    };
    route_counts(state)[index] > 0
        && humidity_route_counts(humidity)[index] > 0
        && temperature_route_counts(temperature)[index] > 0
}

fn checked_sum(values: &[usize]) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation("counter_partition_overflow", 0, usize::MAX))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}
