//! Fail-closed validation helpers for CP342 evidence.

use ep_runtime::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
};

const EXPECTED_SOURCE_ORDER: &[&str] = &[
    "read-retained-mixed-air-enthalpy-for-supply-enthalpy-difference",
    "read-retained-cooling-sensible-output-for-specific-cooling-output-division",
    "read-retained-supply-mass-flow-rate-for-specific-cooling-output-division",
    "calculate-cooling-sensible-output-divided-by-supply-mass-flow-rate",
    "calculate-mixed-air-enthalpy-minus-specific-cooling-output",
    "assign-local-supply-enthalpy",
];

pub(super) fn validate_source_counters(
    state:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState,
) -> Result<(), String> {
    let assignments = state.capacity_limit_sensible_output_supply_enthalpy_assignment_count;
    let expected_sites = assignments.checked_mul(6).ok_or_else(|| {
        "direct-zone IdealLoads capacity-limit supply-enthalpy assignment source-site count overflowed"
            .to_string()
    })?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "mixed_air_enthalpy_read_count",
            assignments,
            state.mixed_air_enthalpy_read_count,
        ),
        (
            "cooling_sensible_output_read_count",
            assignments,
            state.cooling_sensible_output_read_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            assignments,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "specific_cooling_output_calculation_count",
            assignments,
            state.specific_cooling_output_calculation_count,
        ),
        (
            "supply_enthalpy_calculation_count",
            assignments,
            state.supply_enthalpy_calculation_count,
        ),
        (
            "supply_enthalpy_assignment_write_count",
            assignments,
            state.supply_enthalpy_assignment_write_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads capacity-limit supply-enthalpy assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    retained: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
) -> bool {
    if snapshot.source_order != EXPECTED_SOURCE_ORDER
        || !inherited_shape_matches(snapshot, predecessor)
    {
        return false;
    }

    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment =
        predecessor.capacity_limit_sensible_output_maximum_capacity_assignment_executed;
    if !guard_false && !assignment {
        return !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
            && !snapshot.capacity_limit_sensible_output_supply_enthalpy_assignment_executed
            && complete_null_shape(snapshot);
    }
    if guard_false == assignment
        || snapshot.capacity_limit_sensible_output_guard_false_fallthrough != guard_false
        || snapshot.capacity_limit_sensible_output_supply_enthalpy_assignment_executed != assignment
    {
        return false;
    }

    let Some(preexisting_supply_enthalpy) = retained.supply_enthalpy_j_per_kg else {
        return false;
    };
    if !option_has_bits(
        snapshot.preexisting_supply_enthalpy_j_per_kg,
        preexisting_supply_enthalpy,
    ) {
        return false;
    }

    if guard_false {
        return skipped_rhs_is_null(snapshot)
            && option_has_bits(
                snapshot.resulting_supply_enthalpy_j_per_kg,
                preexisting_supply_enthalpy,
            );
    }

    let (Some(mixed_air), Some(cooling_sensible_output), Some(supply_mass_flow)) = (
        retained.mixed_air_enthalpy_j_per_kg,
        predecessor.resulting_cooling_sensible_output_w,
        retained.supply_mass_flow_rate_kg_per_s,
    ) else {
        return false;
    };
    active_values_match(
        snapshot,
        mixed_air,
        cooling_sensible_output,
        supply_mass_flow,
    )
}

fn inherited_shape_matches(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
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
            == predecessor.capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
}

fn active_values_match(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    mixed_air: f64,
    cooling_sensible_output: f64,
    supply_mass_flow: f64,
) -> bool {
    if !mixed_air.is_finite()
        || !cooling_sensible_output.is_finite()
        || cooling_sensible_output <= 0.0
        || supply_mass_flow.is_nan()
        || supply_mass_flow <= 0.0
    {
        return false;
    }
    let specific_cooling_output = cooling_sensible_output / supply_mass_flow;
    let calculated_supply_enthalpy = mixed_air - specific_cooling_output;
    snapshot.mixed_air_enthalpy_read
        && option_has_bits(snapshot.mixed_air_enthalpy_j_per_kg, mixed_air)
        && snapshot.cooling_sensible_output_read
        && option_has_bits(snapshot.cooling_sensible_output_w, cooling_sensible_output)
        && snapshot.supply_mass_flow_rate_read
        && option_has_bits(snapshot.supply_mass_flow_rate_kg_per_s, supply_mass_flow)
        && snapshot.specific_cooling_output_calculated
        && option_has_bits(
            snapshot.specific_cooling_output_j_per_kg,
            specific_cooling_output,
        )
        && snapshot.supply_enthalpy_calculated
        && option_has_bits(
            snapshot.calculated_supply_enthalpy_j_per_kg,
            calculated_supply_enthalpy,
        )
        && snapshot.supply_enthalpy_assigned
        && option_has_bits(
            snapshot.assigned_supply_enthalpy_j_per_kg,
            calculated_supply_enthalpy,
        )
        && option_has_bits(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            calculated_supply_enthalpy,
        )
}

