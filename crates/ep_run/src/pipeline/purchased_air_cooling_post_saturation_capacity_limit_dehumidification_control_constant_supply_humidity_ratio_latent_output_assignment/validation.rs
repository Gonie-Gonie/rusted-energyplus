//! Fail-closed validation for CP401 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentLifecycleSummary as OwnerLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary as CorroboratorLifecycle,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::{carriers_are_preserved, links_to_predecessor, operation_shape_is_exact};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp400: Option<&PredecessorLifecycle>,
    total_output_owner_cp384: Option<&OwnerLifecycle>,
    total_output_corroborator_cp385: Option<&CorroboratorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP401 evidence is missing".to_string())?;
    let predecessor = predecessor_cp400
        .ok_or_else(|| "direct-zone IdealLoads CP401 CP400 evidence is missing".to_string())?;
    let owner = total_output_owner_cp384.ok_or_else(|| {
        "direct-zone IdealLoads CP401 CP384 owner evidence is missing".to_string()
    })?;
    let corroborator = total_output_corroborator_cp385.ok_or_else(|| {
        "direct-zone IdealLoads CP401 CP385 corroborator evidence is missing".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP401 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP401 coupling call count is missing".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || owner.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || corroborator.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || corroborator.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len()
            != 4
    {
        return Err("direct-zone IdealLoads CP401 provenance is invalid".to_string());
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
            "owner_transition_count",
            calls,
            owner.state.transition_count,
        ),
        (
            "corroborator_transition_count",
            calls,
            corroborator.state.transition_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP401 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP401 controlled Zone is missing".to_string())?;
    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP401 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP401 CP400 latest evidence is missing".to_string()
    })?;
    let owner_latest = owner
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP401 CP384 latest owner is missing".to_string())?;
    let corroborator_latest = corroborator.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP401 CP385 latest corroborator is missing".to_string()
    })?;
    if state.system != system
        || predecessor_state.system != system
        || owner.state.system != system
        || corroborator.state.system != system
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !links_to_predecessor(latest, predecessor_latest)
        || !operation_shape_is_exact(
            latest,
            predecessor_latest,
            owner_latest,
            corroborator_latest,
        )
        || !carriers_are_preserved(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP401 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    if state.predecessor_route_counts != predecessor.predecessor_route_counts {
        return Err("direct-zone IdealLoads CP401 route lineage is invalid".to_string());
    }
    for (index, value) in state.predecessor_route_counts.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *value != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP401 non-direct route {index} is active"
            ));
        }
    }
    let route_sum = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let assignments = state
        .dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count;
    let expected_assignments = checked_selected_sum(
        &state.predecessor_route_counts,
        &[20, 21, 24, 25, 27, 29],
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
        .ok_or_else(|| "direct-zone IdealLoads CP401 inactive partition underflowed".to_string())?;
    let sites = assignments
        .checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len())
        .ok_or_else(|| "direct-zone IdealLoads CP401 site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, route_sum),
        (
            "predecessor_sensible_output_assignment_count",
            predecessor.dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count,
            assignments,
        ),
        ("active_route_partition", expected_assignments, assignments),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        (
            "cp400_supply_humidity_ratio_state_owner_count",
            humidity_carriers,
            state.cp400_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_carriers,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp400_supply_enthalpy_state_owner_count",
            enthalpy_carriers,
            state.cp400_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_carriers,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp400_supply_temperature_state_owner_count",
            temperature_carriers,
            state.cp400_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_carriers,
            state.unchanged_supply_temperature_preservation_count,
        ),
        ("cooling_total_output_owned_read_count", assignments, state.cooling_total_output_owned_read_count),
        ("cooling_total_output_bit_corroboration_count", assignments, state.cooling_total_output_bit_corroboration_count),
        ("cooling_total_output_read_count", assignments, state.cooling_total_output_read_count),
        ("cooling_sensible_output_owned_read_count", assignments, state.cooling_sensible_output_owned_read_count),
        ("cooling_sensible_output_read_count", assignments, state.cooling_sensible_output_read_count),
        ("cooling_latent_output_calculation_count", assignments, state.cooling_latent_output_calculation_count),
        ("cooling_latent_output_assignment_write_count", assignments, state.cooling_latent_output_assignment_write_count),
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
            .ok_or_else(|| format!("direct-zone IdealLoads CP401 {label} overflowed"))
    })
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP401 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP401 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
