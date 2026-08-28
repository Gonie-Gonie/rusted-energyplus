//! Fail-closed validation for CP431 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE,
    PurchasedAirCalcHeatingModeGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingModeGuardRuntimeState as State,
    PurchasedAirCalcHeatingOrNoLoadCaseEntryLifecycleSummary as PredecessorLifecycle,
    PurchasedAirInitLifecycleSummary,
};

mod lineage;
use lineage::lineage_is_exact;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];
const ORDER: &[&str] = &[
    "read-minimum-outdoor-air-sensible-output",
    "read-heating-setpoint-demand",
    "compare-strict-less-than",
    "read-zone-temperature-control-type-after-short-circuit",
    "exclude-exact-single-cooling-control",
    "enter-heating-mode-body-if-admitted",
];

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp430: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP431 evidence is missing".to_string())?;
    let predecessor = predecessor_cp430
        .ok_or_else(|| "direct-zone IdealLoads CP431 CP430 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP431 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP431 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source != PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OR_NO_LOAD_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER != ORDER
    {
        return Err("direct-zone IdealLoads CP431 provenance is invalid".to_string());
    }
    validate_public_route_contract(&lifecycle.state, &predecessor.state)?;
    ensure_count(lifecycle.state.transition_count, calls, "transition_count")?;
    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP431 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP431 controlled Zone is missing".to_string())?;
    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP431 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP431 CP430 latest evidence is missing".to_string()
    })?;
    if lifecycle.state.system != system
        || predecessor.state.system != system
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || latest.source != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE
        || latest.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_FIRST_EXCLUDED_SOURCE
        || latest.source_order != PURCHASED_AIR_CALC_HEATING_MODE_GUARD_SOURCE_ORDER
        || !lineage_is_exact(latest, predecessor_latest)
    {
        return Err("direct-zone IdealLoads CP431 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn validate_public_route_contract(
    state: &State,
    predecessor: &ep_runtime::PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState,
) -> Result<(), String> {
    if state.transition_count != predecessor.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || state.heating_mode_guard_evaluation_route_counts
            != predecessor.heating_or_no_load_case_entry_route_counts
    {
        return Err("direct-zone IdealLoads CP431 route lineage is invalid".to_string());
    }
    for index in 0..36 {
        for values in [
            &state.predecessor_route_counts,
            &state.heating_mode_guard_evaluation_route_counts,
            &state.heating_operating_mode_body_entry_route_counts,
            &state.heating_mode_guard_false_fallthrough_route_counts,
        ] {
            if !PUBLIC.contains(&index) && values[index] != 0 {
                return Err(format!(
                    "direct-zone IdealLoads CP431 non-direct route {index} is active"
                ));
            }
        }
        let evaluated = state.heating_mode_guard_evaluation_route_counts[index];
        let terminal = state.heating_operating_mode_body_entry_route_counts[index]
            .checked_add(state.heating_mode_guard_false_fallthrough_route_counts[index])
            .ok_or_else(|| {
                "direct-zone IdealLoads CP431 active partition overflowed".to_string()
            })?;
        ensure_count(
            terminal,
            if index == 1 { evaluated } else { 0 },
            "active_route_partition",
        )?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts)?;
    let evaluations = checked_sum(&state.heating_mode_guard_evaluation_route_counts)?;
    let bodies = checked_sum(&state.heating_operating_mode_body_entry_route_counts)?;
    let fallthroughs = checked_sum(&state.heating_mode_guard_false_fallthrough_route_counts)?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| "direct-zone IdealLoads CP431 inactive partition underflowed".to_string())?;
    let source_sites = evaluations
        .checked_add(bodies)
        .and_then(|count| count.checked_mul(3))
        .ok_or_else(|| "direct-zone IdealLoads CP431 source-site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("guard_evaluation_count", evaluations, state.heating_mode_guard_evaluation_count),
        ("body_entry_count", bodies, state.heating_operating_mode_body_entry_count),
        ("false_fallthrough_count", fallthroughs, state.heating_mode_guard_false_fallthrough_count),
        ("source_site_execution_count", source_sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp429_supply_humidity_ratio_state_owner_count, state.cp430_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", state.cp430_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp429_supply_enthalpy_state_owner_count, state.cp430_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", state.cp430_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp429_supply_temperature_state_owner_count, state.cp430_supply_temperature_state_owner_count),
        ("temperature_preservation_count", state.cp430_supply_temperature_state_owner_count, state.unchanged_supply_temperature_preservation_count),
        ("cp311_owner_read_count", evaluations, state.cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count),
        ("cp312_corroboration_count", evaluations, state.cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count),
        ("minimum_oa_read_count", evaluations, state.minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count),
        ("cp310_owner_read_count", evaluations, state.cp310_retained_heating_setpoint_demand_owner_read_count),
        ("heating_demand_read_count", evaluations, state.heating_setpoint_demand_for_heating_mode_guard_read_count),
        ("sensible_comparison_count", evaluations, state.minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count),
        ("sensible_true_count", bodies, state.minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count),
        ("temperature_type_owner_read_count", bodies, state.prevalidated_temperature_control_type_owner_read_count),
        ("temperature_type_read_count", bodies, state.temperature_control_type_read_after_sensible_comparison_short_circuit_count),
        ("single_cool_comparison_count", bodies, state.temperature_control_type_single_cool_comparison_count),
        ("permits_heating_count", bodies, state.temperature_control_type_permits_heating_count),
        ("single_cool_block_count", 0, state.single_cool_block_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn checked_sum(values: &[usize]) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| "direct-zone IdealLoads CP431 count overflowed".to_string())
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP431 invariant {field} expected {expected}, got {actual}"
        ))
    }
}
