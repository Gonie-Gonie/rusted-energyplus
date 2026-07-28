//! Fail-closed CP329 snapshot validation.

use ep_model::NodeId;
use ep_runtime::{
    IdealLoadsSensibleMode, PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot, moist_air_enthalpy_j_per_kg,
};

pub(super) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingMixedAirCallSnapshot,
    predecessor: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    expected_recirculation_node: Option<NodeId>,
) -> bool {
    if !predecessor.cooling_body_entered {
        return !snapshot.cooling_call_executed
            && usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
                == 1
            && all_call_sites_skipped(snapshot);
    }

    let Some(predecessor_supply) = predecessor.resulting_supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(recirculation_temperature) = snapshot.recirculation_temperature_c else {
        return false;
    };
    let Some(recirculation_humidity) = snapshot.recirculation_humidity_ratio else {
        return false;
    };
    let Some(recirculation_enthalpy) = snapshot.recirculation_enthalpy_projection_j_per_kg else {
        return false;
    };
    snapshot.cooling_call_executed
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.state_reference_bound
        && snapshot.purchased_air_number_read
        && snapshot.outdoor_air_mass_flow_rate_read
        && option_has_bits(snapshot.outdoor_air_mass_flow_rate_kg_per_s, 0.0)
        && snapshot.supply_mass_flow_rate_read
        && option_has_bits(snapshot.supply_mass_flow_rate_kg_per_s, predecessor_supply)
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
        && snapshot.recirculation_node == expected_recirculation_node
        && snapshot.recirculation_mass_flow_rate_initialized
        && option_has_bits(snapshot.initial_recirculation_mass_flow_rate_kg_per_s, 0.0)
        && snapshot.recirculation_temperature_read
        && recirculation_temperature.is_finite()
        && snapshot.recirculation_humidity_ratio_read
        && recirculation_humidity.is_finite()
        && snapshot.recirculation_enthalpy_projection_read
        && recirculation_enthalpy.is_finite()
        && enthalpy_projection_is_coherent(
            recirculation_temperature,
            recirculation_humidity,
            recirculation_enthalpy,
        )
        && snapshot.outdoor_air_initialization_guard_evaluated
        && snapshot.outdoor_air_enabled == Some(false)
        && six_oa_locals_are_positive_zero(snapshot)
        && snapshot.heat_recovery_on_false_assigned
        && snapshot.heat_recovery_on == Some(false)
        && snapshot.outdoor_air_active_guard_first_operand_evaluated
        && !snapshot.outdoor_air_mass_flow_positive_comparison_evaluated
        && snapshot.no_outdoor_air_fallback_entered
        && snapshot.child_supply_mass_flow_rate_read
        && option_has_bits(
            snapshot.child_supply_mass_flow_rate_kg_per_s,
            predecessor_supply,
        )
        && snapshot.recirculation_mass_flow_rate_assigned_from_supply
        && option_has_bits(
            snapshot.resulting_recirculation_mass_flow_rate_kg_per_s,
            predecessor_supply,
        )
        && snapshot.mixed_air_temperature_assigned
        && option_has_bits(snapshot.mixed_air_temperature_c, recirculation_temperature)
        && snapshot.mixed_air_humidity_ratio_assigned
        && option_has_bits(snapshot.mixed_air_humidity_ratio, recirculation_humidity)
        && snapshot.mixed_air_enthalpy_projection_assigned
        && option_has_bits(
            snapshot.mixed_air_enthalpy_projection_j_per_kg,
            recirculation_enthalpy,
        )
        && snapshot.heat_recovery_sensible_output_positive_zero_assigned
        && option_has_bits(snapshot.heat_recovery_sensible_output_w, 0.0)
        && snapshot.heat_recovery_latent_output_positive_zero_assigned
        && option_has_bits(snapshot.heat_recovery_latent_output_w, 0.0)
}

