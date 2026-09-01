//! Fail-closed validation for CP438 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as PredecessorState,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] =
    &["increment-state-owned-outdoor-air-flow-maximum-heating-output-error-count"];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp437: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP438 evidence is missing".to_string())?;
    let predecessor = predecessor_cp437
        .ok_or_else(|| "direct-zone IdealLoads CP438 CP437 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP438 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP438 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE_ORDER
            != ORDER
    {
        return Err("direct-zone IdealLoads CP438 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP438 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP438 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP438 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP438 CP437 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_COUNTER_INCREMENT_SOURCE_ORDER
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP438 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.predecessor_guard_body_entry_route_counts
        || state.predecessor_volume_flow_assignment_route_counts
            != predecessor.predecessor_volume_flow_assignment_route_counts
        || state.predecessor_first_warning_guard_false_fallthrough_route_counts
            != predecessor
                .heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts
        || state.predecessor_first_warning_branch_entry_route_counts
            != predecessor.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts
    {
        return Err("direct-zone IdealLoads CP438 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        let arrays = [
            state.predecessor_route_counts[index],
            state.predecessor_guard_false_fallthrough_route_counts[index],
            state.predecessor_guard_body_entry_route_counts[index],
            state.predecessor_volume_flow_assignment_route_counts[index],
            state.predecessor_first_warning_guard_false_fallthrough_route_counts[index],
            state.predecessor_first_warning_branch_entry_route_counts[index],
            state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts
                [index],
        ];
        if (!PUBLIC.contains(&index) && arrays.into_iter().any(|count| count != 0))
            || arrays[2..].iter().any(|count| *count != 0)
        {
            return Err(format!(
                "direct-zone IdealLoads CP438 route {index} is not public-release-ready"
            ));
        }
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let increments = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts,
    )?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            transitions,
            state.inactive_transition_count,
        ),
        (
            "counter_increment_count",
            increments,
            state.outdoor_air_flow_maximum_heating_output_error_count_increment_count,
        ),
        (
            "source_site_execution_count",
            increments,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.unchanged_supply_humidity_ratio_preservation_count,
            state.cp437_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp437_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.unchanged_supply_enthalpy_preservation_count,
            state.cp437_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp437_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.unchanged_supply_temperature_preservation_count,
            state.cp437_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp437_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "warning_counter_state_owner_count",
            increments,
            state.cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        ),
        (
            "warning_counter_increment_write_count",
            increments,
            state.outdoor_air_flow_maximum_heating_output_error_count_increment_write_count,
        ),
        ("public_counter_increment_count", 0, increments),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "direct-zone IdealLoads CP438 route count overflow".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP438 {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_uses_seven_route_arrays_one_source_site_and_no_numerical_input() {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        let route_arrays = [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_body_entry_route_counts",
            "predecessor_volume_flow_assignment_route_counts",
            "predecessor_first_warning_guard_false_fallthrough_route_counts",
            "predecessor_first_warning_branch_entry_route_counts",
            "heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts",
        ];
        assert_eq!(route_arrays.len(), 7);
        for field in route_arrays {
            assert!(source.contains(field), "{field}");
        }
        assert!(source.contains("public_counter_increment_count"));
        assert!(source.contains("source_site_execution_count"));
        assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
        assert!(!source.contains("private_characterization"));
    }
}
