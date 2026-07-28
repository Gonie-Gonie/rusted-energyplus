//! Fail-closed validation helpers for CP331 evidence.

use ep_runtime::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    psychrometrics::energyplus_psy_cp_air_fn_w,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
) -> Result<(), String> {
    let assignments = state.cp_air_assignment_count;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            checked_product(assignments, 3, "source-site count")?,
            state.source_site_execution_count,
        ),
        (
            "zone_humidity_ratio_read_count",
            assignments,
            state.zone_humidity_ratio_read_count,
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
                "direct-zone IdealLoads cooling positive-supply CpAir assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let assignment_expected = predecessor.positive_supply_mass_flow_body_entered;
    let guard_false_expected =
        predecessor.cooling_body_entered && predecessor.active_guard_false_fallthrough;
    if snapshot.cp_air_assignment_executed != assignment_expected
        || snapshot.positive_guard_false_fallthrough_skipped != guard_false_expected
    {
        return false;
    }

    if !assignment_expected {
        return !snapshot.zone_humidity_ratio_read
            && snapshot.zone_humidity_ratio.is_none()
            && !snapshot.psychrometric_cp_air_evaluated
            && snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none()
            && !snapshot.cp_air_assigned
            && snapshot.cp_air_j_per_kg_k.is_none();
    }

    let Some(humidity_ratio) = snapshot.zone_humidity_ratio else {
        return false;
    };
    let expected_cp_air = energyplus_psy_cp_air_fn_w(humidity_ratio);
    humidity_ratio.is_finite()
        && humidity_ratio >= 0.0
        && expected_cp_air.is_finite()
        && snapshot.zone_humidity_ratio_read
        && option_has_bits(mixed_air.recirculation_humidity_ratio, humidity_ratio)
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
        format!(
            "direct-zone IdealLoads cooling positive-supply CpAir assignment {label} overflowed"
        )
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
        let mut state = PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.cp_air_assignment_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }
}
