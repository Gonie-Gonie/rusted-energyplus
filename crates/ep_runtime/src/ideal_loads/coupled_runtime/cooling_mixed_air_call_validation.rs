//! Release validation for the bounded Cooling mixed-air call and no-OA child route.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    moist_air_enthalpy_j_per_kg,
};

use super::super::calc::cooling_mixed_air_call_snapshot_is_exact_direct_release;
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_supply_mass_flow_very_small_guard_body;
    let snapshot = output.calculation_cooling_mixed_air_call;

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && snapshot.unit_body_entered == predecessor.unit_body_entered
        && snapshot.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && snapshot.predecessor_zero_flow_reset_body_entered
            == predecessor.zero_flow_reset_body_entered
        && snapshot.predecessor_active_guard_false_fallthrough
            == predecessor.active_guard_false_fallthrough
        && snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.cooling_call_executed == predecessor.cooling_body_entered
        && cooling_mixed_air_call_snapshot_is_exact_direct_release(snapshot)
        && active_projection_matches(
            snapshot,
            predecessor.resulting_supply_mass_flow_rate_kg_per_s,
            binding.return_node,
        )
}

fn active_projection_matches(
    snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    predecessor_supply_mass_flow_rate_kg_per_s: Option<f64>,
    recirculation_node: ep_model::NodeId,
) -> bool {
    if !snapshot.cooling_call_executed {
        return predecessor_supply_mass_flow_rate_kg_per_s.is_none()
            && snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none()
            && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
            && snapshot.recirculation_node.is_none()
            && snapshot.mixed_air_temperature_c.is_none()
            && snapshot.mixed_air_humidity_ratio.is_none()
            && snapshot.mixed_air_enthalpy_projection_j_per_kg.is_none()
            && snapshot.heat_recovery_sensible_output_w.is_none()
            && snapshot.heat_recovery_latent_output_w.is_none();
    }

    let Some(recirculation_temperature_c) = snapshot.recirculation_temperature_c else {
        return false;
    };
    let Some(recirculation_humidity_ratio) = snapshot.recirculation_humidity_ratio else {
        return false;
    };
    let recirculation_enthalpy_j_per_kg =
        moist_air_enthalpy_j_per_kg(recirculation_temperature_c, recirculation_humidity_ratio);

    option_has_bits(snapshot.outdoor_air_mass_flow_rate_kg_per_s, Some(0.0))
        && options_have_exact_bits(
            snapshot.supply_mass_flow_rate_kg_per_s,
            predecessor_supply_mass_flow_rate_kg_per_s,
        )
        && snapshot.recirculation_node == Some(recirculation_node)
        && option_has_bits(
            snapshot.recirculation_temperature_c,
            Some(recirculation_temperature_c),
        )
        && option_has_bits(
            snapshot.recirculation_humidity_ratio,
            Some(recirculation_humidity_ratio),
        )
        && option_has_bits(
            snapshot.recirculation_enthalpy_projection_j_per_kg,
            Some(recirculation_enthalpy_j_per_kg),
        )
        && options_have_exact_bits(
            snapshot.child_supply_mass_flow_rate_kg_per_s,
            predecessor_supply_mass_flow_rate_kg_per_s,
        )
        && options_have_exact_bits(
            snapshot.resulting_recirculation_mass_flow_rate_kg_per_s,
            predecessor_supply_mass_flow_rate_kg_per_s,
        )
        && option_has_bits(
            snapshot.mixed_air_temperature_c,
            Some(recirculation_temperature_c),
        )
        && option_has_bits(
            snapshot.mixed_air_humidity_ratio,
            Some(recirculation_humidity_ratio),
        )
        && option_has_bits(
            snapshot.mixed_air_enthalpy_projection_j_per_kg,
            Some(recirculation_enthalpy_j_per_kg),
        )
        && option_has_bits(snapshot.heat_recovery_sensible_output_w, Some(0.0))
        && option_has_bits(snapshot.heat_recovery_latent_output_w, Some(0.0))
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skip_count = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let transition_partition = checked_add(
        skip_count,
        state.cooling_call_count,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let predecessor_active_partition = checked_add(
        predecessor.zero_flow_reset_body_entry_count,
        predecessor.active_guard_false_fallthrough_count,
        "predecessor_active_partition_overflow",
        state.cooling_call_count,
    )?;
    let caller_sites = checked_mul(
        state.cooling_call_count,
        PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER.len(),
        "caller_source_site_execution_count_overflow",
        state.caller_source_site_execution_count,
    )?;
    let child_sites = checked_mul(
        state.cooling_call_count,
        PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER.len(),
        "child_source_site_execution_count_overflow",
        state.child_source_site_execution_count,
    )?;
    let output_reference_binds = checked_mul(
        state.cooling_call_count,
        3,
        "mixed_air_output_reference_bind_count_overflow",
        state.mixed_air_output_reference_bind_count,
    )?;
    let output_assignments = checked_mul(
        state.cooling_call_count,
        3,
        "mixed_air_output_assignment_count_overflow",
        state.mixed_air_output_assignment_count,
    )?;
    let recovery_assignments = checked_mul(
        state.cooling_call_count,
        2,
        "heat_recovery_output_positive_zero_assignment_count_overflow",
        state.heat_recovery_output_positive_zero_assignment_count,
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
            "cooling_call_count",
            predecessor.cooling_body_entry_count,
            state.cooling_call_count,
        ),
        (
            "predecessor_active_partition",
            state.cooling_call_count,
            predecessor_active_partition,
        ),
        (
            "caller_source_site_execution_count",
            caller_sites,
            state.caller_source_site_execution_count,
        ),
        (
            "child_source_site_execution_count",
            child_sites,
            state.child_source_site_execution_count,
        ),
        (
            "state_reference_bind_count",
            state.cooling_call_count,
            state.state_reference_bind_count,
        ),
        (
            "purchased_air_number_read_count",
            state.cooling_call_count,
            state.purchased_air_number_read_count,
        ),
        (
            "outdoor_air_mass_flow_rate_read_count",
            state.cooling_call_count,
            state.outdoor_air_mass_flow_rate_read_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            state.cooling_call_count,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "mixed_air_output_reference_bind_count",
            output_reference_binds,
            state.mixed_air_output_reference_bind_count,
        ),
        (
            "operating_mode_read_count",
            state.cooling_call_count,
            state.operating_mode_read_count,
        ),
        (
            "mixed_air_child_call_count",
            state.cooling_call_count,
            state.mixed_air_child_call_count,
        ),
        (
            "no_outdoor_air_fallback_count",
            state.cooling_call_count,
            state.no_outdoor_air_fallback_count,
        ),
        (
            "recirculation_enthalpy_projection_count",
            state.cooling_call_count,
            state.recirculation_enthalpy_projection_count,
        ),
        (
            "mixed_air_output_assignment_count",
            output_assignments,
            state.mixed_air_output_assignment_count,
        ),
        (
            "heat_recovery_output_positive_zero_assignment_count",
            recovery_assignments,
            state.heat_recovery_output_positive_zero_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        || lifecycle.child_source != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || !snapshots_match_exact_bits(latest, &latest_output.calculation_cooling_mixed_air_call)
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn snapshots_match_exact_bits(
    left: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    right: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let values_match = [
        (
            left.outdoor_air_mass_flow_rate_kg_per_s,
            right.outdoor_air_mass_flow_rate_kg_per_s,
        ),
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.initial_recirculation_mass_flow_rate_kg_per_s,
            right.initial_recirculation_mass_flow_rate_kg_per_s,
        ),
        (
            left.recirculation_temperature_c,
            right.recirculation_temperature_c,
        ),
        (
            left.recirculation_humidity_ratio,
            right.recirculation_humidity_ratio,
        ),
        (
            left.recirculation_enthalpy_projection_j_per_kg,
            right.recirculation_enthalpy_projection_j_per_kg,
        ),
        (
            left.outdoor_air_inlet_temperature_c,
            right.outdoor_air_inlet_temperature_c,
        ),
        (
            left.outdoor_air_inlet_humidity_ratio,
            right.outdoor_air_inlet_humidity_ratio,
        ),
        (
            left.outdoor_air_inlet_enthalpy_j_per_kg,
            right.outdoor_air_inlet_enthalpy_j_per_kg,
        ),
        (
            left.outdoor_air_after_heat_recovery_temperature_c,
            right.outdoor_air_after_heat_recovery_temperature_c,
        ),
        (
            left.outdoor_air_after_heat_recovery_humidity_ratio,
            right.outdoor_air_after_heat_recovery_humidity_ratio,
        ),
        (
            left.outdoor_air_after_heat_recovery_enthalpy_j_per_kg,
            right.outdoor_air_after_heat_recovery_enthalpy_j_per_kg,
        ),
        (
            left.child_supply_mass_flow_rate_kg_per_s,
            right.child_supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.resulting_recirculation_mass_flow_rate_kg_per_s,
            right.resulting_recirculation_mass_flow_rate_kg_per_s,
        ),
        (left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        (
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        ),
        (
            left.mixed_air_enthalpy_projection_j_per_kg,
            right.mixed_air_enthalpy_projection_j_per_kg,
        ),
        (
            left.heat_recovery_sensible_output_w,
            right.heat_recovery_sensible_output_w,
        ),
        (
            left.heat_recovery_latent_output_w,
            right.heat_recovery_latent_output_w,
        ),
    ]
    .into_iter()
    .all(|(left, right)| options_have_exact_bits(left, right));
    let mut left_without_values = *left;
    let mut right_without_values = *right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.outdoor_air_mass_flow_rate_kg_per_s = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.initial_recirculation_mass_flow_rate_kg_per_s = None;
        snapshot.recirculation_temperature_c = None;
        snapshot.recirculation_humidity_ratio = None;
        snapshot.recirculation_enthalpy_projection_j_per_kg = None;
        snapshot.outdoor_air_inlet_temperature_c = None;
        snapshot.outdoor_air_inlet_humidity_ratio = None;
        snapshot.outdoor_air_inlet_enthalpy_j_per_kg = None;
        snapshot.outdoor_air_after_heat_recovery_temperature_c = None;
        snapshot.outdoor_air_after_heat_recovery_humidity_ratio = None;
        snapshot.outdoor_air_after_heat_recovery_enthalpy_j_per_kg = None;
        snapshot.child_supply_mass_flow_rate_kg_per_s = None;
        snapshot.resulting_recirculation_mass_flow_rate_kg_per_s = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.mixed_air_enthalpy_projection_j_per_kg = None;
        snapshot.heat_recovery_sensible_output_w = None;
        snapshot.heat_recovery_latent_output_w = None;
    }
    values_match && left_without_values == right_without_values
}

fn option_has_bits(actual: Option<f64>, expected: Option<f64>) -> bool {
    options_have_exact_bits(actual, expected)
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
    Error::CalcCoolingMixedAirCallLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_comparison_preserves_signed_zero() {
        assert!(options_have_exact_bits(Some(0.0), Some(0.0)));
        assert!(!options_have_exact_bits(Some(0.0), Some(-0.0)));
    }

    #[test]
    fn site_count_multiplication_overflow_fails_closed() {
        let error = checked_mul(usize::MAX, 2, "test_site_count_overflow", usize::MAX)
            .expect_err("site-count overflow must fail closed");
        assert!(matches!(
            error,
            Error::CalcCoolingMixedAirCallLifecycleInvariant {
                field: "test_site_count_overflow",
                expected: usize::MAX,
                actual: usize::MAX,
            }
        ));
    }
}
