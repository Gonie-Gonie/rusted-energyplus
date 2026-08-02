//! Fail-closed validation for CP402 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary as OwnerLifecycle,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary as CorroboratorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::{carriers_are_preserved, links_to_predecessor, operation_shape_is_exact};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp401: Option<&PredecessorLifecycle>,
    capacity_owner_cp321: Option<&OwnerLifecycle>,
    capacity_corroborator_cp340: Option<&CorroboratorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP402 evidence is missing".to_string())?;
    let predecessor = predecessor_cp401.ok_or_else(|| {
        "direct-zone IdealLoads CP402 CP401 latest evidence is missing".to_string()
    })?;
    let owner = capacity_owner_cp321
        .ok_or_else(|| "direct-zone IdealLoads CP402 CP321 latest owner is missing".to_string())?;
    let corroborator = capacity_corroborator_cp340.ok_or_else(|| {
        "direct-zone IdealLoads CP402 CP340 latest corroborator is missing".to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP402 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP402 coupling call count is missing".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || owner.source != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || corroborator.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE
        || corroborator.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER.len()
            != 4
    {
        return Err("direct-zone IdealLoads CP402 provenance is invalid".to_string());
    }

    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    for (field, expected, actual) in [
        ("transition_count", calls, lifecycle.state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.state.transition_count,
            lifecycle.state.transition_count,
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
        .ok_or_else(|| "direct-zone IdealLoads CP402 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP402 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP402 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP402 CP401 latest evidence is missing".to_string()
    })?;
    let owner_latest = owner
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP402 CP321 latest owner is missing".to_string())?;
    let corroborator_latest = corroborator.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP402 CP340 latest corroborator is missing".to_string()
    })?;
    if [lifecycle.state.system, predecessor.state.system, owner.state.system, corroborator.state.system]
        .into_iter()
        .any(|actual| actual != system)
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !links_to_predecessor(latest, predecessor_latest)
        || !operation_shape_is_exact(latest, predecessor_latest, owner_latest, corroborator_latest)
        || !carriers_are_preserved(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP402 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState,
) -> Result<(), String> {
    if state.predecessor_route_counts != predecessor.predecessor_route_counts {
        return Err("direct-zone IdealLoads CP402 route lineage is invalid".to_string());
    }
    for (index, count) in state.predecessor_route_counts.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP402 non-direct route {index} is active"
            ));
        }
    }
    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let evaluations = checked_selected_sum(
        &state.predecessor_route_counts,
        &[20, 21, 24, 25, 27, 29],
        "active route partition",
    )?;
    let guard_false = checked_sum(
        &state.guard_false_fallthrough_route_counts,
        "guard-false partition",
    )?;
    let body = checked_sum(
        &state.adjustment_body_entry_route_counts,
        "body-entry partition",
    )?;
    for index in 0..30 {
        let successor = state.guard_false_fallthrough_route_counts[index]
            .checked_add(state.adjustment_body_entry_route_counts[index])
            .ok_or_else(|| {
                "direct-zone IdealLoads CP402 successor partition overflowed".to_string()
            })?;
        let expected = if matches!(index, 20 | 21 | 24 | 25 | 27 | 29) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(successor, expected, "successor_route_partition")?;
    }
    let inactive = state
        .transition_count
        .checked_sub(evaluations)
        .ok_or_else(|| "direct-zone IdealLoads CP402 inactive partition underflowed".to_string())?;
    let sites = evaluations
        .checked_mul(3)
        .and_then(|sites| sites.checked_add(body))
        .ok_or_else(|| "direct-zone IdealLoads CP402 site count overflowed".to_string())?;
    let humidity = selected(&state.predecessor_route_counts, &[18, 19, 22, 23, 26, 28])?;
    let enthalpy = selected(
        &state.predecessor_route_counts,
        &[
            5, 8, 11, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        ],
    )?;
    let temperature = selected(
        &state.predecessor_route_counts,
        &[
            3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            26, 27, 28, 29,
        ],
    )?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("active_route_partition", evaluations, state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count),
        ("predecessor_assignment_count", predecessor.dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count, evaluations),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("guard_outcome_partition", evaluations, guard_false.checked_add(body).ok_or_else(|| "direct-zone IdealLoads CP402 guard partition overflowed".to_string())?),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("cp401_supply_humidity_ratio_state_owner_count", humidity, state.cp401_supply_humidity_ratio_state_owner_count),
        ("unchanged_supply_humidity_ratio_preservation_count", humidity, state.unchanged_supply_humidity_ratio_preservation_count),
        ("cp401_supply_enthalpy_state_owner_count", enthalpy, state.cp401_supply_enthalpy_state_owner_count),
        ("unchanged_supply_enthalpy_preservation_count", enthalpy, state.unchanged_supply_enthalpy_preservation_count),
        ("cp401_supply_temperature_state_owner_count", temperature, state.cp401_supply_temperature_state_owner_count),
        ("unchanged_supply_temperature_preservation_count", temperature, state.unchanged_supply_temperature_preservation_count),
        ("cp401_cooling_latent_output_owned_read_count", evaluations, state.cp401_cooling_latent_output_owned_read_count),
        ("cooling_latent_output_read_count", evaluations, state.cooling_latent_output_read_count),
        ("cp321_maximum_total_cooling_capacity_owned_read_count", evaluations, state.cp321_maximum_total_cooling_capacity_owned_read_count),
        ("cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count", evaluations, state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count),
        ("maximum_total_cooling_capacity_read_count", evaluations, state.maximum_total_cooling_capacity_read_count),
        ("cooling_latent_output_maximum_total_cooling_capacity_comparison_count", evaluations, state.cooling_latent_output_maximum_total_cooling_capacity_comparison_count),
        ("greater_than_or_equal_count", body, state.cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count),
        ("body_entry_count", body, state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count),
        ("guard_false_count", guard_false, state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn selected(values: &[usize; 30], indices: &[usize]) -> Result<usize, String> {
    checked_selected_sum(values, indices, "carrier partition")
}

fn checked_selected_sum(
    values: &[usize; 30],
    indices: &[usize],
    label: &str,
) -> Result<usize, String> {
    indices.iter().try_fold(0usize, |sum, index| {
        sum.checked_add(values[*index])
            .ok_or_else(|| format!("direct-zone IdealLoads CP402 {label} overflowed"))
    })
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP402 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP402 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
