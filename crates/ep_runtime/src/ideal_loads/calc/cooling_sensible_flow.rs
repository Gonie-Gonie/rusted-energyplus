//! Bounded `CalcPurchAirLoads` cooling sensible-flow calculation.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::super::PurchasedAirRuntimeState;

pub(in crate::ideal_loads::calc) mod release;
mod state;
mod transition;

pub use release::*;
pub(super) use state::PurchasedAirCalcCoolingSensibleFlowRetainedRoute;
pub use state::PurchasedAirCalcCoolingSensibleFlowRuntimeState;
pub(super) use transition::advance_cooling_sensible_flow_state;

/// EnergyPlus source slice represented by this bounded transition.
pub const PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2109-2116";

/// Lexically first executable statement deliberately left for the next slice.
pub const PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2119";

/// Exact source-order sites represented by the bounded calculation.
pub const PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER: &[&str] = &[
    "assign-supply-mass-flow-rate-for-cool-zero",
    "read-cooling-on",
    "enter-cooling-on-body-if-true",
    "read-controlled-zone-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air",
    "read-minimum-cooling-supply-air-temperature",
    "read-zone-node-temperature",
    "subtract-zone-temperature-from-minimum-cooling-supply-air-temperature",
    "assign-local-delta-temperature",
    "read-delta-temperature-for-small-temperature-difference-gate",
    "compare-strict-delta-temperature-below-negative-small-temperature-difference",
    "enter-delta-temperature-body-if-satisfied",
    "read-zone-cooling-setpoint-load-after-delta-temperature-match",
    "read-local-cp-air-for-first-division",
    "calculate-zone-cooling-setpoint-load-divided-by-cp-air",
    "read-local-delta-temperature-for-second-division",
    "calculate-first-division-intermediate-divided-by-delta-temperature",
    "assign-supply-mass-flow-rate-for-cool",
];

/// `HVAC::SmallTempDiff` used by the source strict comparison.
pub const PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SMALL_TEMP_DIFF_C: f64 = 1.0e-5;

/// Inputs used only by the internal source-characterization transition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSensibleFlowInput {
    pub cooling_on: bool,
    pub zone_humidity_ratio: f64,
    pub minimum_cooling_supply_air_temperature_c: f64,
    pub zone_temperature_c: f64,
    pub zone_cooling_setpoint_load_w: f64,
}

/// One CP317-to-CP318 cooling sensible-flow result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSensibleFlowSnapshot {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP317 parent call ordinal consumed by this transition.
    pub parent_call_ordinal: usize,
    /// Source-order sites represented by the bounded route.
    pub source_order: &'static [&'static str],
    /// Controlled Zone inherited from CP317.
    pub controlled_zone: ZoneId,
    /// Whether the enclosing CP310 unit body was entered.
    pub unit_body_entered: bool,
    /// Whether CP312 entered cooling.
    pub predecessor_cooling_body_entered: bool,
    /// Whether the CP313 true sibling skipped CP317.
    pub predecessor_maximum_cooling_flow_body_sibling_skipped: bool,
    /// Whether a false CP315 outer guard skipped CP317.
    pub predecessor_no_economizer_outer_guard_fallthrough_skipped: bool,
    /// Whether CP316 evaluated false and skipped CP317.
    pub predecessor_economizer_condition_fallthrough_skipped: bool,
    /// Whether the CP317 economizer calculation body executed.
    pub predecessor_economizer_calculation_body_executed: bool,
    /// Whether UnitOff skipped every CP318 source site.
    pub unit_off_skipped: bool,
    /// Whether an active non-cooling route skipped every CP318 source site.
    pub non_cooling_skipped: bool,
    /// Whether the enclosing cooling body reached CP318.
    pub cooling_body_entered: bool,
    /// Whether `SupplyMassFlowRateForCool = 0.0` executed.
    pub supply_mass_flow_rate_for_cool_reset_assigned: bool,
    /// Value assigned by the unconditional cooling-body reset.
    pub reset_supply_mass_flow_rate_for_cool_kg_per_s: Option<f64>,
    /// Whether retained `CoolOn` was read.
    pub cooling_on_read: bool,
    /// Retained `CoolOn`, absent when the cooling body was skipped.
    pub cooling_on: Option<bool>,
    /// Whether the `CoolOn` true body was entered.
    pub cooling_on_body_entered: bool,
    /// Whether the controlled Zone humidity ratio was read.
    pub zone_humidity_ratio_read: bool,
    /// Raw controlled Zone humidity ratio.
    pub zone_humidity_ratio: Option<f64>,
    /// Whether `PsyCpAirFnW` was evaluated.
    pub psychrometric_cp_air_evaluated: bool,
    /// Raw result returned by `PsyCpAirFnW`.
    pub psychrometric_cp_air_result_j_per_kg_k: Option<f64>,
    /// Whether local `CpAir` was assigned.
    pub cp_air_assigned: bool,
    /// Assigned local `CpAir`.
    pub cp_air_j_per_kg_k: Option<f64>,
    /// Whether `MinCoolSuppAirTemp` was read.
    pub minimum_cooling_supply_air_temperature_read: bool,
    /// Raw minimum cooling supply-air temperature.
    pub minimum_cooling_supply_air_temperature_c: Option<f64>,
    /// Whether the Zone Node temperature was read.
    pub zone_temperature_read: bool,
    /// Raw Zone Node temperature.
    pub zone_temperature_c: Option<f64>,
    /// Whether `DeltaT` was calculated.
    pub delta_temperature_calculated: bool,
    /// Source subtraction result.
    pub delta_temperature_c: Option<f64>,
    /// Whether local `DeltaT` was assigned.
    pub delta_temperature_assigned: bool,
    /// Value assigned to local `DeltaT`.
    pub assigned_delta_temperature_c: Option<f64>,
    /// Whether `DeltaT` was re-read for the strict gate.
    pub delta_temperature_for_gate_read: bool,
    /// `DeltaT` value read for the strict gate.
    pub delta_temperature_for_gate_c: Option<f64>,
    /// Whether the strict `DeltaT < -SmallTempDiff` comparison executed.
    pub delta_temperature_comparison_evaluated: bool,
    /// Result of the strict delta-temperature comparison.
    pub delta_temperature_below_negative_small_temp_diff: Option<bool>,
    /// Whether the true delta-temperature body was entered.
    pub delta_temperature_body_entered: bool,
    /// Whether `QZnCoolSP` was read.
    pub zone_cooling_setpoint_load_read: bool,
    /// Raw `QZnCoolSP`.
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
    pub supply_mass_flow_rate_for_cool_calculated: bool,
    /// Raw result of `(QZnCoolSP / CpAir) / DeltaT`.
    pub calculated_supply_mass_flow_rate_for_cool_kg_per_s: Option<f64>,
    /// Whether `SupplyMassFlowRateForCool` was assigned the calculation.
    pub supply_mass_flow_rate_for_cool_assigned: bool,
    /// Value assigned by the final source site.
    pub assigned_supply_mass_flow_rate_for_cool_kg_per_s: Option<f64>,
    /// Result after the reset and optional calculation assignment.
    pub resulting_supply_mass_flow_rate_for_cool_kg_per_s: Option<f64>,
}

/// Final selected-unit CP318 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSensibleFlowLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSensibleFlowRuntimeState,
}

/// Returns the bounded selected-unit CP318 lifecycle summary.
pub fn purchased_air_calc_cooling_sensible_flow_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
    PurchasedAirCalcCoolingSensibleFlowError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingSensibleFlowError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingSensibleFlowLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_sensible_flow.clone(),
    })
}
