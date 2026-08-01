//! Bounded cooling humidification supply-humidity-ratio assignment evidence.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId, ZoneId,
};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_links_to_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentActiveOperands,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_state,
};

/// EnergyPlus source statement represented by CP373.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2249";
/// First lexically subsequent executable source statement excluded after CP373.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2250";
/// Exact six dependency-ordered source sites represented by CP373.
///
/// The side-effect-free reads do not claim C++ operand evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-local-zone-humidifying-setpoint-moisture-demand-for-supply-humidity-ratio-division",
    "read-retained-supply-mass-flow-rate-for-supply-humidity-ratio-division",
    "calculate-zone-humidifying-setpoint-moisture-demand-divided-by-supply-mass-flow-rate",
    "read-zone-node-humidity-ratio-for-humidification-supply-humidity-ratio",
    "add-zone-node-humidity-ratio-to-moisture-demand-derived-supply-humidity-ratio",
    "assign-local-supply-humidity-ratio-for-humidification",
];

/// One CP372-to-CP373 source-ordered local humidity-ratio assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_body_entered: bool,
    pub predecessor_cooling_body_entered: bool,
    pub predecessor_no_outdoor_air_fallback_entered: bool,
    pub predecessor_positive_supply_mass_flow_body_entered: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub predecessor_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_dehumidification_control_none_case_completed_skip: bool,
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub predecessor_dehumidification_control_humidistat_case_completed_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: bool,
    pub predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_case_completed_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: bool,
    pub predecessor_heating_on_read: bool,
    pub predecessor_heating_on: Option<bool>,
    pub predecessor_cooling_supply_humidity_ratio_humidification_body_entered: bool,
    pub predecessor_heating_on_guard_false_fallthrough: bool,
    pub predecessor_humidification_control_type_read: bool,
    pub predecessor_humidification_control_type: Option<HumidificationControlType>,
    pub predecessor_humidification_control_type_humidistat: Option<bool>,
    pub predecessor_humidification_control_body_entered: bool,
    pub predecessor_humidification_control_guard_false_fallthrough: bool,
    pub predecessor_dehumidification_control_type_first_read: bool,
    pub predecessor_first_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_dehumidification_control_type_humidistat: Option<bool>,
    pub predecessor_dehumidification_control_type_second_read: bool,
    pub predecessor_second_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_dehumidification_control_type_none: Option<bool>,
    pub predecessor_dehumidification_control_body_entered: bool,
    pub predecessor_dehumidification_control_guard_false_fallthrough: bool,
    pub predecessor_humidification_moisture_demand_assignment_executed: bool,
    pub predecessor_zone_humidifying_setpoint_moisture_demand_read: bool,
    pub predecessor_zone_humidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub predecessor_zone_humidifying_setpoint_moisture_demand_assigned: bool,
    pub predecessor_assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed: bool,
    pub dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed: bool,
    pub zone_humidifying_setpoint_moisture_demand_read: bool,
    pub zone_humidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub moisture_demand_derived_supply_humidity_ratio_calculated: bool,
    pub moisture_demand_derived_supply_humidity_ratio: Option<f64>,
    pub zone_node_humidity_ratio_read: bool,
    pub zone_node_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_for_humidification_calculated: bool,
    pub calculated_supply_humidity_ratio_for_humidification: Option<f64>,
    pub supply_humidity_ratio_for_humidification_assigned: bool,
    pub assigned_supply_humidity_ratio_for_humidification: Option<f64>,
    pub resulting_supply_humidity_ratio_for_humidification: Option<f64>,
}

/// Final selected-unit CP373 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP373 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError::UnknownSystem { system },
    )?;
    Ok(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment.clone(),
    })
}
