//! Bounded `CalcPurchAirLoads` cooling humidification-flow calculation.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId, ZoneId,
};

use super::super::PurchasedAirRuntimeState;

pub(in crate::ideal_loads::calc) mod release;
mod state;
mod transition;

pub use release::*;
pub(super) use state::PurchasedAirCalcCoolingHumidificationFlowRetainedRoute;
pub use state::PurchasedAirCalcCoolingHumidificationFlowRuntimeState;
pub(super) use transition::advance_cooling_humidification_flow_state;

/// EnergyPlus source slice represented by CP320.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2133-2144";
/// First executable source statement deliberately excluded.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2147";
/// Source `SmallDeltaHumRat` constant.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SMALL_DELTA_HUMIDITY_RATIO: f64 = 0.00025;

/// Exact 26 source-order sites represented by CP320.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER: &[&str] = &[
    "assign-supply-mass-flow-rate-for-humidification-zero",
    "read-heating-on",
    "enter-heating-on-body-if-true",
    "read-humidification-control-type",
    "compare-humidification-control-type-equal-to-humidistat",
    "enter-humidistat-control-body-if-matched",
    "read-dehumidification-control-type-for-humidistat-comparison",
    "compare-dehumidification-control-type-equal-to-humidistat",
    "read-dehumidification-control-type-for-none-comparison-after-first-false",
    "compare-dehumidification-control-type-equal-to-none",
    "enter-admitted-humidification-body-if-control-condition-satisfied",
    "read-zone-humidifying-setpoint-moisture-demand",
    "assign-local-zone-humidifying-setpoint-moisture-demand",
    "read-maximum-heating-supply-air-humidity-ratio",
    "read-zone-node-humidity-ratio",
    "subtract-zone-humidity-ratio-from-maximum-heating-supply-air-humidity-ratio",
    "assign-local-delta-humidity-ratio",
    "read-delta-humidity-ratio-for-small-difference-gate",
    "compare-strict-delta-humidity-ratio-above-small-delta-humidity-ratio",
    "read-zone-humidifying-setpoint-moisture-demand-after-delta-match",
    "compare-strict-zone-humidifying-setpoint-moisture-demand-above-zero",
    "enter-humidification-flow-body-if-compound-condition-satisfied",
    "reread-zone-humidifying-setpoint-moisture-demand-for-division",
    "reread-delta-humidity-ratio-for-division",
    "calculate-zone-humidifying-setpoint-moisture-demand-divided-by-delta-humidity-ratio",
    "assign-supply-mass-flow-rate-for-humidification",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingHumidificationFlowInput {
    pub heating_on: bool,
    pub humidification_control_type: HumidificationControlType,
    pub dehumidification_control_type: DehumidificationControlType,
    pub zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
    pub maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air: f64,
    pub zone_humidity_ratio_kg_water_per_kg_dry_air: f64,
}

/// One CP319-to-CP320 cooling humidification-flow witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidificationFlowSnapshot {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_body_entered: bool,
    pub predecessor_cooling_body_entered: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub cooling_body_entered: bool,
    pub supply_mass_flow_rate_for_humidification_reset_assigned: bool,
    pub reset_supply_mass_flow_rate_for_humidification_kg_per_s: Option<f64>,
    pub heating_on_read: bool,
    pub heating_on: Option<bool>,
    pub heating_on_body_entered: bool,
    pub humidification_control_type_read: bool,
    pub humidification_control_type: Option<HumidificationControlType>,
    pub humidification_control_type_humidistat: Option<bool>,
    pub humidification_control_body_entered: bool,
    pub dehumidification_control_type_first_read: bool,
    pub first_dehumidification_control_type: Option<DehumidificationControlType>,
    pub dehumidification_control_type_humidistat: Option<bool>,
    pub dehumidification_control_type_second_read: bool,
    pub second_dehumidification_control_type: Option<DehumidificationControlType>,
    pub dehumidification_control_type_none: Option<bool>,
    pub humidification_control_condition_admitted: bool,
    pub zone_humidifying_setpoint_moisture_demand_read: bool,
    pub zone_humidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub zone_humidifying_setpoint_moisture_demand_assigned: bool,
    pub assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub maximum_heating_supply_air_humidity_ratio_read: bool,
    pub maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air: Option<f64>,
    pub zone_humidity_ratio_read: bool,
    pub zone_humidity_ratio_kg_water_per_kg_dry_air: Option<f64>,
    pub delta_humidity_ratio_calculated: bool,
    pub delta_humidity_ratio_kg_water_per_kg_dry_air: Option<f64>,
    pub delta_humidity_ratio_assigned: bool,
    pub assigned_delta_humidity_ratio_kg_water_per_kg_dry_air: Option<f64>,
    pub delta_humidity_ratio_for_gate_read: bool,
    pub delta_humidity_ratio_for_gate_kg_water_per_kg_dry_air: Option<f64>,
    pub delta_humidity_ratio_comparison_evaluated: bool,
    pub delta_humidity_ratio_above_small_delta: Option<bool>,
    pub zone_humidifying_setpoint_moisture_demand_for_gate_read: bool,
    pub zone_humidifying_setpoint_moisture_demand_for_gate_kg_per_s: Option<f64>,
    pub zone_humidifying_setpoint_moisture_demand_comparison_evaluated: bool,
    pub zone_humidifying_setpoint_moisture_demand_above_zero: Option<bool>,
    pub humidification_flow_body_entered: bool,
    pub zone_humidifying_setpoint_moisture_demand_for_division_read: bool,
    pub zone_humidifying_setpoint_moisture_demand_for_division_kg_per_s: Option<f64>,
    pub delta_humidity_ratio_for_division_read: bool,
    pub delta_humidity_ratio_for_division_kg_water_per_kg_dry_air: Option<f64>,
    pub supply_mass_flow_rate_for_humidification_calculated: bool,
    pub calculated_supply_mass_flow_rate_for_humidification_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_for_humidification_assigned: bool,
    pub assigned_supply_mass_flow_rate_for_humidification_kg_per_s: Option<f64>,
    pub resulting_supply_mass_flow_rate_for_humidification_kg_per_s: Option<f64>,
}

/// Final selected-unit CP320 lifecycle summary.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub state: PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
}

/// Returns the bounded selected-unit CP320 lifecycle summary.
pub fn purchased_air_calc_cooling_humidification_flow_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingHumidificationFlowError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingHumidificationFlowError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_humidification_flow.clone(),
    })
}
