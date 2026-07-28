//! Release validation for the bounded cooling positive-supply temperature assignment.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
};

use super::super::calc::cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_positive_supply_cp_air_assignment;
    let snapshot = output.calculation_cooling_positive_supply_temperature_assignment;
    let expected = expected_snapshot(output, predecessor);

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(snapshot)
        && source_lineage_matches(output, &snapshot)
        && snapshots_match_exact_bits(&snapshot, &expected)
}

fn expected_snapshot(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
    let assignment_executed = predecessor.cp_air_assignment_executed;
    let zone_cooling_setpoint_load = assignment_executed.then_some(
        output
            .calculation_entry
            .demand
            .remaining_output_req_to_cool_sp_w,
    );
    let cp_air = assignment_executed
        .then_some(predecessor.cp_air_j_per_kg_k)
        .flatten();
    let supply_mass_flow_rate = assignment_executed
        .then_some(
            output
                .calculation_cooling_supply_mass_flow_positive_guard
                .supply_mass_flow_rate_kg_per_s,
        )
        .flatten();
    let denominator = cp_air
        .zip(supply_mass_flow_rate)
        .map(|(cp_air, supply_mass_flow_rate)| cp_air * supply_mass_flow_rate);
    let quotient = zone_cooling_setpoint_load
        .zip(denominator)
        .map(|(zone_cooling_setpoint_load, denominator)| zone_cooling_setpoint_load / denominator);
    let zone_node_temperature = assignment_executed
        .then_some(
            output
                .calculation_cooling_positive_supply_temperature_assignment
                .zone_node_temperature_c,
        )
        .flatten();
    let supply_temperature = quotient
        .zip(zone_node_temperature)
        .map(|(quotient, zone_node_temperature)| quotient + zone_node_temperature);

    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor
            .predecessor_active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        supply_temperature_assignment_executed: assignment_executed,
        zone_cooling_setpoint_load_read: assignment_executed,
        zone_cooling_setpoint_load_w: zone_cooling_setpoint_load,
        cp_air_read: assignment_executed,
        cp_air_j_per_kg_k: cp_air,
        supply_mass_flow_rate_read: assignment_executed,
        supply_mass_flow_rate_kg_per_s: supply_mass_flow_rate,
        cp_air_times_supply_mass_flow_rate_calculated: assignment_executed,
        cp_air_times_supply_mass_flow_rate_w_per_k: denominator,
        zone_cooling_setpoint_load_over_denominator_calculated: assignment_executed,
        zone_cooling_setpoint_load_over_denominator_c: quotient,
        zone_node_temperature_read: assignment_executed,
        zone_node_temperature_c: zone_node_temperature,
        supply_temperature_calculated: assignment_executed,
        calculated_supply_temperature_c: supply_temperature,
        supply_temperature_assigned: assignment_executed,
        supply_temperature_c: supply_temperature,
    }
}

