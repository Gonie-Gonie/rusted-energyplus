//! Fail-closed validation for CP413 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const SPLIT_PREDECESSOR_INDICES: [usize; 6] = [20, 21, 24, 25, 27, 29];
const PUBLIC_LOGICAL_INDICES: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const FIRST_ACTIVE_LOGICAL_INDEX: usize = 18;
const EXPECTED_SOURCE_ORDER: [&str; 4] = [
    "read-local-saturation-supply-humidity-ratio-for-saturation-guard",
    "read-local-original-supply-humidity-ratio-for-saturation-guard",
    "compare-local-saturation-supply-humidity-ratio-strictly-less-than-local-original-supply-humidity-ratio",
    "enter-saturation-supply-humidity-ratio-guard-body-if-comparison-satisfied",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp412: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP413 evidence is missing".to_string())?;
    let predecessor = predecessor_cp412
        .ok_or_else(|| "direct-zone IdealLoads CP413 CP412 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP413 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP413 coupling call count is missing".to_string())?;
    if calls == 0 {
        return Err("direct-zone IdealLoads CP413 coupling call count is invalid".to_string());
    }

    validate_provenance(lifecycle, predecessor)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP413 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP413 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP413 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP413 CP412 latest is missing".to_string())?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE_ORDER
        || predecessor_latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || predecessor_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP413 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_GUARD_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER.is_empty()
    {
        return Err("direct-zone IdealLoads CP413 provenance is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
) -> Result<(), String> {
    let expected_routes = flatten_predecessor_routes(predecessor)?;
    if state.predecessor_route_counts != expected_routes {
        return Err("direct-zone IdealLoads CP413 route lineage is invalid".to_string());
    }
    for values in [
        &state.predecessor_route_counts,
        &state.guard_false_fallthrough_route_counts,
        &state.guard_body_entry_route_counts,
    ] {
        ensure_public_routes_only(values)?;
    }
    for index in 0..36 {
        let outcomes = state.guard_false_fallthrough_route_counts[index]
            .checked_add(state.guard_body_entry_route_counts[index])
            .ok_or_else(|| format!("direct-zone IdealLoads CP413 route {index} overflowed"))?;
        let expected = if index >= FIRST_ACTIVE_LOGICAL_INDEX {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(outcomes, expected, "guard_outcome_route_partition")?;
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let evaluations = checked_sum(
        &state.predecessor_route_counts[FIRST_ACTIVE_LOGICAL_INDEX..],
        "active route partition",
    )?;
    let guard_false = checked_sum(
        &state.guard_false_fallthrough_route_counts,
        "guard-false partition",
    )?;
    let body_entries = checked_sum(&state.guard_body_entry_route_counts, "body partition")?;
    let outcomes = guard_false
        .checked_add(body_entries)
        .ok_or_else(|| "direct-zone IdealLoads CP413 outcome partition overflowed".to_string())?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| "direct-zone IdealLoads CP413 inactive partition underflowed".to_string())?;
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
    let sites = evaluations
        .checked_mul(3)
        .and_then(|sites| sites.checked_add(body_entries))
        .ok_or_else(|| "direct-zone IdealLoads CP413 site count overflowed".to_string())?;

    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("predecessor_transition_count", predecessor.transition_count, state.transition_count),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("guard_evaluation_count", evaluations, state.saturation_supply_humidity_ratio_guard_evaluation_count),
        ("guard_outcome_partition", evaluations, outcomes),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("cp412_supply_humidity_ratio_state_owner_count", evaluations, state.cp412_supply_humidity_ratio_state_owner_count),
        ("unchanged_supply_humidity_ratio_preservation_count", evaluations, state.unchanged_supply_humidity_ratio_preservation_count),
        ("cp412_supply_enthalpy_state_owner_count", enthalpy_owners, state.cp412_supply_enthalpy_state_owner_count),
        ("unchanged_supply_enthalpy_preservation_count", enthalpy_owners, state.unchanged_supply_enthalpy_preservation_count),
        ("cp412_supply_temperature_state_owner_count", temperature_owners, state.cp412_supply_temperature_state_owner_count),
        ("unchanged_supply_temperature_preservation_count", temperature_owners, state.unchanged_supply_temperature_preservation_count),
        ("cp412_saturation_owned_read_count", evaluations, state.cp412_saturation_supply_humidity_ratio_owned_read_count),
        ("saturation_read_count", evaluations, state.saturation_supply_humidity_ratio_for_guard_read_count),
        ("cp411_original_owned_read_count", evaluations, state.cp411_original_supply_humidity_ratio_owned_read_count),
        ("same_call_original_corroboration_count", evaluations, state.cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count),
        ("original_read_count", evaluations, state.original_supply_humidity_ratio_for_guard_read_count),
        ("comparison_count", evaluations, state.saturation_original_supply_humidity_ratio_comparison_count),
        ("strictly_less_than_count", body_entries, state.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count),
        ("body_entry_count", body_entries, state.saturation_supply_humidity_ratio_guard_body_entry_count),
        ("false_fallthrough_count", guard_false, state.saturation_supply_humidity_ratio_guard_false_fallthrough_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn flatten_predecessor_routes(predecessor: &PredecessorState) -> Result<[usize; 36], String> {
    let mut flattened = [0usize; 36];
    let mut logical = 0usize;
    for index in 0..30 {
        if SPLIT_PREDECESSOR_INDICES.contains(&index) {
            let guard_false = predecessor.predecessor_guard_false_fallthrough_route_counts[index];
            let maximum = predecessor.predecessor_maximum_capacity_assignment_route_counts[index];
            let combined = guard_false.checked_add(maximum).ok_or_else(|| {
                "direct-zone IdealLoads CP413 predecessor split overflowed".to_string()
            })?;
            ensure_count(
                combined,
                predecessor.predecessor_route_counts[index],
                "predecessor_split_route_partition",
            )?;
            flattened[logical] = guard_false;
            flattened[logical + 1] = maximum;
            logical += 2;
        } else {
            flattened[logical] = predecessor.predecessor_route_counts[index];
            logical += 1;
        }
    }
    ensure_count(logical, flattened.len(), "flattened_route_width")?;
    Ok(flattened)
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
                .ok_or_else(|| format!("direct-zone IdealLoads CP413 {label} overflowed"))?;
        }
        logical += width;
    }
    Ok(total)
}

fn ensure_public_routes_only(values: &[usize; 36]) -> Result<(), String> {
    for (index, count) in values.iter().enumerate() {
        if !PUBLIC_LOGICAL_INDICES.contains(&index) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP413 non-direct route {index} is active"
            ));
        }
    }
    Ok(())
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP413 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP413 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
