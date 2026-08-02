//! Fail-closed validation for CP400 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedOwnerLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary as FlowOwnerLifecycle,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::{carriers_are_preserved, links_to_predecessor, operation_shape_is_exact};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp399: Option<&PredecessorLifecycle>,
    flow_owner_cp330: Option<&FlowOwnerLifecycle>,
    mixed_owner_cp329: Option<&MixedOwnerLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP400 evidence is missing".to_string())?;
    let predecessor = predecessor_cp399
        .ok_or_else(|| "direct-zone IdealLoads CP400 CP399 evidence is missing".to_string())?;
    let flow_owner = flow_owner_cp330.ok_or_else(|| {
        "direct-zone IdealLoads CP400 CP330 owner evidence is missing".to_string()
    })?;
    let mixed_owner = mixed_owner_cp329.ok_or_else(|| {
        "direct-zone IdealLoads CP400 CP329 owner evidence is missing".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP400 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP400 coupling call count is missing".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || flow_owner.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || flow_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || mixed_owner.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_owner.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len()
            != 8
    {
        return Err("direct-zone IdealLoads CP400 provenance is invalid".to_string());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    validate_public_route_contract(state, predecessor_state)?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        (
            "flow_owner_transition_count",
            calls,
            flow_owner.state.transition_count,
        ),
        (
            "mixed_owner_transition_count",
            calls,
            mixed_owner.state.transition_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP400 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP400 controlled Zone is missing".to_string())?;
    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP400 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP400 CP399 latest evidence is missing".to_string()
    })?;
    let flow_latest = flow_owner
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP400 CP330 latest owner is missing".to_string())?;
    let mixed_latest = mixed_owner
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP400 CP329 latest owner is missing".to_string())?;
    if state.system != system
        || predecessor_state.system != system
        || flow_owner.state.system != system
        || mixed_owner.state.system != system
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !links_to_predecessor(latest, predecessor_latest)
        || !operation_shape_is_exact(latest, predecessor_latest, flow_latest, mixed_latest)
        || !carriers_are_preserved(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP400 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    if state.predecessor_route_counts != predecessor.predecessor_route_counts {
        return Err("direct-zone IdealLoads CP400 route lineage is invalid".to_string());
    }
    for (index, value) in state.predecessor_route_counts.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *value != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP400 non-direct route {index} is active"
            ));
        }
    }
    let route_sum = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let assignments = state
        .dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count;
    let expected_assignments = checked_sum(
        &[
            state.predecessor_route_counts[20],
            state.predecessor_route_counts[21],
            state.predecessor_route_counts[24],
            state.predecessor_route_counts[25],
            state.predecessor_route_counts[27],
            state.predecessor_route_counts[29],
        ],
        "active route partition",
    )?;
    let humidity_carriers = checked_selected_sum(
        &state.predecessor_route_counts,
        &[18, 19, 22, 23, 26, 28],
        "supply humidity-ratio carrier partition",
    )?;
    let enthalpy_carriers = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            5, 8, 11, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        ],
        "supply enthalpy carrier partition",
    )?;
    let temperature_carriers = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            26, 27, 28, 29,
        ],
        "supply temperature carrier partition",
    )?;
    let inactive = state
        .transition_count
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP400 inactive partition underflowed".to_string())?;
    let sites = assignments
        .checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len())
        .ok_or_else(|| "direct-zone IdealLoads CP400 site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, route_sum),
        (
            "predecessor_cp_air_assignment_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count,
            assignments,
        ),
        ("active_route_partition", expected_assignments, assignments),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp399_supply_humidity_ratio_state_owner_count",
            humidity_carriers,
            state.cp399_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_carriers,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp399_supply_enthalpy_state_owner_count",
            enthalpy_carriers,
            state.cp399_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_carriers,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp399_supply_temperature_state_owner_count",
            temperature_carriers,
            state.cp399_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_carriers,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "supply_mass_flow_rate_owned_read_count",
            assignments,
            state.supply_mass_flow_rate_owned_read_count,
        ),
        (
            "supply_mass_flow_rate_bit_corroboration_count",
            assignments,
            state.supply_mass_flow_rate_bit_corroboration_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            assignments,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "cp_air_owned_read_count",
            assignments,
            state.cp_air_owned_read_count,
        ),
        ("cp_air_read_count", assignments, state.cp_air_read_count),
        (
            "supply_mass_flow_rate_times_cp_air_calculation_count",
            assignments,
            state.supply_mass_flow_rate_times_cp_air_calculation_count,
        ),
        (
            "mixed_air_temperature_owned_read_count",
            assignments,
            state.mixed_air_temperature_owned_read_count,
        ),
        (
            "mixed_air_temperature_read_count",
            assignments,
            state.mixed_air_temperature_read_count,
        ),
        (
            "supply_temperature_owned_read_count",
            assignments,
            state.supply_temperature_owned_read_count,
        ),
        (
            "supply_temperature_read_count",
            assignments,
            state.supply_temperature_read_count,
        ),
        (
            "mixed_air_minus_supply_temperature_calculation_count",
            assignments,
            state.mixed_air_minus_supply_temperature_calculation_count,
        ),
        (
            "cooling_sensible_output_calculation_count",
            assignments,
            state.cooling_sensible_output_calculation_count,
        ),
        (
            "cooling_sensible_output_assignment_write_count",
            assignments,
            state.cooling_sensible_output_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_selected_sum(
    values: &[usize; 30],
    indices: &[usize],
    label: &str,
) -> Result<usize, String> {
    indices.iter().try_fold(0usize, |sum, index| {
        sum.checked_add(values[*index])
            .ok_or_else(|| format!("direct-zone IdealLoads CP400 {label} overflowed"))
    })
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP400 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP400 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
