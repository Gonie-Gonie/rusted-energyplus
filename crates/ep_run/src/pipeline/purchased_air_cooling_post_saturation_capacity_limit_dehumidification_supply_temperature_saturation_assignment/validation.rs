//! Fail-closed validation for CP414 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const FIRST_ACTIVE_LOGICAL_INDEX: usize = 18;
const EXPECTED_SOURCE_ORDER: [&str; 4] = [
    "read-cp413-retained-supply-enthalpy-for-saturation-temperature",
    "read-environment-outdoor-barometric-pressure-for-saturation-temperature",
    "evaluate-psy-tsat-fn-h-pb",
    "assign-purchased-air-supply-temperature-to-saturation-temperature",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp413: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP414 evidence is missing".to_string())?;
    let predecessor = predecessor_cp413
        .ok_or_else(|| "direct-zone IdealLoads CP414 CP413 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP414 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP414 coupling call count is missing".to_string())?;
    if calls == 0 {
        return Err("direct-zone IdealLoads CP414 coupling call count is invalid".to_string());
    }

    validate_provenance(lifecycle, predecessor)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP414 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP414 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP414 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP414 CP413 latest is missing".to_string())?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER
        || predecessor_latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE
        || predecessor_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP414 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE_ORDER.is_empty()
    {
        return Err("direct-zone IdealLoads CP414 provenance is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    if state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.guard_false_fallthrough_route_counts
        || state.predecessor_guard_body_entry_route_counts
            != predecessor.guard_body_entry_route_counts
    {
        return Err("direct-zone IdealLoads CP414 route lineage is invalid".to_string());
    }
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.supply_temperature_saturation_assignment_route_counts,
    ] {
        ensure_public_routes_only(values)?;
    }
    validate_route_evidence(
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_guard_body_entry_route_counts,
        &state.supply_temperature_saturation_assignment_route_counts,
    )?;

    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard-false partition",
    )?;
    let body_entries = checked_sum(
        &state.predecessor_guard_body_entry_route_counts,
        "guard-body partition",
    )?;
    let assignments = checked_sum(
        &state.supply_temperature_saturation_assignment_route_counts,
        "assignment partition",
    )?;
    let guard_outcomes = guard_false.checked_add(body_entries).ok_or_else(|| {
        "direct-zone IdealLoads CP414 guard outcome partition overflowed".to_string()
    })?;
    let active_outcomes = checked_sum(
        &state.predecessor_route_counts[FIRST_ACTIVE_LOGICAL_INDEX..],
        "active outcome partition",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP414 inactive partition underflowed".to_string())?;
    let humidity_ratio_owners = active_outcomes;
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
    let unchanged_temperature = temperature_owners.checked_sub(assignments).ok_or_else(|| {
        "direct-zone IdealLoads CP414 temperature preservation underflowed".to_string()
    })?;
    let sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| "direct-zone IdealLoads CP414 site count overflowed".to_string())?;

    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("guard_outcome_partition", active_outcomes, guard_outcomes),
        ("assignment_route_partition", body_entries, assignments),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "assignment_count",
            assignments,
            state.saturation_supply_temperature_assignment_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp413_supply_humidity_ratio_state_owner_count",
            humidity_ratio_owners,
            state.cp413_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_ratio_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp413_supply_enthalpy_state_owner_count",
            enthalpy_owners,
            state.cp413_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp413_supply_temperature_state_owner_count",
            temperature_owners,
            state.cp413_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            unchanged_temperature,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp414_saturation_supply_temperature_state_owner_count",
            assignments,
            state.cp414_saturation_supply_temperature_state_owner_count,
        ),
        (
            "cp413_retained_supply_enthalpy_owned_read_count",
            assignments,
            state.cp413_retained_supply_enthalpy_owned_read_count,
        ),
        (
            "supply_enthalpy_for_saturation_temperature_read_count",
            assignments,
            state.supply_enthalpy_for_saturation_temperature_read_count,
        ),
        (
            "environment_pressure_owner_count",
            assignments,
            state.environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count,
        ),
        (
            "environment_pressure_read_count",
            assignments,
            state.environment_outdoor_barometric_pressure_for_saturation_temperature_read_count,
        ),
        (
            "psy_tsat_fn_h_pb_evaluation_count",
            assignments,
            state.psy_tsat_fn_h_pb_evaluation_count,
        ),
        (
            "supply_temperature_assignment_write_count",
            assignments,
            state.purchased_air_supply_temperature_saturation_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_evidence(
    routes: &[usize; 36],
    guard_false_routes: &[usize; 36],
    guard_body_routes: &[usize; 36],
    assignment_routes: &[usize; 36],
) -> Result<(), String> {
    for index in 0..routes.len() {
        let outcomes = guard_false_routes[index]
            .checked_add(guard_body_routes[index])
            .ok_or_else(|| {
                format!("direct-zone IdealLoads CP414 route {index} outcome overflowed")
            })?;
        let expected_outcomes = if index >= FIRST_ACTIVE_LOGICAL_INDEX {
            routes[index]
        } else {
            0
        };
        ensure_count(
            outcomes,
            expected_outcomes,
            "predecessor_guard_outcome_partition",
        )?;
        ensure_count(
            assignment_routes[index],
            guard_body_routes[index],
            "assignment_body_route_partition",
        )?;
    }
    Ok(())
}

fn sum_predecessor_indices(
    values: &[usize; 36],
    include: impl Fn(usize) -> bool,
    label: &str,
) -> Result<usize, String> {
    let mut logical = 0usize;
    let mut total = 0usize;
    for predecessor_index in 0..30 {
        let width = 1 + usize::from(SPLIT_PREDECESSOR_INDICES.contains(&predecessor_index));
        if include(predecessor_index) {
            total = values[logical..logical + width]
                .iter()
                .try_fold(total, |sum, value| sum.checked_add(*value))
                .ok_or_else(|| format!("direct-zone IdealLoads CP414 {label} overflowed"))?;
        }
        logical += width;
    }
    Ok(total)
}

fn ensure_public_routes_only(values: &[usize; 36]) -> Result<(), String> {
    for (index, count) in values.iter().enumerate() {
        if !PUBLIC_LOGICAL_INDICES.contains(&index) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP414 non-direct route {index} is active"
            ));
        }
    }
    Ok(())
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP414 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP414 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
