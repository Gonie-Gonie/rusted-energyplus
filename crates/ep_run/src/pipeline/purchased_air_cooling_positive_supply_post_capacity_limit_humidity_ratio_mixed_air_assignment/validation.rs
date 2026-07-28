//! Fail-closed validation helpers for CP345 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
};

pub(super) fn validate_source_counters(
    state:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
) -> Result<(), String> {
    let executions = state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count;
    let expected_sites = executions
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment source-site count overflowed"
                .to_string()
        })?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            expected_sites,
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
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads post-capacity-limit humidity-ratio assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    corroborating: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    if snapshot.source_order
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
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
        && option_bits_equal(corroborating.mixed_air_humidity_ratio, Some(source))
        && corroborating.supply_humidity_ratio_assignment_performed
        && option_bits_equal(corroborating.assigned_supply_humidity_ratio, Some(source))
        && snapshot.mixed_air_humidity_ratio_read
        && option_bits_equal(snapshot.mixed_air_humidity_ratio, Some(source))
        && snapshot.supply_humidity_ratio_assignment_performed
        && option_bits_equal(snapshot.assigned_supply_humidity_ratio, Some(source))
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

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn source_counter_overflow_fails_closed() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count = usize::MAX;
        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }

    #[test]
    fn bit_comparison_distinguishes_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
