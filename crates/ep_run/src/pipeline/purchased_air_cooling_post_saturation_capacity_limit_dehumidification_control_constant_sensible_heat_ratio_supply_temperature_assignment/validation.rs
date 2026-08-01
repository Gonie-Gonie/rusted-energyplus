//! Fail-closed validation for CP389 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary as TemperatureOwnerLifecycle,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp388: Option<&PredecessorLifecycle>,
    temperature_owner_cp379: Option<&TemperatureOwnerLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle =
        lifecycle.ok_or_else(|| "direct-zone IdealLoads CP389 evidence is missing".to_string())?;
    let predecessor = predecessor_cp388
        .ok_or_else(|| "direct-zone IdealLoads CP389 CP388 evidence is missing".to_string())?;
    let temperature_owner = temperature_owner_cp379
        .ok_or_else(|| "direct-zone IdealLoads CP389 CP379 evidence is missing".to_string())?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP389 initialization evidence is missing".to_string()
    })?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP389 coupling call count is missing".to_string())?;
    if calls == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
    {
        return Err("direct-zone IdealLoads CP389 provenance is invalid".to_string());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let owner_state = &temperature_owner.state;
    let assignments = state
        .dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count;
    let inactive = state
        .transition_count
        .checked_sub(assignments)
        .ok_or_else(|| "direct-zone IdealLoads CP389 inactive partition underflowed".to_string())?;
    let route_sum = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let sites = assignments.checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len()).ok_or_else(|| "direct-zone IdealLoads CP389 site count overflowed".to_string())?;
    let owner_count = owner_state
        .cp334_supply_temperature_mixed_air_limit_owner_count
        .checked_add(
            owner_state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        )
        .ok_or_else(|| "direct-zone IdealLoads CP389 CP379 owner count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        ("predecessor_transition_count", predecessor_state.transition_count, state.transition_count),
        ("route_partition", state.transition_count, route_sum),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("predecessor_assignment_count", predecessor_state.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count, assignments),
        ("direct_assignment_count", 0, assignments),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("cp379_supply_temperature_state_owner_count", owner_count, state.cp379_supply_temperature_state_owner_count),
        ("unchanged_supply_temperature_preservation_count", owner_count, state.unchanged_supply_temperature_preservation_count),
        ("mixed_air_temperature_owned_read_count", assignments, state.mixed_air_temperature_owned_read_count),
        ("cooling_sensible_output_owned_read_count", assignments, state.cooling_sensible_output_owned_read_count),
        ("cp_air_owned_read_count", assignments, state.cp_air_owned_read_count),
        ("supply_mass_flow_rate_owned_read_count", assignments, state.supply_mass_flow_rate_owned_read_count),
        ("supply_mass_flow_rate_bit_corroboration_count", assignments, state.supply_mass_flow_rate_bit_corroboration_count),
        ("air_capacity_rate_calculation_count", assignments, state.air_capacity_rate_calculation_count),
        ("sensible_temperature_drop_calculation_count", assignments, state.sensible_temperature_drop_calculation_count),
        ("supply_temperature_calculation_count", assignments, state.supply_temperature_calculation_count),
        ("supply_temperature_assignment_write_count", assignments, state.supply_temperature_assignment_write_count),
    ] {
        ensure_count(actual, expected, field)?;
    }
    if state.predecessor_route_counts != predecessor_state.predecessor_route_counts {
        return Err("direct-zone IdealLoads CP389 route lineage is invalid".to_string());
    }

    let system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP389 declared system is missing".to_string())?;
    let zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP389 controlled Zone is missing".to_string())?;
    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP389 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP389 CP388 latest evidence is missing".to_string()
    })?;
    let owner_latest = owner_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP389 CP379 latest evidence is missing".to_string()
    })?;
    if state.system != system
        || predecessor_state.system != system
        || owner_state.system != system
        || latest.source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || latest.first_excluded_source != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || latest.source_order != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        || latest.system != system
        || latest.controlled_zone != zone
        || latest.parent_call_ordinal != calls
        || !links_to_predecessor(latest, predecessor_latest)
        || !direct_skip_shape(latest)
        || latest.cp379_retained_supply_temperature_state_owned
            != owner_latest.supply_temperature_c.is_some()
        || !option_bits_equal(latest.preexisting_supply_temperature_c, owner_latest.supply_temperature_c)
        || !option_bits_equal(latest.resulting_supply_temperature_c, owner_latest.supply_temperature_c)
    {
        return Err("direct-zone IdealLoads CP389 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn links_to_predecessor(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    predecessor.source == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && inherited_flags(snapshot) == predecessor_flags(predecessor)
        && snapshot.predecessor_supply_enthalpy_assignment_executed == predecessor.predecessor_supply_enthalpy_assignment_executed
        && snapshot.predecessor_dehumidification_control_type_read == predecessor.predecessor_dehumidification_control_type_read
        && snapshot.predecessor_dehumidification_control_type == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_dehumidification_control_switch_dispatched == predecessor.predecessor_dehumidification_control_switch_dispatched
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered == predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed == predecessor.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && snapshot.predecessor_mixed_air_humidity_ratio_read == predecessor.predecessor_mixed_air_humidity_ratio_read
        && snapshot.predecessor_psychrometric_cp_air_evaluated == predecessor.predecessor_psychrometric_cp_air_evaluated
        && snapshot.predecessor_cp_air_assigned == predecessor.predecessor_cp_air_assigned
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed == predecessor.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
        && snapshot.predecessor_cp384_retained_cooling_total_output_owned_read == predecessor.cp384_retained_cooling_total_output_owned_read
        && snapshot.predecessor_cp385_cooling_total_output_bit_corroborated == predecessor.cp385_cooling_total_output_bit_corroborated
        && snapshot.predecessor_cooling_total_output_read == predecessor.cooling_total_output_read
        && snapshot.predecessor_cooling_sensible_heat_ratio_read == predecessor.cooling_sensible_heat_ratio_read
        && snapshot.predecessor_cooling_sensible_output_calculated == predecessor.cooling_sensible_output_calculated
        && snapshot.predecessor_cooling_sensible_output_assigned == predecessor.cooling_sensible_output_assigned
        && predecessor_values(snapshot).into_iter().zip(predecessor_values_from_cp388(predecessor)).all(|(left, right)| option_bits_equal(left, right))
}

fn direct_skip_shape(snapshot: Snapshot) -> bool {
    !snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed
        && !snapshot.cp329_retained_mixed_air_temperature_owned_read
        && !snapshot.mixed_air_temperature_read && snapshot.mixed_air_temperature_c.is_none()
        && !snapshot.cp388_retained_cooling_sensible_output_owned_read
        && !snapshot.cooling_sensible_output_read && snapshot.cooling_sensible_output_w.is_none()
        && !snapshot.cp387_retained_cp_air_owned_read
        && !snapshot.cp_air_read && snapshot.cp_air_j_per_kg_k.is_none()
        && !snapshot.cp330_retained_supply_mass_flow_rate_owned_read
        && !snapshot.cp329_supply_mass_flow_rate_bit_corroborated
        && !snapshot.supply_mass_flow_rate_read && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.cp_air_times_supply_mass_flow_rate_calculated && snapshot.cp_air_times_supply_mass_flow_rate_w_per_k.is_none()
        && !snapshot.cooling_sensible_output_over_air_capacity_rate_calculated && snapshot.cooling_sensible_output_over_air_capacity_rate_k.is_none()
        && !snapshot.supply_temperature_calculated && snapshot.calculated_supply_temperature_c.is_none()
        && !snapshot.supply_temperature_assigned && snapshot.assigned_supply_temperature_c.is_none()
}

fn inherited_flags(snapshot: Snapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}
fn predecessor_flags(snapshot: PredecessorSnapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}
fn predecessor_values(snapshot: Snapshot) -> [Option<f64>; 8] {
    [
        snapshot.predecessor_mixed_air_humidity_ratio,
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        snapshot.predecessor_cp_air_j_per_kg_k,
        snapshot.predecessor_cooling_total_output_w,
        snapshot.predecessor_cooling_sensible_heat_ratio,
        snapshot.predecessor_calculated_cooling_sensible_output_w,
        snapshot.predecessor_cooling_sensible_output_w,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ]
}
fn predecessor_values_from_cp388(snapshot: PredecessorSnapshot) -> [Option<f64>; 8] {
    [
        snapshot.predecessor_mixed_air_humidity_ratio,
        snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        snapshot.predecessor_cp_air_j_per_kg_k,
        snapshot.cooling_total_output_w,
        snapshot.cooling_sensible_heat_ratio,
        snapshot.calculated_cooling_sensible_output_w,
        snapshot.cooling_sensible_output_w,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ]
}
fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP389 {label} overflowed"))
    })
}
fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP389 invariant {field} expected {expected}, got {actual}"
        ))
    }
}
