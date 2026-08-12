//! Fail-closed validation for CP419 direct-release CpAir assignment evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as OwnerLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState as PredecessorState,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ASSIGNMENT_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];
const EXPECTED_SOURCE_ORDER: [&str; 3] = [
    "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air",
    "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air",
    "assign-local-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp418: Option<&PredecessorLifecycle>,
    owner_cp329: Option<&OwnerLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP419 evidence is missing".to_string())?;
    let predecessor = predecessor_cp418
        .ok_or_else(|| "direct-zone IdealLoads CP419 CP418 evidence is missing".to_string())?;
    let owner = owner_cp329.ok_or_else(|| {
        "direct-zone IdealLoads CP419 CP329 owner evidence is missing".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP419 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP419 coupling call count is missing".to_string())?;
    if calls == 0 {
        return Err("direct-zone IdealLoads CP419 coupling call count is invalid".to_string());
    }

    validate_provenance(lifecycle, predecessor, owner)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    ensure_count(
        owner.state.transition_count,
        calls,
        "owner_transition_count",
    )?;

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP419 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP419 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP419 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP419 CP418 latest is missing".to_string())?;
    let owner_latest = owner
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP419 CP329 latest owner is missing".to_string())?;

    if [
        lifecycle.state.system,
        predecessor.state.system,
        owner.state.system,
    ]
    .into_iter()
    .any(|actual| actual != system)
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !lineage_is_exact(latest, predecessor_latest, owner_latest)
    {
        return Err("direct-zone IdealLoads CP419 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    owner: &OwnerLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || owner.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || owner.first_excluded_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
    {
        return Err("direct-zone IdealLoads CP419 provenance is invalid".to_string());
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
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.predecessor_guard_body_entry_route_counts
        || state.predecessor_supply_temperature_saturation_assignment_route_counts
            != predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        || state.predecessor_supply_temperature_mixed_air_limit_route_counts
            != predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts
        || state.predecessor_supply_humidity_ratio_assignment_route_counts
            != predecessor.predecessor_supply_humidity_ratio_assignment_route_counts
        || state.predecessor_supply_enthalpy_assignment_route_counts
            != predecessor.predecessor_supply_enthalpy_assignment_route_counts
        || state.predecessor_dehumidification_guard_else_branch_entry_route_counts
            != predecessor.dehumidification_guard_else_branch_entry_route_counts
    {
        return Err("direct-zone IdealLoads CP419 route lineage is invalid".to_string());
    }
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.predecessor_supply_temperature_saturation_assignment_route_counts,
        &state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        &state.predecessor_supply_humidity_ratio_assignment_route_counts,
        &state.predecessor_supply_enthalpy_assignment_route_counts,
        &state.predecessor_dehumidification_guard_else_branch_entry_route_counts,
        &state.dehumidification_guard_else_branch_cp_air_assignment_route_counts,
    ] {
        ensure_public_routes_only(values)?;
    }
    for (index, (&route_count, &assignment_count)) in state
        .predecessor_route_counts
        .iter()
        .zip(&state.dehumidification_guard_else_branch_cp_air_assignment_route_counts)
        .enumerate()
    {
        let expected = usize::from(ASSIGNMENT_LOGICAL_INDICES.contains(&index)) * route_count;
        ensure_count(assignment_count, expected, "assignment_route_partition")?;
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let assignments = checked_sum(
        &state.dehumidification_guard_else_branch_cp_air_assignment_route_counts,
        "assignment partition",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP419 inactive partition underflowed".to_string())?;
    let sites = assignments
        .checked_mul(EXPECTED_SOURCE_ORDER.len())
        .ok_or_else(|| "direct-zone IdealLoads CP419 site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "predecessor_else_entry_count",
            predecessor.dehumidification_guard_else_branch_entry_count,
            state.predecessor_dehumidification_guard_else_branch_entry_count,
        ),
        (
            "assignment_count",
            assignments,
            state.dehumidification_guard_else_branch_cp_air_assignment_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp419_owner_count",
            assignments,
            state.cp419_psychrometric_cp_air_state_owner_count,
        ),
        (
            "cp329_owned_read_count",
            assignments,
            state.cp329_retained_mixed_air_humidity_ratio_owned_read_count,
        ),
        (
            "mixed_air_humidity_ratio_read_count",
            assignments,
            state.mixed_air_humidity_ratio_for_cp_air_read_count,
        ),
        (
            "psychrometric_cp_air_evaluation_count",
            assignments,
            state.psychrometric_cp_air_evaluation_count,
        ),
        (
            "cp_air_assignment_write_count",
            assignments,
            state.cp_air_assignment_write_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp417_supply_humidity_ratio_state_owner_count,
            state.cp418_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            predecessor.unchanged_supply_humidity_ratio_preservation_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp417_supply_enthalpy_state_owner_count,
            state.cp418_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            predecessor.unchanged_supply_enthalpy_preservation_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp417_supply_temperature_state_owner_count,
            state.cp418_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            predecessor.unchanged_supply_temperature_preservation_count,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn ensure_public_routes_only(values: &[usize; 36]) -> Result<(), String> {
    for (index, count) in values.iter().enumerate() {
        if !PUBLIC_LOGICAL_INDICES.contains(&index) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP419 private route {index} has count {count}"
            ));
        }
    }
    Ok(())
}

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP419 {field} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP419 {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
