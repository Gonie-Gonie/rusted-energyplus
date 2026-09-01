//! Fail-closed validation for CP437 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE_ORDER,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] = &[
    "read-state-owned-outdoor-air-flow-maximum-heating-output-error-count",
    "compare-outdoor-air-flow-maximum-heating-output-error-count-less-than-one",
    "enter-heating-outdoor-air-maximum-flow-first-warning-branch-if-satisfied",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp436: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP437 evidence is missing".to_string())?;
    let predecessor = predecessor_cp436
        .ok_or_else(|| "direct-zone IdealLoads CP437 CP436 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP437 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP437 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_BODY_VOLUME_FLOW_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE_ORDER
            != ORDER
    {
        return Err("direct-zone IdealLoads CP437 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP437 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP437 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP437 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP437 CP436 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_FIRST_WARNING_GUARD_SOURCE_ORDER
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP437 latest lineage is invalid".to_string());
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
            != predecessor.heating_outdoor_air_volume_flow_assignment_route_counts
    {
        return Err("direct-zone IdealLoads CP437 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        let predecessor_count = state.predecessor_route_counts[index];
        let inherited_false_count = state.predecessor_guard_false_fallthrough_route_counts[index];
        let inherited_body_count = state.predecessor_guard_body_entry_route_counts[index];
        let assignment_count = state.predecessor_volume_flow_assignment_route_counts[index];
        let false_count = state
            .heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts
            [index];
        let entry_count =
            state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts[index];
        if (!PUBLIC.contains(&index)
            && (predecessor_count != 0
                || inherited_false_count != 0
                || inherited_body_count != 0
                || assignment_count != 0
                || false_count != 0
                || entry_count != 0))
            || assignment_count != 0
            || false_count != 0
            || entry_count != 0
        {
            return Err(format!(
                "direct-zone IdealLoads CP437 route {index} is not public-release-ready"
            ));
        }
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let false_fallthroughs = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts,
    )?;
    let branch_entries = checked_sum(
        &state.heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts,
    )?;
    let evaluations = false_fallthroughs
        .checked_add(branch_entries)
        .ok_or_else(|| "direct-zone IdealLoads CP437 guard evaluations overflowed".to_string())?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| "direct-zone IdealLoads CP437 inactive partition underflowed".to_string())?;
    let source_sites = false_fallthroughs
        .checked_mul(2)
        .and_then(|count| {
            branch_entries
                .checked_mul(3)
                .and_then(|entries| count.checked_add(entries))
        })
        .ok_or_else(|| "direct-zone IdealLoads CP437 source sites overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "guard_evaluation_count",
            evaluations,
            state.guard_evaluation_count,
        ),
        (
            "first_warning_branch_entry_count",
            branch_entries,
            state.first_warning_branch_entry_count,
        ),
        (
            "guard_false_fallthrough_count",
            false_fallthroughs,
            state.guard_false_fallthrough_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "counter_state_owner_count",
            evaluations,
            state.outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        ),
        (
            "counter_read_count",
            evaluations,
            state.outdoor_air_flow_maximum_heating_output_error_count_read_count,
        ),
        (
            "comparison_count",
            evaluations,
            state
                .outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_count,
        ),
        (
            "unchanged_counter",
            0,
            state.outdoor_air_flow_maximum_heating_output_error_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp435_supply_humidity_ratio_state_owner_count,
            state.cp436_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp436_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp435_supply_enthalpy_state_owner_count,
            state.cp436_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp436_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp435_supply_temperature_state_owner_count,
            state.cp436_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp436_supply_temperature_state_owner_count,
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
            .ok_or_else(|| "direct-zone IdealLoads CP437 route count overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP437 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_is_structural_public_only_and_has_no_numerical_or_dto_feed() {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        for required in [
            "predecessor_route_counts",
            "predecessor_volume_flow_assignment_route_counts",
            "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough_route_counts",
            "heating_outdoor_air_maximum_flow_first_warning_branch_entry_route_counts",
            "assignment_count != 0",
        ] {
            assert!(source.contains(required), "{required}");
        }
        for forbidden in [
            "calculation.mode",
            "DirectZonePurchasedAirCouplingInput",
            "private_characterization",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
