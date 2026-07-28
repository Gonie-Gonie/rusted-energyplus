//! Fail-closed validation helpers for CP336 direct-release evidence.

use ep_runtime::psychrometrics::energyplus_psy_h_fn_tdb_w;
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState,
) -> Result<(), String> {
    let executions = state.supply_enthalpy_assignment_count;
    let source_sites = checked_product(
        executions,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER.len(),
        "source-site count",
    )?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_temperature_for_enthalpy_read_count",
            executions,
            state.supply_temperature_for_enthalpy_read_count,
        ),
        (
            "supply_humidity_ratio_for_enthalpy_read_count",
            executions,
            state.supply_humidity_ratio_for_enthalpy_read_count,
        ),
        (
            "psychrometric_supply_enthalpy_evaluation_count",
            executions,
            state.psychrometric_supply_enthalpy_evaluation_count,
        ),
        (
            "supply_enthalpy_assignment_write_count",
            executions,
            state.supply_enthalpy_assignment_write_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling positive-supply enthalpy assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    temperature: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let execution_expected = predecessor.supply_humidity_ratio_mixed_air_assignment_executed;
    if snapshot.supply_enthalpy_assignment_executed != execution_expected
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
    {
        return false;
    }
    if !execution_expected {
        return skipped_source_shape(snapshot);
    }

    let Some(supply_temperature_c) = snapshot.supply_temperature_c else {
        return false;
    };
    let Some(supply_humidity_ratio) = snapshot.supply_humidity_ratio else {
        return false;
    };
    let expected = energyplus_psy_h_fn_tdb_w(supply_temperature_c, supply_humidity_ratio);
    if !supply_temperature_c.is_finite()
        || !source_humidity_is_finite_nonnegative(supply_humidity_ratio)
        || !expected.is_finite()
    {
        return false;
    }

    snapshot.supply_temperature_for_enthalpy_read
        && temperature.supply_temperature_mixed_air_limit_executed
        && same_option(
            snapshot.supply_temperature_c,
            temperature.assigned_supply_temperature_c,
        )
        && snapshot.supply_humidity_ratio_for_enthalpy_read
        && predecessor.supply_humidity_ratio_assignment_performed
        && same_option(
            snapshot.supply_humidity_ratio,
            predecessor.assigned_supply_humidity_ratio,
        )
        && snapshot.psychrometric_supply_enthalpy_evaluated
        && same_option(
            snapshot.psychrometric_supply_enthalpy_result_j_per_kg,
            Some(expected),
        )
        && snapshot.supply_enthalpy_assigned
        && same_option(snapshot.supply_enthalpy_j_per_kg, Some(expected))
}

fn skipped_source_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    !snapshot.supply_temperature_for_enthalpy_read
        && snapshot.supply_temperature_c.is_none()
        && !snapshot.supply_humidity_ratio_for_enthalpy_read
        && snapshot.supply_humidity_ratio.is_none()
        && !snapshot.psychrometric_supply_enthalpy_evaluated
        && snapshot
            .psychrometric_supply_enthalpy_result_j_per_kg
            .is_none()
        && !snapshot.supply_enthalpy_assigned
        && snapshot.supply_enthalpy_j_per_kg.is_none()
}

fn checked_product(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads cooling positive-supply enthalpy assignment {label} overflowed"
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

fn source_humidity_is_finite_nonnegative(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;

    use super::*;

    #[test]
    fn source_counter_overflow_fails_closed() {
        let mut state = PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.supply_enthalpy_assignment_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }

    #[test]
    fn active_humidity_domain_accepts_negative_zero_but_rejects_negative_and_nonfinite_values() {
        assert!(source_humidity_is_finite_nonnegative(-0.0));
        assert!(source_humidity_is_finite_nonnegative(0.008));
        assert!(!source_humidity_is_finite_nonnegative(-0.001));
        assert!(!source_humidity_is_finite_nonnegative(f64::NAN));
        assert!(!source_humidity_is_finite_nonnegative(f64::INFINITY));
    }
}
