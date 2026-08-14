//! Fail-closed bounded validation for CP420 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary as SupplyFlowLifecycle,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ASSIGNMENT_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];

#[allow(clippy::too_many_arguments)]
pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp419: Option<&PredecessorLifecycle>,
    supply_flow_cp330: Option<&SupplyFlowLifecycle>,
    mixed_air_cp329: Option<&MixedAirLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = required(lifecycle, "CP420 evidence")?;
    let predecessor = required(predecessor_cp419, "CP420 CP419 predecessor evidence")?;
    let supply_flow = required(supply_flow_cp330, "CP420 CP330 owner evidence")?;
    let mixed_air = required(mixed_air_cp329, "CP420 CP329 owner evidence")?;
    let init = required(init_lifecycle, "CP420 initialization evidence")?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP420 coupling call count is missing".to_string())?;
    if calls == 0 {
        return Err("direct-zone IdealLoads CP420 coupling call count is invalid".to_string());
    }
    validate_provenance(lifecycle, predecessor, supply_flow, mixed_air)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    ensure_count(
        supply_flow.state.transition_count,
        calls,
        "CP330 owner transition_count",
    )?;
    ensure_count(
        mixed_air.state.transition_count,
        calls,
        "CP329 owner transition_count",
    )?;

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP420 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP420 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP420 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP420 CP419 latest is missing".to_string())?;
    let active = predecessor_latest
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed;
    let mixed_air_latest = if active {
        Some(mixed_air.state.latest.ok_or_else(|| {
            "direct-zone IdealLoads CP420 active CP329 owner is missing".to_string()
        })?)
    } else {
        None
    };
    let supply_flow_latest = if active {
        Some(supply_flow.state.latest.ok_or_else(|| {
            "direct-zone IdealLoads CP420 active CP330 owner is missing".to_string()
        })?)
    } else {
        None
    };
    if [
        lifecycle.state.system,
        predecessor.state.system,
        supply_flow.state.system,
        mixed_air.state.system,
    ]
    .into_iter()
    .any(|actual| actual != system)
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !lineage_is_exact(
            latest,
            predecessor_latest,
            mixed_air_latest,
            supply_flow_latest,
        )
    {
        return Err("direct-zone IdealLoads CP420 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn required<'a, T>(value: Option<&'a T>, what: &str) -> Result<&'a T, String> {
    value.ok_or_else(|| format!("direct-zone IdealLoads {what} is missing"))
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    supply_flow: &SupplyFlowLifecycle,
    mixed_air: &MixedAirLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || supply_flow.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || supply_flow.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || mixed_air.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len() != 8
    {
        return Err("direct-zone IdealLoads CP420 provenance is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    if !route_prefix_matches(state, predecessor) {
        return Err("direct-zone IdealLoads CP420 route lineage is invalid".to_string());
    }
    for values in route_arrays(state) {
        ensure_public_routes_only(values)?;
    }
    for (index, (&route_count, &assignment_count)) in state
        .predecessor_route_counts
        .iter()
        .zip(&state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts)
        .enumerate()
    {
        ensure_count(
            assignment_count,
            usize::from(ASSIGNMENT_LOGICAL_INDICES.contains(&index)) * route_count,
            "assignment_route_partition",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let assignments = checked_sum(
        &state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts,
        "assignment partition",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP420 inactive partition underflowed".to_string())?;
    let sites = assignments
        .checked_mul(8)
        .ok_or_else(|| "direct-zone IdealLoads CP420 site count overflowed".to_string())?;
    for (field, expected, actual) in base_counts(
        state,
        predecessor,
        transitions,
        inactive,
        assignments,
        sites,
    ) {
        ensure_count(actual, expected, field)?;
    }
    for (field, actual) in active_counters(state) {
        ensure_count(actual, assignments, field)?;
    }
    Ok(())
}

fn route_prefix_matches(state: &State, predecessor: &PredecessorState) -> bool {
    state.predecessor_route_counts == predecessor.predecessor_route_counts
        && state.predecessor_guard_false_fallthrough_route_counts
            == predecessor.predecessor_guard_false_fallthrough_route_counts
        && state.predecessor_guard_body_entry_route_counts
            == predecessor.predecessor_guard_body_entry_route_counts
        && state.predecessor_supply_temperature_saturation_assignment_route_counts
            == predecessor.predecessor_supply_temperature_saturation_assignment_route_counts
        && state.predecessor_supply_temperature_mixed_air_limit_route_counts
            == predecessor.predecessor_supply_temperature_mixed_air_limit_route_counts
        && state.predecessor_supply_humidity_ratio_assignment_route_counts
            == predecessor.predecessor_supply_humidity_ratio_assignment_route_counts
        && state.predecessor_supply_enthalpy_assignment_route_counts
            == predecessor.predecessor_supply_enthalpy_assignment_route_counts
        && state.predecessor_dehumidification_guard_else_branch_entry_route_counts
            == predecessor.predecessor_dehumidification_guard_else_branch_entry_route_counts
        && state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts
            == predecessor.dehumidification_guard_else_branch_cp_air_assignment_route_counts
}

fn route_arrays(state: &State) -> [&[usize; 36]; 10] {
    [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.predecessor_supply_temperature_saturation_assignment_route_counts,
        &state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        &state.predecessor_supply_humidity_ratio_assignment_route_counts,
        &state.predecessor_supply_enthalpy_assignment_route_counts,
        &state.predecessor_dehumidification_guard_else_branch_entry_route_counts,
        &state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts,
        &state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts,
    ]
}

fn ensure_public_routes_only(values: &[usize; 36]) -> Result<(), String> {
    for (index, count) in values.iter().enumerate() {
        if !PUBLIC_LOGICAL_INDICES.contains(&index) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP420 private route {index} has count {count}"
            ));
        }
    }
    Ok(())
}

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP420 {field} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP420 {field} expected {expected}, got {actual}"
        ))
    }
}

include!("validation/counts.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn direct_validator_is_bounded_and_has_no_recursive_exact_route() {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        assert!(source.contains("lineage_is_exact"));
        for forbidden in [
            "private_characterization",
            "snapshot_is_exact(",
            "predecessor_route(",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
