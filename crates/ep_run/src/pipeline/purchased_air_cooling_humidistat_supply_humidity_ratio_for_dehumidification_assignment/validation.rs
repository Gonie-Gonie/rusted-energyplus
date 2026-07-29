//! Fail-closed validation for CP360 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) moisture_demand_assignment_cp359:
        Option<&'a PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<
        &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleSummary,
    >,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose Humidistat supply-humidity-ratio-for-dehumidification assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessors
        .moisture_demand_assignment_cp359
        .ok_or_else(|| {
            "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment has no CP359 evidence"
                .to_string()
        })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment has no coupling call count"
            .to_string()
    })?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER
            .len()
            != 6
    {
        return Err(
            "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment provenance is invalid"
                .into(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let constant_shr =
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count;
    let humidistat = state
        .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count;
    validate_route_partition(state)?;
    validate_source_counters(state)?;
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
            "humidistat_supply_humidity_ratio_for_dehumidification_assignment_count",
            predecessor_state.dehumidification_control_humidistat_moisture_demand_assignment_count,
            humidistat,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            predecessor_state
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "direct_constant_shr_case_completed_skip_count",
            0,
            constant_shr,
        ),
        (
            "direct_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count",
            0,
            humidistat,
        ),
        (
            "direct_constant_supply_humidity_ratio_case_selected_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment has no latest snapshot"
            .to_string()
    })?;
    let predecessor_latest = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment has no latest CP359 snapshot"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment has no controlled Zone"
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
        || !snapshots_match_exact_bits(latest, &expected_snapshot(*predecessor_latest))
    {
        return Err(
            "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment latest state is not release-ready"
                .into(),
        );
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState,
) -> Result<(), String> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state
            .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_counters(
    state: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState,
) -> Result<(), String> {
    let assignments = state
        .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count;
    let source_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| "source_site_execution_count overflowed".to_string())?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "zone_dehumidifying_setpoint_moisture_demand_read_count",
            assignments,
            state.zone_dehumidifying_setpoint_moisture_demand_read_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            assignments,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "moisture_demand_derived_supply_humidity_ratio_calculation_count",
            assignments,
            state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
        ),
        (
            "zone_node_humidity_ratio_read_count",
            assignments,
            state.zone_node_humidity_ratio_read_count,
        ),
        (
            "supply_humidity_ratio_for_dehumidification_calculation_count",
            assignments,
            state.supply_humidity_ratio_for_dehumidification_calculation_count,
        ),
        (
            "supply_humidity_ratio_for_dehumidification_assignment_count",
            assignments,
            state.supply_humidity_ratio_for_dehumidification_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
) -> PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot {
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_humidistat_moisture_demand_assignment_executed:
            predecessor
                .dehumidification_control_humidistat_moisture_demand_assignment_executed,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s:
            predecessor.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed:
            predecessor
                .dehumidification_control_humidistat_moisture_demand_assignment_executed,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        zone_dehumidifying_setpoint_moisture_demand_read: false,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: None,
        supply_mass_flow_rate_read: false,
        supply_mass_flow_rate_kg_per_s: None,
        moisture_demand_derived_supply_humidity_ratio_calculated: false,
        moisture_demand_derived_supply_humidity_ratio: None,
        zone_node_humidity_ratio_read: false,
        zone_node_humidity_ratio: None,
        supply_humidity_ratio_for_dehumidification_calculated: false,
        calculated_supply_humidity_ratio_for_dehumidification: None,
        supply_humidity_ratio_for_dehumidification_assigned: false,
        assigned_supply_humidity_ratio_for_dehumidification: None,
        resulting_supply_humidity_ratio_for_dehumidification: None,
    }
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    right: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.moisture_demand_derived_supply_humidity_ratio,
            right.moisture_demand_derived_supply_humidity_ratio,
        ),
        (
            left.zone_node_humidity_ratio,
            right.zone_node_humidity_ratio,
        ),
        (
            left.calculated_supply_humidity_ratio_for_dehumidification,
            right.calculated_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.assigned_supply_humidity_ratio_for_dehumidification,
            right.assigned_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.resulting_supply_humidity_ratio_for_dehumidification,
            right.resulting_supply_humidity_ratio_for_dehumidification,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.moisture_demand_derived_supply_humidity_ratio = None;
        snapshot.zone_node_humidity_ratio = None;
        snapshot.calculated_supply_humidity_ratio_for_dehumidification = None;
        snapshot.assigned_supply_humidity_ratio_for_dehumidification = None;
        snapshot.resulting_supply_humidity_ratio_for_dehumidification = None;
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
            .ok_or_else(|| "transition_partition overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads Humidistat supply-humidity-ratio-for-dehumidification assignment {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
