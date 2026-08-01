//! Release validation for the bounded humidifying-setpoint moisture-demand assignment.

use ep_model::{DehumidificationControlType, HumidificationControlType};

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState as PredecessorState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Snapshot,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard;
    let snapshot =
        output.calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && snapshot == expected_snapshot(predecessor)
}

pub(super) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_lifecycle: &PredecessorLifecycle,
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
    if binding.system.humidification_control_type != HumidificationControlType::None
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER.len()
            != 5
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len()
            != 2
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || !cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release(
            *predecessor_latest,
        )
        || predecessor_latest
            != &latest_output.calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
        || !cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release(
            *latest,
        )
        || *latest != expected_snapshot(*predecessor_latest)
        || latest
            != &latest_output.calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment
        || !snapshot_matches_release(latest_output, timestep_count, binding)
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
            "heating_availability_guard_false_fallthrough_count",
            predecessor.heating_on_guard_false_fallthrough_count,
            state.heating_availability_guard_false_fallthrough_count,
        ),
        (
            "humidification_control_guard_false_fallthrough_count",
            predecessor.humidification_control_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_count,
        ),
        (
            "dehumidification_control_humidistat_moisture_demand_assignment_count",
            predecessor.dehumidification_control_type_humidistat_match_count,
            state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        ),
        (
            "dehumidification_control_none_moisture_demand_assignment_count",
            predecessor.dehumidification_control_type_none_match_count,
            state.dehumidification_control_none_moisture_demand_assignment_count,
        ),
        (
            "dehumidification_control_guard_false_fallthrough_count",
            predecessor.dehumidification_control_guard_false_fallthrough_count,
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
        (
            "humidification_moisture_demand_assignment_count",
            predecessor.dehumidification_control_body_entry_count,
            state.humidification_moisture_demand_assignment_count,
        ),
        (
            "direct_dehumidification_control_humidistat_moisture_demand_assignment_count",
            0,
            state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        ),
        (
            "direct_dehumidification_control_none_moisture_demand_assignment_count",
            0,
            state.dehumidification_control_none_moisture_demand_assignment_count,
        ),
        (
            "direct_dehumidification_control_guard_false_fallthrough_count",
            0,
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
        (
            "direct_humidification_moisture_demand_assignment_count",
            0,
            state.humidification_moisture_demand_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_partition(state: &State) -> Result<(), Error> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        state.dehumidification_control_none_moisture_demand_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")?;
    let assignments = checked_sum(&[
        state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        state.dehumidification_control_none_moisture_demand_assignment_count,
    ])?;
    ensure_count(
        assignments,
        state.humidification_moisture_demand_assignment_count,
        "assignment_route_partition",
    )
}

fn validate_source_counters(state: &State) -> Result<(), Error> {
    let assignments = state.humidification_moisture_demand_assignment_count;
    let source_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER.len(),
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
            "zone_humidifying_setpoint_moisture_demand_read_count",
            assignments,
            state.zone_humidifying_setpoint_moisture_demand_read_count,
        ),
        (
            "zone_humidifying_setpoint_moisture_demand_assignment_count",
            assignments,
            state.zone_humidifying_setpoint_moisture_demand_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

pub(super) fn expected_snapshot(predecessor: PredecessorSnapshot) -> Snapshot {
    Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type:
            predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip:
            predecessor.predecessor_dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_case_completed_skip:
            predecessor.predecessor_dehumidification_control_humidistat_case_completed_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            predecessor
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
            predecessor
                .predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        dehumidification_control_humidistat_case_completed_skip:
            predecessor.dehumidification_control_humidistat_case_completed_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip,
        predecessor_heating_on_read: predecessor.predecessor_heating_on_read,
        predecessor_heating_on: predecessor.predecessor_heating_on,
        predecessor_cooling_supply_humidity_ratio_humidification_body_entered:
            predecessor.predecessor_cooling_supply_humidity_ratio_humidification_body_entered,
        predecessor_heating_on_guard_false_fallthrough:
            predecessor.predecessor_heating_on_guard_false_fallthrough,
        predecessor_humidification_control_type_read:
            predecessor.predecessor_humidification_control_type_read,
        predecessor_humidification_control_type:
            predecessor.predecessor_humidification_control_type,
        predecessor_humidification_control_type_humidistat:
            predecessor.predecessor_humidification_control_type_humidistat,
        predecessor_humidification_control_body_entered:
            predecessor.predecessor_humidification_control_body_entered,
        predecessor_humidification_control_guard_false_fallthrough:
            predecessor.predecessor_humidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type_first_read:
            predecessor.dehumidification_control_type_first_read,
        predecessor_first_dehumidification_control_type:
            predecessor.first_dehumidification_control_type,
        predecessor_dehumidification_control_type_humidistat:
            predecessor.dehumidification_control_type_humidistat,
        predecessor_dehumidification_control_type_second_read:
            predecessor.dehumidification_control_type_second_read,
        predecessor_second_dehumidification_control_type:
            predecessor.second_dehumidification_control_type,
        predecessor_dehumidification_control_type_none:
            predecessor.dehumidification_control_type_none,
        predecessor_dehumidification_control_body_entered:
            predecessor.dehumidification_control_body_entered,
        predecessor_dehumidification_control_guard_false_fallthrough:
            predecessor.dehumidification_control_guard_false_fallthrough,
        humidification_moisture_demand_assignment_executed: false,
        zone_humidifying_setpoint_moisture_demand_read: false,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
        zone_humidifying_setpoint_moisture_demand_assigned: false,
        assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
        resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: None,
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
    Error::CalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleInvariant {
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
        let mut state = State::new(system);
        state.unit_off_skip_count = usize::MAX;
        state.non_cooling_skip_count = 1;
        assert!(validate_route_partition(&state).is_err());

        let mut active = State::new(system);
        active.transition_count = 1;
        active.dehumidification_control_none_moisture_demand_assignment_count = 1;
        active.humidification_moisture_demand_assignment_count = 1;
        active.source_site_execution_count = 2;
        active.zone_humidifying_setpoint_moisture_demand_read_count = 1;
        active.zone_humidifying_setpoint_moisture_demand_assignment_count = 1;
        assert!(validate_route_partition(&active).is_ok());
        assert!(validate_source_counters(&active).is_ok());
        active.source_site_execution_count = 1;
        assert!(validate_source_counters(&active).is_err());
    }
}
