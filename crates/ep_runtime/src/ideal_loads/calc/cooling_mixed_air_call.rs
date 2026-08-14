//! Bounded Cooling `CalcPurchAirMixedAir` call and no-outdoor-air child route.

use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneId};

use crate::ideal_loads::{IdealLoadsSensibleMode, PurchasedAirRuntimeState};

mod release;
#[cfg(test)]
pub(in crate::ideal_loads::calc) mod release_tests;
mod state;
#[cfg(test)]
pub(in crate::ideal_loads::calc) mod tests;
mod transition;

pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_mixed_air_call_is_consistent;
pub(in crate::ideal_loads::calc) use release::cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio;
pub(in crate::ideal_loads::calc) use release::{
    PurchasedAirCalcCoolingMixedAirCallCommittedSensibleOutputInputs,
    cooling_mixed_air_call_committed_latest_mixed_air_enthalpy,
    cooling_mixed_air_call_committed_latest_sensible_output_inputs,
};
pub use release::{
    PurchasedAirCalcCoolingMixedAirCallError,
    PurchasedAirCalcCoolingMixedAirCallRecirculationInput,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use release::{
    cooling_mixed_air_call_clear_latest_route_for_test,
    cooling_mixed_air_call_forge_latest_ordinal_for_test,
};
pub(in crate::ideal_loads) use release::{
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};
pub(super) use state::PurchasedAirCalcCoolingMixedAirCallRetainedRoute;
pub use state::PurchasedAirCalcCoolingMixedAirCallRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingMixedAirCallActiveInput, advance_cooling_mixed_air_call_state,
};

/// Cooling caller source slice represented by CP329.
pub const PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2171-2178";
/// `CalcPurchAirMixedAir` child definition and exact direct no-OA route.
pub const PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2812-2939; bounded no-OA route 2851,2854-2861,2869-2874,2876,2878,2932-2937";
/// First executable source statement deliberately excluded after the call returns.
pub const PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2183";
/// Nine textual caller sites; this inventory claims no C++ argument evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER: &[&str] = &[
    "bind-state-reference",
    "read-purchased-air-number",
    "read-outdoor-air-mass-flow-rate",
    "read-supply-mass-flow-rate",
    "bind-mixed-air-temperature-output-reference",
    "bind-mixed-air-humidity-ratio-output-reference",
    "bind-mixed-air-enthalpy-output-reference",
    "read-operating-mode",
    "call-calc-purch-air-mixed-air",
];
/// Twenty-two source heads/statements executed by the exact direct no-OA child route.
pub const PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER: &[&str] = &[
    "bind-purchased-air-alias",
    "copy-outdoor-air-node-number",
    "copy-recirculation-node-number",
    "initialize-recirculation-mass-flow-rate-positive-zero",
    "read-recirculation-temperature",
    "read-recirculation-humidity-ratio",
    "read-recirculation-enthalpy-projection",
    "evaluate-outdoor-air-initialization-guard",
    "assign-outdoor-air-inlet-temperature-positive-zero",
    "assign-outdoor-air-inlet-humidity-ratio-positive-zero",
    "assign-outdoor-air-inlet-enthalpy-positive-zero",
    "assign-outdoor-air-after-heat-recovery-temperature",
    "assign-outdoor-air-after-heat-recovery-humidity-ratio",
    "assign-outdoor-air-after-heat-recovery-enthalpy",
    "assign-heat-recovery-on-false",
    "evaluate-outdoor-air-active-guard-first-operand",
    "assign-recirculation-mass-flow-rate-from-supply",
    "assign-mixed-air-temperature",
    "assign-mixed-air-humidity-ratio",
    "assign-mixed-air-enthalpy-projection",
    "assign-heat-recovery-sensible-output-positive-zero",
    "assign-heat-recovery-latent-output-positive-zero",
];

