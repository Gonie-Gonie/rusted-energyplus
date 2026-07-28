//! Release validation for the bounded capacity-limit supply-temperature mixed-air limit.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};

use super::super::calc::cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;
    let mixed_air = output.calculation_cooling_mixed_air_call;
    let corroborating = output.calculation_cooling_positive_supply_temperature_mixed_air_limit;
    let snapshot = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;

    release_identities_match(
        binding.ideal_loads_air_system,
        binding.zone,
        call_ordinal,
        &[
            identity(&snapshot),
            identity(&predecessor),
            identity(&mixed_air),
            identity(&corroborating),
        ],
    ) && cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
        snapshot,
    ) && snapshot_shape(&snapshot, &predecessor, &mixed_air, &corroborating)
}

pub(super) fn validate_lifecycle(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary,
    mixed_air_lifecycle: &PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    corroborating_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let mixed_air = &mixed_air_lifecycle.state;
    let corroborating = &corroborating_lifecycle.state;

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
        state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        "active_partition_overflow",
        timestep_count,
    )?;
    let transition_partition = checked_add(
        inherited_skips,
        active_partition,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let predecessor_active_partition = checked_add(
        predecessor.capacity_limit_sensible_output_guard_false_fallthrough_count,
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_count,
        "predecessor_active_partition_overflow",
        timestep_count,
    )?;
    let executions = state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count;
    let source_sites = checked_mul(
        executions,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
            .len(),
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
            "corroborating_transition_count",
            corroborating.transition_count,
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
            "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count",
            predecessor.capacity_limit_sensible_output_supply_temperature_assignment_count,
            executions,
        ),
        (
            "active_partition",
            predecessor_active_partition,
            active_partition,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_temperature_for_minimum_read_count",
            executions,
            state.supply_temperature_for_minimum_read_count,
        ),
        (
            "mixed_air_temperature_for_minimum_read_count",
            executions,
            state.mixed_air_temperature_for_minimum_read_count,
        ),
        (
            "source_shaped_two_argument_minimum_evaluation_count",
            executions,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "supply_temperature_assignment_write_count",
            executions,
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
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let mixed_air_latest = mixed_air
        .latest
        .as_ref()
        .ok_or_else(|| violation("mixed_air_latest_snapshot_ready", 1, 0))?;
    let corroborating_latest = corroborating
        .latest
        .as_ref()
        .ok_or_else(|| violation("corroborating_latest_snapshot_ready", 1, 0))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || ![
            state.system,
            predecessor.system,
            mixed_air.system,
            corroborating.system,
        ]
        .into_iter()
        .all(|system| system == binding.ideal_loads_air_system)
        || !release_identities_match(
            binding.ideal_loads_air_system,
            binding.zone,
            timestep_count,
            &[
                identity(latest),
                identity(predecessor_latest),
                identity(mixed_air_latest),
                identity(corroborating_latest),
            ],
        )
        || !snapshot_shape(
            latest,
            predecessor_latest,
            mixed_air_latest,
            corroborating_latest,
        )
        || !snapshots_match_exact_bits(
            latest,
            &latest_output
                .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    corroborating: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order != EXPECTED_SOURCE_ORDER
        || !inherited_shape_matches(snapshot, predecessor)
    {
        return false;
    }

    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let execution =
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    if !guard_false && !execution {
        return !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
            && !snapshot
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            && complete_null_shape(snapshot);
    }
    if guard_false == execution
        || snapshot.capacity_limit_sensible_output_guard_false_fallthrough != guard_false
        || snapshot.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            != execution
    {
        return false;
    }

    let Some(preexisting) = predecessor.resulting_supply_temperature_c else {
        return false;
    };
    if !option_has_bits(snapshot.preexisting_supply_temperature_c, preexisting) {
        return false;
    }

    if guard_false {
        return skipped_rhs_is_null(snapshot)
            && option_has_bits(snapshot.resulting_supply_temperature_c, preexisting);
    }

    let Some(right) = mixed_air.mixed_air_temperature_c else {
        return false;
    };
    if !mixed_air.mixed_air_temperature_assigned
        || !right.is_finite()
        || !option_has_bits(corroborating.mixed_air_temperature_c, right)
    {
        return false;
    }
    let expected = if preexisting < right {
        preexisting
    } else {
        right
    };

    snapshot.supply_temperature_for_minimum_read
        && option_has_bits(
            snapshot.supply_temperature_before_mixed_air_limit_c,
            preexisting,
        )
        && snapshot.mixed_air_temperature_for_minimum_read
        && option_has_bits(snapshot.mixed_air_temperature_c, right)
        && snapshot.source_shaped_two_argument_minimum_evaluated
        && option_has_bits(snapshot.minimum_supply_temperature_c, expected)
        && snapshot.supply_temperature_assignment_performed
        && option_has_bits(snapshot.assigned_supply_temperature_c, expected)
        && option_has_bits(snapshot.resulting_supply_temperature_c, expected)
}

fn inherited_shape_matches(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
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
            == predecessor
                .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && snapshot.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
            == predecessor
                .predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
            == predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed
        && snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
}

fn complete_null_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    snapshot.preexisting_supply_temperature_c.is_none()
        && skipped_rhs_is_null(snapshot)
        && snapshot.resulting_supply_temperature_c.is_none()
}

fn skipped_rhs_is_null(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    !snapshot.supply_temperature_for_minimum_read
        && snapshot
            .supply_temperature_before_mixed_air_limit_c
            .is_none()
        && !snapshot.mixed_air_temperature_for_minimum_read
        && snapshot.mixed_air_temperature_c.is_none()
        && !snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot.minimum_supply_temperature_c.is_none()
        && !snapshot.supply_temperature_assignment_performed
        && snapshot.assigned_supply_temperature_c.is_none()
}

fn snapshots_match_exact_bits(
    left:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    right:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let values_match = [
        (
            left.preexisting_supply_temperature_c,
            right.preexisting_supply_temperature_c,
        ),
        (
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        (left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        (
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
        ),
        (
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
        (
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_equal(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    for value in [
        &mut left_without_values.preexisting_supply_temperature_c,
        &mut left_without_values.supply_temperature_before_mixed_air_limit_c,
        &mut left_without_values.mixed_air_temperature_c,
        &mut left_without_values.minimum_supply_temperature_c,
        &mut left_without_values.assigned_supply_temperature_c,
        &mut left_without_values.resulting_supply_temperature_c,
    ] {
        *value = None;
    }
    for value in [
        &mut right_without_values.preexisting_supply_temperature_c,
        &mut right_without_values.supply_temperature_before_mixed_air_limit_c,
        &mut right_without_values.mixed_air_temperature_c,
        &mut right_without_values.minimum_supply_temperature_c,
        &mut right_without_values.assigned_supply_temperature_c,
        &mut right_without_values.resulting_supply_temperature_c,
    ] {
        *value = None;
    }
    values_match && left_without_values == right_without_values
}

fn identity<T>(snapshot: &T) -> (ep_model::IdealLoadsAirSystemId, ep_model::ZoneId, usize)
where
    T: SnapshotIdentity,
{
    (
        snapshot.system(),
        snapshot.controlled_zone(),
        snapshot.parent_call_ordinal(),
    )
}

trait SnapshotIdentity {
    fn system(&self) -> ep_model::IdealLoadsAirSystemId;
    fn controlled_zone(&self) -> ep_model::ZoneId;
    fn parent_call_ordinal(&self) -> usize;
}

macro_rules! impl_snapshot_identity {
    ($($snapshot:ty),+ $(,)?) => {
        $(
            impl SnapshotIdentity for $snapshot {
                fn system(&self) -> ep_model::IdealLoadsAirSystemId { self.system }
                fn controlled_zone(&self) -> ep_model::ZoneId { self.controlled_zone }
                fn parent_call_ordinal(&self) -> usize { self.parent_call_ordinal }
            }
        )+
    };
}

impl_snapshot_identity!(
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
);

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
    Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleInvariant {
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
        let error = checked_mul(usize::MAX, 4, "test_source_site_count_overflow", usize::MAX)
            .expect_err("source-site multiplication overflow must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleInvariant {
                ..
            }
        ));
    }

    #[test]
    fn exact_bits_preserve_nan_and_distinguish_signed_zero() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0044);
        assert!(option_has_bits(Some(nan), nan));
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
