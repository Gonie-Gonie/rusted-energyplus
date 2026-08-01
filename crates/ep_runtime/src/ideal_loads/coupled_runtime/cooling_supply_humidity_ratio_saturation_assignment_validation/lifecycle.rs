use super::*;

pub(super) fn validate(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    cp334: &Cp334Lifecycle,
    cp344: &Cp344Lifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || cp334.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || cp334.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || cp344.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || cp344.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || [
            lifecycle.state.system,
            predecessor.state.system,
            cp334.state.system,
            cp344.state.system,
        ]
        .into_iter()
        .any(|system| system != binding.ideal_loads_air_system)
    {
        return Err(violation("source_and_system_identity", 1, 0));
    }
    validate_counts(
        &lifecycle.state,
        &predecessor.state,
        cp344
            .state
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        timestep_count,
    )?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !super::super::cooling_supply_humidity_ratio_pre_saturation_original_assignment_validation::snapshots_match_exact_bits(
        predecessor_latest,
        latest_output.calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
    ) || !snapshot::snapshots_match_exact_bits(
        latest,
        latest_output.calculation_cooling_supply_humidity_ratio_saturation_assignment,
    ) || !snapshot::matches_release(latest_output, timestep_count, binding)
        || !latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            predecessor_latest,
        )
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &PredecessorState,
    cp344_owner_count: usize,
    timestep_count: usize,
) -> Result<(), Error> {
    let carried = [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ];
    let expected = [
        predecessor.unit_off_skip_count,
        predecessor.non_cooling_skip_count,
        predecessor.positive_guard_false_fallthrough_skip_count,
        predecessor.heating_availability_guard_false_fallthrough_count,
        predecessor.humidification_control_guard_false_fallthrough_count,
        predecessor
            .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        predecessor.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        predecessor.dehumidification_control_guard_false_fallthrough_count,
    ];
    if carried != expected {
        return Err(violation("predecessor_route_counters", 1, 0));
    }
    let partition = checked_sum(&carried)?;
    let active = checked_sum(&carried[3..])?;
    let temperature_owners = checked_sum(&[
        state.cp334_supply_temperature_mixed_air_limit_owner_count,
        state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
    ])?;
    let source_sites = active
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("transition_partition", state.transition_count, partition),
        ("temperature_owner_partition", active, temperature_owners),
        (
            "cp344_temperature_owner_count",
            cp344_owner_count,
            state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_temperature_read_count",
            active,
            state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count,
        ),
        (
            "outdoor_barometric_pressure_read_count",
            active,
            state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count,
        ),
        (
            "psychrometric_evaluation_count",
            active,
            state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count,
        ),
        (
            "local_assignment_count",
            active,
            state.local_saturation_supply_humidity_ratio_assignment_count,
        ),
        (
            "outdoor_barometric_pressure_owner_count",
            active,
            state.environment_outdoor_barometric_pressure_owner_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    latest: PredecessorSnapshot,
) -> bool {
    let pair = if latest.unit_off_skipped {
        (state.unit_off_skip_count, predecessor.unit_off_skip_count)
    } else if latest.non_cooling_skipped {
        (
            state.non_cooling_skip_count,
            predecessor.non_cooling_skip_count,
        )
    } else if latest.positive_guard_false_fallthrough_skipped {
        (
            state.positive_guard_false_fallthrough_skip_count,
            predecessor.positive_guard_false_fallthrough_skip_count,
        )
    } else if latest.heating_availability_guard_false_fallthrough {
        (
            state.heating_availability_guard_false_fallthrough_count,
            predecessor.heating_availability_guard_false_fallthrough_count,
        )
    } else if latest.humidification_control_guard_false_fallthrough {
        (
            state.humidification_control_guard_false_fallthrough_count,
            predecessor.humidification_control_guard_false_fallthrough_count,
        )
    } else if latest.dehumidification_control_humidistat_maximum_assignment_executed {
        (
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        )
    } else if latest.dehumidification_control_none_maximum_assignment_executed {
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            predecessor
                .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        )
    } else if latest.dehumidification_control_guard_false_fallthrough {
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            predecessor.dehumidification_control_guard_false_fallthrough_count,
        )
    } else {
        return false;
    };
    pair.0 > 0 && pair.1 > 0
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
