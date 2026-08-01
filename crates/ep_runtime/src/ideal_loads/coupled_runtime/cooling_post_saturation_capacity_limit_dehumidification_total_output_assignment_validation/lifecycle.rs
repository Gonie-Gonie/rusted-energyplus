use super::routes::*;
use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn validate(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    supply_mass_flow_owner: &SupplyMassFlowLifecycle,
    mixed_air_owner: &MixedAirLifecycle,
    early_total_corroborator: &EarlyTotalLifecycle,
    supply_enthalpy_owner: &SupplyEnthalpyLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE
        || supply_mass_flow_owner.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || supply_mass_flow_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || mixed_air_owner.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || mixed_air_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || early_total_corroborator.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || early_total_corroborator.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || supply_enthalpy_owner.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE
        || supply_enthalpy_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len()
            != 6
        || [
            lifecycle.state.system,
            predecessor.state.system,
            supply_mass_flow_owner.state.system,
            mixed_air_owner.state.system,
            early_total_corroborator.state.system,
            supply_enthalpy_owner.state.system,
        ]
        .into_iter()
        .any(|system| system != binding.ideal_loads_air_system)
    {
        return Err(violation("source_owner_and_system_identity", 1, 0));
    }

    validate_counts(&lifecycle.state, &predecessor.state, timestep_count)?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let supply_mass_flow_latest = supply_mass_flow_owner
        .state
        .latest
        .ok_or_else(|| violation("cp330_latest_owner_snapshot_ready", 1, 0))?;
    let mixed_air_latest = mixed_air_owner
        .state
        .latest
        .ok_or_else(|| violation("cp329_latest_owner_snapshot_ready", 1, 0))?;
    let early_total_latest = early_total_corroborator
        .state
        .latest
        .ok_or_else(|| violation("cp339_latest_corroborator_snapshot_ready", 1, 0))?;
    let supply_enthalpy_latest = supply_enthalpy_owner
        .state
        .latest
        .ok_or_else(|| violation("cp379_latest_owner_snapshot_ready", 1, 0))?;
    let expected = snapshot::expected_snapshot(
        predecessor_latest,
        supply_mass_flow_latest,
        mixed_air_latest,
        early_total_latest,
        supply_enthalpy_latest,
    )
    .ok_or_else(|| violation("latest_owner_lineage_ready", 1, 0))?;

    if !snapshot::snapshots_match_exact_bits(latest, expected)
        || !snapshot::snapshots_match_exact_bits(
            latest,
            latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment,
        )
        || !snapshot::matches_release(latest_output, timestep_count, binding)
        || !latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            &supply_mass_flow_owner.state,
            &mixed_air_owner.state,
            &early_total_corroborator.state,
            &supply_enthalpy_owner.state,
            latest,
        )
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &PredecessorState,
    timestep_count: usize,
) -> Result<(), Error> {
    if base_route_counts(state) != predecessor_base_route_counts(predecessor) {
        return Err(violation("predecessor_route_counters", 1, 0));
    }
    let refined = refined_route_counts(state);
    let predecessor_refined = predecessor_refined_route_counts(predecessor);
    if refined != predecessor_refined {
        return Err(violation("predecessor_refined_route_counters", 1, 0));
    }
    let base = base_route_counts(state);
    let capacity_false = route_capacity_false_counts(state);
    let body_entries = route_body_entry_counts(state);
    let guard_false = route_guard_false_counts(state);
    for (route, ((capacity_false, body_entry), guard_false)) in base[3..].iter().zip(
        capacity_false
            .into_iter()
            .zip(body_entries)
            .zip(guard_false),
    ) {
        let partition = checked_sum(
            &[capacity_false, body_entry, guard_false],
            "active_route_partition_overflow",
        )?;
        ensure_count(partition, *route, "active_route_partition")?;
    }
    let route_assignments = route_assignment_counts(state);
    if route_assignments != body_entries {
        return Err(violation("route_body_assignment_counters", 1, 0));
    }
    let assigned = checked_sum(&route_assignments, "assignment_route_sum_overflow")?;
    let sites = checked_mul(
        assigned,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
        "source_site_execution_count_overflow",
    )?;
    let transition_partition = checked_sum(&refined, "transition_partition_overflow")?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "dehumidification_total_output_assignment_count",
            predecessor.dehumidification_body_entry_count,
            state.dehumidification_total_output_assignment_count,
        ),
        (
            "assignment_route_sum",
            assigned,
            state.dehumidification_total_output_assignment_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
        (
            "cp330_supply_mass_flow_rate_owned_read_count",
            assigned,
            state.cp330_supply_mass_flow_rate_owned_read_count,
        ),
        (
            "cp329_same_call_supply_mass_flow_rate_bit_corroboration_count",
            assigned,
            state.cp329_same_call_supply_mass_flow_rate_bit_corroboration_count,
        ),
        (
            "cp339_same_call_supply_mass_flow_rate_bit_corroboration_count",
            assigned,
            state.cp339_same_call_supply_mass_flow_rate_bit_corroboration_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            assigned,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "cp329_mixed_air_enthalpy_owned_read_count",
            assigned,
            state.cp329_mixed_air_enthalpy_owned_read_count,
        ),
        (
            "cp329_same_call_recirculation_enthalpy_bit_corroboration_count",
            assigned,
            state.cp329_same_call_recirculation_enthalpy_bit_corroboration_count,
        ),
        (
            "cp339_same_call_mixed_air_enthalpy_bit_corroboration_count",
            assigned,
            state.cp339_same_call_mixed_air_enthalpy_bit_corroboration_count,
        ),
        (
            "mixed_air_enthalpy_read_count",
            assigned,
            state.mixed_air_enthalpy_read_count,
        ),
        (
            "cp379_post_saturation_supply_enthalpy_owned_read_count",
            assigned,
            state.cp379_post_saturation_supply_enthalpy_owned_read_count,
        ),
        (
            "cp379_same_call_supply_enthalpy_bits_corroboration_count",
            assigned,
            state.cp379_same_call_supply_enthalpy_bits_corroboration_count,
        ),
        (
            "supply_enthalpy_read_count",
            assigned,
            state.supply_enthalpy_read_count,
        ),
        (
            "enthalpy_difference_calculation_count",
            assigned,
            state.enthalpy_difference_calculation_count,
        ),
        (
            "cooling_total_output_calculation_count",
            assigned,
            state.cooling_total_output_calculation_count,
        ),
        (
            "cooling_total_output_assignment_write_count",
            assigned,
            state.cooling_total_output_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    supply_mass_flow_owner: &SupplyMassFlowState,
    mixed_air_owner: &MixedAirState,
    early_total_corroborator: &EarlyTotalState,
    supply_enthalpy_owner: &SupplyEnthalpyState,
    latest: Snapshot,
) -> bool {
    let Some(index) = refined_route_index(latest) else {
        return false;
    };
    let base_index = if index < 3 {
        index
    } else {
        3 + (index - 3) / 3
    };
    let cp330_index = if base_index < 3 { base_index } else { 3 };
    let cp329_index = if base_index < 2 { base_index } else { 2 };
    let cp339_index = if base_index < 3 {
        base_index
    } else if (index - 3) % 3 == 0 {
        3
    } else {
        4
    };
    refined_route_counts(state)[index] > 0
        && predecessor_refined_route_counts(predecessor)[index] > 0
        && supply_mass_flow_route_counts(supply_mass_flow_owner)[cp330_index] > 0
        && mixed_air_route_counts(mixed_air_owner)[cp329_index] > 0
        && early_total_route_counts(early_total_corroborator)[cp339_index] > 0
        && supply_enthalpy_route_counts(supply_enthalpy_owner)[base_index] > 0
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
    })
}

fn checked_mul(left: usize, right: usize, field: &'static str) -> Result<usize, Error> {
    left.checked_mul(right)
        .ok_or_else(|| violation(field, 0, usize::MAX))
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}
