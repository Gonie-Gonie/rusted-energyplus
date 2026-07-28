use ep_model::NodeId;

use crate::ideal_loads::{
    IdealLoadsSensibleMode, IdealLoadsZoneState,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot, moist_air_enthalpy_j_per_kg,
};

pub(super) fn calculation_cooling_mixed_air_call_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    recirculation_node: NodeId,
    recirculation_state: IdealLoadsZoneState,
) -> PurchasedAirCalcCoolingMixedAirCallSnapshot {
    let cooling = predecessor.cooling_body_entered;
    let supply = if cooling {
        predecessor.resulting_supply_mass_flow_rate_kg_per_s
    } else {
        None
    };
    let recirculation_enthalpy = cooling.then(|| {
        moist_air_enthalpy_j_per_kg(
            recirculation_state.air_temperature_c,
            recirculation_state.air_humidity_ratio,
        )
    });
    let zero = cooling.then_some(0.0_f64);

    PurchasedAirCalcCoolingMixedAirCallSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
        child_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
        no_oa_child_source_order:
            PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_zero_flow_reset_body_entered: predecessor.zero_flow_reset_body_entered,
        predecessor_active_guard_false_fallthrough: predecessor.active_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_call_executed: cooling,
        state_reference_bound: cooling,
        purchased_air_number_read: cooling,
        outdoor_air_mass_flow_rate_read: cooling,
        outdoor_air_mass_flow_rate_kg_per_s: zero,
        supply_mass_flow_rate_read: cooling,
        supply_mass_flow_rate_kg_per_s: supply,
        mixed_air_temperature_output_reference_bound: cooling,
        mixed_air_humidity_ratio_output_reference_bound: cooling,
        mixed_air_enthalpy_output_reference_bound: cooling,
        operating_mode_read: cooling,
        operating_mode: cooling.then_some(IdealLoadsSensibleMode::Cooling),
        calc_purch_air_mixed_air_called: cooling,
        purchased_air_alias_bound: cooling,
        outdoor_air_node_number_copied: cooling,
        outdoor_air_node: None,
        recirculation_node_number_copied: cooling,
        recirculation_node: cooling.then_some(recirculation_node),
        recirculation_mass_flow_rate_initialized: cooling,
        initial_recirculation_mass_flow_rate_kg_per_s: zero,
        recirculation_temperature_read: cooling,
        recirculation_temperature_c: cooling.then_some(recirculation_state.air_temperature_c),
        recirculation_humidity_ratio_read: cooling,
        recirculation_humidity_ratio: cooling.then_some(recirculation_state.air_humidity_ratio),
        recirculation_enthalpy_projection_read: cooling,
        recirculation_enthalpy_projection_j_per_kg: recirculation_enthalpy,
        outdoor_air_initialization_guard_evaluated: cooling,
        outdoor_air_enabled: cooling.then_some(false),
        outdoor_air_inlet_temperature_c: zero,
        outdoor_air_inlet_humidity_ratio: zero,
        outdoor_air_inlet_enthalpy_j_per_kg: zero,
        outdoor_air_after_heat_recovery_temperature_c: zero,
        outdoor_air_after_heat_recovery_humidity_ratio: zero,
        outdoor_air_after_heat_recovery_enthalpy_j_per_kg: zero,
        heat_recovery_on_false_assigned: cooling,
        heat_recovery_on: cooling.then_some(false),
        outdoor_air_active_guard_first_operand_evaluated: cooling,
        outdoor_air_mass_flow_positive_comparison_evaluated: false,
        no_outdoor_air_fallback_entered: cooling,
        child_supply_mass_flow_rate_read: cooling,
        child_supply_mass_flow_rate_kg_per_s: supply,
        recirculation_mass_flow_rate_assigned_from_supply: cooling,
        resulting_recirculation_mass_flow_rate_kg_per_s: supply,
        mixed_air_temperature_assigned: cooling,
        mixed_air_temperature_c: cooling.then_some(recirculation_state.air_temperature_c),
        mixed_air_humidity_ratio_assigned: cooling,
        mixed_air_humidity_ratio: cooling.then_some(recirculation_state.air_humidity_ratio),
        mixed_air_enthalpy_projection_assigned: cooling,
        mixed_air_enthalpy_projection_j_per_kg: recirculation_enthalpy,
        heat_recovery_sensible_output_positive_zero_assigned: cooling,
        heat_recovery_sensible_output_w: zero,
        heat_recovery_latent_output_positive_zero_assigned: cooling,
        heat_recovery_latent_output_w: zero,
    }
}
