//! Fail-closed validation for CP369 direct-release evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRuntimeState,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot,
    PurchasedAirInitLifecycleSummary,
};

type Lifecycle =
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleSummary;
type State =
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState;
type Snapshot =
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot;
type PredecessorLifecycle =
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycleSummary;
type PredecessorState = PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRuntimeState;
type PredecessorSnapshot = PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot;

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) case_break_cp368: Option<&'a PredecessorLifecycle>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP369 heating-availability guard evidence"
            .to_string()
    })?;
    let predecessor = predecessors.case_break_cp368.ok_or_else(|| {
        "direct-zone IdealLoads CP369 heating-availability guard has no CP368 evidence".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP369 heating-availability guard has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads CP369 heating-availability guard has no coupling call count"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads CP369 heating-availability guard has no declared system".to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads CP369 heating-availability guard has no controlled Zone".to_string()
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
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER.len()
            != 2
        || PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER.len()
            != 1
    {
        return Err("direct-zone IdealLoads CP369 provenance is invalid".into());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    validate_current_counters(state)?;
    validate_predecessor_counters(predecessor_state)?;

    let active = selected_count(state)?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        (
            "unit_off_skip_count",
            predecessor_state.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor_state.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "positive_guard_false_fallthrough_skip_count",
            predecessor_state.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "none_case_completed_skip_count",
            predecessor_state.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
        ),
        (
            "constant_shr_case_completed_skip_count",
            predecessor_state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        (
            "humidistat_case_completed_skip_count",
            predecessor_state.dehumidification_control_humidistat_case_completed_skip_count,
            state.dehumidification_control_humidistat_case_completed_skip_count,
        ),
        (
            "constant_supply_case_completed_skip_count",
            predecessor_state
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        ),
        (
            "direct_constant_shr_case_completed_skip_count",
            0,
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        (
            "direct_humidistat_case_completed_skip_count",
            0,
            state.dehumidification_control_humidistat_case_completed_skip_count,
        ),
        (
            "direct_constant_supply_case_completed_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        ),
        (
            "direct_heating_on_body_entry_count",
            active,
            state.heating_on_body_entry_count,
        ),
        (
            "direct_heating_on_guard_false_fallthrough_count",
            0,
            state.heating_on_guard_false_fallthrough_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads CP369 heating-availability guard has no latest snapshot".to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads CP369 heating-availability guard has no latest CP368 snapshot"
            .to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || latest.system != expected_system
        || predecessor_latest.system != expected_system
        || latest.parent_call_ordinal != calls
        || predecessor_latest.parent_call_ordinal != calls
        || latest.controlled_zone != expected_zone
        || predecessor_latest.controlled_zone != expected_zone
        || !predecessor_latest_is_exact_direct_shape(*predecessor_latest)
        || !latest_route_has_cumulative_evidence(state, predecessor_state, *predecessor_latest)
        || *latest != expected_snapshot(*predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP369 latest state is not release-ready".into());
    }
    Ok(())
}

fn validate_current_counters(state: &State) -> Result<(), String> {
    let selected = selected_count(state)?;
    let route_partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        selected,
    ])?;
    let guard_partition = state
        .heating_on_body_entry_count
        .checked_add(state.heating_on_guard_false_fallthrough_count)
        .ok_or_else(|| "heating_on_guard_partition overflowed".to_string())?;
    let source = state
        .heating_on_read_count
        .checked_add(state.heating_on_body_entry_count)
        .ok_or_else(|| "source_site_execution_count overflowed".to_string())?;
    for (field, expected, actual) in [
        (
            "transition_partition",
            state.transition_count,
            route_partition,
        ),
        (
            "heating_on_read_count",
            selected,
            state.heating_on_read_count,
        ),
        (
            "heating_on_guard_partition",
            state.heating_on_read_count,
            guard_partition,
        ),
        (
            "source_site_execution_count",
            source,
            state.source_site_execution_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn selected_count(state: &State) -> Result<usize, String> {
    checked_sum(&[
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ])
}

fn validate_predecessor_counters(state: &PredecessorState) -> Result<(), String> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
    ])?;
    for (field, expected, actual) in [
        (
            "predecessor_transition_partition",
            state.transition_count,
            partition,
        ),
        (
            "predecessor_default_case_break_count",
            0,
            state.dehumidification_control_default_supply_humidity_ratio_case_break_count,
        ),
        (
            "predecessor_source_site_execution_count",
            0,
            state.source_site_execution_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn predecessor_latest_is_exact_direct_shape(snapshot: PredecessorSnapshot) -> bool {
    if snapshot.source != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER
    {
        return false;
    }
    let predecessor_count = usize::from(
        snapshot.predecessor_dehumidification_control_none_case_completed_skip,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
    ) + usize::from(
        snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
    );
    let local_count = usize::from(snapshot.dehumidification_control_none_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(snapshot.dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        );
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0;
    let positive_guard_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
        && predecessor_count == 0
        && local_count == 0;
    let none_case = active_prefix(snapshot)
        && predecessor_count == 1
        && local_count == 1
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::None)
        && snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && snapshot.dehumidification_control_none_case_completed_skip;
    !snapshot
        .predecessor_dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
        && !snapshot.dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
        && (unit_off || non_cooling || positive_guard_false || none_case)
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor_state: &PredecessorState,
    latest: PredecessorSnapshot,
) -> bool {
    let (current_count, predecessor_count) = if latest.unit_off_skipped {
        (
            state.unit_off_skip_count,
            predecessor_state.unit_off_skip_count,
        )
    } else if latest.non_cooling_skipped {
        (
            state.non_cooling_skip_count,
            predecessor_state.non_cooling_skip_count,
        )
    } else if latest.positive_guard_false_fallthrough_skipped {
        (
            state.positive_guard_false_fallthrough_skip_count,
            predecessor_state.positive_guard_false_fallthrough_skip_count,
        )
    } else if latest.dehumidification_control_none_case_completed_skip {
        (
            state.dehumidification_control_none_case_completed_skip_count,
            predecessor_state.dehumidification_control_none_case_completed_skip_count,
        )
    } else {
        return false;
    };
    current_count > 0 && predecessor_count > 0
}

fn active_prefix(snapshot: PredecessorSnapshot) -> bool {
    !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_some()
}

fn inactive_prefix(snapshot: PredecessorSnapshot) -> bool {
    !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
}

fn expected_snapshot(predecessor: PredecessorSnapshot) -> Snapshot {
    let active = predecessor.dehumidification_control_none_case_completed_skip
        || predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        || predecessor.dehumidification_control_humidistat_case_completed_skip
        || predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip;
    Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type:
            predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip:
            predecessor.dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
            predecessor
                .dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip:
            predecessor.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        heating_on_read: active,
        heating_on: active.then_some(true),
        cooling_supply_humidity_ratio_humidification_body_entered: active,
        heating_on_guard_false_fallthrough: false,
    }
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "transition_partition overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP369 heating-availability guard {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
