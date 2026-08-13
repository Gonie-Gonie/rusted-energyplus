//! Fail-closed validation for CP423 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::{links_to_predecessor, operation_shape_is_exact};

const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ACTIVE_LOGICAL_INDICES: [usize; 5] = [4, 7, 10, 13, 16];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp422: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP423 evidence is missing".to_string())?;
    let predecessor = predecessor_cp422
        .ok_or_else(|| "direct-zone IdealLoads CP423 CP422 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP423 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP423 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len() != 8
    {
        return Err("direct-zone IdealLoads CP423 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP423 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP423 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP423 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP423 CP422 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        || !links_to_predecessor(latest, predecessor_latest)
        || !operation_shape_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP423 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.cooling_sensible_output_supply_temperature_assignment_route_counts
            != predecessor.cooling_sensible_output_maximum_capacity_assignment_route_counts
    {
        return Err("direct-zone IdealLoads CP423 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        for values in [
            &state.predecessor_route_counts,
            &state.predecessor_guard_false_fallthrough_route_counts,
            &state.cooling_sensible_output_supply_temperature_assignment_route_counts,
        ] {
            if !PUBLIC_LOGICAL_INDICES.contains(&index) && values[index] != 0 {
                return Err(format!(
                    "direct-zone IdealLoads CP423 non-direct route {index} is active"
                ));
            }
        }
        let successor = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(
                state.cooling_sensible_output_supply_temperature_assignment_route_counts[index],
            )
            .ok_or_else(|| "direct-zone IdealLoads CP423 route partition overflowed".to_string())?;
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
        &state.cooling_sensible_output_supply_temperature_assignment_route_counts,
        "assignment partition",
    )?;
    let active = guard_false
        .checked_add(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP423 active partition overflowed".to_string())?;
    let inactive = transitions
        .checked_sub(active)
        .ok_or_else(|| "direct-zone IdealLoads CP423 inactive partition underflowed".to_string())?;
    let temperature_preservations = state
        .cp422_supply_temperature_state_owner_count
        .checked_sub(assignments)
        .ok_or_else(|| {
            "direct-zone IdealLoads CP423 temperature preservation underflowed".to_string()
        })?;
    let sites = assignments
        .checked_mul(8)
        .ok_or_else(|| "direct-zone IdealLoads CP423 site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("predecessor_inactive_transition_count", predecessor.inactive_transition_count, state.inactive_transition_count),
        ("predecessor_guard_false_fallthrough_count", guard_false, state.predecessor_guard_false_fallthrough_count),
        ("cp422_guard_false_fallthrough_count", predecessor.predecessor_guard_false_fallthrough_count, guard_false),
        ("assignment_count", assignments, state.cooling_sensible_output_supply_temperature_assignment_count),
        ("cp422_assignment_count", predecessor.cooling_sensible_output_maximum_capacity_assignment_count, assignments),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp421_supply_humidity_ratio_state_owner_count, state.cp422_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", state.cp422_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp421_supply_enthalpy_state_owner_count, state.cp422_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", state.cp422_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp421_supply_temperature_state_owner_count, state.cp422_supply_temperature_state_owner_count),
        ("temperature_preservation_count", temperature_preservations, state.unchanged_supply_temperature_preservation_count),
        ("cp423_temperature_owner_count", assignments, state.cp423_sensible_output_supply_temperature_state_owner_count),
        ("mixed_air_owned_read_count", assignments, state.cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read_count),
        ("mixed_air_read_count", assignments, state.mixed_air_temperature_for_sensible_output_supply_temperature_read_count),
        ("cooling_output_owned_read_count", assignments, state.cp422_retained_cooling_sensible_output_owned_read_count),
        ("cooling_output_read_count", assignments, state.cooling_sensible_output_for_supply_temperature_read_count),
        ("mass_flow_owned_read_count", assignments, state.cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read_count),
        ("mass_flow_corroboration_count", assignments, state.cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroboration_count),
        ("mass_flow_read_count", assignments, state.supply_mass_flow_rate_for_sensible_output_supply_temperature_read_count),
        ("cp_air_owned_read_count", assignments, state.cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read_count),
        ("cp_air_read_count", assignments, state.cp_air_for_sensible_output_supply_temperature_read_count),
        ("air_capacity_rate_count", assignments, state.supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculation_count),
        ("temperature_drop_count", assignments, state.cooling_sensible_output_over_air_capacity_rate_calculation_count),
        ("temperature_calculation_count", assignments, state.sensible_output_supply_temperature_calculation_count),
        ("temperature_write_count", assignments, state.sensible_output_supply_temperature_assignment_write_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize], field: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP423 {field} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP423 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_stays_on_the_bounded_cp422_prefix_and_cp423_local_tail() {
        let source = include_str!("validation.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("validation.rs"), |(production, _)| production);
        for required in [
            "links_to_predecessor",
            "operation_shape_is_exact",
            "predecessor_route_counts",
            "cooling_sensible_output_supply_temperature_assignment_route_counts",
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
