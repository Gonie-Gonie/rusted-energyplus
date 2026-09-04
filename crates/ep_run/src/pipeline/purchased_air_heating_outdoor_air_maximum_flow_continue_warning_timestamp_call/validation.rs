//! Fail-closed validation for CP441 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE_ORDER,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallRuntimeState as PredecessorState,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] =
    &["reach-heating-outdoor-air-maximum-flow-continue-warning-timestamp-call-site"];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp440: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP441 evidence is missing".to_string())?;
    let predecessor = predecessor_cp440
        .ok_or_else(|| "direct-zone IdealLoads CP441 CP440 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP441 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP441 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE_ORDER
            != ORDER
    {
        return Err("direct-zone IdealLoads CP441 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP441 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP441 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP441 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP441 CP440 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE_ORDER
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP441 latest lineage is invalid".to_string());
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
            != predecessor.predecessor_first_warning_guard_false_fallthrough_route_counts
        || state.predecessor_first_warning_branch_entry_route_counts
            != predecessor.predecessor_first_warning_branch_entry_route_counts
        || state.predecessor_first_warning_counter_increment_route_counts
            != predecessor.predecessor_first_warning_counter_increment_route_counts
        || state.predecessor_first_warning_call_route_counts
            != predecessor.predecessor_first_warning_call_route_counts
        || state.predecessor_continue_warning_call_route_counts
            != predecessor.heating_outdoor_air_maximum_flow_continue_warning_call_route_counts
        || state.heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_route_counts
            != state.predecessor_continue_warning_call_route_counts
    {
        return Err("direct-zone IdealLoads CP441 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        let arrays = [
            state.predecessor_route_counts[index],
            state.predecessor_guard_false_fallthrough_route_counts[index],
            state.predecessor_guard_body_entry_route_counts[index],
            state.predecessor_volume_flow_assignment_route_counts[index],
            state.predecessor_first_warning_guard_false_fallthrough_route_counts[index],
            state.predecessor_first_warning_branch_entry_route_counts[index],
            state.predecessor_first_warning_counter_increment_route_counts[index],
            state.predecessor_first_warning_call_route_counts[index],
            state.predecessor_continue_warning_call_route_counts[index],
            state.heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_route_counts
                [index],
        ];
        if (!PUBLIC.contains(&index) && arrays.into_iter().any(|count| count != 0))
            || arrays[2..].iter().any(|count| *count != 0)
        {
            return Err(format!(
                "direct-zone IdealLoads CP441 route {index} is not public-release-ready"
            ));
        }
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let call_sites = checked_sum(
        &state.heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_route_counts,
    )?;
    let inactive = transitions
        .checked_sub(call_sites)
        .ok_or_else(|| "direct-zone IdealLoads CP441 transition partition underflow".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "call_site_count",
            call_sites,
            state.heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_site_count,
        ),
        (
            "source_site_execution_count",
            call_sites,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.unchanged_supply_humidity_ratio_preservation_count,
            state.cp440_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp440_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.unchanged_supply_enthalpy_preservation_count,
            state.cp440_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp440_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.unchanged_supply_temperature_preservation_count,
            state.cp440_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp440_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "warning_counter_owner_count",
            call_sites,
            state.cp440_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        ),
        (
            "warning_counter_preservation_count",
            state.cp440_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
            state.unchanged_outdoor_air_flow_maximum_heating_output_error_count_preservation_count,
        ),
        (
            "predecessor_continue_warning_call_count",
            predecessor.heating_outdoor_air_maximum_flow_continue_warning_call_site_count,
            call_sites,
        ),
        (
            "public_continue_warning_timestamp_call_count",
            0,
            call_sites,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "direct-zone IdealLoads CP441 route count overflow".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP441 {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_uses_ten_route_arrays_one_source_site_and_no_numerical_input() {
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
            "predecessor_first_warning_counter_increment_route_counts",
            "predecessor_first_warning_call_route_counts",
            "predecessor_continue_warning_call_route_counts",
            "heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_route_counts",
        ];
        assert_eq!(route_arrays.len(), 10);
        for field in route_arrays {
            assert!(source.contains(field), "{field}");
        }
        assert!(source.contains("public_continue_warning_timestamp_call_count"));
        assert!(source.contains("source_site_execution_count"));
        assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
        assert!(!source.contains("private_characterization"));
    }
}
