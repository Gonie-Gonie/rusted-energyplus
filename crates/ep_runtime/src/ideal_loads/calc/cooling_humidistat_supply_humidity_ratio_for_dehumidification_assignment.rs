//! Bounded Humidistat local dehumidification supply-humidity-ratio assignment evidence.

use crate::ideal_loads::PurchasedAirRuntimeState;
use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent;
pub(in crate::ideal_loads) use release::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentActiveOperands,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_state,
};

/// EnergyPlus source statement represented by CP360.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2230";
/// First lexically subsequent executable source statement excluded after CP360.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2231";
/// Exact six dependency-ordered source sites represented by CP360.
///
/// The side-effect-free reads do not claim C++ operand evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
    "read-local-zone-dehumidifying-setpoint-moisture-demand-for-supply-humidity-ratio-division",
    "read-retained-supply-mass-flow-rate-for-supply-humidity-ratio-division",
    "calculate-zone-dehumidifying-setpoint-moisture-demand-divided-by-supply-mass-flow-rate",
    "read-zone-node-humidity-ratio-for-dehumidification-supply-humidity-ratio",
    "add-zone-node-humidity-ratio-to-moisture-demand-derived-supply-humidity-ratio",
    "assign-local-supply-humidity-ratio-for-dehumidification",
];

/// One CP359-to-CP360 source-ordered local humidity-ratio assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot
{
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
    pub predecessor_dehumidification_control_humidistat_moisture_demand_assignment_executed: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed:
        bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: bool,
    pub zone_dehumidifying_setpoint_moisture_demand_read: bool,
    pub zone_dehumidifying_setpoint_moisture_demand_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub moisture_demand_derived_supply_humidity_ratio_calculated: bool,
    pub moisture_demand_derived_supply_humidity_ratio: Option<f64>,
    pub zone_node_humidity_ratio_read: bool,
    pub zone_node_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_for_dehumidification_calculated: bool,
    pub calculated_supply_humidity_ratio_for_dehumidification: Option<f64>,
    pub supply_humidity_ratio_for_dehumidification_assigned: bool,
    pub assigned_supply_humidity_ratio_for_dehumidification: Option<f64>,
    pub resulting_supply_humidity_ratio_for_dehumidification: Option<f64>,
}

/// Final selected-unit CP360 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP360 lifecycle summary.
pub fn purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError,
>{
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleSummary {
        source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: unit
            .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment
            .clone(),
    })
}
