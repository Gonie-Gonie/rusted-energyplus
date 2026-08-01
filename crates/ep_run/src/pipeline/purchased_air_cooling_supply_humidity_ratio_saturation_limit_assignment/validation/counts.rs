use super::*;

pub(super) fn validate(
    state: &State,
    predecessor: &PredecessorState,
    calls: usize,
) -> Result<(), String> {
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
        return Err("direct-zone IdealLoads CP378 carried CP377 counters are invalid".into());
    }
    let partition = checked_sum(&carried, "transition partition")?;
    let active = checked_sum(&carried[3..], "active route partition")?;
    let source_sites = active
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| "CP378 source-site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("transition_partition", state.transition_count, partition),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "original_local_read_count",
            active,
            state.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read_count,
        ),
        (
            "saturation_local_read_count",
            active,
            state.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read_count,
        ),
        (
            "minimum_evaluation_count",
            active,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "purchased_air_assignment_count",
            active,
            state.purchased_air_supply_humidity_ratio_saturation_limit_assignment_count,
        ),
        (
            "cp376_owner_count",
            active,
            state.cp376_original_supply_humidity_ratio_owner_count,
        ),
        (
            "cp377_owner_count",
            active,
            state.cp377_saturation_supply_humidity_ratio_owner_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

pub(super) fn latest_route_has_cumulative_evidence(
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

fn checked_sum(values: &[usize], partition: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("CP378 {partition} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP378 saturation-limit assignment {field} expected {expected}, got {actual}"
        ))
    }
}
