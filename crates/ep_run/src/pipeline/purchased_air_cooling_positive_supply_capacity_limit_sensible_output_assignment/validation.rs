//! Fail-closed validation helpers for CP339 evidence.

use ep_runtime::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
) -> Result<(), String> {
    let assignments = state.capacity_limit_sensible_output_assignment_count;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            checked_product(assignments, 6, "source-site count")?,
            state.source_site_execution_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            assignments,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "mixed_air_enthalpy_read_count",
            assignments,
            state.mixed_air_enthalpy_read_count,
        ),
        (
            "supply_enthalpy_read_count",
            assignments,
            state.supply_enthalpy_read_count,
        ),
        (
            "enthalpy_difference_calculation_count",
            assignments,
            state.enthalpy_difference_calculation_count,
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
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads capacity-limit sensible-output assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    supply_flow: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    mixed_air: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    supply_enthalpy: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let assignment_expected = predecessor.capacity_limit_cp_air_assignment_executed;
    if snapshot.unit_body_entered != predecessor.unit_body_entered
        || snapshot.predecessor_cooling_body_entered != predecessor.predecessor_cooling_body_entered
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
            != predecessor.capacity_limit_cp_air_assignment_executed
        || snapshot.unit_off_skipped != predecessor.unit_off_skipped
        || snapshot.non_cooling_skipped != predecessor.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_guard_false_fallthrough_skipped
            != predecessor.capacity_limit_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_sensible_output_assignment_executed != assignment_expected
    {
        return false;
    }

    if !assignment_expected {
        return !snapshot.supply_mass_flow_rate_read
            && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
            && !snapshot.mixed_air_enthalpy_read
            && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
            && !snapshot.supply_enthalpy_read
            && snapshot.supply_enthalpy_j_per_kg.is_none()
            && !snapshot.enthalpy_difference_calculated
            && snapshot.mixed_air_minus_supply_enthalpy_j_per_kg.is_none()
            && !snapshot.cooling_sensible_output_calculated
            && snapshot.calculated_cooling_sensible_output_w.is_none()
            && !snapshot.cooling_sensible_output_assigned
            && snapshot.cooling_sensible_output_w.is_none();
    }

    let (Some(expected_flow), Some(expected_mixed_air), Some(expected_supply_enthalpy)) = (
        supply_flow.supply_mass_flow_rate_kg_per_s,
        mixed_air.mixed_air_enthalpy_projection_j_per_kg,
        supply_enthalpy.supply_enthalpy_j_per_kg,
    ) else {
        return false;
    };
    let expected_difference = expected_mixed_air - expected_supply_enthalpy;
    let expected_output = expected_flow * expected_difference;

    supply_flow.supply_mass_flow_rate_read
        && supply_flow.positive_supply_mass_flow_body_entered
        && option_has_bits(mixed_air.supply_mass_flow_rate_kg_per_s, expected_flow)
        && option_has_bits(
            mixed_air.child_supply_mass_flow_rate_kg_per_s,
            expected_flow,
        )
        && mixed_air.mixed_air_enthalpy_projection_assigned
        && supply_enthalpy.supply_enthalpy_assigned
        && snapshot.supply_mass_flow_rate_read
        && option_has_bits(snapshot.supply_mass_flow_rate_kg_per_s, expected_flow)
        && snapshot.mixed_air_enthalpy_read
        && option_has_bits(snapshot.mixed_air_enthalpy_j_per_kg, expected_mixed_air)
        && snapshot.supply_enthalpy_read
        && option_has_bits(snapshot.supply_enthalpy_j_per_kg, expected_supply_enthalpy)
        && snapshot.enthalpy_difference_calculated
        && option_has_bits(
            snapshot.mixed_air_minus_supply_enthalpy_j_per_kg,
            expected_difference,
        )
        && snapshot.cooling_sensible_output_calculated
        && option_has_bits(
            snapshot.calculated_cooling_sensible_output_w,
            expected_output,
        )
        && snapshot.cooling_sensible_output_assigned
        && option_has_bits(snapshot.cooling_sensible_output_w, expected_output)
}

fn checked_product(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| {
        format!(
            "direct-zone IdealLoads capacity-limit sensible-output assignment {label} overflowed"
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
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_sensible_output_assignment_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }
}
