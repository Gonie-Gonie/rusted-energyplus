//! Bounded `CalcPurchAirLoads` cooling dehumidification-flow calculation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::super::PurchasedAirRuntimeState;

pub(in crate::ideal_loads::calc) mod release;
mod state;
mod transition;

pub use release::*;
pub(super) use state::PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute;
pub use state::PurchasedAirCalcCoolingDehumidificationFlowRuntimeState;
pub(super) use transition::advance_cooling_dehumidification_flow_state;

/// EnergyPlus source slice represented by this bounded transition.
pub const PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2119-2128";

/// Lexically first executable statement deliberately left for the next slice.
pub const PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2133";

/// Exact source-order sites represented by the bounded calculation.
pub const PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER: &[&str] = &[
    "assign-supply-mass-flow-rate-for-dehumidification-zero",
    "read-cooling-on-for-dehumidification",
    "enter-cooling-on-dehumidification-body-if-true",
    "read-dehumidification-control-type",
    "compare-dehumidification-control-type-equal-to-humidistat",
    "enter-humidistat-dehumidification-body-if-matched",
    "read-zone-dehumidifying-setpoint-moisture-demand",
    "assign-local-zone-dehumidifying-setpoint-moisture-demand",
    "read-minimum-cooling-supply-air-humidity-ratio",
    "read-zone-node-humidity-ratio",
    "subtract-zone-humidity-ratio-from-minimum-cooling-supply-air-humidity-ratio",
    "assign-local-delta-humidity-ratio",
    "read-delta-humidity-ratio-for-small-difference-gate",
    "compare-strict-delta-humidity-ratio-below-negative-small-delta-humidity-ratio",
    "read-zone-dehumidifying-setpoint-moisture-demand-after-delta-match",
    "compare-strict-zone-dehumidifying-setpoint-moisture-demand-below-zero",
    "enter-dehumidification-flow-body-if-compound-condition-satisfied",
    "reread-zone-dehumidifying-setpoint-moisture-demand-for-division",
    "reread-delta-humidity-ratio-for-division",
    "calculate-zone-dehumidifying-setpoint-moisture-demand-divided-by-delta-humidity-ratio",
    "assign-supply-mass-flow-rate-for-dehumidification",
];

/// Source `SmallDeltaHumRat`.
pub const PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO: f64 =
    0.00025;

/// Inputs used only by the internal source-characterization transition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingDehumidificationFlowInput {
    pub cooling_on: bool,
    pub dehumidification_control_type: DehumidificationControlType,
    pub zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
    pub minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air: f64,
    pub zone_humidity_ratio_kg_water_per_kg_dry_air: f64,
}

