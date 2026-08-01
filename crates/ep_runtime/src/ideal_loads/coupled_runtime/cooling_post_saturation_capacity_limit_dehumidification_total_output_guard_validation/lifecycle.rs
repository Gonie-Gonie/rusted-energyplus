use super::routes::*;
use super::*;

pub(super) fn validate(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    capacity_owner: &CapacityLifecycle,
    capacity_corroborator: &CapacityCorroboratorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || capacity_owner.source != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || capacity_owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || capacity_corroborator.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE
        || capacity_corroborator.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER.len()
            != 4
        || [
            lifecycle.state.system,
            predecessor.state.system,
            capacity_owner.state.system,
            capacity_corroborator.state.system,
        ]
        .into_iter()
        .any(|system| system != binding.ideal_loads_air_system)
    {
        return Err(violation("source_owner_and_system_identity", 1, 0));
    }

    super::counts::validate(&lifecycle.state, &predecessor.state, timestep_count)?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .state
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let capacity_latest = capacity_owner
        .state
        .latest
        .ok_or_else(|| violation("cp321_latest_owner_snapshot_ready", 1, 0))?;
    let corroborator_latest = capacity_corroborator
        .state
        .latest
        .ok_or_else(|| violation("cp340_latest_corroborator_snapshot_ready", 1, 0))?;
    let expected =
        snapshot::expected_snapshot(predecessor_latest, capacity_latest, corroborator_latest)
            .ok_or_else(|| violation("latest_owner_lineage_ready", 1, 0))?;

    if !snapshot::snapshots_match_exact_bits(latest, expected)
        || !snapshot::snapshots_match_exact_bits(
            latest,
            latest_output
                .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard,
        )
        || !snapshot::matches_release(latest_output, timestep_count, binding)
        || !latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            &capacity_owner.state,
            &capacity_corroborator.state,
            latest,
            corroborator_latest,
        )
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    capacity_owner: &CapacityState,
    capacity_corroborator: &CapacityCorroboratorState,
    latest: Snapshot,
    corroborator_latest: CapacityCorroboratorSnapshot,
) -> bool {
    let Some(index) = refined_route_index(latest) else {
        return false;
    };
    let Some(predecessor_index) = predecessor_route_index(index) else {
        return false;
    };
    let route_exists = refined_route_counts(state)[index] > 0
        && predecessor_refined_route_counts(predecessor)[predecessor_index] > 0;
    if !route_exists {
        return false;
    }
    if !latest.dehumidification_total_output_capacity_guard_evaluated {
        return true;
    }
    capacity_owner.maximum_total_cooling_capacity_nonzero_count > 0
        && corroborator_latest_route_has_count(capacity_corroborator, corroborator_latest)
}
