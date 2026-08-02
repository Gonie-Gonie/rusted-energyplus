//! Fail-closed validation for CP395 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::{carriers_are_preserved, direct_skip_shape, links_to_predecessor};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp394: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP395 evidence is missing".to_string())?;
    let predecessor = predecessor_cp394
        .ok_or_else(|| "direct-zone IdealLoads CP395 CP394 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP395 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP395 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_SOURCE
        || predecessor.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
    {
        return Err("direct-zone IdealLoads CP395 provenance is invalid".to_string());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let assignments =
        state.dehumidification_control_humidistat_supply_humidity_ratio_assignment_count;
    let inactive = state
        .transition_count
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP395 inactive partition underflowed".to_string())?;
    let route_sum = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let active = checked_sum(
        &[
            state.predecessor_route_counts[19],
            state.predecessor_route_counts[23],
            state.predecessor_route_counts[26],
        ],
        "active route partition",
    )?;
    let humidity_ratio_owners = checked_sum(
        &[
            state.predecessor_route_counts[18],
            state.predecessor_route_counts[22],
            state.predecessor_route_counts[28],
        ],
        "CP394 humidity-ratio owner partition",
    )?;
    let temperature_owners = checked_sum(
        &state.predecessor_route_counts[3..],
        "CP394 temperature owner partition",
    )?;
    let enthalpy_owners = checked_sum(
        &[
            state.predecessor_route_counts[5],
            state.predecessor_route_counts[8],
            state.predecessor_route_counts[11],
            state.predecessor_route_counts[14],
            state.predecessor_route_counts[17],
            checked_sum(
                &state.predecessor_route_counts[18..],
                "CP394 enthalpy owner tail",
            )?,
        ],
        "CP394 enthalpy owner partition",
    )?;
    let sites = assignments.checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER.len())
        .ok_or_else(|| "direct-zone IdealLoads CP395 site count overflowed".to_string())?;
    validate_all_public_inactive_contract(state)?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        ("route_partition", state.transition_count, route_sum),
        ("active_route_partition", active, assignments),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "predecessor_humidistat_case_entry_count",
            predecessor_state.dehumidification_control_humidistat_case_entry_count,
            assignments,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp394_supply_humidity_ratio_state_owner_count",
            humidity_ratio_owners,
            state.cp394_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_ratio_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp394_supply_temperature_state_owner_count",
            temperature_owners,
            state.cp394_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_owners,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp394_supply_enthalpy_state_owner_count",
            enthalpy_owners,
            state.cp394_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "supply_temperature_owned_read_count",
            assignments,
            state.supply_temperature_owned_read_count,
        ),
        (
            "supply_temperature_for_humidity_ratio_inversion_read_count",
            assignments,
            state.supply_temperature_for_humidity_ratio_inversion_read_count,
        ),
        (
            "supply_enthalpy_owned_read_count",
            assignments,
            state.supply_enthalpy_owned_read_count,
        ),
        (
            "supply_enthalpy_for_humidity_ratio_inversion_read_count",
            assignments,
            state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
        ),
        (
            "psychrometric_supply_humidity_ratio_evaluation_count",
            assignments,
            state.psychrometric_supply_humidity_ratio_evaluation_count,
        ),
        (
            "supply_humidity_ratio_assignment_write_count",
            assignments,
            state.supply_humidity_ratio_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    if state.predecessor_route_counts != predecessor_state.predecessor_route_counts {
        return Err("direct-zone IdealLoads CP395 route lineage is invalid".to_string());
    }

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP395 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP395 controlled Zone is missing".to_string())?;
    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP395 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP395 CP394 latest evidence is missing".to_string()
    })?;
    if state.system != system
        || predecessor_state.system != system
        || latest.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE
        || latest.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !links_to_predecessor(latest, predecessor_latest)
        || !direct_skip_shape(latest)
        || !carriers_are_preserved(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP395 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_all_public_inactive_contract(state: &State) -> Result<(), String> {
    for (field, actual) in [
        (
            "direct_assignment_count",
            state.dehumidification_control_humidistat_supply_humidity_ratio_assignment_count,
        ),
        (
            "source_site_execution_count",
            state.source_site_execution_count,
        ),
        (
            "supply_temperature_owned_read_count",
            state.supply_temperature_owned_read_count,
        ),
        (
            "supply_temperature_for_humidity_ratio_inversion_read_count",
            state.supply_temperature_for_humidity_ratio_inversion_read_count,
        ),
        (
            "supply_enthalpy_owned_read_count",
            state.supply_enthalpy_owned_read_count,
        ),
        (
            "supply_enthalpy_for_humidity_ratio_inversion_read_count",
            state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
        ),
        (
            "psychrometric_supply_humidity_ratio_evaluation_count",
            state.psychrometric_supply_humidity_ratio_evaluation_count,
        ),
        (
            "supply_humidity_ratio_assignment_write_count",
            state.supply_humidity_ratio_assignment_write_count,
        ),
    ] {
        ensure_count(actual, 0, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP395 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP395 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
