//! Release validation for the bounded cooling positive-supply Cp-air assignment.

use crate::{
    ideal_loads::{
        DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    },
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

use super::super::calc::cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_supply_mass_flow_positive_guard;
    let snapshot = output.calculation_cooling_positive_supply_cp_air_assignment;
    let mixed_air = output.calculation_cooling_mixed_air_call;
    let source_humidity_ratio = snapshot
        .zone_humidity_ratio
        .or(mixed_air.recirculation_humidity_ratio)
        .unwrap_or(0.0);
    let source_lineage_matches = !snapshot.cp_air_assignment_executed
        || (options_have_exact_bits(
            snapshot.zone_humidity_ratio,
            mixed_air.recirculation_humidity_ratio,
        ) && options_have_exact_bits(
            snapshot.zone_humidity_ratio,
            mixed_air.mixed_air_humidity_ratio,
        ));

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release(snapshot)
        && source_lineage_matches
        && snapshots_match_exact_bits(
            &snapshot,
            &expected_snapshot(predecessor, source_humidity_ratio),
        )
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    source_humidity_ratio: f64,
) -> PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
    let assignment_executed = predecessor.positive_supply_mass_flow_body_entered;
    let zone_humidity_ratio = assignment_executed.then_some(source_humidity_ratio);
    let cp_air = zone_humidity_ratio.map(energyplus_psy_cp_air_fn_w);

    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .positive_supply_mass_flow_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.active_guard_false_fallthrough,
        cp_air_assignment_executed: assignment_executed,
        zone_humidity_ratio_read: assignment_executed,
        zone_humidity_ratio,
        psychrometric_cp_air_evaluated: assignment_executed,
        psychrometric_cp_air_result_j_per_kg_k: cp_air,
        cp_air_assigned: assignment_executed,
        cp_air_j_per_kg_k: cp_air,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
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
        state.cp_air_assignment_count,
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
        state.cp_air_assignment_count,
        3,
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
            predecessor.active_guard_false_fallthrough_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "cp_air_assignment_count",
            predecessor.positive_supply_mass_flow_body_entry_count,
            state.cp_air_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "zone_humidity_ratio_read_count",
            state.cp_air_assignment_count,
            state.zone_humidity_ratio_read_count,
        ),
        (
            "psychrometric_cp_air_evaluation_count",
            state.cp_air_assignment_count,
            state.psychrometric_cp_air_evaluation_count,
        ),
        (
            "cp_air_assignment_write_count",
            state.cp_air_assignment_count,
            state.cp_air_assignment_write_count,
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
    let source_humidity_ratio = latest
        .zone_humidity_ratio
        .or(latest_output
            .calculation_cooling_mixed_air_call
            .recirculation_humidity_ratio)
        .unwrap_or(0.0);
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || !snapshots_match_exact_bits(
            latest,
            &expected_snapshot(predecessor_latest, source_humidity_ratio),
        )
        || !snapshots_match_exact_bits(
            latest,
            &latest_output.calculation_cooling_positive_supply_cp_air_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    right: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
) -> bool {
    let values_match = options_have_exact_bits(left.zone_humidity_ratio, right.zone_humidity_ratio)
        && options_have_exact_bits(
            left.psychrometric_cp_air_result_j_per_kg_k,
            right.psychrometric_cp_air_result_j_per_kg_k,
        )
        && options_have_exact_bits(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k);
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.zone_humidity_ratio = None;
    right_without_values.zone_humidity_ratio = None;
    left_without_values.psychrometric_cp_air_result_j_per_kg_k = None;
    right_without_values.psychrometric_cp_air_result_j_per_kg_k = None;
    left_without_values.cp_air_j_per_kg_k = None;
    right_without_values.cp_air_j_per_kg_k = None;
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
    Error::CalcCoolingPositiveSupplyCpAirAssignmentLifecycleInvariant {
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
        let error = checked_mul(usize::MAX, 3, "test_source_site_count_overflow", usize::MAX)
            .expect_err("source-site multiplication overflow must fail closed");

        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyCpAirAssignmentLifecycleInvariant {
                field: "test_source_site_count_overflow",
                expected: usize::MAX,
                actual: usize::MAX,
            }
        ));
    }

    #[test]
    fn snapshot_comparison_detects_signed_zero_source_corruption() {
        let predecessor = PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
            source: crate::ideal_loads::
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
            first_excluded_source: crate::ideal_loads::
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order: crate::ideal_loads::
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER,
            system: ep_model::IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ep_model::ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_call_executed: true,
            predecessor_zero_flow_reset_body_entered: false,
            predecessor_active_guard_false_fallthrough: false,
            predecessor_no_outdoor_air_fallback_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_rate_read: true,
            supply_mass_flow_rate_kg_per_s: Some(1.0),
            supply_mass_flow_rate_strictly_positive_comparison_evaluated: true,
            supply_mass_flow_rate_strictly_positive: Some(true),
            positive_supply_mass_flow_body_entered: true,
            active_guard_false_fallthrough: false,
        };
        let snapshot = expected_snapshot(predecessor, 0.0);
        let negative_zero = expected_snapshot(predecessor, -0.0);

        assert_eq!(snapshot, negative_zero);
        assert!(snapshots_match_exact_bits(&snapshot, &snapshot));
        assert!(!snapshots_match_exact_bits(&snapshot, &negative_zero));
    }
}
