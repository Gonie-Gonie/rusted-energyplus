//! Fail-closed validation for CP372 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot,
    PurchasedAirInitLifecycleSummary,
};

type Lifecycle =
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary;
type State =
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState;
type Snapshot =
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot;
type PredecessorLifecycle = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary;
type PredecessorState = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState;
type PredecessorSnapshot = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot;

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) dehumidification_control_guard_cp371: Option<&'a PredecessorLifecycle>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP372 humidifying-demand assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessors
        .dehumidification_control_guard_cp371
        .ok_or_else(|| {
            "direct-zone IdealLoads CP372 assignment has no CP371 evidence".to_string()
        })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP372 assignment has no initialization evidence".to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads CP372 assignment has no coupling call count".to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads CP372 assignment has no declared system".to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads CP372 assignment has no controlled Zone".to_string()
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
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len()
            != 2
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER.len()
            != 5
    {
        return Err("direct-zone IdealLoads CP372 provenance is invalid".into());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    validate_counts(state, predecessor_state, calls)?;
    if state.system != expected_system || predecessor_state.system != expected_system {
        return Err("direct-zone IdealLoads CP372 system identity is invalid".into());
    }

    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP372 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP372 predecessor latest evidence is missing".to_string()
    })?;
    if latest.system != expected_system
        || predecessor_latest.system != expected_system
        || latest.controlled_zone != expected_zone
        || predecessor_latest.controlled_zone != expected_zone
        || latest.parent_call_ordinal != calls
        || predecessor_latest.parent_call_ordinal != calls
        || latest != expected_snapshot(predecessor_latest)
        || !latest_route_has_cumulative_evidence(state, predecessor_state, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP372 latest lineage is invalid".into());
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
        state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        state.dehumidification_control_none_moisture_demand_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ];
    let expected = [
        predecessor.transition_count,
        predecessor.unit_off_skip_count,
        predecessor.non_cooling_skip_count,
        predecessor.positive_guard_false_fallthrough_skip_count,
        predecessor.heating_on_guard_false_fallthrough_count,
        predecessor.humidification_control_guard_false_fallthrough_count,
        predecessor.dehumidification_control_type_humidistat_match_count,
        predecessor.dehumidification_control_type_none_match_count,
        predecessor.dehumidification_control_guard_false_fallthrough_count,
    ];
    if carried != expected {
        return Err("direct-zone IdealLoads CP372 carried CP371 counters are invalid".into());
    }

    let partition = checked_sum(&carried[1..])?;
    let assignments = state
        .dehumidification_control_humidistat_moisture_demand_assignment_count
        .checked_add(state.dehumidification_control_none_moisture_demand_assignment_count)
        .ok_or_else(|| "CP372 assignment count overflowed".to_string())?;
    let source_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| "CP372 source-site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        ("transition_partition", state.transition_count, partition),
        (
            "humidification_moisture_demand_assignment_count",
            assignments,
            state.humidification_moisture_demand_assignment_count,
        ),
        (
            "zone_humidifying_setpoint_moisture_demand_read_count",
            assignments,
            state.zone_humidifying_setpoint_moisture_demand_read_count,
        ),
        (
            "zone_humidifying_setpoint_moisture_demand_assignment_count",
            assignments,
            state.zone_humidifying_setpoint_moisture_demand_assignment_count,
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
        (
            state.non_cooling_skip_count,
            predecessor.non_cooling_skip_count,
        )
    } else if latest.positive_guard_false_fallthrough_skipped {
        (
            state.positive_guard_false_fallthrough_skip_count,
            predecessor.positive_guard_false_fallthrough_skip_count,
        )
    } else if latest.predecessor_heating_on_guard_false_fallthrough {
        (
            state.heating_availability_guard_false_fallthrough_count,
            predecessor.heating_on_guard_false_fallthrough_count,
        )
    } else if latest.predecessor_humidification_control_guard_false_fallthrough {
        (
            state.humidification_control_guard_false_fallthrough_count,
            predecessor.humidification_control_guard_false_fallthrough_count,
        )
    } else if latest.dehumidification_control_type_humidistat == Some(true) {
        (
            state.dehumidification_control_humidistat_moisture_demand_assignment_count,
            predecessor.dehumidification_control_type_humidistat_match_count,
        )
    } else if latest.dehumidification_control_type_none == Some(true) {
        (
            state.dehumidification_control_none_moisture_demand_assignment_count,
            predecessor.dehumidification_control_type_none_match_count,
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

fn expected_snapshot(predecessor: PredecessorSnapshot) -> Snapshot {
    Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_type_first_read: predecessor.dehumidification_control_type_first_read,
        predecessor_first_dehumidification_control_type: predecessor.first_dehumidification_control_type,
        predecessor_dehumidification_control_type_humidistat: predecessor.dehumidification_control_type_humidistat,
        predecessor_dehumidification_control_type_second_read: predecessor.dehumidification_control_type_second_read,
        predecessor_second_dehumidification_control_type: predecessor.second_dehumidification_control_type,
        predecessor_dehumidification_control_type_none: predecessor.dehumidification_control_type_none,
        predecessor_dehumidification_control_body_entered: predecessor.dehumidification_control_body_entered,
        predecessor_dehumidification_control_guard_false_fallthrough: predecessor.dehumidification_control_guard_false_fallthrough,
        humidification_moisture_demand_assignment_executed: false,
        zone_humidifying_setpoint_moisture_demand_read: false,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
        zone_humidifying_setpoint_moisture_demand_assigned: false,
        assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
        resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
    }
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "CP372 transition partition overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP372 assignment {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
