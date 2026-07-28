//! Fail-closed validation helpers for CP335 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState,
) -> Result<(), String> {
    let executions = state.supply_humidity_ratio_mixed_air_assignment_count;
    let source_sites = checked_product(
        executions,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER
            .len(),
        "source-site count",
    )?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
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
                "direct-zone IdealLoads cooling positive-supply mixed-air humidity-ratio assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let execution_expected = predecessor.supply_temperature_mixed_air_limit_executed;
    if snapshot.supply_humidity_ratio_mixed_air_assignment_executed != execution_expected
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
    {
        return false;
    }
    if !execution_expected {
        return skipped_source_shape(snapshot);
    }

    let Some(source) = snapshot.mixed_air_humidity_ratio else {
        return false;
    };
    if !source_is_finite_nonnegative(source) {
        return false;
    }

    snapshot.mixed_air_humidity_ratio_read
        && mixed_air.mixed_air_humidity_ratio_assigned
        && same_option(
            snapshot.mixed_air_humidity_ratio,
            mixed_air.mixed_air_humidity_ratio,
        )
        && snapshot.supply_humidity_ratio_assignment_performed
        && same_option(snapshot.assigned_supply_humidity_ratio, Some(source))
}

fn skipped_source_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    !snapshot.mixed_air_humidity_ratio_read
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.supply_humidity_ratio_assignment_performed
        && snapshot.assigned_supply_humidity_ratio.is_none()
}

fn checked_product(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply mixed-air humidity-ratio assignment {label} overflowed"
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

fn source_is_finite_nonnegative(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn source_counter_overflow_fails_closed() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.supply_humidity_ratio_mixed_air_assignment_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }

    #[test]
    fn active_source_domain_accepts_negative_zero_but_rejects_negative_and_nonfinite_values() {
        assert!(source_is_finite_nonnegative(-0.0));
        assert!(source_is_finite_nonnegative(0.008));
        assert!(!source_is_finite_nonnegative(-0.001));
        assert!(!source_is_finite_nonnegative(f64::NAN));
        assert!(!source_is_finite_nonnegative(f64::INFINITY));
    }
}