/// One CP328-to-CP329 caller and bounded-child witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingMixedAirCallSnapshot {
    pub source: &'static str,
    pub child_source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub no_oa_child_source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_body_entered: bool,
    pub predecessor_cooling_body_entered: bool,
    pub predecessor_zero_flow_reset_body_entered: bool,
    pub predecessor_active_guard_false_fallthrough: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub cooling_call_executed: bool,
    pub state_reference_bound: bool,
    pub purchased_air_number_read: bool,
    pub outdoor_air_mass_flow_rate_read: bool,
    pub outdoor_air_mass_flow_rate_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub mixed_air_temperature_output_reference_bound: bool,
    pub mixed_air_humidity_ratio_output_reference_bound: bool,
    pub mixed_air_enthalpy_output_reference_bound: bool,
    pub operating_mode_read: bool,
    pub operating_mode: Option<IdealLoadsSensibleMode>,
    pub calc_purch_air_mixed_air_called: bool,
    pub purchased_air_alias_bound: bool,
    pub outdoor_air_node_number_copied: bool,
    pub outdoor_air_node: Option<NodeId>,
    pub recirculation_node_number_copied: bool,
    pub recirculation_node: Option<NodeId>,
    pub recirculation_mass_flow_rate_initialized: bool,
    pub initial_recirculation_mass_flow_rate_kg_per_s: Option<f64>,
    pub recirculation_temperature_read: bool,
    pub recirculation_temperature_c: Option<f64>,
    pub recirculation_humidity_ratio_read: bool,
    pub recirculation_humidity_ratio: Option<f64>,
    pub recirculation_enthalpy_projection_read: bool,
    pub recirculation_enthalpy_projection_j_per_kg: Option<f64>,
    pub outdoor_air_initialization_guard_evaluated: bool,
    pub outdoor_air_enabled: Option<bool>,
    pub outdoor_air_inlet_temperature_c: Option<f64>,
    pub outdoor_air_inlet_humidity_ratio: Option<f64>,
    pub outdoor_air_inlet_enthalpy_j_per_kg: Option<f64>,
    pub outdoor_air_after_heat_recovery_temperature_c: Option<f64>,
    pub outdoor_air_after_heat_recovery_humidity_ratio: Option<f64>,
    pub outdoor_air_after_heat_recovery_enthalpy_j_per_kg: Option<f64>,
    pub heat_recovery_on_false_assigned: bool,
    pub heat_recovery_on: Option<bool>,
    pub outdoor_air_active_guard_first_operand_evaluated: bool,
    pub outdoor_air_mass_flow_positive_comparison_evaluated: bool,
    pub no_outdoor_air_fallback_entered: bool,
    pub child_supply_mass_flow_rate_read: bool,
    pub child_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub recirculation_mass_flow_rate_assigned_from_supply: bool,
    pub resulting_recirculation_mass_flow_rate_kg_per_s: Option<f64>,
    pub mixed_air_temperature_assigned: bool,
    pub mixed_air_temperature_c: Option<f64>,
    pub mixed_air_humidity_ratio_assigned: bool,
    pub mixed_air_humidity_ratio: Option<f64>,
    pub mixed_air_enthalpy_projection_assigned: bool,
    pub mixed_air_enthalpy_projection_j_per_kg: Option<f64>,
    pub heat_recovery_sensible_output_positive_zero_assigned: bool,
    pub heat_recovery_sensible_output_w: Option<f64>,
    pub heat_recovery_latent_output_positive_zero_assigned: bool,
    pub heat_recovery_latent_output_w: Option<f64>,
}

/// Final selected-unit CP329 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingMixedAirCallLifecycleSummary {
    /// Cooling caller source slice.
    pub source: &'static str,
    /// Bounded child dependency.
    pub child_source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingMixedAirCallRuntimeState,
}

/// Returns the bounded selected-unit CP329 lifecycle summary.
pub fn purchased_air_calc_cooling_mixed_air_call_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingMixedAirCallError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingMixedAirCallLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
        child_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_mixed_air_call.clone(),
    })
}
