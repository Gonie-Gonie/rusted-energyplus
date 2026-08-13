//! Fail-closed validation for CP424 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntryRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] =
    &["enter-cooling-supply-mass-flow-positive-guard-else-branch-after-guard-false-fallthrough"];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp423: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP424 evidence is missing".to_string())?;
    let predecessor = predecessor_cp423
        .ok_or_else(|| "direct-zone IdealLoads CP424 CP423 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP424 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP424 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || lifecycle.first_excluded_source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER != ORDER
    {
        return Err("direct-zone IdealLoads CP424 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP424 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP424 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP424 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP424 CP423 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || latest.first_excluded_source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || latest.source_order != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP424 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
    {
        return Err("direct-zone IdealLoads CP424 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        for values in [
            &state.predecessor_route_counts,
            &state.positive_supply_mass_flow_guard_else_branch_entry_route_counts,
        ] {
            if !PUBLIC.contains(&index) && values[index] != 0 {
                return Err(format!(
                    "direct-zone IdealLoads CP424 non-direct route {index} is active"
                ));
            }
        }
        let expected = if index == 2 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(
            state.positive_supply_mass_flow_guard_else_branch_entry_route_counts[index],
            expected,
            "else_entry_route_partition",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let entries =
        checked_sum(&state.positive_supply_mass_flow_guard_else_branch_entry_route_counts)?;
    let inactive = transitions
        .checked_sub(entries)
        .ok_or_else(|| "direct-zone IdealLoads CP424 inactive partition underflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "entry_count",
            entries,
            state.positive_supply_mass_flow_guard_else_branch_entry_count,
        ),
        (
            "source_site_execution_count",
            entries,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp422_supply_humidity_ratio_state_owner_count,
            state.cp423_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp423_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp422_supply_enthalpy_state_owner_count,
            state.cp423_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp423_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp422_supply_temperature_state_owner_count,
            state.cp423_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp423_supply_temperature_state_owner_count,
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
            .ok_or_else(|| "direct-zone IdealLoads CP424 count overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP424 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_stays_on_the_bounded_cp423_prefix_and_local_marker() {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        for required in [
            "lineage_is_exact",
            "predecessor_route_counts",
            "positive_supply_mass_flow_guard_else_branch_entry_route_counts",
        ] {
            assert!(source.contains(required), "{required}");
        }
        for forbidden in [
            "snapshot_is_exact",
            "private_characterization",
            "DirectZonePurchasedAirCouplingInput",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
