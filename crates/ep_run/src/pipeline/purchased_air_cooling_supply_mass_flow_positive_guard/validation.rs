//! Fail-closed validation helpers for CP330 evidence.

use ep_runtime::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
) -> Result<(), String> {
    let cooling = state.cooling_body_entry_count;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            checked_product(cooling, 2, "unconditional source-site count")?
                .checked_add(state.positive_supply_mass_flow_body_entry_count)
                .ok_or_else(|| {
                    "direct-zone IdealLoads cooling positive-supply-flow guard source-site count overflowed"
                        .to_string()
                })?,
            state.source_site_execution_count,
        ),
        (
            "supply_mass_flow_rate_read_count",
            cooling,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "supply_mass_flow_rate_strictly_positive_comparison_count",
            cooling,
            state.supply_mass_flow_rate_strictly_positive_comparison_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling positive-supply-flow guard invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    predecessor: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    if !snapshot.cooling_body_entered {
        return !snapshot.supply_mass_flow_rate_read
            && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
            && !snapshot.supply_mass_flow_rate_strictly_positive_comparison_evaluated
            && snapshot.supply_mass_flow_rate_strictly_positive.is_none()
            && !snapshot.positive_supply_mass_flow_body_entered
            && !snapshot.active_guard_false_fallthrough;
    }

    let Some(supply) = snapshot.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let predecessor_supply = predecessor.supply_mass_flow_rate_kg_per_s;
    let child_supply = predecessor.child_supply_mass_flow_rate_kg_per_s;
    let recirculation_supply = predecessor.resulting_recirculation_mass_flow_rate_kg_per_s;
    let positive = supply > 0.0;

    snapshot.supply_mass_flow_rate_read
        && option_has_bits(predecessor_supply, supply)
        && option_has_bits(child_supply, supply)
        && option_has_bits(recirculation_supply, supply)
        && snapshot.supply_mass_flow_rate_strictly_positive_comparison_evaluated
        && snapshot.supply_mass_flow_rate_strictly_positive == Some(positive)
        && snapshot.positive_supply_mass_flow_body_entered == positive
        && snapshot.active_guard_false_fallthrough != positive
}

fn checked_product(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling positive-supply-flow guard {label} overflowed")
    })
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
        PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn source_counter_overflow_fails_closed() {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.cooling_body_entry_count = usize::MAX;

        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }

    #[test]
    fn active_shape_rejects_a_forged_predecessor_supply_copy() {
        let supply = 0.25;
        let guard = PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
            source: "",
            first_excluded_source: "",
            source_order: &[],
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_call_executed: true,
            predecessor_zero_flow_reset_body_entered: false,
            predecessor_active_guard_false_fallthrough: true,
            predecessor_no_outdoor_air_fallback_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            supply_mass_flow_rate_read: true,
            supply_mass_flow_rate_kg_per_s: Some(supply),
            supply_mass_flow_rate_strictly_positive_comparison_evaluated: true,
            supply_mass_flow_rate_strictly_positive: Some(true),
            positive_supply_mass_flow_body_entered: true,
            active_guard_false_fallthrough: false,
        };
        let mut predecessor = PurchasedAirCalcCoolingMixedAirCallSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
            child_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
            no_oa_child_source_order:
                PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_zero_flow_reset_body_entered: false,
            predecessor_active_guard_false_fallthrough: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_call_executed: true,
            state_reference_bound: true,
            purchased_air_number_read: true,
            outdoor_air_mass_flow_rate_read: true,
            outdoor_air_mass_flow_rate_kg_per_s: Some(0.0),
            supply_mass_flow_rate_read: true,
            supply_mass_flow_rate_kg_per_s: Some(supply),
            mixed_air_temperature_output_reference_bound: true,
            mixed_air_humidity_ratio_output_reference_bound: true,
            mixed_air_enthalpy_output_reference_bound: true,
            operating_mode_read: true,
            operating_mode: None,
            calc_purch_air_mixed_air_called: true,
            purchased_air_alias_bound: true,
            outdoor_air_node_number_copied: true,
            outdoor_air_node: None,
            recirculation_node_number_copied: true,
            recirculation_node: None,
            recirculation_mass_flow_rate_initialized: true,
            initial_recirculation_mass_flow_rate_kg_per_s: Some(0.0),
            recirculation_temperature_read: true,
            recirculation_temperature_c: Some(20.0),
            recirculation_humidity_ratio_read: true,
            recirculation_humidity_ratio: Some(0.008),
            recirculation_enthalpy_projection_read: true,
            recirculation_enthalpy_projection_j_per_kg: Some(40_000.0),
            outdoor_air_initialization_guard_evaluated: true,
            outdoor_air_enabled: Some(false),
            outdoor_air_inlet_temperature_c: Some(0.0),
            outdoor_air_inlet_humidity_ratio: Some(0.0),
            outdoor_air_inlet_enthalpy_j_per_kg: Some(0.0),
            outdoor_air_after_heat_recovery_temperature_c: Some(0.0),
            outdoor_air_after_heat_recovery_humidity_ratio: Some(0.0),
            outdoor_air_after_heat_recovery_enthalpy_j_per_kg: Some(0.0),
            heat_recovery_on_false_assigned: true,
            heat_recovery_on: Some(false),
            outdoor_air_active_guard_first_operand_evaluated: true,
            outdoor_air_mass_flow_positive_comparison_evaluated: false,
            no_outdoor_air_fallback_entered: true,
            child_supply_mass_flow_rate_read: true,
            child_supply_mass_flow_rate_kg_per_s: Some(supply),
            recirculation_mass_flow_rate_assigned_from_supply: true,
            resulting_recirculation_mass_flow_rate_kg_per_s: Some(supply),
            mixed_air_temperature_assigned: true,
            mixed_air_temperature_c: Some(20.0),
            mixed_air_humidity_ratio_assigned: true,
            mixed_air_humidity_ratio: Some(0.008),
            mixed_air_enthalpy_projection_assigned: true,
            mixed_air_enthalpy_projection_j_per_kg: Some(40_000.0),
            heat_recovery_sensible_output_positive_zero_assigned: true,
            heat_recovery_sensible_output_w: Some(0.0),
            heat_recovery_latent_output_positive_zero_assigned: true,
            heat_recovery_latent_output_w: Some(0.0),
        };
        assert!(snapshot_shape(&guard, &predecessor));

        predecessor.child_supply_mass_flow_rate_kg_per_s = Some(0.5);
        assert!(!snapshot_shape(&guard, &predecessor));
    }
}
