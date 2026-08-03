//! Fail-closed validation for CP406 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentRuntimeState as PredecessorState,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::{branch_shape_is_exact, carriers_are_preserved, links_to_predecessor};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp405: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP406 evidence is missing".to_string())?;
    let predecessor = predecessor_cp405
        .ok_or_else(|| "direct-zone IdealLoads CP406 CP405 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP406 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP406 coupling call count is missing".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER.len()
            != 1
    {
        return Err("direct-zone IdealLoads CP406 provenance is invalid".to_string());
    }

    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    for (field, expected, actual) in [
        ("transition_count", calls, lifecycle.state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.state.transition_count,
            lifecycle.state.transition_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP406 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP406 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP406 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP406 CP405 latest evidence is missing".to_string()
    })?;
    let latest_route_count = if latest
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered
    {
        lifecycle.state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count
    } else if latest
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed
    {
        lifecycle.state.predecessor_maximum_capacity_assignment_count
    } else {
        lifecycle.state.inactive_transition_count
    };
    if [lifecycle.state.system, predecessor.state.system]
        .into_iter()
        .any(|actual| actual != system)
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest_route_count == 0
        || !links_to_predecessor(latest, predecessor_latest)
        || !branch_shape_is_exact(latest, predecessor_latest)
        || !carriers_are_preserved(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP406 latest lineage is invalid".to_string());
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
            != predecessor.cooling_latent_output_maximum_capacity_assignment_route_counts
        || state.else_branch_entry_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
    {
        return Err("direct-zone IdealLoads CP406 route lineage is invalid".to_string());
    }

    for (index, count) in state.predecessor_route_counts.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP406 non-direct route {index} is active"
            ));
        }
        let successors = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(state.predecessor_maximum_capacity_assignment_route_counts[index])
            .ok_or_else(|| {
                "direct-zone IdealLoads CP406 successor partition overflowed".to_string()
            })?;
        let expected = if predecessor_index_is_active(index) {
            *count
        } else {
            0
        };
        ensure_count(successors, expected, "successor_route_partition")?;
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard-false partition",
    )?;
    let maximum_assignments = checked_sum(
        &state.predecessor_maximum_capacity_assignment_route_counts,
        "maximum-assignment partition",
    )?;
    let entries = checked_sum(
        &state.else_branch_entry_route_counts,
        "else-entry partition",
    )?;
    let inactive = transitions
        .checked_sub(entries)
        .ok_or_else(|| "direct-zone IdealLoads CP406 inactive partition underflowed".to_string())?;
    let predecessor_inactive = predecessor
        .inactive_transition_count
        .checked_add(predecessor.cooling_latent_output_maximum_capacity_assignment_count)
        .ok_or_else(|| {
            "direct-zone IdealLoads CP406 predecessor inactive partition overflowed".to_string()
        })?;

    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("predecessor_inactive_partition", predecessor_inactive, inactive),
        (
            "predecessor_guard_false_fallthrough_count",
            guard_false,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            "cp405_guard_false_fallthrough_count",
            predecessor.predecessor_guard_false_fallthrough_count,
            guard_false,
        ),
        (
            "predecessor_maximum_capacity_assignment_count",
            maximum_assignments,
            state.predecessor_maximum_capacity_assignment_count,
        ),
        (
            "cp405_maximum_capacity_assignment_count",
            predecessor.cooling_latent_output_maximum_capacity_assignment_count,
            maximum_assignments,
        ),
        (
            "else_branch_entry_count",
            entries,
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count,
        ),
        ("guard_false_else_entry_parity", guard_false, entries),
        ("source_site_execution_count", entries, state.source_site_execution_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP406 {label} overflowed"))
    })
}

const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP406 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
