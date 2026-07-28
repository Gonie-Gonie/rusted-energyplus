//! Fail-closed validation helpers for CP344 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};

pub(super) fn validate_source_counters(
    state:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
) -> Result<(), String> {
    let executions = state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count;
    let expected_sites = executions
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit source-site count overflowed"
                .to_string()
        })?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            expected_sites,
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
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads capacity-limit supply-temperature mixed-air limit invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    corroborating: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    if snapshot.source_order
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER
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

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn source_counter_overflow_fails_closed() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count = usize::MAX;
        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }

    #[test]
    fn bit_comparison_keeps_nan_payloads_and_signed_zero_distinct() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0044);
        assert!(option_has_bits(Some(nan), nan));
        assert!(option_has_bits(Some(-0.0), -0.0));
        assert!(!option_has_bits(Some(0.0), -0.0));
    }
}
