//! Release validation for the bounded dehumidification-control `None` case.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let snapshot = output
        .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
    let predecessor = output
        .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;
    let mixed_air_owner = output.calculation_cooling_mixed_air_call;

    identities_match(
        binding,
        call_ordinal,
        &[
            identity(
                snapshot.system,
                snapshot.controlled_zone,
                snapshot.parent_call_ordinal,
            ),
            identity(
                predecessor.system,
                predecessor.controlled_zone,
                predecessor.parent_call_ordinal,
            ),
            identity(
                mixed_air_owner.system,
                mixed_air_owner.controlled_zone,
                mixed_air_owner.parent_call_ordinal,
            ),
        ],
    ) && snapshot_shape(&snapshot, &predecessor, &mixed_air_owner)
}

pub(super) fn validate_lifecycle(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    mixed_air_lifecycle: &PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let mixed_air = &mixed_air_lifecycle.state;
    let completed = state.dehumidification_control_none_case_completion_count;
    let transition_partition = checked_sum(
        &[
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            completed,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
            state.dehumidification_control_humidistat_case_selection_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        ],
        "transition_partition_overflow",
    )?;
    let source_sites = completed
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| {
            violation(
                "source_site_execution_count_overflow",
                usize::MAX,
                completed,
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
            "none_case_completion_count",
            predecessor.dehumidification_control_none_case_selection_count,
            completed,
        ),
        (
            "direct_none_case_completion_count",
            predecessor.dehumidification_control_switch_count,
            completed,
        ),
        (
            "constant_sensible_heat_ratio_case_selection_count",
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
            state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        ),
        (
            "humidistat_case_selection_count",
            predecessor.dehumidification_control_humidistat_case_selection_count,
            state.dehumidification_control_humidistat_case_selection_count,
        ),
        (
            "constant_supply_humidity_ratio_case_selection_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
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
            "none_case_entry_count",
            completed,
            state.dehumidification_control_none_case_entry_count,
        ),
        (
            "mixed_air_humidity_ratio_read_count",
            completed,
            state.mixed_air_humidity_ratio_read_count,
        ),
        (
            "supply_humidity_ratio_assignment_count",
            completed,
            state.supply_humidity_ratio_assignment_count,
        ),
        (
            "none_case_break_count",
            completed,
            state.dehumidification_control_none_case_break_count,
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
    let mixed_air_latest = mixed_air
        .latest
        .as_ref()
        .ok_or_else(|| violation("mixed_air_latest_release_snapshot_ready", 1, 0))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || !identities_match(
            binding,
            timestep_count,
            &[
                identity(latest.system, latest.controlled_zone, latest.parent_call_ordinal),
                identity(
                    predecessor_latest.system,
                    predecessor_latest.controlled_zone,
                    predecessor_latest.parent_call_ordinal,
                ),
                identity(
                    mixed_air_latest.system,
                    mixed_air_latest.controlled_zone,
                    mixed_air_latest.parent_call_ordinal,
                ),
            ],
        )
        || !snapshot_shape(latest, predecessor_latest, mixed_air_latest)
        || !snapshots_match_exact_bits(
            latest,
            &latest_output
                .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    mixed_air_owner: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER
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
            != predecessor.predecessor_capacity_limit_guard_false_fallthrough
        || snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            != predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        || snapshot
            .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            != predecessor
                .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        || snapshot
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
            != predecessor
                .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        || !option_bits_equal(
            snapshot.predecessor_assigned_supply_humidity_ratio,
            predecessor.predecessor_assigned_supply_humidity_ratio,
        )
        || snapshot.predecessor_dehumidification_control_type_read
            != predecessor.dehumidification_control_type_read
        || snapshot.predecessor_dehumidification_control_type
            != predecessor.dehumidification_control_type
        || snapshot.predecessor_dehumidification_control_switch_dispatched
            != predecessor.dehumidification_control_switch_dispatched
        || !cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release(
            *snapshot,
        )
    {
        return false;
    }

    let active = predecessor.dehumidification_control_switch_dispatched
        && predecessor.dehumidification_control_type == Some(DehumidificationControlType::None);
    if !active {
        return !snapshot.dehumidification_control_none_case_entered
            && !snapshot.mixed_air_humidity_ratio_read
            && snapshot.mixed_air_humidity_ratio.is_none()
            && !snapshot.supply_humidity_ratio_assignment_performed
            && snapshot.assigned_supply_humidity_ratio.is_none()
            && snapshot.resulting_supply_humidity_ratio.is_none()
            && !snapshot.dehumidification_control_none_case_exited_via_break;
    }

    snapshot.dehumidification_control_none_case_entered
        && snapshot.mixed_air_humidity_ratio_read
        && snapshot.supply_humidity_ratio_assignment_performed
        && snapshot.dehumidification_control_none_case_exited_via_break
        && mixed_air_owner.mixed_air_humidity_ratio_assigned
        && option_bits_equal(
            snapshot.predecessor_assigned_supply_humidity_ratio,
            mixed_air_owner.mixed_air_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.mixed_air_humidity_ratio,
            mixed_air_owner.mixed_air_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.assigned_supply_humidity_ratio,
            mixed_air_owner.mixed_air_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.resulting_supply_humidity_ratio,
            mixed_air_owner.mixed_air_humidity_ratio,
        )
}

fn snapshots_match_exact_bits(
    left:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    right:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_assigned_supply_humidity_ratio,
            right.predecessor_assigned_supply_humidity_ratio,
        ),
        (
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        ),
        (
            left.assigned_supply_humidity_ratio,
            right.assigned_supply_humidity_ratio,
        ),
        (
            left.resulting_supply_humidity_ratio,
            right.resulting_supply_humidity_ratio,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_equal(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.predecessor_assigned_supply_humidity_ratio = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
        snapshot.resulting_supply_humidity_ratio = None;
    }
    values_match && left_without_values == right_without_values
}

fn identity(
    system: ep_model::IdealLoadsAirSystemId,
    zone: ep_model::ZoneId,
    ordinal: usize,
) -> (ep_model::IdealLoadsAirSystemId, ep_model::ZoneId, usize) {
    (system, zone, ordinal)
}

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
    Error::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_partition_overflow_fails_closed() {
        let error = checked_sum(&[usize::MAX, 1], "test_route_partition_overflow")
            .expect_err("route partition overflow must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleInvariant {
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
