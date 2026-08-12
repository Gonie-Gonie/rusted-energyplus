//! Fail-closed validation for CP418 direct-release else-entry evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState as PredecessorState,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ELSE_ENTRY_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];
const EXPECTED_SOURCE_ORDER: [&str; 1] = [
    "enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-after-guard-false-fallthrough",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp417: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP418 evidence is missing".to_string())?;
    let predecessor = predecessor_cp417
        .ok_or_else(|| "direct-zone IdealLoads CP418 CP417 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP418 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP418 coupling call count is missing".to_string())?;
    if calls == 0 {
        return Err("direct-zone IdealLoads CP418 coupling call count is invalid".to_string());
    }

    validate_provenance(lifecycle, predecessor)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP418 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP418 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP418 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP418 CP417 latest is missing".to_string())?;

    if [lifecycle.state.system, predecessor.state.system]
        .into_iter()
        .any(|actual| actual != system)
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER
        || predecessor_latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || predecessor_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
        || latest.system != system
        || predecessor_latest.system != system
        || latest.controlled_zone != zone
        || predecessor_latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || predecessor_latest.parent_call_ordinal != calls
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP418 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
    {
        return Err("direct-zone IdealLoads CP418 provenance is invalid".to_string());
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
            != predecessor.supply_enthalpy_assignment_route_counts
    {
        return Err("direct-zone IdealLoads CP418 route lineage is invalid".to_string());
    }
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.predecessor_supply_temperature_saturation_assignment_route_counts,
        &state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        &state.predecessor_supply_humidity_ratio_assignment_route_counts,
        &state.predecessor_supply_enthalpy_assignment_route_counts,
        &state.dehumidification_guard_else_branch_entry_route_counts,
    ] {
        ensure_public_routes_only(values)?;
    }
    for (index, (&route_count, &entry_count)) in state
        .predecessor_route_counts
        .iter()
        .zip(&state.dehumidification_guard_else_branch_entry_route_counts)
        .enumerate()
    {
        let expected = if ELSE_ENTRY_LOGICAL_INDICES.contains(&index) {
            route_count
        } else {
            0
        };
        ensure_count(entry_count, expected, "else_entry_route_partition")?;
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let entries = checked_sum(
        &state.dehumidification_guard_else_branch_entry_route_counts,
        "else-entry partition",
    )?;
    let inactive = transitions
        .checked_sub(entries)
        .ok_or_else(|| "direct-zone IdealLoads CP418 inactive partition underflowed".to_string())?;
    let humidity_ratio_owners = checked_sum(
        &state.predecessor_route_counts[18..],
        "humidity-ratio owner partition",
    )?;
    let enthalpy_owners = sum_predecessor_indices(
        &state.predecessor_route_counts,
        |index| matches!(index, 5 | 8 | 11 | 14 | 17..=29),
        "enthalpy owner partition",
    )?;
    let temperature_owners = sum_predecessor_indices(
        &state.predecessor_route_counts,
        |index| index >= 3,
        "temperature owner partition",
    )?;

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
            "predecessor_supply_temperature_saturation_assignment_count",
            predecessor.predecessor_supply_temperature_saturation_assignment_count,
            state.predecessor_supply_temperature_saturation_assignment_count,
        ),
        (
            "predecessor_supply_temperature_saturation_mixed_air_limit_count",
            predecessor.predecessor_supply_temperature_saturation_mixed_air_limit_count,
            state.predecessor_supply_temperature_saturation_mixed_air_limit_count,
        ),
        (
            "predecessor_supply_humidity_ratio_assignment_count",
            predecessor.predecessor_supply_humidity_ratio_assignment_count,
            state.predecessor_supply_humidity_ratio_assignment_count,
        ),
        (
            "predecessor_supply_enthalpy_assignment_count",
            predecessor.supply_enthalpy_assignment_count,
            state.predecessor_supply_enthalpy_assignment_count,
        ),
        (
            "dehumidification_guard_else_branch_entry_count",
            entries,
            state.dehumidification_guard_else_branch_entry_count,
        ),
        (
            "source_site_execution_count",
            entries,
            state.source_site_execution_count,
        ),
        (
            "cp417_supply_humidity_ratio_state_owner_count",
            humidity_ratio_owners,
            state.cp417_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_ratio_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp417_supply_enthalpy_state_owner_count",
            enthalpy_owners,
            state.cp417_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp417_supply_temperature_state_owner_count",
            temperature_owners,
            state.cp417_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_owners,
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
                "direct-zone IdealLoads CP418 private route {index} has count {count}"
            ));
        }
    }
    Ok(())
}

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP418 {field} overflowed"))
    })
}

fn sum_predecessor_indices(
    values: &[usize; 36],
    include: impl Fn(usize) -> bool,
    field: &str,
) -> Result<usize, String> {
    let mut logical_index = 0usize;
    let mut total = 0usize;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(matches!(predecessor_index, 20 | 21 | 24 | 25 | 27 | 29));
        if include(predecessor_index) {
            total = values[logical_index..logical_index + width]
                .iter()
                .try_fold(total, |sum, value| sum.checked_add(*value))
                .ok_or_else(|| format!("direct-zone IdealLoads CP418 {field} overflowed"))?;
        }
        logical_index += width;
    }
    if logical_index == 36 {
        Ok(total)
    } else {
        Err("direct-zone IdealLoads CP418 logical partition width is invalid".to_string())
    }
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP418 {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conceptual_cp418_contract_is_54_routes_49_inactive_and_5_entries() {
        assert_eq!((54 - 5, ELSE_ENTRY_LOGICAL_INDICES.len()), (49, 5));
    }

    #[test]
    fn overflow_helper_fails_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "overflow").is_err());
    }
}
