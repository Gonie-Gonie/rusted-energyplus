//! Pure CP328-to-CP329 Cooling mixed-air call transition.

use ep_model::NodeId;

use super::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallRetainedRoute,
    PurchasedAirCalcCoolingMixedAirCallRuntimeState, PurchasedAirCalcCoolingMixedAirCallSnapshot,
};
use crate::ideal_loads::{
    IdealLoadsSensibleMode, PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingMixedAirCallActiveInput {
    pub recirculation_node: NodeId,
    pub recirculation_temperature_c: f64,
    pub recirculation_humidity_ratio: f64,
    pub recirculation_enthalpy_projection_j_per_kg: f64,
    pub outdoor_air_mass_flow_rate_kg_per_s: f64,
    pub supply_mass_flow_rate_kg_per_s: f64,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_mixed_air_call_state(
    state: &mut PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    active_input: Option<PurchasedAirCalcCoolingMixedAirCallActiveInput>,
) -> PurchasedAirCalcCoolingMixedAirCallSnapshot {
    let cooling = predecessor.cooling_body_entered;
    debug_assert_eq!(cooling, active_input.is_some());
    state.transition_count += 1;

    let route = if predecessor.unit_off_skipped {
        state.unit_off_skip_count += 1;
        PurchasedAirCalcCoolingMixedAirCallRetainedRoute::UnitOff
    } else if predecessor.non_cooling_skipped {
        state.non_cooling_skip_count += 1;
        PurchasedAirCalcCoolingMixedAirCallRetainedRoute::NonCooling
    } else {
        state.cooling_call_count += 1;
        state.caller_source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER.len();
        state.child_source_site_execution_count +=
            PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER.len();
        state.state_reference_bind_count += 1;
        state.purchased_air_number_read_count += 1;
        state.outdoor_air_mass_flow_rate_read_count += 1;
        state.supply_mass_flow_rate_read_count += 1;
        state.mixed_air_output_reference_bind_count += 3;
        state.operating_mode_read_count += 1;
        state.mixed_air_child_call_count += 1;
        state.no_outdoor_air_fallback_count += 1;
        state.recirculation_enthalpy_projection_count += 1;
        state.mixed_air_output_assignment_count += 3;
        state.heat_recovery_output_positive_zero_assignment_count += 2;
        PurchasedAirCalcCoolingMixedAirCallRetainedRoute::NoOutdoorAirFallback
    };

    let zero = active_input.map(|_| 0.0_f64);
    let snapshot = PurchasedAirCalcCoolingMixedAirCallSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
        child_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
        no_oa_child_source_order:
            PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
        system: state.system,
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
        outdoor_air_mass_flow_rate_kg_per_s: active_input
            .map(|input| input.outdoor_air_mass_flow_rate_kg_per_s),
        supply_mass_flow_rate_read: cooling,
        supply_mass_flow_rate_kg_per_s: active_input
            .map(|input| input.supply_mass_flow_rate_kg_per_s),
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
        recirculation_node: active_input.map(|input| input.recirculation_node),
        recirculation_mass_flow_rate_initialized: cooling,
        initial_recirculation_mass_flow_rate_kg_per_s: zero,
        recirculation_temperature_read: cooling,
        recirculation_temperature_c: active_input.map(|input| input.recirculation_temperature_c),
        recirculation_humidity_ratio_read: cooling,
        recirculation_humidity_ratio: active_input.map(|input| input.recirculation_humidity_ratio),
        recirculation_enthalpy_projection_read: cooling,
        recirculation_enthalpy_projection_j_per_kg: active_input
            .map(|input| input.recirculation_enthalpy_projection_j_per_kg),
        outdoor_air_initialization_guard_evaluated: cooling,
        outdoor_air_enabled: active_input.map(|_| false),
        outdoor_air_inlet_temperature_c: zero,
        outdoor_air_inlet_humidity_ratio: zero,
        outdoor_air_inlet_enthalpy_j_per_kg: zero,
        outdoor_air_after_heat_recovery_temperature_c: zero,
        outdoor_air_after_heat_recovery_humidity_ratio: zero,
        outdoor_air_after_heat_recovery_enthalpy_j_per_kg: zero,
        heat_recovery_on_false_assigned: cooling,
        heat_recovery_on: active_input.map(|_| false),
        outdoor_air_active_guard_first_operand_evaluated: cooling,
        outdoor_air_mass_flow_positive_comparison_evaluated: false,
        no_outdoor_air_fallback_entered: cooling,
        child_supply_mass_flow_rate_read: cooling,
        child_supply_mass_flow_rate_kg_per_s: active_input
            .map(|input| input.supply_mass_flow_rate_kg_per_s),
        recirculation_mass_flow_rate_assigned_from_supply: cooling,
        resulting_recirculation_mass_flow_rate_kg_per_s: active_input
            .map(|input| input.supply_mass_flow_rate_kg_per_s),
        mixed_air_temperature_assigned: cooling,
        mixed_air_temperature_c: active_input.map(|input| input.recirculation_temperature_c),
        mixed_air_humidity_ratio_assigned: cooling,
        mixed_air_humidity_ratio: active_input.map(|input| input.recirculation_humidity_ratio),
        mixed_air_enthalpy_projection_assigned: cooling,
        mixed_air_enthalpy_projection_j_per_kg: active_input
            .map(|input| input.recirculation_enthalpy_projection_j_per_kg),
        heat_recovery_sensible_output_positive_zero_assigned: cooling,
        heat_recovery_sensible_output_w: zero,
        heat_recovery_latent_output_positive_zero_assigned: cooling,
        heat_recovery_latent_output_w: zero,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
