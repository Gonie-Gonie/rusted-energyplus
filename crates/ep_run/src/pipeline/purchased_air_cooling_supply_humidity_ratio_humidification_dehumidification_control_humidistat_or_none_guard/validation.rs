//! Fail-closed validation for CP371 direct-release evidence.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId, ZoneId,
};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot,
    PurchasedAirInitLifecycleSummary,
};

type Lifecycle = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary;
type State = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState;
type Snapshot = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot;
type PredecessorLifecycle =
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary;
type PredecessorState =
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState;
type PredecessorSnapshot =
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot;

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) humidification_control_humidistat_guard_cp370:
        Option<&'a PredecessorLifecycle>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP371 nested dehumidification-control guard evidence".to_string()
    })?;
    let predecessor = predecessors
        .humidification_control_humidistat_guard_cp370
        .ok_or_else(|| "direct-zone IdealLoads CP371 guard has no CP370 evidence".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP371 guard has no initialization evidence".to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads CP371 guard has no coupling call count".to_string()
    })?;
    let expected_system =
        init.declared_system_order.first().copied().ok_or_else(|| {
            "direct-zone IdealLoads CP371 guard has no declared system".to_string()
        })?;
    let expected_zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP371 guard has no controlled Zone".to_string())?;
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
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER.len() != 5
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER.len() != 3
    {
        return Err("direct-zone IdealLoads CP371 provenance is invalid".into());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    validate_current_counters(state)?;
    validate_predecessor_counters(predecessor_state)?;
    if state.system != expected_system || predecessor_state.system != expected_system {
        return Err("direct-zone IdealLoads CP371 system identity is invalid".into());
    }
    ensure_count(state.transition_count, calls, "transition_count")?;
    let current_carried = [
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        state.heating_on_read_count,
        state.heating_on_body_entry_count,
        state.heating_on_guard_false_fallthrough_count,
        state.humidification_control_type_read_count,
        state.humidification_control_type_humidistat_comparison_count,
        state.humidification_control_body_entry_count,
        state.humidification_control_guard_false_fallthrough_count,
    ];
    let predecessor_carried = [
        predecessor_state.transition_count,
        predecessor_state.unit_off_skip_count,
        predecessor_state.non_cooling_skip_count,
        predecessor_state.positive_guard_false_fallthrough_skip_count,
        predecessor_state.dehumidification_control_none_case_completed_skip_count,
        predecessor_state
            .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        predecessor_state.dehumidification_control_humidistat_case_completed_skip_count,
        predecessor_state
            .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        predecessor_state.heating_on_read_count,
        predecessor_state.heating_on_body_entry_count,
        predecessor_state.heating_on_guard_false_fallthrough_count,
        predecessor_state.humidification_control_type_read_count,
        predecessor_state.humidification_control_type_humidistat_comparison_count,
        predecessor_state.humidification_control_body_entry_count,
        predecessor_state.humidification_control_guard_false_fallthrough_count,
    ];
    if current_carried != predecessor_carried {
        return Err("direct-zone IdealLoads CP371 carried CP370 counters are invalid".into());
    }
    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP371 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP371 predecessor latest evidence is missing".to_string()
    })?;
    if latest.system != expected_system
        || predecessor_latest.system != expected_system
        || latest.controlled_zone != expected_zone
        || predecessor_latest.controlled_zone != expected_zone
        || latest.parent_call_ordinal != calls
        || predecessor_latest.parent_call_ordinal != calls
        || !predecessor_latest_is_exact_direct_shape(predecessor_latest)
        || latest != expected_snapshot(predecessor_latest)
        || !latest_route_has_cumulative_evidence(state, predecessor_state, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP371 latest lineage is invalid".into());
    }
    Ok(())
}

fn validate_current_counters(state: &State) -> Result<(), String> {
    validate_carried_direct_counters(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        state.heating_on_read_count,
        state.heating_on_body_entry_count,
        state.heating_on_guard_false_fallthrough_count,
        state.humidification_control_type_read_count,
        state.humidification_control_type_humidistat_comparison_count,
        state.humidification_control_body_entry_count,
        state.humidification_control_guard_false_fallthrough_count,
    )?;
    let current_sites = [
        state.dehumidification_control_type_first_read_count,
        state.dehumidification_control_type_humidistat_comparison_count,
        state.dehumidification_control_type_humidistat_match_count,
        state.dehumidification_control_type_second_read_count,
        state.dehumidification_control_type_none_comparison_count,
        state.dehumidification_control_type_none_match_count,
        state.dehumidification_control_body_entry_count,
        state.dehumidification_control_guard_false_fallthrough_count,
        state.source_site_execution_count,
    ];
    if current_sites != [0; 9] {
        return Err("direct-zone IdealLoads public CP371 current-site counts are nonzero".into());
    }
    Ok(())
}

fn validate_predecessor_counters(state: &PredecessorState) -> Result<(), String> {
    validate_carried_direct_counters(
        state.transition_count,
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_case_completed_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
        state.heating_on_read_count,
        state.heating_on_body_entry_count,
        state.heating_on_guard_false_fallthrough_count,
        state.humidification_control_type_read_count,
        state.humidification_control_type_humidistat_comparison_count,
        state.humidification_control_body_entry_count,
        state.humidification_control_guard_false_fallthrough_count,
    )?;
    let expected_source = state
        .humidification_control_type_read_count
        .checked_mul(2)
        .and_then(|value| value.checked_add(state.humidification_control_body_entry_count))
        .ok_or_else(|| "CP370 source-site count overflowed".to_string())?;
    ensure_count(
        state.source_site_execution_count,
        expected_source,
        "predecessor_source_site_execution_count",
    )
}

