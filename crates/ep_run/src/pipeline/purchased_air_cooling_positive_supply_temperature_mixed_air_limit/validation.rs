//! Fail-closed validation helpers for CP334 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState,
) -> Result<(), String> {
    let executions = state.supply_temperature_mixed_air_limit_count;
    let source_sites = checked_product(
        executions,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER.len(),
        "source-site count",
    )?;
    for (field, expected, actual) in [
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
            "supply_temperature_assignment_count",
            executions,
            state.supply_temperature_assignment_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let execution_expected = predecessor.supply_temperature_minimum_limit_executed;
    if snapshot.supply_temperature_mixed_air_limit_executed != execution_expected
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
    {
        return false;
    }
    if !execution_expected {
        return skipped_source_shape(snapshot);
    }

    let Some(left) = snapshot.supply_temperature_before_mixed_air_limit_c else {
        return false;
    };
    let Some(right) = snapshot.mixed_air_temperature_c else {
        return false;
    };
    if !right.is_finite() {
        return false;
    }
    let minimum = if left < right { left } else { right };

    snapshot.supply_temperature_for_minimum_read
        && same_option(
            snapshot.supply_temperature_before_mixed_air_limit_c,
            predecessor.assigned_supply_temperature_c,
        )
        && snapshot.mixed_air_temperature_for_minimum_read
        && mixed_air.mixed_air_temperature_assigned
        && same_option(
            snapshot.mixed_air_temperature_c,
            mixed_air.mixed_air_temperature_c,
        )
        && snapshot.source_shaped_two_argument_minimum_evaluated
        && same_option(snapshot.minimum_supply_temperature_c, Some(minimum))
        && snapshot.supply_temperature_assignment_performed
        && same_option(snapshot.assigned_supply_temperature_c, Some(minimum))
}

fn skipped_source_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
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

fn checked_product(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply mixed-air-temperature limit {label} overflowed"
        )
    })
}

fn same_option(left: Option<f64>, right: Option<f64>) -> bool {
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
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.supply_temperature_mixed_air_limit_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }
}
