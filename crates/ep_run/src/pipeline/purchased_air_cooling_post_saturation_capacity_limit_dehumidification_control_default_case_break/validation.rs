//! Fail-closed validation for CP410 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-default-case-via-break",
];
const ACTIVE_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp409: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP410 evidence is missing".to_string())?;
    let predecessor = predecessor_cp409
        .ok_or_else(|| "direct-zone IdealLoads CP410 CP409 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP410 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP410 coupling call count is missing".to_string())?;

    validate_provenance(lifecycle, predecessor)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP410 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP410 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP410 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP410 CP409 latest is missing".to_string())?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER
        || predecessor_latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE
        || predecessor_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || predecessor_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP410 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_DEFAULT_CASE_BREAK_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER.is_empty()
    {
        return Err("direct-zone IdealLoads CP410 provenance is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    if state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_maximum_capacity_assignment_route_counts
            != predecessor.predecessor_maximum_capacity_assignment_route_counts
    {
        return Err("direct-zone IdealLoads CP410 route lineage is invalid".to_string());
    }
    for (index, count) in state.predecessor_route_counts.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP410 non-direct route {index} is active"
            ));
        }
    }
    validate_route_evidence(
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_maximum_capacity_assignment_route_counts,
    )?;

    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard-false partition",
    )?;
    let maximum_assignments = checked_sum(
        &state.predecessor_maximum_capacity_assignment_route_counts,
        "maximum-assignment partition",
    )?;
    let predecessor_breaks = guard_false
        .checked_add(maximum_assignments)
        .ok_or_else(|| {
            "direct-zone IdealLoads CP410 predecessor break partition overflowed".to_string()
        })?;
    let predecessor_inactive = transitions.checked_sub(predecessor_breaks).ok_or_else(|| {
        "direct-zone IdealLoads CP410 predecessor inactive partition underflowed".to_string()
    })?;

    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "inactive_transition_count",
            transitions,
            state.inactive_transition_count,
        ),
        (
            "predecessor_inactive_transition_count",
            predecessor_inactive,
            predecessor.inactive_transition_count,
        ),
        (
            "predecessor_guard_false_fallthrough_count",
            guard_false,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            "cp409_guard_false_fallthrough_count",
            predecessor.predecessor_guard_false_fallthrough_count,
            guard_false,
        ),
        (
            "predecessor_maximum_capacity_assignment_count",
            maximum_assignments,
            state.predecessor_maximum_capacity_assignment_count,
        ),
        (
            "cp409_maximum_capacity_assignment_count",
            predecessor.predecessor_maximum_capacity_assignment_count,
            maximum_assignments,
        ),
        (
            "cp409_shared_case_break_count",
            predecessor
                .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count,
            predecessor_breaks,
        ),
        (
            "cp409_source_site_execution_count",
            predecessor.source_site_execution_count,
            predecessor_breaks,
        ),
        (
            "default_case_break_count",
            0,
            state.dehumidification_control_default_case_break_count,
        ),
        ("source_site_execution_count", 0, state.source_site_execution_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_evidence(
    routes: &[usize; 30],
    guard_false_routes: &[usize; 30],
    maximum_assignment_routes: &[usize; 30],
) -> Result<(), String> {
    for index in 0..routes.len() {
        let branch_count = guard_false_routes[index]
            .checked_add(maximum_assignment_routes[index])
            .ok_or_else(|| {
                format!("direct-zone IdealLoads CP410 route {index} branch count overflowed")
            })?;
        let expected = if ACTIVE_PREDECESSOR_INDICES.contains(&index) {
            routes[index]
        } else {
            0
        };
        ensure_count(branch_count, expected, "active_route_evidence_partition")?;
    }
    Ok(())
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP410 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP410 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
