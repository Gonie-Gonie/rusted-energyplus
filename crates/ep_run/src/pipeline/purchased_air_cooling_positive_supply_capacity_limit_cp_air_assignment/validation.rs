//! Fail-closed validation helpers for CP338 evidence.

use ep_runtime::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState,
) -> Result<(), String> {
    let assignments = state.capacity_limit_cp_air_assignment_count;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            checked_product(assignments, 3, "source-site count")?,
            state.source_site_execution_count,
        ),
        (
            "mixed_air_humidity_ratio_read_count",
            assignments,
            state.mixed_air_humidity_ratio_read_count,
        ),
        (
            "psychrometric_cp_air_evaluation_count",
            assignments,
            state.psychrometric_cp_air_evaluation_count,
        ),
        (
            "cp_air_assignment_write_count",
            assignments,
            state.cp_air_assignment_write_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads capacity-limit CpAir assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let assignment_expected = predecessor.capacity_limit_body_entered;
    let capacity_guard_false_expected = predecessor.active_guard_false_fallthrough;
    if snapshot.positive_guard_false_fallthrough_skipped
        != predecessor.positive_guard_false_fallthrough_skipped
        || snapshot.predecessor_capacity_limit_guard_evaluated
            != predecessor.capacity_limit_guard_evaluated
        || snapshot.predecessor_capacity_limit_body_entered
            != predecessor.capacity_limit_body_entered
        || snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            != predecessor.active_guard_false_fallthrough
        || snapshot.capacity_limit_guard_false_fallthrough_skipped != capacity_guard_false_expected
        || snapshot.capacity_limit_cp_air_assignment_executed != assignment_expected
    {
        return false;
    }

    if !assignment_expected {
        return !snapshot.mixed_air_humidity_ratio_read
            && snapshot.mixed_air_humidity_ratio.is_none()
            && !snapshot.psychrometric_cp_air_evaluated
            && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
            && !snapshot.cp_air_assigned
            && snapshot.cp_air_j_per_kg_k.is_none();
    }

    let Some(humidity_ratio) = snapshot.mixed_air_humidity_ratio else {
        return false;
    };
    let expected_cp_air = energyplus_psy_cp_air_fn_w(humidity_ratio);
    humidity_ratio.is_finite()
        && humidity_ratio >= 0.0
        && expected_cp_air.is_finite()
        && snapshot.mixed_air_humidity_ratio_read
        && option_has_bits(mixed_air.mixed_air_humidity_ratio, humidity_ratio)
        && snapshot.psychrometric_cp_air_evaluated
        && option_has_bits(
            snapshot.psychrometric_cp_air_result_j_per_kg_k,
            expected_cp_air,
        )
        && snapshot.cp_air_assigned
        && option_has_bits(snapshot.cp_air_j_per_kg_k, expected_cp_air)
}

fn checked_product(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| {
        format!("direct-zone IdealLoads capacity-limit CpAir assignment {label} overflowed")
    })
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
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_cp_air_assignment_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }
}
