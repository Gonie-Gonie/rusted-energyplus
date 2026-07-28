//! Release validation for the bounded post-capacity-limit control switch.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
    PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-dehumidification-control-type",
    "dispatch-dehumidification-control-switch",
];

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output
        .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
    let corroborating = output.calculation_cooling_dehumidification_flow;
    let snapshot = output
        .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;

    identities_match(
        binding,
        call_ordinal,
        &[
            identity(&snapshot),
            identity(&predecessor),
            identity(&corroborating),
        ],
    ) && snapshot_shape(
        &snapshot,
        &predecessor,
        &corroborating,
        binding.system.dehumidification_control_type,
    )
}

pub(super) fn validate_lifecycle(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    corroborating_lifecycle: &PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let corroborating = &corroborating_lifecycle.state;
    let switch_count = state.dehumidification_control_switch_count;
    let transition_partition = checked_sum(
        &[
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            switch_count,
        ],
        "transition_partition_overflow",
    )?;
    let case_partition = checked_sum(
        &[
            state.dehumidification_control_none_case_selection_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
            state.dehumidification_control_humidistat_case_selection_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        ],
        "case_partition_overflow",
    )?;
    let predecessor_routes = checked_sum(
        &[
            predecessor.assignment_after_capacity_limit_guard_false_fallthrough_count,
            predecessor
                .assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,
            predecessor
                .assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count,
        ],
        "predecessor_route_partition_overflow",
    )?;
    let source_sites = switch_count
        .checked_mul(EXPECTED_SOURCE_ORDER.len())
        .ok_or_else(|| {
            violation(
                "source_site_execution_count_overflow",
                usize::MAX,
                switch_count,
            )
        })?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
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
            "predecessor_assignment_count",
            predecessor.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,
            switch_count,
        ),
        (
            "predecessor_route_partition",
            switch_count,
            predecessor_routes,
        ),
        ("case_partition", switch_count, case_partition),
        (
            "direct_none_case_selection_count",
            switch_count,
            state.dehumidification_control_none_case_selection_count,
        ),
        (
            "direct_constant_sensible_heat_ratio_case_selection_count",
            0,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        ),
        (
            "direct_humidistat_case_selection_count",
            0,
            state.dehumidification_control_humidistat_case_selection_count,
        ),
        (
            "direct_constant_supply_humidity_ratio_case_selection_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "dehumidification_control_type_read_count",
            switch_count,
            state.dehumidification_control_type_read_count,
        ),
        (
            "dehumidification_control_switch_dispatch_count",
            switch_count,
            state.dehumidification_control_switch_dispatch_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    if binding.system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(violation("direct_binding_selector_is_none", 1, 0));
    }
    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    let corroborating_latest = corroborating
        .latest
        .as_ref()
        .ok_or_else(|| violation("corroborating_latest_release_snapshot_ready", 1, 0))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || !identities_match(
            binding,
            timestep_count,
            &[
                identity(latest),
                identity(predecessor_latest),
                identity(corroborating_latest),
            ],
        )
        || !snapshot_shape(
            latest,
            predecessor_latest,
            corroborating_latest,
            binding.system.dehumidification_control_type,
        )
        || !snapshots_match_exact_bits(
            latest,
            &latest_output
                .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    corroborating: &PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    selected_control: DehumidificationControlType,
) -> bool {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE
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
        || snapshot.unit_off_skipped != predecessor.unit_off_skipped
        || snapshot.non_cooling_skipped != predecessor.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
        || snapshot.predecessor_capacity_limit_guard_false_fallthrough
            != predecessor.capacity_limit_guard_false_fallthrough_skipped
        || snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            != predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            != predecessor
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        || snapshot
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
            != predecessor
                .post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        || !option_bits_equal(
            snapshot.predecessor_assigned_supply_humidity_ratio,
            predecessor.assigned_supply_humidity_ratio,
        )
        || !cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(
            *snapshot,
        )
    {
        return false;
    }

    let active =
        predecessor.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed;
    let predecessor_route_count =
        usize::from(predecessor.capacity_limit_guard_false_fallthrough_skipped)
            + usize::from(predecessor.capacity_limit_sensible_output_guard_false_fallthrough)
            + usize::from(
                predecessor
                    .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed,
            );
    if predecessor_route_count != usize::from(active) {
        return false;
    }
    if !active {
        return !snapshot.dehumidification_control_type_read
            && snapshot.dehumidification_control_type.is_none()
            && !snapshot.dehumidification_control_switch_dispatched;
    }

    snapshot.dehumidification_control_type_read
        && snapshot.dehumidification_control_type == Some(selected_control)
        && snapshot.dehumidification_control_switch_dispatched
        && corroborating.dehumidification_control_type_read
        && corroborating.dehumidification_control_type == Some(selected_control)
}

fn snapshots_match_exact_bits(
    left:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    right:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
) -> bool {
    let values_match = option_bits_equal(
        left.predecessor_assigned_supply_humidity_ratio,
        right.predecessor_assigned_supply_humidity_ratio,
    );
    let mut left_without_value = *left;
    let mut right_without_value = *right;
    left_without_value.predecessor_assigned_supply_humidity_ratio = None;
    right_without_value.predecessor_assigned_supply_humidity_ratio = None;
    values_match && left_without_value == right_without_value
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
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
);

fn identities_match(
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    expected_ordinal: usize,
    identities: &[(ep_model::IdealLoadsAirSystemId, ep_model::ZoneId, usize)],
) -> bool {
    identities.iter().all(|(system, zone, ordinal)| {
        *system == binding.ideal_loads_air_system
            && *zone == binding.zone
            && *ordinal == expected_ordinal
    })
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, usize::MAX, sum))
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
    Error::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_partition_overflow_fails_closed() {
        let error = checked_sum(&[usize::MAX, 1], "test_case_partition_overflow")
            .expect_err("case partition overflow must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleInvariant {
                ..
            }
        ));
    }

    #[test]
    fn exact_bits_distinguish_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(0.0), Some(-0.0)));
    }
}
