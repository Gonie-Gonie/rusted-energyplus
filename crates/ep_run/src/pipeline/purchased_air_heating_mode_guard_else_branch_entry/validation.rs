//! Fail-closed validation for CP433 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE,
    PurchasedAirCalcHeatingModeGuardElseBranchEntryLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingModeGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] = &["enter-heating-mode-guard-else-branch-after-guard-false-fallthrough"];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp432: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP433 evidence is missing".to_string())?;
    let predecessor = predecessor_cp432
        .ok_or_else(|| "direct-zone IdealLoads CP433 CP432 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP433 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP433 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor.source != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_HEAT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER != ORDER
    {
        return Err("direct-zone IdealLoads CP433 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP433 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP433 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP433 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP433 CP432 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP433 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &ep_runtime::PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.heating_mode_guard_else_branch_entry_route_counts
            != predecessor.predecessor_heating_mode_guard_false_fallthrough_route_counts
    {
        return Err("direct-zone IdealLoads CP433 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        for values in [
            &state.predecessor_route_counts,
            &state.heating_mode_guard_else_branch_entry_route_counts,
        ] {
            if !PUBLIC.contains(&index) && values[index] != 0 {
                return Err(format!(
                    "direct-zone IdealLoads CP433 non-direct route {index} is active"
                ));
            }
        }
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let entries = checked_sum(&state.heating_mode_guard_else_branch_entry_route_counts)?;
    let inactive = predecessor
        .inactive_transition_count
        .checked_add(predecessor.heating_operating_mode_heat_assignment_count)
        .ok_or_else(|| "direct-zone IdealLoads CP433 inactive partition overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "else_entry_count",
            entries,
            state.heating_mode_guard_else_branch_entry_count,
        ),
        (
            "source_site_execution_count",
            entries,
            state.source_site_execution_count,
        ),
        (
            "humidity_owner_count",
            predecessor.cp431_supply_humidity_ratio_state_owner_count,
            state.cp432_supply_humidity_ratio_state_owner_count,
        ),
        (
            "humidity_preservation_count",
            state.cp432_supply_humidity_ratio_state_owner_count,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "enthalpy_owner_count",
            predecessor.cp431_supply_enthalpy_state_owner_count,
            state.cp432_supply_enthalpy_state_owner_count,
        ),
        (
            "enthalpy_preservation_count",
            state.cp432_supply_enthalpy_state_owner_count,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "temperature_owner_count",
            predecessor.cp431_supply_temperature_state_owner_count,
            state.cp432_supply_temperature_state_owner_count,
        ),
        (
            "temperature_preservation_count",
            state.cp432_supply_temperature_state_owner_count,
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
            .ok_or_else(|| "direct-zone IdealLoads CP433 count overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP433 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_is_structural_and_has_no_deadband_numerical_or_dto_feed() {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        for required in [
            "predecessor_route_counts",
            "heating_mode_guard_else_branch_entry_route_counts",
            "predecessor_heating_mode_guard_false_fallthrough_route_counts",
        ] {
            assert!(source.contains(required), "{required}");
        }
        for forbidden in [
            "IdealLoadsSensibleMode::Deadband",
            "calculation.mode",
            "DirectZonePurchasedAirCouplingInput",
            "private_characterization",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
}
