use super::*;

pub(super) fn validate(
    state: &State,
    humidity: &HumidityState,
    temperature: &TemperatureState,
    calls: usize,
) -> Result<(), String> {
    let carried = route_counts(state);
    let humidity_counts = humidity_route_counts(humidity);
    let temperature_counts = temperature_route_counts(temperature);
    if carried != humidity_counts || carried != temperature_counts {
        return Err("direct-zone IdealLoads CP379 carried route counters are invalid".into());
    }
    for (field, actual) in [
        (
            "private_humidistat_route_count",
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "private_none_maximum_route_count",
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "private_dehumidification_guard_fallthrough_route_count",
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
    ] {
        ensure_count(actual, 0, field)?;
    }
    let partition = checked_sum(&carried, "transition partition")?;
    let active = checked_sum(&carried[3..], "active route partition")?;
    let source_sites = active
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| "CP379 source-site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
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
        (
            "cp378_assignment_count",
            humidity.purchased_air_supply_humidity_ratio_saturation_limit_assignment_count,
            state.local_supply_enthalpy_after_saturation_limit_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    let temperature_owners = state
        .cp334_supply_temperature_mixed_air_limit_owner_count
        .checked_add(state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count)
        .ok_or_else(|| "CP379 temperature-owner count overflowed".to_string())?;
    ensure_count(temperature_owners, active, "temperature_owner_partition")
}

pub(super) fn latest_route_has_cumulative_evidence(
    state: &State,
    humidity: &HumidityState,
    temperature: &TemperatureState,
    latest: Snapshot,
) -> bool {
    let Some(index) = route_flags(latest).into_iter().position(|route| route) else {
        return false;
    };
    route_counts(state)[index] > 0
        && humidity_route_counts(humidity)[index] > 0
        && temperature_route_counts(temperature)[index] > 0
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

fn checked_sum(values: &[usize], partition: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("CP379 {partition} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP379 post-saturation supply-enthalpy assignment {field} expected {expected}, got {actual}"
        ))
    }
}
