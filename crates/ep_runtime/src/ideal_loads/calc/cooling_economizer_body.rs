//! Bounded `CalcPurchAirLoads` cooling economizer true body.

use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};

use super::super::PurchasedAirRuntimeState;

pub(in crate::ideal_loads::calc) mod release;
mod state;
mod transition;

pub use release::*;
pub(super) use state::PurchasedAirCalcCoolingEconomizerBodyRetainedRoute;
pub use state::PurchasedAirCalcCoolingEconomizerBodyRuntimeState;
pub(super) use transition::advance_cooling_economizer_body_state;

/// EnergyPlus source slice represented by this bounded transition.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2089-2101";

/// Lexically first executable statement deliberately left for the next slice.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2109";

/// Exact source-order sites represented by the bounded body.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER: &[&str] = &[
    "read-controlled-zone-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air",
    "read-outdoor-air-node-temperature",
    "read-zone-node-temperature",
    "subtract-zone-temperature-from-outdoor-air-temperature",
    "assign-local-delta-temperature",
    "read-delta-temperature-for-small-temperature-difference-gate",
    "compare-strict-delta-temperature-below-negative-small-temperature-difference",
    "enter-delta-temperature-body-if-satisfied",
    "read-zone-cooling-setpoint-load-after-delta-temperature-match",
    "read-local-cp-air-for-first-division",
    "calculate-zone-cooling-setpoint-load-divided-by-cp-air",
    "read-local-delta-temperature-for-second-division",
    "calculate-first-division-intermediate-divided-by-delta-temperature",
    "assign-initial-supply-mass-flow-rate",
    "read-cooling-limit-for-flow-rate",
    "compare-cooling-limit-equal-to-flow-rate",
    "read-cooling-limit-for-flow-rate-and-capacity-after-short-circuit",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "read-maximum-cooling-air-mass-flow-after-selector-match",
    "compare-strict-maximum-cooling-air-mass-flow-above-zero",
    "enter-maximum-flow-clamp-body-if-satisfied",
    "read-supply-mass-flow-rate-for-inner-maximum",
    "apply-source-shaped-maximum-with-zero",
    "reread-maximum-cooling-air-mass-flow-as-clamp-upper-bound",
    "apply-source-shaped-minimum-with-maximum-cooling-air-mass-flow",
    "assign-clamped-supply-mass-flow-rate",
    "read-resulting-supply-mass-flow-rate",
    "read-current-outdoor-air-mass-flow-rate",
    "compare-strict-supply-mass-flow-above-outdoor-air-mass-flow",
    "enter-economizer-activation-body-if-satisfied",
    "assign-economizer-on-true-after-mass-flow-match",
    "reread-supply-mass-flow-for-outdoor-air-mass-flow-assignment",
    "assign-outdoor-air-mass-flow-from-supply-mass-flow",
    "read-system-time-step",
    "assign-economizer-active-time",
];

/// `HVAC::SmallTempDiff` used by the source strict comparison.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SMALL_TEMP_DIFF_C: f64 = 1.0e-5;

/// Inputs used only by the internal source-characterization transition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingEconomizerBodyInput {
    pub zone_humidity_ratio: f64,
    pub outdoor_air_temperature_c: f64,
    pub zone_temperature_c: f64,
    pub zone_cooling_setpoint_load_w: f64,
    pub cooling_limit: IdealLoadsLimit,
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    pub outdoor_air_mass_flow_rate_kg_per_s: f64,
    pub system_time_step_hours: f64,
}

