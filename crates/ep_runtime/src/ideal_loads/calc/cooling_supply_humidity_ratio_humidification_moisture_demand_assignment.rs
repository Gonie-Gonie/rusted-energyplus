//! Bounded cooling humidification moisture-demand assignment evidence.

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
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentError,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_is_consistent,
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshots_match_bit_exact,
    private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_links_to_direct_release,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentActiveInput,
    advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_state,
};

/// EnergyPlus source statement represented by CP372.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2248";
/// First lexically subsequent executable source statement excluded after CP372.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2249";
/// Exact read and local-assignment sites represented by the single source line.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-zone-humidifying-setpoint-moisture-demand",
    "assign-local-zone-humidifying-setpoint-moisture-demand",
];

/// One CP371-to-CP372 source-ordered humidifying-demand assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot {
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
        bool,
    pub predecessor_dehumidification_control_humidistat_case_completed_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
        bool,
    pub predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
        bool,
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
    pub humidification_moisture_demand_assignment_executed: bool,
    pub zone_humidifying_setpoint_moisture_demand_read: bool,
    pub zone_humidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub zone_humidifying_setpoint_moisture_demand_assigned: bool,
    pub assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
}

/// Final selected-unit CP372 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP372 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentError::UnknownSystem { system },
    )?;
    Ok(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: unit
            .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment
            .clone(),
    })
}
