//! Fail-closed validation for CP435 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE_ORDER,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] = &[
    "compare-heating-limit-to-flow-rate",
    "compare-heating-limit-to-flow-rate-and-capacity-after-short-circuit",
    "read-outdoor-air-mass-flow-after-limit-short-circuit",
    "read-maximum-heating-air-mass-flow-after-limit-short-circuit",
    "compare-strict-outdoor-air-above-maximum-heating-flow",
    "enter-maximum-heating-flow-body-if-satisfied",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp434: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP435 evidence is missing".to_string())?;
    let predecessor = predecessor_cp434
        .ok_or_else(|| "direct-zone IdealLoads CP435 CP434 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP435 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP435 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE_ORDER != ORDER
    {
        return Err("direct-zone IdealLoads CP435 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP435 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP435 controlled Zone is missing".to_string())?;
    let maximum_heating_air_mass_flow_rate_kg_per_s =
        init.maximum_heating_air_mass_flow_rate_kg_per_s;
    if !maximum_heating_air_mass_flow_rate_kg_per_s.is_finite()
        || maximum_heating_air_mass_flow_rate_kg_per_s < 0.0
    {
        return Err("direct-zone IdealLoads CP435 maximum heating flow is invalid".to_string());
    }
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP435 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP435 CP434 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE_ORDER
        || !lineage_is_exact(
            latest,
            predecessor_latest,
            maximum_heating_air_mass_flow_rate_kg_per_s,
        )
    {
        return Err("direct-zone IdealLoads CP435 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &ep_runtime::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRuntimeState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
    {
        return Err("direct-zone IdealLoads CP435 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        let predecessor_count = state.predecessor_route_counts[index];
        let false_count =
            state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts[index];
        let body_count = state.maximum_heating_flow_body_entry_route_counts[index];
        let evaluated = checked_add(false_count, body_count, "route evaluation partition")?;
        let expected_evaluated = if index == 1 { predecessor_count } else { 0 };
        if evaluated != expected_evaluated
            || (!PUBLIC.contains(&index)
                && (predecessor_count != 0 || false_count != 0 || body_count != 0))
            || body_count != 0
        {
            return Err(format!(
                "direct-zone IdealLoads CP435 route {index} is not public-release-ready"
            ));
        }
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let false_fallthroughs =
        checked_sum(&state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts)?;
    let body_entries = checked_sum(&state.maximum_heating_flow_body_entry_route_counts)?;
    let evaluations = checked_add(false_fallthroughs, body_entries, "evaluation partition")?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| "direct-zone IdealLoads CP435 inactive partition underflowed".to_string())?;
    let second_comparisons = evaluations
        .checked_sub(state.heating_limit_flow_rate_match_count)
        .ok_or_else(|| "direct-zone IdealLoads CP435 second selector underflowed".to_string())?;
    let selected_flow = checked_add(
        state.heating_limit_flow_rate_match_count,
        state.heating_limit_flow_rate_and_capacity_match_count,
        "selected-flow partition",
    )?;
    let selector_rejections = evaluations
        .checked_sub(selected_flow)
        .ok_or_else(|| "direct-zone IdealLoads CP435 selector partition underflowed".to_string())?;
    let source_sites = checked_add(
        checked_add(
            checked_add(evaluations, second_comparisons, "selector site partition")?,
            selected_flow.checked_mul(3).ok_or_else(|| {
                "direct-zone IdealLoads CP435 strict-site count overflowed".to_string()
            })?,
            "read/comparison site partition",
        )?,
        body_entries,
        "source-site partition",
    )?;
    if (state.heating_limit_flow_rate_match_count != 0
        && state.heating_limit_flow_rate_match_count != evaluations)
        || (state.heating_limit_flow_rate_and_capacity_match_count != 0
            && state.heating_limit_flow_rate_and_capacity_match_count != second_comparisons)
    {
        return Err("direct-zone IdealLoads CP435 selector history is inconsistent".to_string());
    }
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        (
            "guard_evaluation_count",
            evaluations,
            state.heating_outdoor_air_maximum_flow_guard_evaluation_count,
        ),
        (
            "false_fallthrough_count",
            false_fallthroughs,
            state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_count,
        ),
        ("body_entry_count", body_entries, state.maximum_heating_flow_body_entry_count),
        (
            "first_selector_comparison_count",
            evaluations,
            state.heating_limit_flow_rate_comparison_count,
        ),
        (
            "second_selector_comparison_count",
            second_comparisons,
            state.heating_limit_flow_rate_and_capacity_comparison_count,
        ),
        (
            "selector_rejection_count",
            selector_rejections,
            state.heating_flow_limit_selector_rejection_count,
        ),
        (
            "cp311_corroboration_count",
            selected_flow,
            state.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count,
        ),
        (
            "outdoor_air_read_count",
            selected_flow,
            state.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count,
        ),
        (
            "maximum_heating_flow_read_count",
            selected_flow,
            state.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count,
        ),
        (
            "strict_comparison_count",
            selected_flow,
            state.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count,
        ),
        (
            "strict_comparison_satisfied_count",
            body_entries,
            state.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count,
        ),
        ("source_site_execution_count", source_sites, state.source_site_execution_count),
        (
            "humidity_owner_count",
            predecessor.cp433_supply_humidity_ratio_state_owner_count,
            state.cp434_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp434_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp433_supply_enthalpy_state_owner_count,
            state.cp434_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp434_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp433_supply_temperature_state_owner_count,
            state.cp434_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp434_supply_temperature_state_owner_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values
        .iter()
        .try_fold(0usize, |sum, value| checked_add(sum, *value, "route count"))
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads CP435 {label} overflowed"))
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP435 invariant {field} expected {expected}, got {actual}"
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
            "heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts",
            "maximum_heating_flow_body_entry_route_counts",
            "expected_evaluated",
            "body_count != 0",
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