fn complete_null_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    snapshot.preexisting_supply_enthalpy_j_per_kg.is_none()
        && skipped_rhs_is_null(snapshot)
        && snapshot.resulting_supply_enthalpy_j_per_kg.is_none()
}

fn skipped_rhs_is_null(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    !snapshot.mixed_air_enthalpy_read
        && snapshot.mixed_air_enthalpy_j_per_kg.is_none()
        && !snapshot.cooling_sensible_output_read
        && snapshot.cooling_sensible_output_w.is_none()
        && !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.specific_cooling_output_calculated
        && snapshot.specific_cooling_output_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_calculated
        && snapshot.calculated_supply_enthalpy_j_per_kg.is_none()
        && !snapshot.supply_enthalpy_assigned
        && snapshot.assigned_supply_enthalpy_j_per_kg.is_none()
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    };

    use super::*;

    #[test]
    fn source_counter_overflow_fails_closed() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_sensible_output_supply_enthalpy_assignment_count = usize::MAX;
        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }

    #[test]
    fn pure_snapshot_characterization_accepts_positive_infinite_flow_grouping() {
        // This isolates CP342 arithmetic shape only. The validated direct chain
        // cannot reach this true route because CP339 observes `+inf * 0` first.
        let mixed_air = 50_000.0;
        let cooling_sensible_output = 1.0;
        let flow = f64::INFINITY;
        let mut snapshot = active_snapshot(mixed_air, cooling_sensible_output, flow);
        assert!(active_values_match(
            &snapshot,
            mixed_air,
            cooling_sensible_output,
            flow
        ));

        snapshot.calculated_supply_enthalpy_j_per_kg = Some(-0.0);
        assert!(!active_values_match(
            &snapshot,
            mixed_air,
            cooling_sensible_output,
            flow
        ));
    }

    fn active_snapshot(
        mixed_air: f64,
        sensible: f64,
        flow: f64,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot
    {
        let specific = sensible / flow;
        let result = mixed_air - specific;
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order: EXPECTED_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_no_outdoor_air_fallback_entered: true,
            predecessor_positive_supply_mass_flow_body_entered: true,
            predecessor_active_guard_false_fallthrough: false,
            predecessor_capacity_limit_guard_evaluated: true,
            predecessor_capacity_limit_body_entered: true,
            predecessor_active_capacity_limit_guard_false_fallthrough: false,
            predecessor_capacity_limit_cp_air_assignment_executed: true,
            predecessor_capacity_limit_sensible_output_assignment_executed: true,
            predecessor_capacity_limit_sensible_output_guard_evaluated: true,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: false,
            predecessor_capacity_limit_sensible_output_adjustment_body_entered: true,
            predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            capacity_limit_guard_false_fallthrough_skipped: false,
            capacity_limit_sensible_output_guard_false_fallthrough: false,
            capacity_limit_sensible_output_supply_enthalpy_assignment_executed: true,
            preexisting_supply_enthalpy_j_per_kg: Some(40_000.0),
            mixed_air_enthalpy_read: true,
            mixed_air_enthalpy_j_per_kg: Some(mixed_air),
            cooling_sensible_output_read: true,
            cooling_sensible_output_w: Some(sensible),
            supply_mass_flow_rate_read: true,
            supply_mass_flow_rate_kg_per_s: Some(flow),
            specific_cooling_output_calculated: true,
            specific_cooling_output_j_per_kg: Some(specific),
            supply_enthalpy_calculated: true,
            calculated_supply_enthalpy_j_per_kg: Some(result),
            supply_enthalpy_assigned: true,
            assigned_supply_enthalpy_j_per_kg: Some(result),
            resulting_supply_enthalpy_j_per_kg: Some(result),
        }
    }
}
