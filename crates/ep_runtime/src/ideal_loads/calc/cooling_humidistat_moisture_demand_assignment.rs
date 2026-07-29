//! Bounded Humidistat zone-dehumidifying-setpoint demand assignment evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent;
pub(in crate::ideal_loads) use release::cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use release::cooling_humidistat_moisture_demand_assignment_snapshots_match_bit_exact;
pub use release::{
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
};
pub(in crate::ideal_loads::calc) use release::{
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentActiveOperands,
    advance_cooling_humidistat_moisture_demand_assignment_state,
};

/// EnergyPlus source statement represented by CP359.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2229";
/// First lexically subsequent executable source statement excluded after CP359.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2230";
/// Exact demand read and local assignment sites represented by CP359.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER: &[&str] =
    &[
        "read-zone-dehumidifying-setpoint-moisture-demand",
        "assign-local-zone-dehumidifying-setpoint-moisture-demand",
    ];

/// One CP358-to-CP359 source-ordered Humidistat demand-assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot {
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
    pub predecessor_dehumidification_control_humidistat_case_entered: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_moisture_demand_assignment_executed: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: bool,
    pub zone_dehumidifying_setpoint_moisture_demand_read: bool,
    pub zone_dehumidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub zone_dehumidifying_setpoint_moisture_demand_assigned: bool,
    pub assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
}

/// Final selected-unit CP359 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP359 lifecycle summary.
pub fn purchased_air_calc_cooling_humidistat_moisture_demand_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_humidistat_moisture_demand_assignment
                .clone(),
        },
    )
}
