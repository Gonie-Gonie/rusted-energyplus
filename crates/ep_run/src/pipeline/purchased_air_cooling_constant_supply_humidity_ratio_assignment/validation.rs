//! Fail-closed validation for CP365 direct-release evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) case_entry_cp364:
        Option<&'a PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
    >,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose constant-supply-humidity-ratio assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessors.case_entry_cp364.ok_or_else(|| {
        "direct-zone IdealLoads constant-supply-humidity-ratio assignment has no CP364 evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads constant-supply-humidity-ratio assignment has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads constant-supply-humidity-ratio assignment has no coupling call count"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads constant-supply-humidity-ratio assignment has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads constant-supply-humidity-ratio assignment has no controlled Zone"
            .to_string()
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
    lifecycle: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
    predecessor: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> Result<(), String> {
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER.len()
            != 2
        || PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER.len()
            != 1
    {
        return Err(
            "direct-zone IdealLoads constant-supply-humidity-ratio assignment provenance is invalid"
                .into(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    validate_route_partition(state)?;
    validate_source_counters(state)?;
    validate_predecessor_counters(predecessor_state)?;

    let constant_shr =
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count;
    let humidistat = state.dehumidification_control_humidistat_case_completed_skip_count;
    let assigned = state.dehumidification_control_constant_supply_humidity_ratio_assignment_count;
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
            constant_shr,
        ),
        (
            "humidistat_case_completed_skip_count",
            predecessor_state.dehumidification_control_humidistat_case_completed_skip_count,
            humidistat,
        ),
        (
            "constant_supply_humidity_ratio_assignment_count",
            predecessor_state
                .dehumidification_control_constant_supply_humidity_ratio_case_entry_count,
            assigned,
        ),
        ("direct_constant_shr_skip_count", 0, constant_shr),
        ("direct_humidistat_completed_skip_count", 0, humidistat),
        ("direct_constant_supply_assignment_count", 0, assigned),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads constant-supply-humidity-ratio assignment has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads constant-supply-humidity-ratio assignment has no latest CP364 snapshot"
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
        || !predecessor_latest_is_exact_direct_shape(predecessor_latest)
        || *latest != expected_snapshot(*predecessor_latest)
    {
        return Err(
            "direct-zone IdealLoads constant-supply-humidity-ratio assignment latest state is not release-ready"
                .into(),
        );
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState,
) -> Result<(), String> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_assignment_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_counters(
    state: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState,
) -> Result<(), String> {
    let assigned = state.dehumidification_control_constant_supply_humidity_ratio_assignment_count;
    let source_sites = assigned
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads constant-supply-humidity-ratio assignment source counter overflow"
                .to_string()
        })?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "minimum_cooling_supply_air_humidity_ratio_read_count",
            assigned,
            state.minimum_cooling_supply_air_humidity_ratio_read_count,
        ),
        (
            "supply_humidity_ratio_assignment_count",
            assigned,
            state.supply_humidity_ratio_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_predecessor_counters(
    state: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState,
) -> Result<(), String> {
    let entered = state.dehumidification_control_constant_supply_humidity_ratio_case_entry_count;
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        entered,
    ])?;
    let source_sites = entered
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads CP364 predecessor source counter overflow".to_string()
        })?;
    for (field, expected, actual) in [
        (
            "predecessor_transition_partition",
            state.transition_count,
            partition,
        ),
        (
            "predecessor_source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn predecessor_latest_is_exact_direct_shape(
    snapshot: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
) -> bool {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER
    {
        return false;
    }
    let predecessor_count = usize::from(
        snapshot.predecessor_dehumidification_control_none_case_completed_skip,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
    ) + usize::from(
        snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
    ) + usize::from(
        snapshot
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
    );
    let local_count = usize::from(snapshot.dehumidification_control_none_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        )
        + usize::from(snapshot.dehumidification_control_humidistat_case_completed_skip)
        + usize::from(
            snapshot.dehumidification_control_constant_supply_humidity_ratio_case_entered,
        );
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && inactive_predecessor_prefix(snapshot)
        && predecessor_count == 0
        && local_count == 0;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && inactive_predecessor_prefix(snapshot)
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
    let none_case = active_predecessor_prefix(snapshot)
        && predecessor_count == 1
        && local_count == 1
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::None)
        && snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && snapshot.dehumidification_control_none_case_completed_skip;
    unit_off || non_cooling || positive_guard_false || none_case
}

fn active_predecessor_prefix(
    snapshot: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
) -> bool {
    !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_some()
}

fn inactive_predecessor_prefix(
    snapshot: &PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
) -> bool {
    !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type.is_none()
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
) -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot {
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_entered,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip:
            predecessor.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_assignment_executed:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_entered,
        minimum_cooling_supply_air_humidity_ratio_read: false,
        minimum_cooling_supply_air_humidity_ratio: None,
        supply_humidity_ratio_assigned: false,
        assigned_supply_humidity_ratio: None,
        resulting_supply_humidity_ratio: None,
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
            "direct-zone IdealLoads constant-supply-humidity-ratio assignment {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
