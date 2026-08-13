//! Fail-closed bounded validation for CP421 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary as CapacityLifecycle,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary as CorroboratorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ACTIVE_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];

#[allow(clippy::too_many_arguments)]
pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp420: Option<&PredecessorLifecycle>,
    capacity_cp321: Option<&CapacityLifecycle>,
    corroborator_cp340: Option<&CorroboratorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = required(lifecycle, "CP421 evidence")?;
    let predecessor = required(predecessor_cp420, "CP421 CP420 predecessor evidence")?;
    let capacity = required(capacity_cp321, "CP421 CP321 capacity-owner evidence")?;
    let corroborator = required(
        corroborator_cp340,
        "CP421 CP340 capacity-corroborator evidence",
    )?;
    let init = required(init_lifecycle, "CP421 initialization evidence")?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP421 coupling call count is missing".to_string())?;
    if calls == 0 {
        return Err("direct-zone IdealLoads CP421 coupling call count is invalid".to_string());
    }
    validate_provenance(lifecycle, predecessor, capacity, corroborator)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    for (field, actual) in [
        ("transition_count", lifecycle.state.transition_count),
        (
            "CP420 predecessor transition_count",
            predecessor.state.transition_count,
        ),
        (
            "CP321 owner transition_count",
            capacity.state.transition_count,
        ),
        (
            "CP340 corroborator transition_count",
            corroborator.state.transition_count,
        ),
    ] {
        ensure_count(actual, calls, field)?;
    }

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP421 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP421 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP421 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP421 CP420 latest is missing".to_string())?;
    let capacity_latest = capacity
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP421 CP321 latest is missing".to_string())?;
    let corroborator_latest = corroborator
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP421 CP340 latest is missing".to_string())?;
    if [
        lifecycle.state.system,
        predecessor.state.system,
        capacity.state.system,
        corroborator.state.system,
    ]
    .into_iter()
    .any(|actual| actual != system)
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !lineage_is_exact(
            latest,
            predecessor_latest,
            capacity_latest,
            corroborator_latest,
        )
    {
        return Err("direct-zone IdealLoads CP421 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn required<'a, T>(value: Option<&'a T>, what: &str) -> Result<&'a T, String> {
    value.ok_or_else(|| format!("direct-zone IdealLoads {what} is missing"))
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    capacity: &CapacityLifecycle,
    corroborator: &CorroboratorLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || capacity.source != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || capacity.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || corroborator.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE
        || corroborator.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER.len() != 4
    {
        return Err("direct-zone IdealLoads CP421 provenance is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    if state.predecessor_route_counts != predecessor.predecessor_route_counts {
        return Err("direct-zone IdealLoads CP421 route lineage is invalid".to_string());
    }
    for values in route_arrays(state) {
        ensure_public_routes_only(values)?;
    }
    for index in 0..36 {
        let partition = state.guard_false_fallthrough_route_counts[index]
            .checked_add(state.adjustment_body_entry_route_counts[index])
            .ok_or_else(|| "direct-zone IdealLoads CP421 route partition overflowed".to_string())?;
        let expected = usize::from(ACTIVE_LOGICAL_INDICES.contains(&index))
            .checked_mul(state.predecessor_route_counts[index])
            .ok_or_else(|| "direct-zone IdealLoads CP421 route partition overflowed".to_string())?;
        ensure_count(partition, expected, "guard route partition")?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let false_fallthroughs = checked_sum(
        &state.guard_false_fallthrough_route_counts,
        "false partition",
    )?;
    let body_entries = checked_sum(&state.adjustment_body_entry_route_counts, "body partition")?;
    let evaluations = false_fallthroughs
        .checked_add(body_entries)
        .ok_or_else(|| {
            "direct-zone IdealLoads CP421 evaluation partition overflowed".to_string()
        })?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| "direct-zone IdealLoads CP421 inactive partition underflowed".to_string())?;
    let sites = evaluations
        .checked_mul(3)
        .and_then(|value| value.checked_add(body_entries))
        .ok_or_else(|| "direct-zone IdealLoads CP421 source-site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("guard_evaluation_count", evaluations, state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp419_supply_humidity_ratio_state_owner_count, state.cp420_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", predecessor.unchanged_supply_humidity_ratio_preservation_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp419_supply_enthalpy_state_owner_count, state.cp420_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", predecessor.unchanged_supply_enthalpy_preservation_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp419_supply_temperature_state_owner_count, state.cp420_supply_temperature_state_owner_count),
        ("temperature_preservation_count", predecessor.unchanged_supply_temperature_preservation_count, state.unchanged_supply_temperature_preservation_count),
        ("comparison_true_count", body_entries, state.cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count),
        ("adjustment_body_entry_count", body_entries, state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count),
        ("guard_false_fallthrough_count", false_fallthroughs, state.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    for (field, actual) in [
        (
            "cp420_cooling_sensible_output_owned_read_count",
            state.cp420_cooling_sensible_output_owned_read_count,
        ),
        (
            "cooling_sensible_output_read_count",
            state.cooling_sensible_output_read_count,
        ),
        (
            "cp321_maximum_total_cooling_capacity_owned_read_count",
            state.cp321_maximum_total_cooling_capacity_owned_read_count,
        ),
        (
            "cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count",
            state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_sensible_output_maximum_total_cooling_capacity_comparison_count",
            state.cooling_sensible_output_maximum_total_cooling_capacity_comparison_count,
        ),
    ] {
        ensure_count(actual, evaluations, field)?;
    }
    Ok(())
}

fn route_arrays(state: &State) -> [&[usize; 36]; 3] {
    [
        &state.predecessor_route_counts,
        &state.guard_false_fallthrough_route_counts,
        &state.adjustment_body_entry_route_counts,
    ]
}

fn ensure_public_routes_only(values: &[usize; 36]) -> Result<(), String> {
    for (index, count) in values.iter().enumerate() {
        if !PUBLIC_LOGICAL_INDICES.contains(&index) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP421 private route {index} has count {count}"
            ));
        }
    }
    Ok(())
}

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP421 {field} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP421 {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn direct_validator_has_three_arrays_and_bounded_lineage() -> Result<(), &'static str> {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        let lineage = include_str!("validation/lineage.rs");
        let route_arrays = source
            .split_once("fn route_arrays")
            .ok_or("route-array helper")?
            .1
            .split_once("fn ensure_public_routes_only")
            .ok_or("route-array helper boundary")?
            .0;
        assert_eq!(route_arrays.matches("route_counts,").count(), 3);
        for required in [
            "snapshot.source",
            "snapshot.first_excluded_source",
            "snapshot.source_order",
            "cp420_cooling_sensible_output_for_capacity_guard_w\n            .is_none()",
            "sensible_output_capacity_adjustment_body_entered",
            "sensible_output_capacity_guard_false_fallthrough",
        ] {
            assert!(lineage.contains(required), "{required}");
        }
        for forbidden in ["private_characterization", "predecessor_route("] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
        Ok(())
    }
}
