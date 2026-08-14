//! Bounded committed CP329 sensible-output operand capability.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE as CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER as CHILD_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER as ORDER,
};
use super::runtime_validation::committed_no_oa_humidity_owner_state_is_consistent;
use crate::ideal_loads::{
    IdealLoadsSensibleMode, PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirUnitRuntimeState, moist_air_enthalpy_j_per_kg,
};

/// Sealed same-call CP329 values needed by a sensible-output expression.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingMixedAirCallCommittedSensibleOutputInputs
{
    pub supply_mass_flow_rate_kg_per_s: f64,
    pub mixed_air_temperature_c: f64,
}

/// Returns CP329's committed, bit-corroborated flow and mixed-air temperature.
pub(in crate::ideal_loads::calc) fn cooling_mixed_air_call_committed_latest_sensible_output_inputs(
    unit: &PurchasedAirUnitRuntimeState,
    witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> Option<PurchasedAirCalcCoolingMixedAirCallCommittedSensibleOutputInputs> {
    let latest = unit.calc_cooling_mixed_air_call.latest?;
    let supply = latest.supply_mass_flow_rate_kg_per_s?;
    let mixed_temperature = latest.mixed_air_temperature_c?;
    (committed_no_oa_humidity_owner_state_is_consistent(unit, witness)
        && committed_no_oa_sensible_snapshot_has_exact_shape(latest)
        && supply.to_bits() == latest.child_supply_mass_flow_rate_kg_per_s?.to_bits()
        && supply.to_bits()
            == latest
                .resulting_recirculation_mass_flow_rate_kg_per_s?
                .to_bits()
        && mixed_temperature.to_bits() == latest.recirculation_temperature_c?.to_bits())
    .then_some(
        PurchasedAirCalcCoolingMixedAirCallCommittedSensibleOutputInputs {
            supply_mass_flow_rate_kg_per_s: supply,
            mixed_air_temperature_c: mixed_temperature,
        },
    )
}

/// Returns CP329's committed same-call mixed-air enthalpy projection.
pub(in crate::ideal_loads::calc) fn cooling_mixed_air_call_committed_latest_mixed_air_enthalpy(
    unit: &PurchasedAirUnitRuntimeState,
    witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> Option<f64> {
    let latest = unit.calc_cooling_mixed_air_call.latest?;
    let enthalpy = latest.mixed_air_enthalpy_projection_j_per_kg?;
    (committed_no_oa_humidity_owner_state_is_consistent(unit, witness)
        && committed_no_oa_sensible_snapshot_has_exact_shape(latest))
    .then_some(enthalpy)
}

fn committed_no_oa_sensible_snapshot_has_exact_shape(
    snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let Some(outdoor_flow) = snapshot.outdoor_air_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(supply) = snapshot.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(child_supply) = snapshot.child_supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(recirculation_flow) = snapshot.resulting_recirculation_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(temperature) = snapshot.recirculation_temperature_c else {
        return false;
    };
    let Some(humidity) = snapshot.recirculation_humidity_ratio else {
        return false;
    };
    let Some(enthalpy) = snapshot.recirculation_enthalpy_projection_j_per_kg else {
        return false;
    };
    snapshot.source == SOURCE
        && snapshot.child_source == CHILD_SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.no_oa_child_source_order == CHILD_ORDER
        && !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && (snapshot.predecessor_zero_flow_reset_body_entered
            != snapshot.predecessor_active_guard_false_fallthrough)
        && snapshot.cooling_call_executed
        && snapshot.state_reference_bound
        && snapshot.purchased_air_number_read
        && snapshot.outdoor_air_mass_flow_rate_read
        && outdoor_flow.to_bits() == 0
        && snapshot.supply_mass_flow_rate_read
        && snapshot.mixed_air_temperature_output_reference_bound
        && snapshot.mixed_air_humidity_ratio_output_reference_bound
        && snapshot.mixed_air_enthalpy_output_reference_bound
        && snapshot.operating_mode_read
        && snapshot.operating_mode == Some(IdealLoadsSensibleMode::Cooling)
        && snapshot.calc_purch_air_mixed_air_called
        && snapshot.purchased_air_alias_bound
        && snapshot.outdoor_air_node_number_copied
        && snapshot.outdoor_air_node.is_none()
        && snapshot.recirculation_node_number_copied
        && snapshot.recirculation_node.is_some()
        && snapshot.recirculation_mass_flow_rate_initialized
        && option_is_positive_zero(snapshot.initial_recirculation_mass_flow_rate_kg_per_s)
        && snapshot.recirculation_temperature_read
        && snapshot.recirculation_humidity_ratio_read
        && snapshot.recirculation_enthalpy_projection_read
        && temperature.is_finite()
        && humidity.is_finite()
        && enthalpy.is_finite()
        && enthalpy.to_bits() == moist_air_enthalpy_j_per_kg(temperature, humidity).to_bits()
        && snapshot.outdoor_air_initialization_guard_evaluated
        && snapshot.outdoor_air_enabled == Some(false)
        && [
            snapshot.outdoor_air_inlet_temperature_c,
            snapshot.outdoor_air_inlet_humidity_ratio,
            snapshot.outdoor_air_inlet_enthalpy_j_per_kg,
            snapshot.outdoor_air_after_heat_recovery_temperature_c,
            snapshot.outdoor_air_after_heat_recovery_humidity_ratio,
            snapshot.outdoor_air_after_heat_recovery_enthalpy_j_per_kg,
        ]
        .into_iter()
        .all(option_is_positive_zero)
        && snapshot.heat_recovery_on_false_assigned
        && snapshot.heat_recovery_on == Some(false)
        && snapshot.outdoor_air_active_guard_first_operand_evaluated
        && !snapshot.outdoor_air_mass_flow_positive_comparison_evaluated
        && snapshot.no_outdoor_air_fallback_entered
        && snapshot.child_supply_mass_flow_rate_read
        && child_supply.to_bits() == supply.to_bits()
        && snapshot.recirculation_mass_flow_rate_assigned_from_supply
        && recirculation_flow.to_bits() == supply.to_bits()
        && snapshot.mixed_air_temperature_assigned
        && option_bits_match(snapshot.mixed_air_temperature_c, Some(temperature))
        && snapshot.mixed_air_humidity_ratio_assigned
        && option_bits_match(snapshot.mixed_air_humidity_ratio, Some(humidity))
        && snapshot.mixed_air_enthalpy_projection_assigned
        && option_bits_match(
            snapshot.mixed_air_enthalpy_projection_j_per_kg,
            Some(enthalpy),
        )
        && snapshot.heat_recovery_sensible_output_positive_zero_assigned
        && option_is_positive_zero(snapshot.heat_recovery_sensible_output_w)
        && snapshot.heat_recovery_latent_output_positive_zero_assigned
        && option_is_positive_zero(snapshot.heat_recovery_latent_output_w)
}

fn option_is_positive_zero(value: Option<f64>) -> bool {
    value.is_some_and(|value| value.to_bits() == 0)
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn cooling_mixed_air_call_forge_latest_ordinal_for_test(
    unit: &mut PurchasedAirUnitRuntimeState,
    ordinal: Option<usize>,
) {
    unit.calc_cooling_mixed_air_call.latest_transition_ordinal = ordinal;
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn cooling_mixed_air_call_clear_latest_route_for_test(
    unit: &mut PurchasedAirUnitRuntimeState,
) {
    unit.calc_cooling_mixed_air_call.latest_route = None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn cp329_sensible_owner_hot_path_has_no_recursive_exact_validation() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        assert!(!hot.contains("completed_"));
        assert!(!hot.contains("snapshot_is_exact"));
        assert!(!hot.contains("predecessor_route("));
    }
}
