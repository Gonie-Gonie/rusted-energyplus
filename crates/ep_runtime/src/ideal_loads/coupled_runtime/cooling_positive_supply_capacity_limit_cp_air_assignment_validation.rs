//! Release validation for the bounded capacity-limit `CpAir` assignment.

use crate::{
    ideal_loads::{
        DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
        PurchasedAirCalcCoolingMixedAirCallSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    },
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

use super::super::calc::cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_positive_supply_capacity_limit_guard;
    let mixed_air = output.calculation_cooling_mixed_air_call;
    let snapshot = output.calculation_cooling_positive_supply_capacity_limit_cp_air_assignment;
    let expected = expected_snapshot(predecessor, mixed_air);

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && snapshots_match_exact_bits(&snapshot, &expected)
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot {
    let assignment_executed = predecessor.capacity_limit_body_entered;
    let mixed_air_humidity_ratio = if assignment_executed {
        mixed_air.mixed_air_humidity_ratio
    } else {
        None
    };
    let cp_air = mixed_air_humidity_ratio.map(energyplus_psy_cp_air_fn_w);

    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_capacity_limit_guard_evaluated: predecessor.capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .active_guard_false_fallthrough,
        capacity_limit_guard_false_fallthrough_skipped: predecessor
            .active_guard_false_fallthrough,
        capacity_limit_cp_air_assignment_executed: assignment_executed,
        mixed_air_humidity_ratio_read: assignment_executed,
        mixed_air_humidity_ratio,
        psychrometric_cp_air_evaluated: assignment_executed,
        psychrometric_cp_air_result_j_per_kg_k: cp_air,
        cp_air_assigned: assignment_executed,
        cp_air_j_per_kg_k: cp_air,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    mixed_air_lifecycle: &PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let mixed_air = &mixed_air_lifecycle.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let skipped = checked_add(
        skipped,
        state.positive_guard_false_fallthrough_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let skipped = checked_add(
        skipped,
        state.capacity_limit_guard_false_fallthrough_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let transition_partition = checked_add(
        skipped,
        state.capacity_limit_cp_air_assignment_count,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let source_sites = checked_mul(
        state.capacity_limit_cp_air_assignment_count,
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
            "mixed_air_transition_count",
            mixed_air.transition_count,
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
            "capacity_limit_guard_false_fallthrough_skip_count",
            predecessor.active_guard_false_fallthrough_count,
            state.capacity_limit_guard_false_fallthrough_skip_count,
        ),
        (
            "capacity_limit_cp_air_assignment_count",
            predecessor.capacity_limit_body_entry_count,
            state.capacity_limit_cp_air_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "mixed_air_humidity_ratio_read_count",
            state.capacity_limit_cp_air_assignment_count,
            state.mixed_air_humidity_ratio_read_count,
        ),
        (
            "psychrometric_cp_air_evaluation_count",
            state.capacity_limit_cp_air_assignment_count,
            state.psychrometric_cp_air_evaluation_count,
        ),
        (
            "cp_air_assignment_write_count",
            state.capacity_limit_cp_air_assignment_count,
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
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let mixed_air_latest = mixed_air
        .latest
        .as_ref()
        .ok_or_else(|| violation("mixed_air_latest_release_snapshot_ready", 1, 0))?;
    let expected = expected_snapshot(*predecessor_latest, *mixed_air_latest);
    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER
            .len()
            != 3
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || mixed_air.system != binding.ideal_loads_air_system
        || !snapshots_match_exact_bits(latest, &expected)
        || !snapshots_match_exact_bits(
            latest,
            &latest_output
                .calculation_cooling_positive_supply_capacity_limit_cp_air_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    right: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> bool {
    let values_match =
        options_have_exact_bits(
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        ) && options_have_exact_bits(
            left.psychrometric_cp_air_result_j_per_kg_k,
            right.psychrometric_cp_air_result_j_per_kg_k,
        ) && options_have_exact_bits(left.cp_air_j_per_kg_k, right.cp_air_j_per_kg_k);
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.mixed_air_humidity_ratio = None;
    right_without_values.mixed_air_humidity_ratio = None;
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

fn checked_add(
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
    Error::CalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn source_site_count_overflow_fails_closed() {
        let error = checked_mul(usize::MAX, 3, "test_source_site_count_overflow", usize::MAX)
            .expect_err("source-site multiplication overflow must fail closed");

        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleInvariant { .. }
        ));
    }

    #[test]
    fn source_counter_corruption_is_rejected() {
        let mut state =
            crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_cp_air_assignment_count = 2;
        state.source_site_execution_count = 6;
        state.mixed_air_humidity_ratio_read_count = 2;
        state.psychrometric_cp_air_evaluation_count = 2;
        state.cp_air_assignment_write_count = 1;

        let error = ensure_count(
            state.cp_air_assignment_write_count,
            state.capacity_limit_cp_air_assignment_count,
            "cp_air_assignment_write_count",
        )
        .expect_err("self-inconsistent write history must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleInvariant {
                field: "cp_air_assignment_write_count",
                expected: 2,
                actual: 1,
            }
        ));
    }
}