/// One CP318-to-CP319 cooling dehumidification-flow result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingDehumidificationFlowSnapshot {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP318 parent call ordinal consumed by this transition.
    pub parent_call_ordinal: usize,
    /// Source-order sites represented by the bounded route.
    pub source_order: &'static [&'static str],
    /// Controlled Zone inherited from CP318.
    pub controlled_zone: ZoneId,
    /// Whether the enclosing CP310 unit body was entered.
    pub unit_body_entered: bool,
    /// Whether CP312 entered the common Cooling body.
    pub predecessor_cooling_body_entered: bool,
    /// Whether CP318 entered its retained `CoolOn` body.
    pub predecessor_cooling_on_body_entered: bool,
    /// Whether CP318 entered its delta-temperature body.
    pub predecessor_delta_temperature_body_entered: bool,
    /// Whether CP318 assigned its cooling sensible-flow candidate.
    pub predecessor_supply_mass_flow_rate_for_cool_assigned: bool,
    /// Whether UnitOff skipped every CP319 source site.
    pub unit_off_skipped: bool,
    /// Whether an active non-cooling route skipped every CP319 source site.
    pub non_cooling_skipped: bool,
    /// Whether the enclosing Cooling body reached CP319.
    pub cooling_body_entered: bool,
    /// Whether the dehumidification-flow zero reset executed.
    pub supply_mass_flow_rate_for_dehumidification_reset_assigned: bool,
    /// Value assigned by the unconditional Cooling-body reset.
    pub reset_supply_mass_flow_rate_for_dehumidification_kg_per_s: Option<f64>,
    /// Whether retained `CoolOn` was read.
    pub cooling_on_read: bool,
    /// Retained `CoolOn`, absent when Cooling was skipped.
    pub cooling_on: Option<bool>,
    /// Whether the `CoolOn` true body was entered.
    pub cooling_on_body_entered: bool,
    /// Whether the dehumidification-control enum was read.
    pub dehumidification_control_type_read: bool,
    /// Dehumidification-control enum read by the source.
    pub dehumidification_control_type: Option<DehumidificationControlType>,
    /// Result of comparing the control enum with `Humidistat`.
    pub dehumidification_control_type_humidistat: Option<bool>,
    /// Whether the Humidistat dehumidification body was entered.
    pub dehumidification_control_body_entered: bool,
    /// Whether the Zone dehumidifying setpoint demand was read.
    pub zone_dehumidifying_setpoint_moisture_demand_read: bool,
    /// Raw Zone dehumidifying setpoint demand.
    pub zone_dehumidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    /// Whether the demand was assigned to local `MdotZnDehumidSP`.
    pub zone_dehumidifying_setpoint_moisture_demand_assigned: bool,
    /// Value assigned to local `MdotZnDehumidSP`.
    pub assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    /// Whether `MinCoolSuppAirHumRat` was read.
    pub minimum_cooling_supply_air_humidity_ratio_read: bool,
    /// Raw minimum cooling supply-air humidity ratio.
    pub minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air: Option<f64>,
    /// Whether the Zone Node humidity ratio was read.
    pub zone_humidity_ratio_read: bool,
    /// Raw Zone Node humidity ratio.
    pub zone_humidity_ratio_kg_water_per_kg_dry_air: Option<f64>,
    /// Whether minimum-supply-minus-Zone humidity ratio was calculated.
    pub delta_humidity_ratio_calculated: bool,
    /// Raw humidity-ratio subtraction result.
    pub delta_humidity_ratio_kg_water_per_kg_dry_air: Option<f64>,
    /// Whether local `DeltaHumRat` was assigned.
    pub delta_humidity_ratio_assigned: bool,
    /// Value assigned to local `DeltaHumRat`.
    pub assigned_delta_humidity_ratio_kg_water_per_kg_dry_air: Option<f64>,
    /// Whether `DeltaHumRat` was re-read for its strict gate.
    pub delta_humidity_ratio_for_gate_read: bool,
    /// `DeltaHumRat` value read for the strict gate.
    pub delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air: Option<f64>,
    /// Whether the strict delta-humidity-ratio comparison executed.
    pub delta_humidity_ratio_comparison_evaluated: bool,
    /// Result of `DeltaHumRat < -SmallDeltaHumRat`.
    pub delta_humidity_ratio_below_negative_small_delta: Option<bool>,
    /// Whether local `MdotZnDehumidSP` was read after the first predicate.
    pub zone_dehumidifying_setpoint_moisture_demand_for_gate_read: bool,
    /// Local demand value read by the second predicate.
    pub zone_dehumidifying_setpoint_moisture_demand_for_gate_kg_per_s: Option<f64>,
    /// Whether the strict negative-demand comparison executed.
    pub zone_dehumidifying_setpoint_moisture_demand_comparison_evaluated: bool,
    /// Result of `MdotZnDehumidSP < 0.0`.
    pub zone_dehumidifying_setpoint_moisture_demand_below_zero: Option<bool>,
    /// Whether both strict predicates admitted the division body.
    pub dehumidification_flow_body_entered: bool,
    /// Whether local `MdotZnDehumidSP` was re-read for division.
    pub zone_dehumidifying_setpoint_moisture_demand_for_division_read: bool,
    /// Local demand value read for division.
    pub zone_dehumidifying_setpoint_moisture_demand_for_division_kg_per_s: Option<f64>,
    /// Whether local `DeltaHumRat` was re-read for division.
    pub delta_humidity_ratio_for_division_read: bool,
    /// Local delta humidity ratio read for division.
    pub delta_humidity_ratio_for_division_kg_water_per_kg_dry_air: Option<f64>,
    /// Whether the source division was calculated.
    pub supply_mass_flow_rate_for_dehumidification_calculated: bool,
    /// Raw `MdotZnDehumidSP / DeltaHumRat` result.
    pub calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s: Option<f64>,
    /// Whether the calculated candidate was assigned.
    pub supply_mass_flow_rate_for_dehumidification_assigned: bool,
    /// Value assigned by the final source site.
    pub assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s: Option<f64>,
    /// Result after the reset and optional calculation assignment.
    pub resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s: Option<f64>,
}

/// Final selected-unit CP319 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit CP319 state.
    pub state: PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
}

/// Returns the bounded selected-unit CP319 lifecycle summary.
pub fn purchased_air_calc_cooling_dehumidification_flow_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingDehumidificationFlowError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingDehumidificationFlowError::UnknownSystem { system })?;
    Ok(
        PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_dehumidification_flow.clone(),
        },
    )
}
