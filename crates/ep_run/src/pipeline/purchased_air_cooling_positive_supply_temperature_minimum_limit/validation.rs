//! Fail-closed validation helpers for CP333 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirCalcCoolingSensibleFlowSnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState,
) -> Result<(), String> {
    let executions = state.supply_temperature_minimum_limit_count;
    let source_sites = checked_product(
        executions,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER.len(),
        "source-site count",
    )?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_temperature_for_maximum_read_count",
            executions,
            state.supply_temperature_for_maximum_read_count,
        ),
        (
            "minimum_cooling_supply_air_temperature_for_maximum_read_count",
            executions,
            state.minimum_cooling_supply_air_temperature_for_maximum_read_count,
        ),
        (
            "source_shaped_two_argument_maximum_evaluation_count",
            executions,
            state.source_shaped_two_argument_maximum_evaluation_count,
        ),
        (
            "supply_temperature_assignment_count",
            executions,
            state.supply_temperature_assignment_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling positive-supply temperature minimum-limit invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    sensible_flow: &PurchasedAirCalcCoolingSensibleFlowSnapshot,
    typed_minimum_cooling_supply_air_temperature_c: f64,
) -> bool {
    let execution_expected = predecessor.supply_temperature_assignment_executed;
    if snapshot.supply_temperature_minimum_limit_executed != execution_expected
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
    {
        return false;
    }
    if !execution_expected {
        return skipped_source_shape(snapshot);
    }

    let Some(left) = snapshot.supply_temperature_before_minimum_limit_c else {
        return false;
    };
    let Some(right) = snapshot.minimum_cooling_supply_air_temperature_c else {
        return false;
    };
    if !right.is_finite() || !typed_minimum_cooling_supply_air_temperature_c.is_finite() {
        return false;
    }
    let maximum = if left < right { right } else { left };

    snapshot.supply_temperature_for_maximum_read
        && same_option(
            snapshot.supply_temperature_before_minimum_limit_c,
            predecessor.supply_temperature_c,
        )
        && snapshot.minimum_cooling_supply_air_temperature_for_maximum_read
        && same_option(
            snapshot.minimum_cooling_supply_air_temperature_c,
            sensible_flow.minimum_cooling_supply_air_temperature_c,
        )
        && same_option(
            snapshot.minimum_cooling_supply_air_temperature_c,
            Some(typed_minimum_cooling_supply_air_temperature_c),
        )
        && snapshot.source_shaped_two_argument_maximum_evaluated
        && same_option(snapshot.maximum_supply_temperature_c, Some(maximum))
        && snapshot.supply_temperature_assignment_performed
        && same_option(snapshot.assigned_supply_temperature_c, Some(maximum))
}

fn skipped_source_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) -> bool {
    !snapshot.supply_temperature_for_maximum_read
        && snapshot.supply_temperature_before_minimum_limit_c.is_none()
        && !snapshot.minimum_cooling_supply_air_temperature_for_maximum_read
        && snapshot.minimum_cooling_supply_air_temperature_c.is_none()
        && !snapshot.source_shaped_two_argument_maximum_evaluated
        && snapshot.maximum_supply_temperature_c.is_none()
        && !snapshot.supply_temperature_assignment_performed
        && snapshot.assigned_supply_temperature_c.is_none()
}

fn checked_product(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply temperature minimum limit {label} overflowed"
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
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.supply_temperature_minimum_limit_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }
}
