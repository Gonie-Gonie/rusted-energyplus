//! Fail-closed validation for CP408 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary as MixedAirLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState as State,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp407: Option<&PredecessorLifecycle>,
    mixed_air_owner_cp329: Option<&MixedAirLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP408 evidence is missing".to_string())?;
    let predecessor = predecessor_cp407
        .ok_or_else(|| "direct-zone IdealLoads CP408 CP407 evidence is missing".to_string())?;
    let mixed_air = mixed_air_owner_cp329
        .ok_or_else(|| "direct-zone IdealLoads CP408 CP329 owner is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP408 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP408 coupling call count is missing".to_string())?;

    validate_provenance(lifecycle, predecessor, mixed_air)?;
    validate_public_route_contract(&lifecycle.state, &predecessor.state, mixed_air)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP408 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP408 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP408 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP408 CP407 latest is missing".to_string())?;
    let active = predecessor_latest
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed;
    let mixed_air_latest = active.then_some(mixed_air.state.latest).flatten();

    if [
        lifecycle.state.system,
        predecessor.state.system,
        mixed_air.state.system,
    ]
    .into_iter()
    .any(|actual| actual != system)
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !latest_metadata_is_exact(latest, predecessor_latest, mixed_air_latest)
        || !lineage_is_exact(latest, predecessor_latest, mixed_air_latest)
    {
        return Err("direct-zone IdealLoads CP408 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_provenance(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    mixed_air: &MixedAirLifecycle,
) -> Result<(), String> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || mixed_air.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || mixed_air.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.is_empty()
        || PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER.is_empty()
    {
        return Err("direct-zone IdealLoads CP408 provenance is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &PredecessorState,
    mixed_air: &MixedAirLifecycle,
) -> Result<(), String> {
    if state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.predecessor_guard_false_fallthrough_route_counts
            != predecessor.predecessor_guard_false_fallthrough_route_counts
        || state.predecessor_maximum_capacity_assignment_route_counts
            != predecessor.predecessor_maximum_capacity_assignment_route_counts
        || state.predecessor_else_branch_entry_route_counts
            != predecessor.predecessor_else_branch_entry_route_counts
        || state.predecessor_supply_temperature_assignment_route_counts
            != predecessor.supply_temperature_assignment_route_counts
        || state.supply_temperature_mixed_air_limit_route_counts
            != predecessor.supply_temperature_assignment_route_counts
    {
        return Err("direct-zone IdealLoads CP408 route lineage is invalid".to_string());
    }
    for (index, count) in state.predecessor_route_counts.iter().enumerate() {
        if !matches!(index, 0..=8 | 20 | 24) && *count != 0 {
            return Err(format!(
                "direct-zone IdealLoads CP408 non-direct route {index} is active"
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
    let predecessor_assignments = checked_sum(
        &state.predecessor_supply_temperature_assignment_route_counts,
        "predecessor-assignment partition",
    )?;
    let assignments = checked_sum(
        &state.supply_temperature_mixed_air_limit_route_counts,
        "mixed-air-limit partition",
    )?;
    let inactive = transitions
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP408 inactive partition underflowed".to_string())?;
    let temperature_owners = checked_selected(
        &state.predecessor_route_counts,
        |index| index >= 3,
        "temperature-owner partition",
    )?;
    let unchanged_temperatures = temperature_owners.checked_sub(assignments).ok_or_else(|| {
        "direct-zone IdealLoads CP408 unchanged-temperature partition underflowed".to_string()
    })?;
    let sites = assignments
        .checked_mul(EXPECTED_SOURCE_ORDER.len())
        .ok_or_else(|| "direct-zone IdealLoads CP408 source-site count overflowed".to_string())?;
    let partition = inactive.checked_add(assignments).ok_or_else(|| {
        "direct-zone IdealLoads CP408 transition partition overflowed".to_string()
    })?;

    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("transition_partition", state.transition_count, partition),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("predecessor_inactive_transition_count", predecessor.inactive_transition_count, state.inactive_transition_count),
        ("predecessor_transition_count", predecessor.transition_count, state.transition_count),
        ("mixed_air_owner_transition_count", mixed_air.state.transition_count, state.transition_count),
        ("predecessor_guard_false_fallthrough_count", guard_false, state.predecessor_guard_false_fallthrough_count),
        ("cp407_guard_false_fallthrough_count", predecessor.predecessor_guard_false_fallthrough_count, guard_false),
        ("predecessor_maximum_capacity_assignment_count", maximum_assignments, state.predecessor_maximum_capacity_assignment_count),
        ("cp407_maximum_capacity_assignment_count", predecessor.predecessor_maximum_capacity_assignment_count, maximum_assignments),
        ("predecessor_else_branch_entry_count", entries, state.predecessor_else_branch_entry_count),
        ("cp407_else_branch_entry_count", predecessor.predecessor_else_branch_entry_count, entries),
        ("predecessor_supply_temperature_assignment_count", predecessor_assignments, state.predecessor_supply_temperature_assignment_count),
        ("cp407_supply_temperature_assignment_count", predecessor.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count, predecessor_assignments),
        ("mixed_air_limit_count", assignments, state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count),
        ("predecessor_assignment_limit_parity", predecessor_assignments, assignments),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("cp407_supply_temperature_state_owner_count", temperature_owners, state.cp407_supply_temperature_state_owner_count),
        ("predecessor_temperature_owner_count", predecessor.cp406_preexisting_supply_temperature_state_owner_count, temperature_owners),
        ("unchanged_supply_humidity_ratio_preservation_count", predecessor.unchanged_supply_humidity_ratio_preservation_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("unchanged_supply_enthalpy_preservation_count", predecessor.unchanged_supply_enthalpy_preservation_count, state.unchanged_supply_enthalpy_preservation_count),
        ("unchanged_supply_temperature_preservation_count", unchanged_temperatures, state.unchanged_supply_temperature_preservation_count),
        ("cp407_retained_supply_temperature_owned_read_count", assignments, state.cp407_retained_supply_temperature_owned_read_count),
        ("supply_temperature_for_minimum_read_count", assignments, state.supply_temperature_for_minimum_read_count),
        ("cp329_retained_mixed_air_temperature_owned_read_count", assignments, state.cp329_retained_mixed_air_temperature_owned_read_count),
        ("mixed_air_temperature_for_minimum_read_count", assignments, state.mixed_air_temperature_for_minimum_read_count),
        ("source_shaped_two_argument_minimum_evaluation_count", assignments, state.source_shaped_two_argument_minimum_evaluation_count),
        ("supply_temperature_assignment_write_count", assignments, state.supply_temperature_assignment_write_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    if mixed_air.state.cooling_call_count < assignments {
        return Err(format!(
            "direct-zone IdealLoads CP408 invariant cp329_owner_coverage expected at least {assignments}, got {}",
            mixed_air.state.cooling_call_count
        ));
    }
    Ok(())
}

fn latest_metadata_is_exact(
    latest: ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot,
    predecessor: ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot,
    mixed_air: Option<ep_runtime::PurchasedAirCalcCoolingMixedAirCallSnapshot>,
) -> bool {
    latest.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        && latest.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        && latest.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        && mixed_air.is_none_or(|owner| {
            owner.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
                && owner.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
                && owner.first_excluded_source
                    == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
                && owner.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        })
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP408 {label} overflowed"))
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
                .ok_or_else(|| format!("direct-zone IdealLoads CP408 {label} overflowed"))
        })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP408 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
