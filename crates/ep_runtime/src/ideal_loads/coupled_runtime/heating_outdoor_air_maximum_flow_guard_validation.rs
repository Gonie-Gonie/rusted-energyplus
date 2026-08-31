//! Cheap coupled validation for CP435 heating outdoor-air maximum-flow guard evidence.

use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE,
    PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE_ORDER,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardLifecycleSummary as Lifecycle,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState as State,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Snapshot,
    PurchasedAirCalcMinimumOaPrefixLifecycleSummary as MinimumOaLifecycle,
    PurchasedAirInitLifecycleSummary as InitLifecycle,
    heating_operating_mode_deadband_assignment_snapshots_match_bit_exact,
    heating_outdoor_air_maximum_flow_guard_predecessor_cp434_snapshot,
    heating_outdoor_air_maximum_flow_guard_snapshot_is_exact,
    heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const PUBLIC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 20, 21, 26, 27];

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_heating_operating_mode_deadband_assignment;
    let snapshot = output.calculation_heating_outdoor_air_maximum_flow_guard;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.controlled_zone == binding.zone
        && snapshot.parent_call_ordinal == call_ordinal
        && heating_operating_mode_deadband_assignment_snapshots_match_bit_exact(
            heating_outdoor_air_maximum_flow_guard_predecessor_cp434_snapshot(snapshot),
            predecessor,
        )
        && heating_outdoor_air_maximum_flow_guard_snapshot_is_exact(snapshot)
        && local_guard_matches(snapshot, predecessor, output, binding)
}

