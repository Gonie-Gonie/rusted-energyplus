//! Fail-closed validation for CP422 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::{links_to_predecessor, operation_shape_is_exact};

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ACTIVE_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp421: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP422 evidence is missing".to_string())?;
    let predecessor = predecessor_cp421
        .ok_or_else(|| "direct-zone IdealLoads CP422 CP421 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP422 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP422 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER.len() != 2
    {
        return Err("direct-zone IdealLoads CP422 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP422 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP422 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP422 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP422 CP421 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER
        || !links_to_predecessor(latest, predecessor_latest)
        || !operation_shape_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP422 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.guard_false_fallthrough_route_counts
        || state.cooling_sensible_output_maximum_capacity_assignment_route_counts
            != predecessor.adjustment_body_entry_route_counts
    {
        return Err("direct-zone IdealLoads CP422 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        for values in [
            &state.predecessor_route_counts,
            &state.predecessor_guard_false_fallthrough_route_counts,
            &state.cooling_sensible_output_maximum_capacity_assignment_route_counts,
        ] {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) && values[index] != 0 {
                return Err(format!(
                    "direct-zone IdealLoads CP422 non-direct route {index} is active"
                ));
            }
        }
        let successor = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(
                state.cooling_sensible_output_maximum_capacity_assignment_route_counts[index],
            )
            .ok_or_else(|| {
                "direct-zone IdealLoads CP422 successor partition overflowed".to_string()
            })?;
        let expected = if ACTIVE_LOGICAL_INDICES.contains(&index) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(successor, expected, "successor_route_partition")?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard-false partition",
    )?;
    let assignments = checked_sum(
        &state.cooling_sensible_output_maximum_capacity_assignment_route_counts,
        "assignment partition",
    )?;
    let active = guard_false
        .checked_add(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP422 active partition overflowed".to_string())?;
    let inactive = transitions
        .checked_sub(active)
        .ok_or_else(|| "direct-zone IdealLoads CP422 inactive partition underflowed".to_string())?;
    let sites = assignments
        .checked_mul(2)
        .ok_or_else(|| "direct-zone IdealLoads CP422 site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("predecessor_inactive_transition_count", predecessor.inactive_transition_count, state.inactive_transition_count),
        ("predecessor_guard_false_fallthrough_count", guard_false, state.predecessor_guard_false_fallthrough_count),
        ("cp421_guard_false_fallthrough_count", predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count, guard_false),
        ("cooling_sensible_output_maximum_capacity_assignment_count", assignments, state.cooling_sensible_output_maximum_capacity_assignment_count),
        ("cp421_body_entry_count", predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count, assignments),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp420_supply_humidity_ratio_state_owner_count, state.cp421_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", state.cp421_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp420_supply_enthalpy_state_owner_count, state.cp421_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", state.cp421_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp420_supply_temperature_state_owner_count, state.cp421_supply_temperature_state_owner_count),
        ("temperature_preservation_count", state.cp421_supply_temperature_state_owner_count, state.unchanged_supply_temperature_preservation_count),
        ("capacity_owned_read_count", assignments, state.cp421_retained_maximum_total_cooling_capacity_owned_read_count),
        ("capacity_read_count", assignments, state.maximum_total_cooling_capacity_for_sensible_output_assignment_read_count),
        ("assignment_write_count", assignments, state.cooling_sensible_output_maximum_capacity_assignment_write_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP422 {field} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP422 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_stays_on_the_bounded_cp421_prefix_and_cp422_local_tail() {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        for required in [
            "links_to_predecessor",
            "operation_shape_is_exact",
            "predecessor_route_counts",
            "cooling_sensible_output_maximum_capacity_assignment_route_counts",
        ] {
            assert!(source.contains(required), "{required}");
        }
        for forbidden in [
            "snapshot_is_exact",
            "private_characterization",
            "predecessor_route(",
            "DirectZonePurchasedAirCouplingInput",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
