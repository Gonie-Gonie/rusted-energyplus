//! Release validation for the bounded capacity-limit sensible-output assignment.

use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

use super::super::calc::cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_positive_supply_capacity_limit_cp_air_assignment;
    let selector = output.calculation_cooling_positive_supply_capacity_limit_guard;
    let supply_flow = output.calculation_cooling_supply_mass_flow_positive_guard;
    let mixed_air = output.calculation_cooling_mixed_air_call;
    let supply_enthalpy = output.calculation_cooling_positive_supply_enthalpy_assignment;
    let snapshot =
        output.calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment;
    let Some(expected) = expected_snapshot(predecessor, supply_flow, mixed_air, supply_enthalpy)
    else {
        return false;
    };

    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && release_identities_match(
            binding.ideal_loads_air_system,
            binding.zone,
            call_ordinal,
            &[
                (
                    predecessor.system,
                    predecessor.controlled_zone,
                    predecessor.parent_call_ordinal,
                ),
                (
                    selector.system,
                    selector.controlled_zone,
                    selector.parent_call_ordinal,
                ),
                (
                    supply_flow.system,
                    supply_flow.controlled_zone,
                    supply_flow.parent_call_ordinal,
                ),
                (
                    mixed_air.system,
                    mixed_air.controlled_zone,
                    mixed_air.parent_call_ordinal,
                ),
                (
                    supply_enthalpy.system,
                    supply_enthalpy.controlled_zone,
                    supply_enthalpy.parent_call_ordinal,
                ),
            ],
        )
        && selector_matches_fixed_limit(selector, binding.system.cooling_limit)
        && predecessor.predecessor_capacity_limit_body_entered
            == selector.capacity_limit_body_entered
        && predecessor.predecessor_capacity_limit_guard_evaluated
            == selector.capacity_limit_guard_evaluated
        && predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
            == selector.active_guard_false_fallthrough
        && cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
        && snapshots_match_exact_bits(&snapshot, &expected)
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    supply_flow: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mixed_air: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    supply_enthalpy: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot> {
    let assignment_executed = predecessor.capacity_limit_cp_air_assignment_executed;
    let values = if assignment_executed {
        let supply_mass_flow_rate_kg_per_s = supply_flow.supply_mass_flow_rate_kg_per_s?;
        if !options_have_exact_bits(
            Some(supply_mass_flow_rate_kg_per_s),
            mixed_air.supply_mass_flow_rate_kg_per_s,
        ) || !options_have_exact_bits(
            Some(supply_mass_flow_rate_kg_per_s),
            mixed_air.child_supply_mass_flow_rate_kg_per_s,
        ) {
            return None;
        }
        let mixed_air_enthalpy_j_per_kg = mixed_air.mixed_air_enthalpy_projection_j_per_kg?;
        let supply_enthalpy_j_per_kg = supply_enthalpy.supply_enthalpy_j_per_kg?;
        let mixed_air_minus_supply_enthalpy_j_per_kg =
            mixed_air_enthalpy_j_per_kg - supply_enthalpy_j_per_kg;
        let cooling_sensible_output_w =
            supply_mass_flow_rate_kg_per_s * mixed_air_minus_supply_enthalpy_j_per_kg;
        Some((
            supply_mass_flow_rate_kg_per_s,
            mixed_air_enthalpy_j_per_kg,
            supply_enthalpy_j_per_kg,
            mixed_air_minus_supply_enthalpy_j_per_kg,
            cooling_sensible_output_w,
        ))
    } else {
        None
    };

    Some(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
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
            predecessor_capacity_limit_guard_evaluated: predecessor
                .predecessor_capacity_limit_guard_evaluated,
            predecessor_capacity_limit_body_entered: predecessor
                .predecessor_capacity_limit_body_entered,
            predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
                .predecessor_active_capacity_limit_guard_false_fallthrough,
            predecessor_capacity_limit_cp_air_assignment_executed: predecessor
                .capacity_limit_cp_air_assignment_executed,
            unit_off_skipped: predecessor.unit_off_skipped,
            non_cooling_skipped: predecessor.non_cooling_skipped,
            positive_guard_false_fallthrough_skipped: predecessor
                .positive_guard_false_fallthrough_skipped,
            capacity_limit_guard_false_fallthrough_skipped: predecessor
                .capacity_limit_guard_false_fallthrough_skipped,
            capacity_limit_sensible_output_assignment_executed: assignment_executed,
            supply_mass_flow_rate_read: assignment_executed,
            supply_mass_flow_rate_kg_per_s: values.map(|values| values.0),
            mixed_air_enthalpy_read: assignment_executed,
            mixed_air_enthalpy_j_per_kg: values.map(|values| values.1),
            supply_enthalpy_read: assignment_executed,
            supply_enthalpy_j_per_kg: values.map(|values| values.2),
            enthalpy_difference_calculated: assignment_executed,
            mixed_air_minus_supply_enthalpy_j_per_kg: values.map(|values| values.3),
            cooling_sensible_output_calculated: assignment_executed,
            calculated_cooling_sensible_output_w: values.map(|values| values.4),
            cooling_sensible_output_assigned: assignment_executed,
            cooling_sensible_output_w: values.map(|values| values.4),
        },
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    selector_lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    supply_flow_lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    mixed_air_lifecycle: &PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    supply_enthalpy_lifecycle: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let selector = &selector_lifecycle.state;
    let supply_flow = &supply_flow_lifecycle.state;
    let mixed_air = &mixed_air_lifecycle.state;
    let supply_enthalpy = &supply_enthalpy_lifecycle.state;
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
        state.capacity_limit_sensible_output_assignment_count,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let source_sites = checked_mul(
        state.capacity_limit_sensible_output_assignment_count,
        6,
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
            "selector_transition_count",
            selector.transition_count,
            state.transition_count,
        ),
        (
            "supply_flow_transition_count",
            supply_flow.transition_count,
            state.transition_count,
        ),
        (
            "mixed_air_transition_count",
            mixed_air.transition_count,
            state.transition_count,
        ),
        (
            "supply_enthalpy_transition_count",
            supply_enthalpy.transition_count,
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
            "capacity_limit_sensible_output_assignment_count",
            predecessor.capacity_limit_cp_air_assignment_count,
            state.capacity_limit_sensible_output_assignment_count,
        ),
        (
            "selector_capacity_limit_body_entry_count",
            selector.capacity_limit_body_entry_count,
            state.capacity_limit_sensible_output_assignment_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            state.capacity_limit_sensible_output_assignment_count,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "mixed_air_enthalpy_read_count",
            state.capacity_limit_sensible_output_assignment_count,
            state.mixed_air_enthalpy_read_count,
        ),
        (
            "supply_enthalpy_read_count",
            state.capacity_limit_sensible_output_assignment_count,
            state.supply_enthalpy_read_count,
        ),
        (
            "enthalpy_difference_calculation_count",
            state.capacity_limit_sensible_output_assignment_count,
            state.enthalpy_difference_calculation_count,
        ),
        (
            "cooling_sensible_output_calculation_count",
            state.capacity_limit_sensible_output_assignment_count,
            state.cooling_sensible_output_calculation_count,
        ),
        (
            "cooling_sensible_output_assignment_write_count",
            state.capacity_limit_sensible_output_assignment_count,
            state.cooling_sensible_output_assignment_write_count,
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
    let selector_latest = selector
        .latest
        .as_ref()
        .ok_or_else(|| violation("selector_latest_release_snapshot_ready", 1, 0))?;
    let supply_flow_latest = supply_flow
        .latest
        .as_ref()
        .ok_or_else(|| violation("supply_flow_latest_release_snapshot_ready", 1, 0))?;
    let mixed_air_latest = mixed_air
        .latest
        .as_ref()
        .ok_or_else(|| violation("mixed_air_latest_release_snapshot_ready", 1, 0))?;
    let supply_enthalpy_latest = supply_enthalpy
        .latest
        .as_ref()
        .ok_or_else(|| violation("supply_enthalpy_latest_release_snapshot_ready", 1, 0))?;
    let expected = expected_snapshot(
        *predecessor_latest,
        *supply_flow_latest,
        *mixed_air_latest,
        *supply_enthalpy_latest,
    )
    .ok_or_else(|| violation("independent_source_operands_ready", 1, 0))?;

    if lifecycle.source
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
            .len()
            != 6
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || selector.system != binding.ideal_loads_air_system
        || supply_flow.system != binding.ideal_loads_air_system
        || mixed_air.system != binding.ideal_loads_air_system
        || supply_enthalpy.system != binding.ideal_loads_air_system
        || predecessor_latest.predecessor_capacity_limit_body_entered
            != selector_latest.capacity_limit_body_entered
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
                    selector_latest.system,
                    selector_latest.controlled_zone,
                    selector_latest.parent_call_ordinal,
                ),
                (
                    supply_flow_latest.system,
                    supply_flow_latest.controlled_zone,
                    supply_flow_latest.parent_call_ordinal,
                ),
                (
                    mixed_air_latest.system,
                    mixed_air_latest.controlled_zone,
                    mixed_air_latest.parent_call_ordinal,
                ),
                (
                    supply_enthalpy_latest.system,
                    supply_enthalpy_latest.controlled_zone,
                    supply_enthalpy_latest.parent_call_ordinal,
                ),
            ],
        )
        || !selector_matches_fixed_limit(*selector_latest, binding.system.cooling_limit)
        || !snapshots_match_exact_bits(latest, &expected)
        || !snapshots_match_exact_bits(
            latest,
            &latest_output
                .calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    right: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    let values_match = [
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.mixed_air_enthalpy_j_per_kg,
            right.mixed_air_enthalpy_j_per_kg,
        ),
        (
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        ),
        (
            left.mixed_air_minus_supply_enthalpy_j_per_kg,
            right.mixed_air_minus_supply_enthalpy_j_per_kg,
        ),
        (
            left.calculated_cooling_sensible_output_w,
            right.calculated_cooling_sensible_output_w,
        ),
        (
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    left_without_values.supply_mass_flow_rate_kg_per_s = None;
    right_without_values.supply_mass_flow_rate_kg_per_s = None;
    left_without_values.mixed_air_enthalpy_j_per_kg = None;
    right_without_values.mixed_air_enthalpy_j_per_kg = None;
    left_without_values.supply_enthalpy_j_per_kg = None;
    right_without_values.supply_enthalpy_j_per_kg = None;
    left_without_values.mixed_air_minus_supply_enthalpy_j_per_kg = None;
    right_without_values.mixed_air_minus_supply_enthalpy_j_per_kg = None;
    left_without_values.calculated_cooling_sensible_output_w = None;
    right_without_values.calculated_cooling_sensible_output_w = None;
    left_without_values.cooling_sensible_output_w = None;
    right_without_values.cooling_sensible_output_w = None;
    values_match && left_without_values == right_without_values
}

fn options_have_exact_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
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

fn selector_matches_fixed_limit(
    selector: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    cooling_limit: IdealLoadsLimit,
) -> bool {
    fixed_selector_shape(
        cooling_limit,
        selector.capacity_limit_guard_evaluated,
        selector.first_cooling_limit,
        selector.cooling_limit_capacity,
        selector.second_cooling_limit,
        selector.cooling_limit_flow_rate_and_capacity,
        selector.capacity_limit_body_entered,
        selector.active_guard_false_fallthrough,
    )
}

#[allow(clippy::too_many_arguments)]
fn fixed_selector_shape(
    cooling_limit: IdealLoadsLimit,
    evaluated: bool,
    first_limit: Option<IdealLoadsLimit>,
    capacity_match: Option<bool>,
    second_limit: Option<IdealLoadsLimit>,
    combined_match: Option<bool>,
    body_entered: bool,
    false_fallthrough: bool,
) -> bool {
    if !evaluated {
        return first_limit.is_none()
            && capacity_match.is_none()
            && second_limit.is_none()
            && combined_match.is_none()
            && !body_entered
            && !false_fallthrough;
    }
    if first_limit != Some(cooling_limit) {
        return false;
    }
    match cooling_limit {
        IdealLoadsLimit::LimitCapacity => {
            capacity_match == Some(true)
                && second_limit.is_none()
                && combined_match.is_none()
                && body_entered
                && !false_fallthrough
        }
        IdealLoadsLimit::LimitFlowRateAndCapacity => {
            capacity_match == Some(false)
                && second_limit == Some(cooling_limit)
                && combined_match == Some(true)
                && body_entered
                && !false_fallthrough
        }
        IdealLoadsLimit::NoLimit | IdealLoadsLimit::LimitFlowRate => {
            capacity_match == Some(false)
                && second_limit == Some(cooling_limit)
                && combined_match == Some(false)
                && !body_entered
                && false_fallthrough
        }
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
    Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};

    use super::*;

    #[test]
    fn source_site_count_overflow_fails_closed() {
        let error = checked_mul(usize::MAX, 6, "test_source_site_count_overflow", usize::MAX)
            .expect_err("source-site multiplication overflow must fail closed");

        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleInvariant {
                ..
            }
        ));
    }

    #[test]
    fn source_counter_corruption_is_rejected() {
        let mut state =
            crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_sensible_output_assignment_count = 2;
        state.source_site_execution_count = 12;
        state.cooling_sensible_output_assignment_write_count = 1;

        let error = ensure_count(
            state.cooling_sensible_output_assignment_write_count,
            state.capacity_limit_sensible_output_assignment_count,
            "cooling_sensible_output_assignment_write_count",
        )
        .expect_err("self-inconsistent write history must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleInvariant {
                field: "cooling_sensible_output_assignment_write_count",
                expected: 2,
                actual: 1,
            }
        ));
    }

    #[test]
    fn same_call_identity_and_fixed_selector_forgeries_are_rejected() {
        let system = IdealLoadsAirSystemId(0);
        let zone = ZoneId(1);
        assert!(release_identities_match(
            system,
            zone,
            3,
            &[(system, zone, 3), (system, zone, 3)],
        ));
        assert!(!release_identities_match(
            system,
            zone,
            3,
            &[(system, zone, 3), (system, ZoneId(2), 3)],
        ));
        assert!(!release_identities_match(
            system,
            zone,
            3,
            &[(system, zone, 3), (system, zone, 2)],
        ));

        assert!(fixed_selector_shape(
            IdealLoadsLimit::LimitCapacity,
            true,
            Some(IdealLoadsLimit::LimitCapacity),
            Some(true),
            None,
            None,
            true,
            false,
        ));
        assert!(!fixed_selector_shape(
            IdealLoadsLimit::LimitCapacity,
            true,
            Some(IdealLoadsLimit::LimitFlowRateAndCapacity),
            Some(false),
            Some(IdealLoadsLimit::LimitFlowRateAndCapacity),
            Some(true),
            true,
            false,
        ));
    }
}