fn all_call_sites_skipped(snapshot: &PurchasedAirCalcCoolingMixedAirCallSnapshot) -> bool {
    !snapshot.state_reference_bound
        && !snapshot.purchased_air_number_read
        && !snapshot.outdoor_air_mass_flow_rate_read
        && snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.mixed_air_temperature_output_reference_bound
        && !snapshot.mixed_air_humidity_ratio_output_reference_bound
        && !snapshot.mixed_air_enthalpy_output_reference_bound
        && !snapshot.operating_mode_read
        && snapshot.operating_mode.is_none()
        && !snapshot.calc_purch_air_mixed_air_called
        && !snapshot.purchased_air_alias_bound
        && !snapshot.outdoor_air_node_number_copied
        && snapshot.outdoor_air_node.is_none()
        && !snapshot.recirculation_node_number_copied
        && snapshot.recirculation_node.is_none()
        && !snapshot.recirculation_mass_flow_rate_initialized
        && snapshot
            .initial_recirculation_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.recirculation_temperature_read
        && snapshot.recirculation_temperature_c.is_none()
        && !snapshot.recirculation_humidity_ratio_read
        && snapshot.recirculation_humidity_ratio.is_none()
        && !snapshot.recirculation_enthalpy_projection_read
        && snapshot
            .recirculation_enthalpy_projection_j_per_kg
            .is_none()
        && !snapshot.outdoor_air_initialization_guard_evaluated
        && snapshot.outdoor_air_enabled.is_none()
        && snapshot.outdoor_air_inlet_temperature_c.is_none()
        && snapshot.outdoor_air_inlet_humidity_ratio.is_none()
        && snapshot.outdoor_air_inlet_enthalpy_j_per_kg.is_none()
        && snapshot
            .outdoor_air_after_heat_recovery_temperature_c
            .is_none()
        && snapshot
            .outdoor_air_after_heat_recovery_humidity_ratio
            .is_none()
        && snapshot
            .outdoor_air_after_heat_recovery_enthalpy_j_per_kg
            .is_none()
        && !snapshot.heat_recovery_on_false_assigned
        && snapshot.heat_recovery_on.is_none()
        && !snapshot.outdoor_air_active_guard_first_operand_evaluated
        && !snapshot.outdoor_air_mass_flow_positive_comparison_evaluated
        && !snapshot.no_outdoor_air_fallback_entered
        && !snapshot.child_supply_mass_flow_rate_read
        && snapshot.child_supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.recirculation_mass_flow_rate_assigned_from_supply
        && snapshot
            .resulting_recirculation_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.mixed_air_temperature_assigned
        && snapshot.mixed_air_temperature_c.is_none()
        && !snapshot.mixed_air_humidity_ratio_assigned
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.mixed_air_enthalpy_projection_assigned
        && snapshot.mixed_air_enthalpy_projection_j_per_kg.is_none()
        && !snapshot.heat_recovery_sensible_output_positive_zero_assigned
        && snapshot.heat_recovery_sensible_output_w.is_none()
        && !snapshot.heat_recovery_latent_output_positive_zero_assigned
        && snapshot.heat_recovery_latent_output_w.is_none()
}

fn six_oa_locals_are_positive_zero(snapshot: &PurchasedAirCalcCoolingMixedAirCallSnapshot) -> bool {
    [
        snapshot.outdoor_air_inlet_temperature_c,
        snapshot.outdoor_air_inlet_humidity_ratio,
        snapshot.outdoor_air_inlet_enthalpy_j_per_kg,
        snapshot.outdoor_air_after_heat_recovery_temperature_c,
        snapshot.outdoor_air_after_heat_recovery_humidity_ratio,
        snapshot.outdoor_air_after_heat_recovery_enthalpy_j_per_kg,
    ]
    .into_iter()
    .all(|value| option_has_bits(value, 0.0))
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn enthalpy_projection_is_coherent(
    temperature_c: f64,
    humidity_ratio: f64,
    enthalpy_projection_j_per_kg: f64,
) -> bool {
    enthalpy_projection_j_per_kg.to_bits()
        == moist_air_enthalpy_j_per_kg(temperature_c, humidity_ratio).to_bits()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coherent_enthalpy_projection_rejects_a_shared_finite_forgery() {
        let temperature_c = 23.5;
        let humidity_ratio = 0.008;
        let expected = moist_air_enthalpy_j_per_kg(temperature_c, humidity_ratio);
        assert!(enthalpy_projection_is_coherent(
            temperature_c,
            humidity_ratio,
            expected
        ));
        assert!(!enthalpy_projection_is_coherent(
            temperature_c,
            humidity_ratio,
            expected + 1.0
        ));
    }
}
