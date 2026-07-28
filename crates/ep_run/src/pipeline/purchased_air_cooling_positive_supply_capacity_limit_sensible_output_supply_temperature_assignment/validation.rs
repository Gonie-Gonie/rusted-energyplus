//! Fail-closed validation helpers for CP343 direct-release evidence.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
};

pub(super) fn validate_source_counters(
    state:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState,
) -> Result<(), String> {
    let assignments = state.capacity_limit_sensible_output_supply_temperature_assignment_count;
    let expected_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| {
            "direct-zone IdealLoads capacity-limit supply-temperature assignment source-site count overflowed"
                .to_string()
        })?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "supply_enthalpy_for_dry_bulb_inversion_read_count",
            assignments,
            state.supply_enthalpy_for_dry_bulb_inversion_read_count,
        ),
        (
            "supply_humidity_ratio_for_dry_bulb_inversion_read_count",
            assignments,
            state.supply_humidity_ratio_for_dry_bulb_inversion_read_count,
        ),
        (
            "psychrometric_supply_temperature_evaluation_count",
            assignments,
            state.psychrometric_supply_temperature_evaluation_count,
        ),
        (
            "supply_temperature_assignment_write_count",
            assignments,
            state.supply_temperature_assignment_write_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads capacity-limit supply-temperature assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    temperature_owner: &PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    humidity_owner: &PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    corroborating: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    if snapshot.source_order
        != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER
        || !inherited_shape_matches(snapshot, predecessor)
    {
        return false;
    }

    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment = predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    if !guard_false && !assignment {
        return !snapshot.capacity_limit_sensible_output_guard_false_fallthrough
            && !snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed
            && complete_null_shape(snapshot);
    }
    if guard_false == assignment
        || snapshot.capacity_limit_sensible_output_guard_false_fallthrough != guard_false
        || snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed
            != assignment
    {
        return false;
    }

    let (Some(preexisting), Some(humidity)) = (
        temperature_owner.assigned_supply_temperature_c,
        humidity_owner.assigned_supply_humidity_ratio,
    ) else {
        return false;
    };
    if !preexisting.is_finite()
        || !humidity.is_finite()
        || humidity < 0.0
        || !option_has_bits(corroborating.supply_temperature_c, preexisting)
        || !option_has_bits(corroborating.supply_humidity_ratio, humidity)
        || !option_has_bits(snapshot.preexisting_supply_temperature_c, preexisting)
    {
        return false;
    }

    if guard_false {
        return skipped_rhs_is_null(snapshot)
            && option_has_bits(snapshot.resulting_supply_temperature_c, preexisting);
    }

    let Some(enthalpy) = predecessor.resulting_supply_enthalpy_j_per_kg else {
        return false;
    };
    let expected = ep_runtime::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
    snapshot.supply_enthalpy_for_dry_bulb_inversion_read
        && option_has_bits(snapshot.supply_enthalpy_j_per_kg, enthalpy)
        && snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read
        && option_has_bits(snapshot.supply_humidity_ratio, humidity)
        && snapshot.psychrometric_supply_temperature_evaluated
        && option_has_bits(snapshot.psychrometric_supply_temperature_result_c, expected)
        && snapshot.supply_temperature_assigned
        && option_has_bits(snapshot.assigned_supply_temperature_c, expected)
        && option_has_bits(snapshot.resulting_supply_temperature_c, expected)
}

fn inherited_shape_matches(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    predecessor:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
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
            == predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
}

fn complete_null_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    snapshot.preexisting_supply_temperature_c.is_none()
        && skipped_rhs_is_null(snapshot)
        && snapshot.resulting_supply_temperature_c.is_none()
}

fn skipped_rhs_is_null(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    !snapshot.supply_enthalpy_for_dry_bulb_inversion_read
        && snapshot.supply_enthalpy_j_per_kg.is_none()
        && !snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read
        && snapshot.supply_humidity_ratio.is_none()
        && !snapshot.psychrometric_supply_temperature_evaluated
        && snapshot.psychrometric_supply_temperature_result_c.is_none()
        && !snapshot.supply_temperature_assigned
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
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_sensible_output_supply_temperature_assignment_count = usize::MAX;
        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }
}
