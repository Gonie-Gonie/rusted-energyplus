//! Fail-closed validation for CP375 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot,
    PurchasedAirInitLifecycleSummary,
};

type Lifecycle = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary;
type State = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState;
type Snapshot = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot;
type PredecessorLifecycle = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitLifecycleSummary;
type PredecessorState = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState;
type PredecessorSnapshot = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot;

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) maximum_limit_cp374: Option<&'a PredecessorLifecycle>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP375 maximum-assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessors.maximum_limit_cp374.ok_or_else(|| {
        "direct-zone IdealLoads CP375 maximum assignment has no CP374 evidence".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP375 maximum assignment has no initialization evidence".to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads CP375 maximum assignment has no coupling call count".to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads CP375 maximum assignment has no declared system".to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads CP375 maximum assignment has no controlled Zone".to_string()
    })?;
    validate_release_state(
        lifecycle,
        predecessor,
        expected_system,
        expected_zone,
        calls,
    )
}

fn validate_release_state(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> Result<(), String> {
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER.len()
            != 4
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE_ORDER.len()
            != 4
    {
        return Err("direct-zone IdealLoads CP375 provenance is invalid".into());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    validate_counts(state, predecessor_state, calls)?;
    if state.system != expected_system || predecessor_state.system != expected_system {
        return Err("direct-zone IdealLoads CP375 system identity is invalid".into());
    }

    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP375 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP375 predecessor latest evidence is missing".to_string()
    })?;
    if latest.system != expected_system
        || predecessor_latest.system != expected_system
        || latest.controlled_zone != expected_zone
        || predecessor_latest.controlled_zone != expected_zone
        || latest.parent_call_ordinal != calls
        || predecessor_latest.parent_call_ordinal != calls
        || !snapshots_match_exact_bits(latest, expected_snapshot(predecessor_latest))
        || !latest_route_has_cumulative_evidence(state, predecessor_state, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP375 latest lineage is invalid".into());
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &PredecessorState,
    calls: usize,
) -> Result<(), String> {
    let carried = [
        state.transition_count,
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
        predecessor.transition_count,
        predecessor.unit_off_skip_count,
        predecessor.non_cooling_skip_count,
        predecessor.positive_guard_false_fallthrough_skip_count,
        predecessor.heating_availability_guard_false_fallthrough_count,
        predecessor.humidification_control_guard_false_fallthrough_count,
        predecessor.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count,
        predecessor.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count,
        predecessor.dehumidification_control_guard_false_fallthrough_count,
    ];
    if carried != expected {
        return Err("direct-zone IdealLoads CP375 carried CP374 counters are invalid".into());
    }

    let partition = checked_sum(&carried[1..])?;
    let assignments = state
        .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count
        .checked_add(
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        )
        .ok_or_else(|| "CP375 assignment count overflowed".to_string())?;
    let source_sites = assignments
        .checked_mul(PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER.len())
        .ok_or_else(|| "CP375 source-site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        ("transition_partition", state.transition_count, partition),
        (
            "purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count",
            assignments,
            state.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count,
        ),
        (
            "supply_humidity_ratio_for_humidification_for_supply_maximum_read_count",
            assignments,
            state.supply_humidity_ratio_for_humidification_for_supply_maximum_read_count,
        ),
        (
            "source_shaped_two_argument_maximum_evaluation_count",
            assignments,
            state.source_shaped_two_argument_maximum_evaluation_count,
        ),
        (
            "purchased_air_supply_humidity_ratio_assignment_count",
            assignments,
            state.purchased_air_supply_humidity_ratio_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        ("direct_assignment_count", 0, assignments),
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
        (state.non_cooling_skip_count, predecessor.non_cooling_skip_count)
    } else if latest.positive_guard_false_fallthrough_skipped {
        (
            state.positive_guard_false_fallthrough_skip_count,
            predecessor.positive_guard_false_fallthrough_skip_count,
        )
    } else if latest.predecessor_heating_on_guard_false_fallthrough {
        (
            state.heating_availability_guard_false_fallthrough_count,
            predecessor.heating_availability_guard_false_fallthrough_count,
        )
    } else if latest.predecessor_humidification_control_guard_false_fallthrough {
        (
            state.humidification_control_guard_false_fallthrough_count,
            predecessor.humidification_control_guard_false_fallthrough_count,
        )
    } else if latest.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed {
        (
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            predecessor.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count,
        )
    } else if latest.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed {
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            predecessor.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count,
        )
    } else if latest.predecessor_dehumidification_control_guard_false_fallthrough {
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            predecessor.dehumidification_control_guard_false_fallthrough_count,
        )
    } else {
        return false;
    };
    pair.0 > 0 && pair.1 > 0
}

