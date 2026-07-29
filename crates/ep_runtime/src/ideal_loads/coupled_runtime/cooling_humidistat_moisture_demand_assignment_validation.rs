//! Release validation for the bounded Humidistat moisture-demand assignment.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState,
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    cooling_humidistat_case_entry_snapshot_is_exact_direct_release,
    cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_humidistat_case_entry;
    let snapshot = output.calculation_cooling_humidistat_moisture_demand_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(snapshot)
        && snapshots_match_bit_exact(&snapshot, &expected_snapshot(predecessor))
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    validate_counts(state, predecessor, timestep_count)?;

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    if binding.system.dehumidification_control_type != DehumidificationControlType::None
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len() != 2
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || !cooling_humidistat_case_entry_snapshot_is_exact_direct_release(*predecessor_latest)
        || !super::cooling_humidistat_case_entry_validation::snapshots_match_exact(
            predecessor_latest,
            &latest_output.calculation_cooling_humidistat_case_entry,
        )
        || !cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(*latest)
        || !snapshots_match_bit_exact(latest, &expected_snapshot(*predecessor_latest))
        || !snapshots_match_bit_exact(
            latest,
            &latest_output.calculation_cooling_humidistat_moisture_demand_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState,
    predecessor: &PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    let constant_shr =
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count;
    let humidistat = state.dehumidification_control_humidistat_moisture_demand_assignment_count;
    validate_route_partition(state)?;
    validate_source_counters(state)?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "unit_off_skip_count",
            predecessor.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "positive_guard_false_fallthrough_skip_count",
            predecessor.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "none_case_completed_skip_count",
            predecessor.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
        ),
        (
            "constant_shr_case_completed_skip_count",
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            constant_shr,
        ),
        (
            "humidistat_moisture_demand_assignment_count",
            predecessor.dehumidification_control_humidistat_case_entry_count,
            humidistat,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        (
            "direct_constant_shr_case_completed_skip_count",
            0,
            constant_shr,
        ),
        (
            "direct_humidistat_moisture_demand_assignment_count",
            0,
            humidistat,
        ),
        (
            "direct_constant_supply_humidity_ratio_case_selected_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState,
) -> Result<(), Error> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_counters(
    state: &PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState,
) -> Result<(), Error> {
    let assignments = state.dehumidification_control_humidistat_moisture_demand_assignment_count;
    let source_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| {
            violation(
                "source_site_execution_count_overflow",
                usize::MAX,
                assignments,
            )
        })?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "zone_dehumidifying_setpoint_moisture_demand_read_count",
            assignments,
            state.zone_dehumidifying_setpoint_moisture_demand_read_count,
        ),
        (
            "zone_dehumidifying_setpoint_moisture_demand_assignment_count",
            assignments,
            state.zone_dehumidifying_setpoint_moisture_demand_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

pub(super) fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot,
) -> PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot {
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_entered: predecessor
            .dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: predecessor
            .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_moisture_demand_assignment_executed: predecessor
            .dehumidification_control_humidistat_case_entered,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: predecessor
            .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        zone_dehumidifying_setpoint_moisture_demand_read: false,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: None,
        zone_dehumidifying_setpoint_moisture_demand_assigned: false,
        assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: None,
        resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: None,
    }
}

pub(super) fn snapshots_match_bit_exact(
    left: &PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    right: &PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
) -> bool {
    left.source == right.source
        && left.first_excluded_source == right.first_excluded_source
        && left.source_order == right.source_order
        && left.system == right.system
        && left.parent_call_ordinal == right.parent_call_ordinal
        && left.controlled_zone == right.controlled_zone
        && left.unit_body_entered == right.unit_body_entered
        && left.predecessor_cooling_body_entered == right.predecessor_cooling_body_entered
        && left.predecessor_no_outdoor_air_fallback_entered
            == right.predecessor_no_outdoor_air_fallback_entered
        && left.predecessor_positive_supply_mass_flow_body_entered
            == right.predecessor_positive_supply_mass_flow_body_entered
        && left.unit_off_skipped == right.unit_off_skipped
        && left.non_cooling_skipped == right.non_cooling_skipped
        && left.positive_guard_false_fallthrough_skipped
            == right.positive_guard_false_fallthrough_skipped
        && left.predecessor_dehumidification_control_type
            == right.predecessor_dehumidification_control_type
        && left.predecessor_dehumidification_control_none_case_completed_skip
            == right.predecessor_dehumidification_control_none_case_completed_skip
        && left.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == right
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && left.predecessor_dehumidification_control_humidistat_case_entered
            == right.predecessor_dehumidification_control_humidistat_case_entered
        && left
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == right
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && left.dehumidification_control_none_case_completed_skip
            == right.dehumidification_control_none_case_completed_skip
        && left.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == right.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && left.dehumidification_control_humidistat_moisture_demand_assignment_executed
            == right.dehumidification_control_humidistat_moisture_demand_assignment_executed
        && left.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == right.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && left.zone_dehumidifying_setpoint_moisture_demand_read
            == right.zone_dehumidifying_setpoint_moisture_demand_read
        && option_bits_eq(
            left.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        )
        && left.zone_dehumidifying_setpoint_moisture_demand_assigned
            == right.zone_dehumidifying_setpoint_moisture_demand_assigned
        && option_bits_eq(
            left.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        )
        && option_bits_eq(
            left.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        )
}

fn option_bits_eq(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize]) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation("transition_partition_overflow", usize::MAX, *value))
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
    Error::CalcCoolingHumidistatMoistureDemandAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::IdealLoadsAirSystemId;

    #[test]
    fn partition_overflow_and_source_corruption_fail_closed() {
        let system = IdealLoadsAirSystemId(0);
        let mut state =
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState::new(system);
        state.unit_off_skip_count = usize::MAX;
        state.non_cooling_skip_count = 1;
        assert!(validate_route_partition(&state).is_err());

        let mut active =
            PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState::new(system);
        active.transition_count = 1;
        active.dehumidification_control_humidistat_moisture_demand_assignment_count = 1;
        active.source_site_execution_count = 2;
        active.zone_dehumidifying_setpoint_moisture_demand_read_count = 1;
        active.zone_dehumidifying_setpoint_moisture_demand_assignment_count = 1;
        assert!(validate_source_counters(&active).is_ok());
        active.source_site_execution_count = 1;
        assert!(validate_source_counters(&active).is_err());
    }
}
