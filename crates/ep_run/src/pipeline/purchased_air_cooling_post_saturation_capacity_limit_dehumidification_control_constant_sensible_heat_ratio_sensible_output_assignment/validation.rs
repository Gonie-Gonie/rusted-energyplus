//! Fail-closed validation for CP388 direct-release evidence.

use ep_model::{IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary as PredecessorLifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as PredecessorSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary as Lifecycle,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Snapshot,
    PurchasedAirInitLifecycleSummary,
};

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessor_cp387: Option<&PredecessorLifecycle>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP388 sensible-output evidence".to_string()
    })?;
    let predecessor = predecessor_cp387
        .ok_or_else(|| "direct-zone IdealLoads CP388 has no CP387 evidence".to_string())?;
    let init = init_lifecycle
        .ok_or_else(|| "direct-zone IdealLoads CP388 has no initialization evidence".to_string())?;
    let calls = coupling_call_count
        .ok_or_else(|| "direct-zone IdealLoads CP388 has no coupling call count".to_string())?;
    let expected_system = init
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| "direct-zone IdealLoads CP388 has no declared system".to_string())?;
    let expected_zone = init
        .controlled_zone
        .ok_or_else(|| "direct-zone IdealLoads CP388 has no controlled Zone".to_string())?;

    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || lifecycle.state.system != expected_system
        || predecessor.state.system != expected_system
        || lifecycle.state.predecessor_route_counts != predecessor.state.predecessor_route_counts
    {
        return Err("direct-zone IdealLoads CP388 provenance is invalid".to_string());
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor.state;
    let route_sum = checked_sum(&state.predecessor_route_counts, "route partition")?;
    let assignments = state
        .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count;
    let expected_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads CP388 source-site count overflowed".to_string()
        })?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        ("predecessor_route_partition", calls, route_sum),
        (
            "inactive_transition_count",
            calls,
            state.inactive_transition_count,
        ),
        (
            "predecessor_cp_air_assignment_count",
            predecessor_state
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count,
            assignments,
        ),
        ("direct_sensible_output_assignment_count", 0, assignments),
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "cooling_total_output_owned_read_count",
            assignments,
            state.cooling_total_output_owned_read_count,
        ),
        (
            "cooling_total_output_bit_corroboration_count",
            assignments,
            state.cooling_total_output_bit_corroboration_count,
        ),
        (
            "cooling_sensible_heat_ratio_read_count",
            assignments,
            state.cooling_sensible_heat_ratio_read_count,
        ),
        (
            "cooling_sensible_output_calculation_count",
            assignments,
            state.cooling_sensible_output_calculation_count,
        ),
        (
            "cooling_sensible_output_assignment_write_count",
            assignments,
            state.cooling_sensible_output_assignment_write_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP388 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor_state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP388 CP387 latest evidence is missing".to_string()
    })?;
    if !latest_matches_direct_release(
        latest,
        predecessor_latest,
        expected_system,
        expected_zone,
        calls,
    ) {
        return Err("direct-zone IdealLoads CP388 latest lineage is invalid".to_string());
    }
    Ok(())
}

fn latest_matches_direct_release(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    expected_ordinal: usize,
) -> bool {
    snapshot.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER
        && snapshot.system == expected_system
        && predecessor.system == expected_system
        && snapshot.parent_call_ordinal == expected_ordinal
        && predecessor.parent_call_ordinal == expected_ordinal
        && snapshot.controlled_zone == expected_zone
        && predecessor.controlled_zone == expected_zone
        && links_to_predecessor(snapshot, predecessor)
}

fn links_to_predecessor(snapshot: Snapshot, predecessor: PredecessorSnapshot) -> bool {
    predecessor.source
        == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER
        && inherited_flags(snapshot) == predecessor_flags(predecessor)
        && snapshot.predecessor_supply_enthalpy_assignment_executed
            == predecessor.predecessor_supply_enthalpy_assignment_executed
        && snapshot.predecessor_dehumidification_control_type_read
            == predecessor.predecessor_dehumidification_control_type_read
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_dehumidification_control_switch_dispatched
            == predecessor.predecessor_dehumidification_control_switch_dispatched
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
            == predecessor.dehumidification_control_constant_sensible_heat_ratio_case_entered
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
            == predecessor.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && snapshot.predecessor_mixed_air_humidity_ratio_read
            == predecessor.mixed_air_humidity_ratio_read
        && option_bits_equal(
            snapshot.predecessor_mixed_air_humidity_ratio,
            predecessor.mixed_air_humidity_ratio,
        )
        && snapshot.predecessor_psychrometric_cp_air_evaluated
            == predecessor.psychrometric_cp_air_evaluated
        && option_bits_equal(
            snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
            predecessor.psychrometric_cp_air_result_j_per_kg_k,
        )
        && snapshot.predecessor_cp_air_assigned == predecessor.cp_air_assigned
        && option_bits_equal(
            snapshot.predecessor_cp_air_j_per_kg_k,
            predecessor.cp_air_j_per_kg_k,
        )
        && option_bits_equal(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && predecessor_is_direct_inactive(predecessor)
        && snapshot_is_direct_inactive(snapshot)
}

fn predecessor_is_direct_inactive(snapshot: PredecessorSnapshot) -> bool {
    !snapshot.dehumidification_control_constant_sensible_heat_ratio_case_entered
        && !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && !snapshot.mixed_air_humidity_ratio_read
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.psychrometric_cp_air_evaluated
        && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
        && !snapshot.cp_air_assigned
        && snapshot.cp_air_j_per_kg_k.is_none()
}

fn snapshot_is_direct_inactive(snapshot: Snapshot) -> bool {
    !snapshot
        .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
        && !snapshot.cp384_retained_cooling_total_output_owned_read
        && !snapshot.cp385_cooling_total_output_bit_corroborated
        && !snapshot.cooling_total_output_read
        && snapshot.cooling_total_output_w.is_none()
        && !snapshot.cooling_sensible_heat_ratio_read
        && snapshot.cooling_sensible_heat_ratio.is_none()
        && !snapshot.cooling_sensible_output_calculated
        && snapshot.calculated_cooling_sensible_output_w.is_none()
        && !snapshot.cooling_sensible_output_assigned
        && snapshot.cooling_sensible_output_w.is_none()
}

fn inherited_flags(snapshot: Snapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn predecessor_flags(snapshot: PredecessorSnapshot) -> [bool; 20] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ]
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("direct-zone IdealLoads CP388 {label} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP388 invariant {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_partition_overflow_fails_closed() {
        assert!(checked_sum(&[usize::MAX, 1], "partition").is_err());
    }

    #[test]
    fn exact_bit_comparison_distinguishes_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