fn expected_snapshot(predecessor: PredecessorSnapshot) -> Snapshot {
    Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor.predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip: predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: predecessor.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: predecessor.predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip: predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip: predecessor.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_heating_on_read: predecessor.predecessor_heating_on_read,
        predecessor_heating_on: predecessor.predecessor_heating_on,
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered: predecessor.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        predecessor_heating_on_guard_false_fallthrough: predecessor.predecessor_heating_on_guard_false_fallthrough,
        predecessor_humidification_control_type_read: predecessor.predecessor_humidification_control_type_read,
        predecessor_humidification_control_type: predecessor.predecessor_humidification_control_type,
        predecessor_humidification_control_type_humidistat: predecessor.predecessor_humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered: predecessor.predecessor_humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough: predecessor.predecessor_humidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type_first_read: predecessor.predecessor_dehumidification_control_type_first_read,
        predecessor_first_dehumidification_control_type: predecessor.predecessor_first_dehumidification_control_type,
        predecessor_dehumidification_control_type_humidistat: predecessor.predecessor_dehumidification_control_type_humidistat,
        predecessor_dehumidification_control_type_second_read: predecessor.predecessor_dehumidification_control_type_second_read,
        predecessor_second_dehumidification_control_type: predecessor.predecessor_second_dehumidification_control_type,
        predecessor_dehumidification_control_type_none: predecessor.predecessor_dehumidification_control_type_none,
        predecessor_dehumidification_control_body_entered: predecessor.predecessor_dehumidification_control_body_entered,
        predecessor_dehumidification_control_guard_false_fallthrough: predecessor.predecessor_dehumidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed: predecessor.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed,
        predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed: predecessor.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed,
        predecessor_resulting_supply_humidity_ratio_for_humidification: predecessor.resulting_supply_humidity_ratio_for_humidification,
        dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed: false,
        dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed: false,
        purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read: false,
        purchased_air_supply_humidity_ratio_before_humidification_supply_maximum: None,
        supply_humidity_ratio_for_humidification_for_supply_maximum_read: false,
        supply_humidity_ratio_for_humidification_for_supply_maximum: None,
        source_shaped_two_argument_maximum_evaluated: false,
        maximum_supply_humidity_ratio: None,
        purchased_air_supply_humidity_ratio_assignment_performed: false,
        assigned_supply_humidity_ratio: None,
        resulting_supply_humidity_ratio: None,
    }
}

fn snapshots_match_exact_bits(left: Snapshot, right: Snapshot) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_supply_humidity_ratio_for_humidification,
            right.predecessor_resulting_supply_humidity_ratio_for_humidification,
        ),
        (
            left.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
            right.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
        ),
        (
            left.supply_humidity_ratio_for_humidification_for_supply_maximum,
            right.supply_humidity_ratio_for_humidification_for_supply_maximum,
        ),
        (
            left.maximum_supply_humidity_ratio,
            right.maximum_supply_humidity_ratio,
        ),
        (
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = left;
    let mut right_without_values = right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification = None;
        snapshot.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum = None;
        snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum = None;
        snapshot.maximum_supply_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left_without_values == right_without_values
}

fn options_have_exact_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "CP375 transition partition overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP375 maximum assignment {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
