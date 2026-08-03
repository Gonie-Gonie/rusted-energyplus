//! Fail-closed validation for CP407 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary as EnthalpyOwnerLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary as HumidityOwnerLifecycle,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

#[allow(clippy::too_many_arguments)]
pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp406: Option<&PredecessorLifecycle>,
    enthalpy_owner_cp385: Option<&EnthalpyOwnerLifecycle>,
    humidity_owner_cp378: Option<&HumidityOwnerLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP407 evidence is missing".to_string())?;
    let predecessor = predecessor_cp406
        .ok_or_else(|| "direct-zone IdealLoads CP407 CP406 evidence is missing".to_string())?;
    let enthalpy_owner = enthalpy_owner_cp385
        .ok_or_else(|| "direct-zone IdealLoads CP407 CP385 owner is missing".to_string())?;
    let humidity_owner = humidity_owner_cp378
        .ok_or_else(|| "direct-zone IdealLoads CP407 CP378 owner is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP407 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP407 coupling call count is missing".to_string())?;

    validate_provenance(lifecycle, predecessor, enthalpy_owner, humidity_owner)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    validate_source_counters(&lifecycle.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    ensure_count(
        lifecycle.state.transition_count,
        predecessor.state.transition_count,
        "predecessor_transition_count",
    )?;

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP407 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP407 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP407 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP407 CP406 latest is missing".to_string())?;
    let enthalpy_latest = enthalpy_owner
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP407 CP385 latest is missing".to_string())?;
    let humidity_latest = humidity_owner
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP407 CP378 latest is missing".to_string())?;
    if [
        lifecycle.state.system,
        predecessor.state.system,
        enthalpy_owner.state.system,
        humidity_owner.state.system,
    ]
    .into_iter()
    .any(|actual| actual != system)
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        || predecessor_latest.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || predecessor_latest.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || predecessor_latest.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER
        || !owner_metadata_matches(enthalpy_latest, system, zone, calls)
        || !humidity_owner_metadata_matches(humidity_latest, system, zone, calls)
        || !lineage_is_exact(
            latest,
            predecessor_latest,
            enthalpy_latest,
            humidity_latest,
        )
    {
        return Err("direct-zone IdealLoads CP407 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    enthalpy_owner: &EnthalpyOwnerLifecycle,
    humidity_owner: &HumidityOwnerLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE
        || enthalpy_owner.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || enthalpy_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || humidity_owner.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        || humidity_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len()
            != 4
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER.len()
            != 1
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER.is_empty()
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER.is_empty()
    {
        return Err("direct-zone IdealLoads CP407 provenance is invalid".to_string());
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
        || state.predecessor_else_branch_entry_route_counts
            != predecessor.else_branch_entry_route_counts
        || state.supply_temperature_assignment_route_counts
            != predecessor.else_branch_entry_route_counts
    {
        return Err("direct-zone IdealLoads CP407 route lineage is invalid".to_string());
    }
    for (index, count) in state.predecessor_route_counts.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP407 non-direct route {index} is active"
            ));
        }
    }

    let transitions = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let guard_false = checked_sum(
        &state.predecessor_guard_false_fallthrough_route_counts,
        "guard-false partition",
    )?;
    let maximum_assignments = checked_sum(
        &state.predecessor_maximum_capacity_assignment_route_counts,
        "maximum-assignment partition",
    )?;
    let entries = checked_sum(
        &state.predecessor_else_branch_entry_route_counts,
        "else-entry partition",
    )?;
    let assignments = checked_sum(
        &state.supply_temperature_assignment_route_counts,
        "assignment partition",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP407 inactive partition underflowed".to_string())?;
    let temperature_owners = checked_sum(
        &state.predecessor_route_counts[3..],
        "temperature-owner partition",
    )?;
    let enthalpy_preservations = checked_selected(
        &state.predecessor_route_counts,
        |index| matches!(index, 5 | 8 | 11 | 14 | 17..=29),
        "enthalpy-preservation partition",
    )?;
    let prior_humidity = checked_selected(
        &state.predecessor_route_counts,
        |index| matches!(index, 18 | 19 | 22 | 23 | 26 | 28),
        "humidity-preservation partition",
    )?;
    let humidity_preservations = assignments
        .checked_add(maximum_assignments)
        .and_then(|value| value.checked_add(prior_humidity))
        .ok_or_else(|| {
            "direct-zone IdealLoads CP407 humidity-preservation partition overflowed".to_string()
        })?;
    let unchanged_temperatures = temperature_owners.checked_sub(assignments).ok_or_else(|| {
        "direct-zone IdealLoads CP407 unchanged-temperature partition underflowed".to_string()
    })?;

    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        (
            "predecessor_inactive_transition_count",
            predecessor.inactive_transition_count,
            inactive,
        ),
        (
            "predecessor_guard_false_fallthrough_count",
            guard_false,
            state.predecessor_guard_false_fallthrough_count,
        ),
        (
            "cp406_guard_false_fallthrough_count",
            predecessor.predecessor_guard_false_fallthrough_count,
            guard_false,
        ),
        (
            "predecessor_maximum_capacity_assignment_count",
            maximum_assignments,
            state.predecessor_maximum_capacity_assignment_count,
        ),
        (
            "cp406_maximum_capacity_assignment_count",
            predecessor.predecessor_maximum_capacity_assignment_count,
            maximum_assignments,
        ),
        (
            "predecessor_else_branch_entry_count",
            entries,
            state.predecessor_else_branch_entry_count,
        ),
        (
            "cp406_else_branch_entry_count",
            predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count,
            entries,
        ),
        (
            "supply_temperature_assignment_count",
            assignments,
            state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count,
        ),
        ("else_entry_assignment_parity", entries, assignments),
        (
            "cp406_preexisting_supply_temperature_state_owner_count",
            temperature_owners,
            state.cp406_preexisting_supply_temperature_state_owner_count,
        ),
        (
            "unchanged_supply_humidity_ratio_preservation_count",
            humidity_preservations,
            state.unchanged_supply_humidity_ratio_preservation_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            enthalpy_preservations,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "unchanged_supply_temperature_preservation_count",
            unchanged_temperatures,
            state.unchanged_supply_temperature_preservation_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_source_counters(state: &State) -> Result<(), String> {
    let assignments = state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count;
    let sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| "direct-zone IdealLoads CP407 source-site count overflowed".to_string())?;
    ensure_count(
        state.source_site_execution_count,
        sites,
        "source_site_execution_count",
    )?;
    for (field, actual) in [
        (
            "cp385_retained_supply_enthalpy_owned_read_count",
            state.cp385_retained_supply_enthalpy_owned_read_count,
        ),
        (
            "cp406_same_call_supply_enthalpy_bit_corroboration_count",
            state.cp406_same_call_supply_enthalpy_bit_corroboration_count,
        ),
        (
            "supply_enthalpy_for_dry_bulb_inversion_read_count",
            state.supply_enthalpy_for_dry_bulb_inversion_read_count,
        ),
        (
            "cp378_retained_supply_humidity_ratio_owned_read_count",
            state.cp378_retained_supply_humidity_ratio_owned_read_count,
        ),
        (
            "supply_humidity_ratio_for_dry_bulb_inversion_read_count",
            state.supply_humidity_ratio_for_dry_bulb_inversion_read_count,
        ),
        (
            "psychrometric_supply_temperature_evaluation_count",
            state.psychrometric_supply_temperature_evaluation_count,
        ),
        (
            "supply_temperature_assignment_write_count",
            state.supply_temperature_assignment_write_count,
        ),
    ] {
        ensure_count(actual, assignments, field)?;
    }
    Ok(())
}

fn owner_metadata_matches(
    snapshot: ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot,
    system: ep_model::IdealLoadsAirSystemId,
    zone: ep_model::ZoneId,
    calls: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == system
        && snapshot.controlled_zone == zone
        && snapshot.parent_call_ordinal == calls
}

fn humidity_owner_metadata_matches(
    snapshot: ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    system: ep_model::IdealLoadsAirSystemId,
    zone: ep_model::ZoneId,
    calls: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == system
        && snapshot.controlled_zone == zone
        && snapshot.parent_call_ordinal == calls
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP407 {label} overflowed"))
    })
}

fn checked_selected(
    values: &[usize],
    selected: impl Fn(usize) -> bool,
    label: &str,
) -> Result<usize, String> {
    values
        .iter()
        .enumerate()
        .filter(|(index, _)| selected(*index))
        .try_fold(0usize, |sum, (_, value)| {
            sum.checked_add(*value)
                .ok_or_else(|| format!("direct-zone IdealLoads CP407 {label} overflowed"))
        })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP407 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