#[allow(clippy::too_many_arguments)]
pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp434: &PredecessorLifecycle,
    minimum_oa_cp311: &MinimumOaLifecycle,
    initialization: &InitLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp434.state;
    if lifecycle.source != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor_cp434.source
            != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE
        || predecessor_cp434.first_excluded_source
            != PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_HEATING_OUTDOOR_AIR_MAXIMUM_FLOW_GUARD_SOURCE_ORDER.len() != 6
        || [
            state.system,
            predecessor.system,
            minimum_oa_cp311.state.system,
        ]
        .into_iter()
        .any(|system| system != binding.ideal_loads_air_system)
        || state.transition_count != predecessor.transition_count
        || state.transition_count != minimum_oa_cp311.state.transition_count
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || initialization.init_call_count != state.transition_count
        || initialization
            .maximum_heating_air_mass_flow_rate_kg_per_s
            .to_bits()
            != latest_output
                .initialization
                .maximum_heating_air_mass_flow_rate_kg_per_s
                .to_bits()
        || !initialization
            .maximum_heating_air_mass_flow_rate_kg_per_s
            .is_finite()
        || initialization.maximum_heating_air_mass_flow_rate_kg_per_s < 0.0
    {
        return Err(violation(
            "source_owner_predecessor_route_and_initialization_identity",
            1,
            0,
        ));
    }
    validate_counts(state, predecessor, timestep_count)?;
    let latest = state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if !heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact(
        latest,
        latest_output.calculation_heating_outdoor_air_maximum_flow_guard,
    ) || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    for values in [
        &state.predecessor_route_counts,
        &state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts,
        &state.maximum_heating_flow_body_entry_route_counts,
    ] {
        for (index, count) in values.iter().enumerate() {
            if !PUBLIC.contains(&index) && *count != 0 {
                return Err(violation("non_direct_route_count", 0, *count));
            }
        }
    }
    for index in 0..36 {
        let actual = checked_add(
            state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts[index],
            state.maximum_heating_flow_body_entry_route_counts[index],
            "route_partition_overflow",
        )?;
        let expected = if index == 1 {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        ensure_count(actual, expected, "guard_route_partition")?;
    }
    let transitions = checked_sum(&state.predecessor_route_counts, "transition_overflow")?;
    let false_fallthroughs = checked_sum(
        &state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts,
        "guard_partition_overflow",
    )?;
    let body_entries = checked_sum(
        &state.maximum_heating_flow_body_entry_route_counts,
        "guard_partition_overflow",
    )?;
    let evaluations = checked_add(false_fallthroughs, body_entries, "guard_partition_overflow")?;
    let inactive = transitions
        .checked_sub(evaluations)
        .ok_or_else(|| violation("transition_partition_underflow", evaluations, transitions))?;
    let flow_limit_active = checked_add(
        state.heating_limit_flow_rate_match_count,
        state.heating_limit_flow_rate_and_capacity_match_count,
        "selector_match_overflow",
    )?;
    let second_selector_comparisons = evaluations
        .checked_sub(state.heating_limit_flow_rate_match_count)
        .ok_or_else(|| {
            violation(
                "first_selector_match_partition_underflow",
                evaluations,
                state.heating_limit_flow_rate_match_count,
            )
        })?;
    let selector_rejections = evaluations.checked_sub(flow_limit_active).ok_or_else(|| {
        violation(
            "selector_rejection_partition_underflow",
            evaluations,
            flow_limit_active,
        )
    })?;
    let expected_sites = [
        state.heating_limit_flow_rate_comparison_count,
        state.heating_limit_flow_rate_and_capacity_comparison_count,
        state.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count,
        state.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count,
        state.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count,
        state.maximum_heating_flow_body_entry_count,
    ]
    .into_iter()
    .try_fold(0usize, |sum, value| sum.checked_add(value))
    .ok_or_else(|| violation("site_count_overflow", 0, usize::MAX))?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        ("route_partition", state.transition_count, transitions),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("guard_evaluation_count", evaluations, state.heating_outdoor_air_maximum_flow_guard_evaluation_count),
        ("first_selector_comparison_count", evaluations, state.heating_limit_flow_rate_comparison_count),
        ("second_selector_comparison_count", second_selector_comparisons, state.heating_limit_flow_rate_and_capacity_comparison_count),
        ("selector_rejection_count", selector_rejections, state.heating_flow_limit_selector_rejection_count),
        ("outdoor_air_read_count", flow_limit_active, state.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count),
        ("maximum_heating_flow_read_count", flow_limit_active, state.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count),
        ("strict_comparison_count", flow_limit_active, state.outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count),
        ("strict_comparison_true_count", body_entries, state.outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count),
        ("body_entry_count", body_entries, state.maximum_heating_flow_body_entry_count),
        ("false_fallthrough_count", false_fallthroughs, state.heating_outdoor_air_maximum_flow_guard_false_fallthrough_count),
        ("same_call_outdoor_air_corroboration_count", flow_limit_active, state.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count),
        ("source_site_execution_count", expected_sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp433_supply_humidity_ratio_state_owner_count, state.cp434_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", state.cp434_supply_humidity_ratio_state_owner_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp433_supply_enthalpy_state_owner_count, state.cp434_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", state.cp434_supply_enthalpy_state_owner_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp433_supply_temperature_state_owner_count, state.cp434_supply_temperature_state_owner_count),
        ("temperature_preservation_count", state.cp434_supply_temperature_state_owner_count, state.unchanged_supply_temperature_preservation_count),
        ("public_body_entry_count", 0, body_entries),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn local_guard_matches(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let active = predecessor.heating_or_no_load_case_entered;
    let heating_limit = binding.system.heating_limit;
    let first_match = active && heating_limit == IdealLoadsLimit::LimitFlowRate;
    let second_evaluated = active && !first_match;
    let second_match =
        second_evaluated && heating_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let flow_limit_active = first_match || second_match;
    let outdoor_air = output
        .calculation_minimum_outdoor_air
        .working_outdoor_air_mass_flow_rate_kg_per_s;
    let maximum = output
        .initialization
        .maximum_heating_air_mass_flow_rate_kg_per_s;
    let comparison = if flow_limit_active {
        outdoor_air.map(|value| value > maximum)
    } else {
        None
    };
    let body = comparison == Some(true);
    snapshot.heating_outdoor_air_maximum_flow_guard_evaluated == active
        && snapshot.heating_limit_flow_rate_comparison_evaluated == active
        && snapshot.heating_limit_flow_rate_value == active.then_some(heating_limit)
        && snapshot.heating_limit_flow_rate_comparison_satisfied == active.then_some(first_match)
        && snapshot.heating_limit_flow_rate_and_capacity_comparison_evaluated == second_evaluated
        && snapshot.heating_limit_flow_rate_and_capacity_value
            == second_evaluated.then_some(heating_limit)
        && snapshot.heating_limit_flow_rate_and_capacity_comparison_satisfied
            == second_evaluated.then_some(second_match)
        && snapshot.heating_flow_limit_active == active.then_some(flow_limit_active)
        && snapshot.heating_flow_limit_selector_rejected == (active && !flow_limit_active)
        && snapshot.cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated == flow_limit_active
        && snapshot.outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit
            == flow_limit_active
        && (!flow_limit_active
            || outdoor_air.is_some_and(|value| value.to_bits() == 0.0f64.to_bits()))
        && (!flow_limit_active
            || option_bits_equal(
                snapshot.outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s,
                outdoor_air,
            ))
        && (flow_limit_active
            || snapshot
                .outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s
                .is_none())
        && snapshot.maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit
            == flow_limit_active
        && option_has_bits_if(
            snapshot.maximum_heating_air_mass_flow_rate_for_guard_kg_per_s,
            maximum,
            flow_limit_active,
        )
        && snapshot
            .outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated
            == flow_limit_active
        && snapshot
            .outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate
            == comparison
        && snapshot.maximum_heating_flow_body_entered == body
        && snapshot.heating_outdoor_air_maximum_flow_guard_false_fallthrough == (active && !body)
        && !body
        && same_wht(snapshot, predecessor)
}

fn same_wht(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
) -> bool {
    option_bits_equal(
        snapshot.predecessor_cp434_resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_equal(
        snapshot.predecessor_cp434_resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        snapshot.predecessor_cp434_resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && option_bits_equal(
        snapshot.resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_equal(
        snapshot.resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        snapshot.resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    ) && snapshot.cp434_retained_supply_humidity_ratio_state_owned
        == predecessor.resulting_supply_humidity_ratio.is_some()
        && snapshot.cp434_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && snapshot.cp434_retained_supply_temperature_state_owned
            == predecessor.resulting_supply_temperature_c.is_some()
}

fn option_has_bits_if(value: Option<f64>, expected: f64, present: bool) -> bool {
    match (present, value) {
        (true, Some(value)) => value.to_bits() == expected.to_bits(),
        (false, None) => true,
        _ => false,
    }
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_add(left: usize, right: usize, field: &'static str) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, 0, usize::MAX))
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcHeatingOutdoorAirMaximumFlowGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validator_uses_three_route_arrays_and_keeps_numerical_input_out() {
        let source = include_str!("heating_outdoor_air_maximum_flow_guard_validation.rs");
        let production = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(production, _)| production);
        for required in [
            "predecessor_cp434_snapshot",
            "heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts",
            "maximum_heating_flow_body_entry_route_counts",
            "cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated",
            "maximum_heating_air_mass_flow_rate_kg_per_s",
        ] {
            assert!(production.contains(required), "{required}");
        }
        assert!(!production.contains("private_characterization"));
        assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
    }
}