#[allow(clippy::too_many_arguments)]
fn validate_carried_direct_counters(
    transitions: usize,
    unit_off: usize,
    non_cooling: usize,
    positive_guard_false: usize,
    none_case: usize,
    constant_shr_case: usize,
    humidistat_case: usize,
    constant_supply_case: usize,
    heating_reads: usize,
    heating_bodies: usize,
    heating_false: usize,
    humidification_reads: usize,
    humidistat_comparisons: usize,
    humidification_bodies: usize,
    humidification_false: usize,
) -> Result<(), String> {
    let partition = checked_sum(&[unit_off, non_cooling, positive_guard_false, none_case])?;
    for (field, expected, actual) in [
        ("transition_partition", transitions, partition),
        ("constant_shr_case_count", 0, constant_shr_case),
        ("humidistat_case_count", 0, humidistat_case),
        ("constant_supply_case_count", 0, constant_supply_case),
        ("heating_on_read_count", none_case, heating_reads),
        ("heating_on_body_entry_count", none_case, heating_bodies),
        ("heating_on_guard_false_fallthrough_count", 0, heating_false),
        (
            "humidification_control_type_read_count",
            none_case,
            humidification_reads,
        ),
        (
            "humidistat_comparison_count",
            humidification_reads,
            humidistat_comparisons,
        ),
        (
            "humidification_control_body_entry_count",
            0,
            humidification_bodies,
        ),
        (
            "humidification_control_guard_false_fallthrough_count",
            humidification_reads,
            humidification_false,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn predecessor_latest_is_exact_direct_shape(snapshot: PredecessorSnapshot) -> bool {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER
        || snapshot.predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
    {
        return false;
    }
    let predecessor_other = snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        || snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip
        || snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip;
    let current_other = snapshot
        .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        || snapshot.dehumidification_control_humidistat_case_completed_skip
        || snapshot.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip;
    let control_skipped = !snapshot.humidification_control_type_read
        && snapshot.humidification_control_type.is_none()
        && snapshot.humidification_control_type_humidistat.is_none()
        && !snapshot.humidification_control_body_entered
        && !snapshot.humidification_control_guard_false_fallthrough;
    let heating_skipped = !snapshot.predecessor_heating_on_read
        && snapshot.predecessor_heating_on.is_none()
        && !snapshot.predecessor_cooling_supply_humidity_ratio_humidification_body_entered
        && !snapshot.predecessor_heating_on_guard_false_fallthrough;
    let inactive_selector = snapshot.predecessor_dehumidification_control_type.is_none()
        && !snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && !predecessor_other
        && !snapshot.dehumidification_control_none_case_completed_skip
        && !current_other;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && inactive_selector
        && heating_skipped
        && control_skipped;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && inactive_selector
        && heating_skipped
        && control_skipped;
    let positive_guard_false = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && !snapshot.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.positive_guard_false_fallthrough_skipped
        && inactive_selector
        && heating_skipped
        && control_skipped;
    let none_case = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
        && !snapshot.positive_guard_false_fallthrough_skipped
        && snapshot.predecessor_dehumidification_control_type
            == Some(DehumidificationControlType::None)
        && snapshot.predecessor_dehumidification_control_none_case_completed_skip
        && !predecessor_other
        && snapshot.dehumidification_control_none_case_completed_skip
        && !current_other
        && snapshot.predecessor_heating_on_read
        && snapshot.predecessor_heating_on == Some(true)
        && snapshot.predecessor_cooling_supply_humidity_ratio_humidification_body_entered
        && !snapshot.predecessor_heating_on_guard_false_fallthrough
        && snapshot.humidification_control_type_read
        && snapshot.humidification_control_type == Some(HumidificationControlType::None)
        && snapshot.humidification_control_type_humidistat == Some(false)
        && !snapshot.humidification_control_body_entered
        && snapshot.humidification_control_guard_false_fallthrough;
    unit_off || non_cooling || positive_guard_false || none_case
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    latest: PredecessorSnapshot,
) -> bool {
    let (current, prior) = if latest.unit_off_skipped {
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
    } else if latest.dehumidification_control_none_case_completed_skip {
        (
            state.dehumidification_control_none_case_completed_skip_count,
            predecessor.dehumidification_control_none_case_completed_skip_count,
        )
    } else {
        return false;
    };
    current > 0 && prior > 0
}

fn expected_snapshot(predecessor: PredecessorSnapshot) -> Snapshot {
    Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
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
        predecessor_humidification_control_type_read: predecessor.humidification_control_type_read,
        predecessor_humidification_control_type: predecessor.humidification_control_type,
        predecessor_humidification_control_type_humidistat: predecessor.humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered: predecessor.humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough: predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_type_first_read: false,
        first_dehumidification_control_type: None,
        dehumidification_control_type_humidistat: None,
        dehumidification_control_type_second_read: false,
        second_dehumidification_control_type: None,
        dehumidification_control_type_none: None,
        dehumidification_control_body_entered: false,
        dehumidification_control_guard_false_fallthrough: false,
    }
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "CP371 transition partition overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual != expected {
        return Err(format!(
            "direct-zone IdealLoads CP371 guard {field} expected {expected}, got {actual}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
