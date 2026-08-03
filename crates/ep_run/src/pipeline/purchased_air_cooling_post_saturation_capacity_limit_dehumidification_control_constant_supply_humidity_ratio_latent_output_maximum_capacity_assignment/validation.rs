//! Fail-closed validation for CP405 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentRuntimeState as PredecessorState,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::{links_to_predecessor, operation_shape_is_exact};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp404: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP405 evidence is missing".to_string())?;
    let predecessor = predecessor_cp404
        .ok_or_else(|| "direct-zone IdealLoads CP405 CP404 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP405 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP405 coupling call count is missing".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER.len()
            != 2
    {
        return Err("direct-zone IdealLoads CP405 provenance is invalid".to_string());
    }

    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    for (field, expected, actual) in [
        ("transition_count", calls, lifecycle.state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.state.transition_count,
            lifecycle.state.transition_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP405 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP405 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP405 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP405 CP404 latest evidence is missing".to_string()
    })?;
    if [lifecycle.state.system, predecessor.state.system]
        .into_iter()
        .any(|actual| actual != system)
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !links_to_predecessor(latest, predecessor_latest)
        || !operation_shape_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP405 latest lineage is invalid".to_string());
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
        || state.cooling_latent_output_maximum_capacity_assignment_route_counts
            != predecessor.supply_humidity_ratio_assignment_route_counts
    {
        return Err("direct-zone IdealLoads CP405 route lineage is invalid".to_string());
    }
    for (index, count) in state.predecessor_route_counts.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP405 non-direct route {index} is active"
            ));
        }
        let successors = state.predecessor_guard_false_fallthrough_route_counts[index]
            .checked_add(
                state.cooling_latent_output_maximum_capacity_assignment_route_counts[index],
            )
            .ok_or_else(|| {
                "direct-zone IdealLoads CP405 successor partition overflowed".to_string()
            })?;
        let expected = if predecessor_index_is_active(index) {
            *count
        } else {
            0
        };
        ensure_count(successors, expected, "successor_route_partition")?;
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let inherited_inactive = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 22, 23, 26, 28,
        ],
        "inherited inactive partition",
    )?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard-false partition",
    )?;
    let assignments = checked_sum(
        &state.cooling_latent_output_maximum_capacity_assignment_route_counts,
        "assignment partition",
    )?;
    let sites = assignments
        .checked_mul(2)
        .ok_or_else(|| "direct-zone IdealLoads CP405 site count overflowed".to_string())?;
    let predecessor_humidity_owners = checked_selected_sum(
        &state.predecessor_route_counts,
        &[18, 19, 22, 23, 26, 28],
        "humidity-ratio owner partition",
    )?;
    let humidity_owners = predecessor_humidity_owners
        .checked_add(assignments)
        .ok_or_else(|| {
            "direct-zone IdealLoads CP405 humidity-ratio owner count overflowed".to_string()
        })?;
    let enthalpy_owners = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            5, 8, 11, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        ],
        "enthalpy owner partition",
    )?;
    let temperature_owners = checked_selected_sum(
        &state.predecessor_route_counts,
        &[
            3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            26, 27, 28, 29,
        ],
        "temperature owner partition",
    )?;

    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "inactive_transition_count",
            inherited_inactive,
            state.inactive_transition_count,
        ),
        (
            "predecessor_inactive_transition_count",
            predecessor.inactive_transition_count,
            state.inactive_transition_count,
        ),
        (
            "predecessor_guard_false_fallthrough_count",
            guard_false,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            "predecessor_guard_false_count_parity",
            predecessor.predecessor_guard_false_fallthrough_count,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            "cooling_latent_output_maximum_capacity_assignment_count",
            assignments,
            state.cooling_latent_output_maximum_capacity_assignment_count,
        ),
        (
            "cp404_supply_humidity_ratio_assignment_count",
            predecessor.supply_humidity_ratio_assignment_count,
            state.cooling_latent_output_maximum_capacity_assignment_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp404_supply_humidity_ratio_state_owner_count",
            humidity_owners,
            state.cp404_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp404_supply_enthalpy_state_owner_count",
            enthalpy_owners,
            state.cp404_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp404_supply_temperature_state_owner_count",
            temperature_owners,
            state.cp404_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_owners,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    for (field, actual) in [
        (
            "cp404_retained_maximum_total_cooling_capacity_owned_read_count",
            state.cp404_retained_maximum_total_cooling_capacity_owned_read_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_latent_output_assignment_write_count",
            state.cooling_latent_output_assignment_write_count,
        ),
    ] {
        ensure_count(actual, assignments, field)?;
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
            .ok_or_else(|| format!("direct-zone IdealLoads CP405 {label} overflowed"))
    })
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP405 {label} overflowed"))
    })
}

const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP405 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
