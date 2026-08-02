//! Coupled-runtime validation for CP391 supply-enthalpy overdrying-limit evidence.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitLifecycleSummary as PredecessorLifecycle,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(in crate::ideal_loads) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit;
    let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && binding.system.dehumidification_control_type == DehumidificationControlType::None
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(snapshot)
        && links_to_predecessor(snapshot, predecessor)
        && snapshot.cp390_retained_supply_enthalpy_state_owned
            == predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        && option_bits_equal(
            snapshot.preexisting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
}

pub(in crate::ideal_loads) fn validate_lifecycle(
    lifecycle: &Lifecycle,
    predecessor_cp390: &PredecessorLifecycle,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_cp390.state;
    let assignments =
        state.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count;
    let inactive = state
        .transition_count
        .checked_sub(assignments)
        .ok_or_else(|| {
            violation(
                "inactive_transition_underflow",
                assignments,
                state.transition_count,
            )
        })?;
    let route_sum = checked_sum(
        &state.predecessor_route_counts,
        "predecessor_route_partition_overflow",
    )?;
    let expected_assignments = checked_sum(
        &[
            state.predecessor_route_counts[18],
            state.predecessor_route_counts[22],
            state.predecessor_route_counts[28],
        ],
        "active_route_count_overflow",
    )?;
    let expected_owners = checked_sum(
        &[
            state.predecessor_route_counts[5],
            state.predecessor_route_counts[8],
            state.predecessor_route_counts[11],
            state.predecessor_route_counts[14],
            state.predecessor_route_counts[17],
            checked_sum(
                &state.predecessor_route_counts[18..],
                "cp390_supply_enthalpy_state_owner_count_overflow",
            )?,
        ],
        "cp390_supply_enthalpy_state_owner_count_overflow",
    )?;
    let expected_unchanged = expected_owners.checked_sub(assignments).ok_or_else(|| {
        violation(
            "unchanged_supply_enthalpy_preservation_underflow",
            assignments,
            expected_owners,
        )
    })?;
    let expected_sites = assignments
        .checked_mul(PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", 0, usize::MAX))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE
        || predecessor_cp390.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE
        || predecessor_cp390.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || state.predecessor_route_counts != predecessor.predecessor_route_counts
        || binding.system.dehumidification_control_type != DehumidificationControlType::None
    {
        return Err(violation("source_owner_route_and_system_identity", 1, 0));
    }

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "predecessor_route_partition",
            state.transition_count,
            route_sum,
        ),
        (
            "inactive_transition_count",
            inactive,
            state.inactive_transition_count,
        ),
        (
            "active_route_count",
            expected_assignments,
            assignments,
        ),
        (
            "predecessor_supply_temperature_mixed_air_limit_count",
            predecessor.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count,
            assignments,
        ),
        ("direct_supply_enthalpy_overdrying_limit_count", 0, assignments),
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "cp390_supply_enthalpy_state_owner_count",
            expected_owners,
            state.cp390_supply_enthalpy_state_owner_count,
        ),
        (
            "unchanged_supply_enthalpy_preservation_count",
            expected_unchanged,
            state.unchanged_supply_enthalpy_preservation_count,
        ),
        (
            "supply_enthalpy_owned_read_count",
            assignments,
            state.supply_enthalpy_owned_read_count,
        ),
        (
            "supply_enthalpy_for_overdrying_limit_maximum_read_count",
            assignments,
            state.supply_enthalpy_for_overdrying_limit_maximum_read_count,
        ),
        (
            "supply_temperature_owned_read_count",
            assignments,
            state.supply_temperature_owned_read_count,
        ),
        (
            "supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count",
            assignments,
            state.supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count,
        ),
        (
            "psychrometric_minimum_supply_enthalpy_evaluation_count",
            assignments,
            state.psychrometric_minimum_supply_enthalpy_evaluation_count,
        ),
        (
            "source_shaped_two_argument_maximum_evaluation_count",
            assignments,
            state.source_shaped_two_argument_maximum_evaluation_count,
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
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    if !same_snapshot(
        latest,
        latest_output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit,
    ) || !links_to_predecessor(latest, predecessor_latest)
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

mod lineage;
use lineage::{links_to_predecessor, option_bits_equal, same_snapshot};
fn checked_sum(values: &[usize], field: &'static str) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation(field, 0, usize::MAX))
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
    Error::CalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
