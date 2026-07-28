//! Release validation for the bounded post-capacity-limit humidity-ratio assignment.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-mixed-air-humidity-ratio",
    "assign-purchased-air-supply-humidity-ratio",
];

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;
    let mixed_air = output.calculation_cooling_mixed_air_call;
    let corroborating =
        output.calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment;
    let snapshot = output
        .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;

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
    ) && snapshot_shape(&snapshot, &predecessor, &mixed_air, &corroborating)
}

pub(super) fn validate_lifecycle(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    mixed_air_lifecycle: &PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    corroborating_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    positive_guard_lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    enthalpy_lifecycle: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    capacity_limit_guard_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let mixed_air = &mixed_air_lifecycle.state;
    let corroborating = &corroborating_lifecycle.state;
    let positive_guard = &positive_guard_lifecycle.state;
    let enthalpy = &enthalpy_lifecycle.state;
    let capacity_limit_guard = &capacity_limit_guard_lifecycle.state;

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
    let assignment_routes = checked_add(
        state.assignment_after_capacity_limit_guard_false_fallthrough_count,
        state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        "assignment_route_partition_overflow",
        timestep_count,
    )?;
    let assignment_routes = checked_add(
        assignment_routes,
        state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        "assignment_route_partition_overflow",
        timestep_count,
    )?;
    let capacity_body_routes = checked_add(
        state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        state
            .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        "capacity_body_route_partition_overflow",
        timestep_count,
    )?;
    let transition_partition = checked_add(
        skipped,
        state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let executions = state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count;
    let source_sites = checked_mul(
        executions,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
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
            "positive_guard_transition_count",
            positive_guard.transition_count,
            state.transition_count,
        ),
        (
            "enthalpy_transition_count",
            enthalpy.transition_count,
            state.transition_count,
        ),
        (
            "capacity_limit_guard_transition_count",
            capacity_limit_guard.transition_count,
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
            "assignment_after_capacity_limit_guard_false_fallthrough_count",
            predecessor.capacity_limit_guard_false_fallthrough_skip_count,
            state.assignment_after_capacity_limit_guard_false_fallthrough_count,
        ),
        (
            "assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count",
            predecessor.capacity_limit_sensible_output_guard_false_fallthrough_count,
            state.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
        ),
        (
            "assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count",
            predecessor
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
            state
                .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        ),
        (
            "assignment_route_partition",
            executions,
            assignment_routes,
        ),
        (
            "corroborating_supply_humidity_ratio_mixed_air_assignment_count",
            corroborating.supply_humidity_ratio_mixed_air_assignment_count,
            executions,
        ),
        (
            "positive_guard_positive_supply_mass_flow_body_entry_count",
            positive_guard.positive_supply_mass_flow_body_entry_count,
            executions,
        ),
        (
            "enthalpy_supply_enthalpy_assignment_count",
            enthalpy.supply_enthalpy_assignment_count,
            executions,
        ),
        (
            "capacity_limit_guard_evaluation_count",
            capacity_limit_guard.capacity_limit_guard_evaluation_count,
            executions,
        ),
        (
            "capacity_limit_guard_active_guard_false_fallthrough_count",
            capacity_limit_guard.active_guard_false_fallthrough_count,
            state.assignment_after_capacity_limit_guard_false_fallthrough_count,
        ),
        (
            "capacity_limit_guard_body_entry_count",
            capacity_limit_guard.capacity_limit_body_entry_count,
            capacity_body_routes,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "mixed_air_humidity_ratio_read_count",
            executions,
            state.mixed_air_humidity_ratio_read_count,
        ),
        (
            "supply_humidity_ratio_assignment_count",
            executions,
            state.supply_humidity_ratio_assignment_count,
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
    let positive_guard_latest = positive_guard
        .latest
        .as_ref()
        .ok_or_else(|| violation("positive_guard_latest_snapshot_ready", 1, 0))?;
    let enthalpy_latest = enthalpy
        .latest
        .as_ref()
        .ok_or_else(|| violation("enthalpy_latest_snapshot_ready", 1, 0))?;
    let capacity_limit_guard_latest = capacity_limit_guard
        .latest
        .as_ref()
        .ok_or_else(|| violation("capacity_limit_guard_latest_snapshot_ready", 1, 0))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || positive_guard_lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE
        || positive_guard_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE
        || enthalpy_lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || enthalpy_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || capacity_limit_guard_lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
        || capacity_limit_guard_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
            != EXPECTED_SOURCE_ORDER
        || ![
            state.system,
            predecessor.system,
            mixed_air.system,
            corroborating.system,
            positive_guard.system,
            enthalpy.system,
            capacity_limit_guard.system,
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
                identity(positive_guard_latest),
                identity(enthalpy_latest),
                identity(capacity_limit_guard_latest),
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
                .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    corroborating: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order != EXPECTED_SOURCE_ORDER
        || snapshot.system != predecessor.system
        || snapshot.parent_call_ordinal != predecessor.parent_call_ordinal
        || snapshot.controlled_zone != predecessor.controlled_zone
        || snapshot.unit_body_entered != predecessor.unit_body_entered
        || snapshot.predecessor_cooling_body_entered
            != predecessor.predecessor_cooling_body_entered
        || snapshot.predecessor_no_outdoor_air_fallback_entered
            != predecessor.predecessor_no_outdoor_air_fallback_entered
        || snapshot.predecessor_positive_supply_mass_flow_body_entered
            != predecessor.predecessor_positive_supply_mass_flow_body_entered
        || snapshot.predecessor_active_guard_false_fallthrough
            != predecessor.predecessor_active_guard_false_fallthrough
        || snapshot.predecessor_capacity_limit_guard_evaluated
            != predecessor.predecessor_capacity_limit_guard_evaluated
        || snapshot.predecessor_capacity_limit_body_entered
            != predecessor.predecessor_capacity_limit_body_entered
        || snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            != predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        || snapshot.predecessor_capacity_limit_cp_air_assignment_executed
            != predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        || snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
            != predecessor.predecessor_capacity_limit_sensible_output_assignment_executed
        || snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated
            != predecessor.predecessor_capacity_limit_sensible_output_guard_evaluated
        || snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            != predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        || snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered
            != predecessor.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        || snapshot
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
            != predecessor
                .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        || snapshot
            .predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
            != predecessor
                .predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        || snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
            != predecessor
                .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        || snapshot.unit_off_skipped != predecessor.unit_off_skipped
        || snapshot.non_cooling_skipped != predecessor.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_guard_false_fallthrough_skipped
            != predecessor.capacity_limit_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_sensible_output_guard_false_fallthrough
            != predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || snapshot.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            != predecessor
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        || !cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            *snapshot,
        )
    {
        return false;
    }

    let execution_route_count =
        usize::from(predecessor.capacity_limit_guard_false_fallthrough_skipped)
            + usize::from(predecessor.capacity_limit_sensible_output_guard_false_fallthrough)
            + usize::from(
                predecessor
                    .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            );
    let active_expected = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.positive_guard_false_fallthrough_skipped;
    if execution_route_count != usize::from(active_expected) {
        return false;
    }
    let execution = active_expected;
    if snapshot.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed != execution
    {
        return false;
    }
    if !execution {
        return skipped_source_shape(snapshot);
    }

    let Some(source) = mixed_air.mixed_air_humidity_ratio else {
        return false;
    };
    source.is_finite()
        && source >= 0.0
        && mixed_air.mixed_air_humidity_ratio_assigned
        && corroborating.supply_humidity_ratio_mixed_air_assignment_executed
        && corroborating.mixed_air_humidity_ratio_read
        && option_has_bits(corroborating.mixed_air_humidity_ratio, source)
        && corroborating.supply_humidity_ratio_assignment_performed
        && option_has_bits(corroborating.assigned_supply_humidity_ratio, source)
        && snapshot.mixed_air_humidity_ratio_read
        && option_has_bits(snapshot.mixed_air_humidity_ratio, source)
        && snapshot.supply_humidity_ratio_assignment_performed
        && option_has_bits(snapshot.assigned_supply_humidity_ratio, source)
}

fn skipped_source_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    !snapshot.mixed_air_humidity_ratio_read
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.supply_humidity_ratio_assignment_performed
        && snapshot.assigned_supply_humidity_ratio.is_none()
}

fn snapshots_match_exact_bits(
    left:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    right:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_equal(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && option_bits_equal(
        left.assigned_supply_humidity_ratio,
        right.assigned_supply_humidity_ratio,
    );
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.mixed_air_humidity_ratio = None;
    left_without_values.assigned_supply_humidity_ratio = None;
    right_without_values.mixed_air_humidity_ratio = None;
    right_without_values.assigned_supply_humidity_ratio = None;
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
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
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
    Error::CalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleInvariant {
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
        let error = checked_mul(usize::MAX, 2, "test_source_site_count_overflow", usize::MAX)
            .expect_err("source-site multiplication overflow must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleInvariant {
                ..
            }
        ));
    }

    #[test]
    fn exact_bits_distinguish_signed_zero() {
        assert!(option_has_bits(Some(-0.0), -0.0));
        assert!(!option_has_bits(Some(0.0), -0.0));
    }
}