/// One CP316-to-CP317 cooling economizer body result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEconomizerBodySnapshot {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP316 parent call ordinal consumed by this transition.
    pub parent_call_ordinal: usize,
    /// Source-order sites represented by the bounded route.
    pub source_order: &'static [&'static str],
    /// Controlled Zone inherited from CP316.
    pub controlled_zone: ZoneId,
    /// Whether the enclosing CP310 unit body was entered.
    pub unit_body_entered: bool,
    /// Whether CP312 entered cooling.
    pub predecessor_cooling_body_entered: bool,
    /// Whether CP313 admitted the CP314 sibling body.
    pub predecessor_maximum_cooling_flow_body_entered: bool,
    /// Whether CP314 fell through toward the economizer path.
    pub predecessor_active_guard_false_economizer_fallthrough: bool,
    /// Whether CP315 evaluated its outer economizer guard.
    pub predecessor_economizer_guard_evaluated: bool,
    /// Whether CP315 admitted the CP316 inner condition.
    pub predecessor_economizer_body_entered: bool,
    /// Whether CP315's `NoEconomizer` comparison fell through.
    pub predecessor_no_economizer_fallthrough: bool,
    /// Whether CP316 evaluated its compound economizer condition.
    pub predecessor_economizer_condition_evaluated: bool,
    /// Result of CP316's compound condition, absent when CP316 was skipped.
    pub predecessor_economizer_condition_satisfied: Option<bool>,
    /// Whether CP316 admitted this true body.
    pub predecessor_economizer_calculation_body_entered: bool,
    /// Whether UnitOff skipped every CP317 source site.
    pub unit_off_skipped: bool,
    /// Whether an active non-cooling route skipped every CP317 site.
    pub non_cooling_skipped: bool,
    /// Whether the CP313 true sibling skipped every CP317 site.
    pub maximum_cooling_flow_body_sibling_skipped: bool,
    /// Whether a false CP315 outer guard skipped every CP317 site.
    pub no_economizer_outer_guard_fallthrough_skipped: bool,
    /// Whether CP316 evaluated false and skipped every CP317 site.
    pub economizer_condition_fallthrough_skipped: bool,
    /// Whether the CP317 true body executed.
    pub economizer_calculation_body_executed: bool,
    /// Whether the controlled Zone humidity ratio was read.
    pub zone_humidity_ratio_read: bool,
    /// Raw controlled Zone humidity ratio, absent when the body was skipped.
    pub zone_humidity_ratio: Option<f64>,
    /// Whether `PsyCpAirFnW` was evaluated.
    pub psychrometric_cp_air_evaluated: bool,
    /// Raw result returned by `PsyCpAirFnW`.
    pub psychrometric_cp_air_result_j_per_kg_k: Option<f64>,
    /// Whether the local `CpAir` assignment executed.
    pub cp_air_assigned: bool,
    /// Local `CpAir`, absent when the body was skipped.
    pub cp_air_j_per_kg_k: Option<f64>,
    /// Whether the outdoor-air Node temperature was read.
    pub outdoor_air_temperature_read: bool,
    /// Raw outdoor-air Node temperature, absent when the body was skipped.
    pub outdoor_air_temperature_c: Option<f64>,
    /// Whether the Zone Node temperature was read.
    pub zone_temperature_read: bool,
    /// Raw Zone Node temperature, absent when the body was skipped.
    pub zone_temperature_c: Option<f64>,
    /// Whether `DeltaT` was calculated.
    pub delta_temperature_calculated: bool,
    /// Source subtraction result, absent when the body was skipped.
    pub delta_temperature_c: Option<f64>,
    /// Whether the local `DeltaT` assignment executed.
    pub delta_temperature_assigned: bool,
    /// Value assigned to local `DeltaT`.
    pub assigned_delta_temperature_c: Option<f64>,
    /// Whether `DeltaT` was read for the strict gate.
    pub delta_temperature_for_gate_read: bool,
    /// `DeltaT` value read for the strict gate.
    pub delta_temperature_for_gate_c: Option<f64>,
    /// Whether the strict `DeltaT < -SmallTempDiff` comparison executed.
    pub delta_temperature_comparison_evaluated: bool,
    /// Result of the strict delta-temperature comparison.
    pub delta_temperature_below_negative_small_temp_diff: Option<bool>,
    /// Whether the true delta-temperature body was entered.
    pub delta_temperature_body_entered: bool,
    /// Whether `QZnCoolSP` was read after a true delta-temperature comparison.
    pub zone_cooling_setpoint_load_read: bool,
    /// Raw `QZnCoolSP`, absent when short-circuited.
    pub zone_cooling_setpoint_load_w: Option<f64>,
    /// Whether local `CpAir` was read for the first division.
    pub cp_air_for_first_division_read: bool,
    /// Local `CpAir` read for the first division.
    pub cp_air_for_first_division_j_per_kg_k: Option<f64>,
    /// Whether `QZnCoolSP / CpAir` was calculated.
    pub zone_cooling_setpoint_load_over_cp_air_calculated: bool,
    /// Exact first-division intermediate.
    pub zone_cooling_setpoint_load_over_cp_air_kg_k_per_s: Option<f64>,
    /// Whether local `DeltaT` was read for the second division.
    pub delta_temperature_for_second_division_read: bool,
    /// Local `DeltaT` read for the second division.
    pub delta_temperature_for_second_division_c: Option<f64>,
    /// Whether the source left-associated supply-flow calculation executed.
    pub supply_mass_flow_rate_calculated: bool,
    /// Raw result of `(QZnCoolSP / CpAir) / DeltaT`.
    pub calculated_supply_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether the initial `SupplyMassFlowRate` assignment executed.
    pub initial_supply_mass_flow_rate_assigned: bool,
    /// Initially assigned `SupplyMassFlowRate`.
    pub initial_supply_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether the first `CoolingLimit == FlowRate` comparison executed.
    pub cooling_limit_flow_rate_comparison_evaluated: bool,
    /// Whether the first comparison read `CoolingLimit`.
    pub cooling_limit_flow_rate_read: bool,
    /// Cooling limit read by the first comparison.
    pub cooling_limit_flow_rate_value: Option<IdealLoadsLimit>,
    /// Result of the first selector comparison.
    pub cooling_limit_flow_rate_comparison_satisfied: Option<bool>,
    /// Whether `||` short-circuiting reached the second selector comparison.
    pub cooling_limit_flow_rate_and_capacity_comparison_evaluated: bool,
    /// Whether the second comparison re-read `CoolingLimit`.
    pub cooling_limit_flow_rate_and_capacity_read: bool,
    /// Cooling limit re-read by the second comparison.
    pub cooling_limit_flow_rate_and_capacity_value: Option<IdealLoadsLimit>,
    /// Result of the second selector comparison.
    pub cooling_limit_flow_rate_and_capacity_comparison_satisfied: Option<bool>,
    /// Result of the complete flow-limit selector, absent when short-circuited.
    pub cooling_flow_limit_active: Option<bool>,
    /// Whether the cached maximum cooling mass flow was read.
    pub maximum_cooling_air_mass_flow_rate_read: bool,
    /// Raw cached `MaxCoolMassFlowRate`, absent when short-circuited.
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether the strict maximum-flow-above-zero comparison executed.
    pub maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: bool,
    /// Result of `MaxCoolMassFlowRate > 0.0`.
    pub maximum_cooling_air_mass_flow_rate_positive: Option<bool>,
    /// Whether the maximum-flow clamp body was entered.
    pub maximum_flow_clamp_body_entered: bool,
    /// Whether the optional source-shaped clamp operation executed.
    pub supply_mass_flow_rate_clamped: bool,
    /// Whether `SupplyMassFlowRate` was read for the inner maximum.
    pub supply_mass_flow_rate_for_clamp_read: bool,
    /// `SupplyMassFlowRate` value read for the inner maximum.
    pub supply_mass_flow_rate_for_clamp_kg_per_s: Option<f64>,
    /// Whether the source-shaped inner maximum executed.
    pub inner_max_evaluated: bool,
    /// Source `max(SupplyMassFlowRate, 0.0)` intermediate.
    pub nonnegative_supply_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether line 2095 re-read the maximum flow as the clamp upper bound.
    pub maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read: bool,
    /// Maximum flow re-read as the clamp upper bound, absent when short-circuited.
    pub maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s: Option<f64>,
    /// Whether the source-shaped outer minimum executed.
    pub outer_min_evaluated: bool,
    /// Result assigned by the source-shaped outer minimum.
    pub clamped_supply_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether clamped `SupplyMassFlowRate` was assigned.
    pub clamped_supply_mass_flow_rate_assigned: bool,
    /// Resulting supply flow after the optional clamp.
    pub resulting_supply_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether the resulting supply mass flow was read for the final comparison.
    pub resulting_supply_mass_flow_rate_read: bool,
    /// Whether the current outdoor-air mass flow was read for the final comparison.
    pub outdoor_air_mass_flow_rate_read: bool,
    /// Current outdoor-air mass flow, absent when short-circuited.
    pub outdoor_air_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether the strict final mass-flow comparison executed.
    pub supply_above_outdoor_air_mass_flow_comparison_evaluated: bool,
    /// Result of `SupplyMassFlowRate > OAMassFlowRate`.
    pub supply_mass_flow_above_outdoor_air_mass_flow: Option<bool>,
    /// Whether the economizer activation body was entered.
    pub economizer_activation_body_entered: bool,
    /// Whether `EconoOn = true` executed.
    pub economizer_on_assigned: bool,
    /// Assigned economizer value, absent when the assignment was skipped.
    pub economizer_on: Option<bool>,
    /// Whether line 2099 re-read supply flow as the outdoor-air assignment source.
    pub supply_mass_flow_rate_for_outdoor_air_assignment_read: bool,
    /// Supply flow re-read as the outdoor-air assignment source.
    pub supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s: Option<f64>,
    /// Whether supply mass flow was copied to outdoor-air mass flow.
    pub outdoor_air_mass_flow_rate_assigned: bool,
    /// Assigned outdoor-air mass flow, absent when the assignment was skipped.
    pub assigned_outdoor_air_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether `TimeStepSys` was read.
    pub system_time_step_read: bool,
    /// Raw system timestep, absent when the read was skipped.
    pub system_time_step_hours: Option<f64>,
    /// Whether `TimeEconoActive = TimeStepSys` executed.
    pub economizer_active_time_assigned: bool,
    /// Assigned economizer-active time, absent when skipped.
    pub assigned_economizer_active_time_hours: Option<f64>,
}

/// Final selected-unit CP317 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
}

/// Returns the bounded selected-unit CP317 lifecycle summary.
pub fn purchased_air_calc_cooling_economizer_body_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerBodyError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingEconomizerBodyError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_economizer_body.clone(),
    })
}
