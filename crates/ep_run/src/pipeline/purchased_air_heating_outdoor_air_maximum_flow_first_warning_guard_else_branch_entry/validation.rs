//! Fail-closed validation for CP442 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallRuntimeState as PredecessorState,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] = &[
    "enter-heating-outdoor-air-maximum-flow-first-warning-guard-else-branch-after-guard-false-fallthrough",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp441: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP442 evidence is missing".to_string())?;
    let predecessor = predecessor_cp441
        .ok_or_else(|| "direct-zone IdealLoads CP442 CP441 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP442 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP442 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_CONTINUE_WARNING_TIMESTAMP_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER
            != ORDER
    {
        return Err("direct-zone IdealLoads CP442 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP442 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP442 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP442 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP442 CP441 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP442 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts
            != predecessor.predecessor_first_warning_guard_false_fallthrough_route_counts
    {
        return Err("direct-zone IdealLoads CP442 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        let arrays = [
            state.predecessor_route_counts[index],
            state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts
                [index],
        ];
        if !PUBLIC.contains(&index) && arrays.into_iter().any(|count| count != 0) {
            return Err(format!(
                "direct-zone IdealLoads CP442 route {index} is not public-release-ready"
            ));
        }
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let entries = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts,
    )?;
    let inactive = transitions
        .checked_sub(entries)
        .ok_or_else(|| "direct-zone IdealLoads CP442 transition partition underflow".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "else_branch_entry_count",
            entries,
            state.heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_count,
        ),
        (
            "source_site_execution_count",
            entries,
            state.source_site_execution_count,
        ),
        ("public_else_branch_entry_count", 0, entries),
        (
            "humidity_owner_count",
            predecessor.unchanged_supply_humidity_ratio_preservation_count,
            state.cp441_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp441_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.unchanged_supply_enthalpy_preservation_count,
            state.cp441_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp441_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.unchanged_supply_temperature_preservation_count,
            state.cp441_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp441_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "direct-zone IdealLoads CP442 route count overflow".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP442 {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_uses_two_route_arrays_one_structural_site_and_no_numerical_input() {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        let route_arrays = [
            "predecessor_route_counts",
            "heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts",
        ];
        assert_eq!(route_arrays.len(), 2);
        for field in route_arrays {
            assert!(source.contains(field), "{field}");
        }
        assert!(source.contains("predecessor_first_warning_guard_false_fallthrough_route_counts"));
        assert!(source.contains("public_else_branch_entry_count"));
        assert!(source.contains("source_site_execution_count"));
        assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
        assert!(!source.contains("private_characterization"));
        assert!(
            !source
                .contains("outdoor_air_flow_maximum_heating_output_error_count_state_owner_count")
        );
    }
}
