//! Release validation for the bounded capacity-limit supply-enthalpy assignment.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
};

use super::super::calc::cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-retained-mixed-air-enthalpy-for-supply-enthalpy-difference",
    "read-retained-cooling-sensible-output-for-specific-cooling-output-division",
    "read-retained-supply-mass-flow-rate-for-specific-cooling-output-division",
    "calculate-cooling-sensible-output-divided-by-supply-mass-flow-rate",
    "calculate-mixed-air-enthalpy-minus-specific-cooling-output",
    "assign-local-supply-enthalpy",
];

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;
    let retained =
        output.calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment;
    let snapshot = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment;

    release_identities_match(
        binding.ideal_loads_air_system,
        binding.zone,
        call_ordinal,
        &[
            (
                snapshot.system,
                snapshot.controlled_zone,
                snapshot.parent_call_ordinal,
            ),
            (
                predecessor.system,
                predecessor.controlled_zone,
                predecessor.parent_call_ordinal,
            ),
            (
                retained.system,
                retained.controlled_zone,
                retained.parent_call_ordinal,
            ),
        ],
    ) && cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
        snapshot,
    ) && snapshot_shape(&snapshot, &predecessor, &retained)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn validate_lifecycle(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
    guard_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    retained_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let guard = &guard_lifecycle.state;
    let retained = &retained_lifecycle.state;

    let inherited_skips = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let inherited_skips = checked_add(
        inherited_skips,
        state.positive_guard_false_fallthrough_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let inherited_skips = checked_add(
        inherited_skips,
        state.capacity_limit_guard_false_fallthrough_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let active_partition = checked_add(
        state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        state.capacity_limit_sensible_output_supply_enthalpy_assignment_count,
        "active_partition_overflow",
        guard.capacity_limit_sensible_output_guard_evaluation_count,
    )?;
    let transition_partition = checked_add(
        inherited_skips,
        active_partition,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let source_sites = checked_mul(
        state.capacity_limit_sensible_output_supply_enthalpy_assignment_count,
        6,
        "source_site_execution_count_overflow",
        state.source_site_execution_count,
    )?;
    let assignments = state.capacity_limit_sensible_output_supply_enthalpy_assignment_count;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "guard_transition_count",
            guard.transition_count,
            state.transition_count,
        ),
        (
            "retained_transition_count",
            retained.transition_count,
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
            predecessor.capacity_limit_guard_false_fallthrough_skip_count,
            state.capacity_limit_guard_false_fallthrough_skip_count,
        ),
        (
            "capacity_limit_sensible_output_guard_false_fallthrough_count",
            predecessor.capacity_limit_sensible_output_guard_false_fallthrough_count,
            state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        ),
        (
            "capacity_limit_sensible_output_supply_enthalpy_assignment_count",
            predecessor.capacity_limit_sensible_output_maximum_capacity_assignment_count,
            assignments,
        ),
        (
            "active_partition",
            guard.capacity_limit_sensible_output_guard_evaluation_count,
            active_partition,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "mixed_air_enthalpy_read_count",
            assignments,
            state.mixed_air_enthalpy_read_count,
        ),
        (
            "cooling_sensible_output_read_count",
            assignments,
            state.cooling_sensible_output_read_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            assignments,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "specific_cooling_output_calculation_count",
            assignments,
            state.specific_cooling_output_calculation_count,
        ),
        (
            "supply_enthalpy_calculation_count",
            assignments,
            state.supply_enthalpy_calculation_count,
        ),
        (
            "supply_enthalpy_assignment_write_count",
            assignments,
            state.supply_enthalpy_assignment_write_count,
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
    let retained_latest = retained
        .latest
        .as_ref()
        .ok_or_else(|| violation("retained_latest_release_snapshot_ready", 1, 0))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || guard.system != binding.ideal_loads_air_system
        || retained.system != binding.ideal_loads_air_system
        || !release_identities_match(
            binding.ideal_loads_air_system,
            binding.zone,
            timestep_count,
            &[
                (
                    latest.system,
                    latest.controlled_zone,
                    latest.parent_call_ordinal,
                ),
                (
                    predecessor_latest.system,
                    predecessor_latest.controlled_zone,
                    predecessor_latest.parent_call_ordinal,
                ),
                (
                    retained_latest.system,
                    retained_latest.controlled_zone,
                    retained_latest.parent_call_ordinal,
                ),
            ],
        )
        || !snapshot_shape(latest, predecessor_latest, retained_latest)
        || !snapshots_match_exact_bits(
            latest,
            &latest_output
                .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    retained: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order != EXPECTED_SOURCE_ORDER
        || !inherited_shape_matches(snapshot, predecessor)
    {
        return false;
    }

    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment =
        predecessor.capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    if !guard_false && !assignment {
        return !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
            && !snapshot.capacity_limit_sensible_output_supply_enthalpy_assignment_executed
            && complete_null_shape(snapshot);
    }
    if guard_false == assignment
        || snapshot.capacity_limit_sensible_output_guard_false_fallthrough != guard_false
        || snapshot.capacity_limit_sensible_output_supply_enthalpy_assignment_executed != assignment
    {
        return false;
    }

    let Some(preexisting_supply_enthalpy) = retained.supply_enthalpy_j_per_kg else {
        return false;
    };
    if !option_has_bits(
        snapshot.preexisting_supply_enthalpy_j_per_kg,
        preexisting_supply_enthalpy,
    ) {
        return false;
    }

    if guard_false {
        return !snapshot.mixed_air_enthalpy_read
            && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
            && !snapshot.cooling_sensible_output_read
            && snapshot.cooling_sensible_output_w.is_none()
            && !snapshot.supply_mass_flow_rate_read
            && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
            && !snapshot.specific_cooling_output_calculated
            && snapshot.specific_cooling_output_j_per_kg.is_none()
            && !snapshot.supply_enthalpy_calculated
            && snapshot.calculated_supply_enthalpy_j_per_kg.is_none()
            && !snapshot.supply_enthalpy_assigned
            && snapshot.assigned_supply_enthalpy_j_per_kg.is_none()
            && option_has_bits(
                snapshot.resulting_supply_enthalpy_j_per_kg,
                preexisting_supply_enthalpy,
            );
    }

    let (Some(mixed_air), Some(cooling_sensible_output), Some(supply_mass_flow)) = (
        retained.mixed_air_enthalpy_j_per_kg,
        predecessor.resulting_cooling_sensible_output_w,
        retained.supply_mass_flow_rate_kg_per_s,
    ) else {
        return false;
    };
    if !mixed_air.is_finite()
        || !cooling_sensible_output.is_finite()
        || cooling_sensible_output <= 0.0
        || supply_mass_flow.is_nan()
        || supply_mass_flow <= 0.0
    {
        return false;
    }
    let specific_cooling_output = cooling_sensible_output / supply_mass_flow;
    let calculated_supply_enthalpy = mixed_air - specific_cooling_output;

    snapshot.mixed_air_enthalpy_read
        && option_has_bits(snapshot.mixed_air_enthalpy_j_per_kg, mixed_air)
        && snapshot.cooling_sensible_output_read
        && option_has_bits(snapshot.cooling_sensible_output_w, cooling_sensible_output)
        && snapshot.supply_mass_flow_rate_read
        && option_has_bits(snapshot.supply_mass_flow_rate_kg_per_s, supply_mass_flow)
        && snapshot.specific_cooling_output_calculated
        && option_has_bits(
            snapshot.specific_cooling_output_j_per_kg,
            specific_cooling_output,
        )
        && snapshot.supply_enthalpy_calculated
        && option_has_bits(
            snapshot.calculated_supply_enthalpy_j_per_kg,
            calculated_supply_enthalpy,
        )
        && snapshot.supply_enthalpy_assigned
        && option_has_bits(
            snapshot.assigned_supply_enthalpy_j_per_kg,
            calculated_supply_enthalpy,
        )
        && option_has_bits(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            calculated_supply_enthalpy,
        )
}

fn inherited_shape_matches(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> bool {
    snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.unit_body_entered == predecessor.unit_body_entered
        && snapshot.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && snapshot.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && snapshot.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_guard_evaluated
            == predecessor.predecessor_capacity_limit_guard_evaluated
        && snapshot.predecessor_capacity_limit_body_entered
            == predecessor.predecessor_capacity_limit_body_entered
        && snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_cp_air_assignment_executed
            == predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
            == predecessor.predecessor_capacity_limit_sensible_output_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
            == predecessor.predecessor_capacity_limit_sensible_output_guard_evaluated
        && snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
            == predecessor.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && snapshot.predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
            == predecessor.capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
}

fn complete_null_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    snapshot.preexisting_supply_enthalpy_j_per_kg.is_none()
        && !snapshot.mixed_air_enthalpy_read
        && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
        && !snapshot.cooling_sensible_output_read
        && snapshot.cooling_sensible_output_w.is_none()
        && !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.specific_cooling_output_calculated
        && snapshot.specific_cooling_output_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_calculated
        && snapshot.calculated_supply_enthalpy_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_assigned
        && snapshot.assigned_supply_enthalpy_j_per_kg.is_none()
        && snapshot.resulting_supply_enthalpy_j_per_kg.is_none()
}

fn snapshots_match_exact_bits(
    left:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    right:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let values_match = [
        (
            left.preexisting_supply_enthalpy_j_per_kg,
            right.preexisting_supply_enthalpy_j_per_kg,
        ),
        (
            left.mixed_air_enthalpy_j_per_kg,
            right.mixed_air_enthalpy_j_per_kg,
        ),
        (
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.specific_cooling_output_j_per_kg,
            right.specific_cooling_output_j_per_kg,
        ),
        (
            left.calculated_supply_enthalpy_j_per_kg,
            right.calculated_supply_enthalpy_j_per_kg,
        ),
        (
            left.assigned_supply_enthalpy_j_per_kg,
            right.assigned_supply_enthalpy_j_per_kg,
        ),
        (
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_equal(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    for value in [
        &mut left_without_values.preexisting_supply_enthalpy_j_per_kg,
        &mut left_without_values.mixed_air_enthalpy_j_per_kg,
        &mut left_without_values.cooling_sensible_output_w,
        &mut left_without_values.supply_mass_flow_rate_kg_per_s,
        &mut left_without_values.specific_cooling_output_j_per_kg,
        &mut left_without_values.calculated_supply_enthalpy_j_per_kg,
        &mut left_without_values.assigned_supply_enthalpy_j_per_kg,
        &mut left_without_values.resulting_supply_enthalpy_j_per_kg,
    ] {
        *value = None;
    }
    for value in [
        &mut right_without_values.preexisting_supply_enthalpy_j_per_kg,
        &mut right_without_values.mixed_air_enthalpy_j_per_kg,
        &mut right_without_values.cooling_sensible_output_w,
        &mut right_without_values.supply_mass_flow_rate_kg_per_s,
        &mut right_without_values.specific_cooling_output_j_per_kg,
        &mut right_without_values.calculated_supply_enthalpy_j_per_kg,
        &mut right_without_values.assigned_supply_enthalpy_j_per_kg,
        &mut right_without_values.resulting_supply_enthalpy_j_per_kg,
    ] {
        *value = None;
    }
    values_match && left_without_values == right_without_values
}

fn release_identities_match(
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    expected_ordinal: usize,
    identities: &[(ep_model::IdealLoadsAirSystemId, ep_model::ZoneId, usize)],
) -> bool {
    identities.iter().all(|(system, zone, ordinal)| {
        *system == expected_system && *zone == expected_zone && *ordinal == expected_ordinal
    })
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
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
    Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_site_count_overflow_fails_closed() {
        let error = checked_mul(usize::MAX, 6, "test_source_site_count_overflow", usize::MAX)
            .expect_err("source-site multiplication overflow must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleInvariant {
                ..
            }
        ));
    }

    #[test]
    fn exact_bits_preserve_nan_and_distinguish_signed_zero() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0042);
        assert!(option_has_bits(Some(nan), nan));
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