fn source_lineage_matches(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    if !snapshot.supply_temperature_assignment_executed {
        return true;
    }
    options_have_exact_bits(
        snapshot.zone_cooling_setpoint_load_w,
        Some(
            output
                .calculation_entry
                .demand
                .remaining_output_req_to_cool_sp_w,
        ),
    ) && options_have_exact_bits(
        snapshot.cp_air_j_per_kg_k,
        output
            .calculation_cooling_positive_supply_cp_air_assignment
            .cp_air_j_per_kg_k,
    ) && options_have_exact_bits(
        snapshot.supply_mass_flow_rate_kg_per_s,
        output
            .calculation_cooling_supply_mass_flow_positive_guard
            .supply_mass_flow_rate_kg_per_s,
    ) && options_have_exact_bits(
        snapshot.zone_node_temperature_c,
        output.calculation_cooling_sensible_flow.zone_temperature_c,
    ) && options_have_exact_bits(
        snapshot.zone_node_temperature_c,
        output
            .calculation_cooling_mixed_air_call
            .recirculation_temperature_c,
    ) && options_have_exact_bits(
        snapshot.zone_node_temperature_c,
        output
            .calculation_cooling_mixed_air_call
            .mixed_air_temperature_c,
    )
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let active_routes = checked_add(
        state.positive_guard_false_fallthrough_skip_count,
        state.supply_temperature_assignment_count,
        "active_route_partition_overflow",
        timestep_count,
    )?;
    let transition_partition = checked_add(
        skipped,
        active_routes,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let source_sites = checked_mul(
        state.supply_temperature_assignment_count,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len(),
        "source_site_execution_count_overflow",
        state.source_site_execution_count,
    )?;

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
            "supply_temperature_assignment_count",
            predecessor.cp_air_assignment_count,
            state.supply_temperature_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "zone_cooling_setpoint_load_read_count",
            state.supply_temperature_assignment_count,
            state.zone_cooling_setpoint_load_read_count,
        ),
        (
            "cp_air_read_count",
            state.supply_temperature_assignment_count,
            state.cp_air_read_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            state.supply_temperature_assignment_count,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "cp_air_times_supply_mass_flow_rate_calculation_count",
            state.supply_temperature_assignment_count,
            state.cp_air_times_supply_mass_flow_rate_calculation_count,
        ),
        (
            "zone_cooling_setpoint_load_over_denominator_calculation_count",
            state.supply_temperature_assignment_count,
            state.zone_cooling_setpoint_load_over_denominator_calculation_count,
        ),
        (
            "zone_node_temperature_read_count",
            state.supply_temperature_assignment_count,
            state.zone_node_temperature_read_count,
        ),
        (
            "supply_temperature_calculation_count",
            state.supply_temperature_assignment_count,
            state.supply_temperature_calculation_count,
        ),
        (
            "supply_temperature_assignment_write_count",
            state.supply_temperature_assignment_count,
            state.supply_temperature_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || predecessor_latest
            != latest_output.calculation_cooling_positive_supply_cp_air_assignment
        || !snapshots_match_exact_bits(latest, &expected_snapshot(latest_output, predecessor_latest))
        || !snapshots_match_exact_bits(
            latest,
            &latest_output.calculation_cooling_positive_supply_temperature_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    right: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let values_match = [
        (
            left.zone_cooling_setpoint_load_w,
            right.zone_cooling_setpoint_load_w,
        ),
        (left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k),
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.cp_air_times_supply_mass_flow_rate_w_per_k,
            right.cp_air_times_supply_mass_flow_rate_w_per_k,
        ),
        (
            left.zone_cooling_setpoint_load_over_denominator_c,
            right.zone_cooling_setpoint_load_over_denominator_c,
        ),
        (left.zone_node_temperature_c, right.zone_node_temperature_c),
        (
            left.calculated_supply_temperature_c,
            right.calculated_supply_temperature_c,
        ),
        (left.supply_temperature_c, right.supply_temperature_c),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.zone_cooling_setpoint_load_w = None;
    right_without_values.zone_cooling_setpoint_load_w = None;
    left_without_values.cp_air_j_per_kg_k = None;
    right_without_values.cp_air_j_per_kg_k = None;
    left_without_values.supply_mass_flow_rate_kg_per_s = None;
    right_without_values.supply_mass_flow_rate_kg_per_s = None;
    left_without_values.cp_air_times_supply_mass_flow_rate_w_per_k = None;
    right_without_values.cp_air_times_supply_mass_flow_rate_w_per_k = None;
    left_without_values.zone_cooling_setpoint_load_over_denominator_c = None;
    right_without_values.zone_cooling_setpoint_load_over_denominator_c = None;
    left_without_values.zone_node_temperature_c = None;
    right_without_values.zone_node_temperature_c = None;
    left_without_values.calculated_supply_temperature_c = None;
    right_without_values.calculated_supply_temperature_c = None;
    left_without_values.supply_temperature_c = None;
    right_without_values.supply_temperature_c = None;
    values_match && left_without_values == right_without_values
}

fn options_have_exact_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

pub(super) fn checked_add(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

fn checked_mul(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_mul(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingPositiveSupplyTemperatureAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_site_count_multiplication_overflow_fails_closed() {
        let error = checked_mul(
            usize::MAX,
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len(),
            "test_source_site_count_overflow",
            usize::MAX,
        )
        .expect_err("source-site multiplication overflow must fail closed");

        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyTemperatureAssignmentLifecycleInvariant {
                field: "test_source_site_count_overflow",
                expected: usize::MAX,
                actual: usize::MAX,
            }
        ));
    }
}
