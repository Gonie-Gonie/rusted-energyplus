//! Fail-closed validation for CP412 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const FIRST_ACTIVE_PREDECESSOR_INDEX: usize = 18;
const EXPECTED_SOURCE_ORDER: [&str; 4] = [
    "read-purchased-air-supply-temperature-for-saturation-humidity-ratio",
    "read-environment-outdoor-barometric-pressure-for-saturation-humidity-ratio",
    "evaluate-psy-w-fn-tdb-rh-pb-at-unity-relative-humidity",
    "assign-local-saturation-supply-humidity-ratio",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp411: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP412 evidence is missing".to_string())?;
    let predecessor = predecessor_cp411
        .ok_or_else(|| "direct-zone IdealLoads CP412 CP411 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP412 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP412 coupling call count is missing".to_string())?;
    if calls == 0 {
        return Err("direct-zone IdealLoads CP412 coupling call count is invalid".to_string());
    }

    validate_provenance(lifecycle, predecessor)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP412 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP412 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP412 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP412 CP411 latest is missing".to_string())?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
        || predecessor_latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        || predecessor_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP412 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER.is_empty()
    {
        return Err("direct-zone IdealLoads CP412 provenance is invalid".to_string());
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
        || state.predecessor_maximum_capacity_assignment_route_counts
            != predecessor.predecessor_maximum_capacity_assignment_route_counts
        || state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts
            != predecessor.supply_humidity_ratio_pre_saturation_original_assignment_route_counts
    {
        return Err("direct-zone IdealLoads CP412 route lineage is invalid".to_string());
    }
    for values in [
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_maximum_capacity_assignment_route_counts,
        &state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts,
        &state.supply_humidity_ratio_saturation_assignment_route_counts,
    ] {
        ensure_public_routes_only(values)?;
    }
    validate_route_evidence(
        &state.predecessor_route_counts,
        &state.predecessor_guard_false_fallthrough_route_counts,
        &state.predecessor_maximum_capacity_assignment_route_counts,
        &state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts,
        &state.supply_humidity_ratio_saturation_assignment_route_counts,
    )?;

    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard-false partition",
    )?;
    let maximum_assignments = checked_sum(
        &state.predecessor_maximum_capacity_assignment_route_counts,
        "maximum-assignment partition",
    )?;
    let predecessor_assignments = checked_sum(
        &state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts,
        "predecessor assignment partition",
    )?;
    let assignments = checked_sum(
        &state.supply_humidity_ratio_saturation_assignment_route_counts,
        "assignment partition",
    )?;
    let active_routes = checked_sum(
        &state.predecessor_route_counts[FIRST_ACTIVE_PREDECESSOR_INDEX..],
        "active route partition",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP412 inactive partition underflowed".to_string())?;
    let humidity_ratio_owners = active_routes;
    let enthalpy_owners = checked_sum(
        &[
            state.predecessor_route_counts[5],
            state.predecessor_route_counts[8],
            state.predecessor_route_counts[11],
            state.predecessor_route_counts[14],
            state.predecessor_route_counts[17],
            active_routes,
        ],
        "enthalpy owner partition",
    )?;
    let temperature_owners = checked_sum(
        &state.predecessor_route_counts[3..],
        "temperature owner partition",
    )?;
    let sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| "direct-zone IdealLoads CP412 site count overflowed".to_string())?;

    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("assignment_route_partition", active_routes, assignments),
        (
            "predecessor_assignment_route_partition",
            active_routes,
            predecessor_assignments,
        ),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "predecessor_guard_false_fallthrough_count",
            guard_false,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            "cp411_guard_false_fallthrough_count",
            predecessor.predecessor_guard_false_fallthrough_count,
            guard_false,
        ),
        (
            "predecessor_maximum_capacity_assignment_count",
            maximum_assignments,
            state.predecessor_maximum_capacity_assignment_count,
        ),
        (
            "cp411_maximum_capacity_assignment_count",
            predecessor.predecessor_maximum_capacity_assignment_count,
            maximum_assignments,
        ),
        (
            "predecessor_original_assignment_count",
            predecessor_assignments,
            state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count,
        ),
        (
            "cp411_original_assignment_count",
            predecessor.supply_humidity_ratio_pre_saturation_original_assignment_count,
            predecessor_assignments,
        ),
        (
            "saturation_assignment_count",
            assignments,
            state.supply_humidity_ratio_saturation_assignment_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp411_supply_humidity_ratio_state_owner_count",
            humidity_ratio_owners,
            state.cp411_supply_humidity_ratio_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_ratio_owners,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "cp411_supply_enthalpy_state_owner_count",
            enthalpy_owners,
            state.cp411_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_owners,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "cp411_supply_temperature_state_owner_count",
            temperature_owners,
            state.cp411_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            temperature_owners,
            state.unchanged_supply_temperature_preservation_count,
        ),
        (
            "cp411_retained_supply_temperature_owned_read_count",
            assignments,
            state.cp411_retained_supply_temperature_owned_read_count,
        ),
        (
            "purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count",
            assignments,
            state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count,
        ),
        (
            "environment_outdoor_barometric_pressure_owner_count",
            assignments,
            state.environment_outdoor_barometric_pressure_owner_count,
        ),
        (
            "environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count",
            assignments,
            state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count,
        ),
        (
            "psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count",
            assignments,
            state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count,
        ),
        (
            "local_saturation_supply_humidity_ratio_assignment_write_count",
            assignments,
            state.local_saturation_supply_humidity_ratio_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_evidence(
    routes: &[usize; 30],
    guard_false_routes: &[usize; 30],
    maximum_assignment_routes: &[usize; 30],
    predecessor_assignment_routes: &[usize; 30],
    assignment_routes: &[usize; 30],
) -> Result<(), String> {
    for index in 0..routes.len() {
        let branch_count = guard_false_routes[index]
            .checked_add(maximum_assignment_routes[index])
            .ok_or_else(|| {
                format!("direct-zone IdealLoads CP412 route {index} branch count overflowed")
            })?;
        let expected_branch = if SPLIT_PREDECESSOR_INDICES.contains(&index) {
            routes[index]
        } else {
            0
        };
        ensure_count(
            branch_count,
            expected_branch,
            "predecessor_split_route_evidence_partition",
        )?;
        let expected_assignment = if index >= FIRST_ACTIVE_PREDECESSOR_INDEX {
            routes[index]
        } else {
            0
        };
        ensure_count(
            predecessor_assignment_routes[index],
            expected_assignment,
            "predecessor_assignment_route_evidence_partition",
        )?;
        ensure_count(
            assignment_routes[index],
            expected_assignment,
            "assignment_route_evidence_partition",
        )?;
    }
    Ok(())
}

fn ensure_public_routes_only(values: &[usize; 30]) -> Result<(), String> {
    for (index, count) in values.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP412 non-direct route {index} is active"
            ));
        }
    }
    Ok(())
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP412 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP412 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
