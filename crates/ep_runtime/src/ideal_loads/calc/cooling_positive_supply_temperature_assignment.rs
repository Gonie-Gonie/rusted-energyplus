//! Bounded Cooling positive-supply temperature assignment from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_temperature_assignment_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentActiveInput,
    advance_cooling_positive_supply_temperature_assignment_state,
};

/// EnergyPlus source statement represented by CP332.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2186";
/// First executable statement deliberately excluded after CP332.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2187";
/// Exact eight textual source sites represented by CP332.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
    "read-zone-cooling-setpoint-load",
    "read-local-cp-air-for-denominator-product",
    "read-retained-supply-mass-flow-rate-for-denominator-product",
    "calculate-cp-air-times-supply-mass-flow-rate",
    "calculate-zone-cooling-setpoint-load-divided-by-denominator-product",
    "read-zone-node-temperature",
    "add-zone-node-temperature-to-load-derived-temperature",
    "assign-purchased-air-supply-temperature",
];

/// One CP331-to-CP332 source-ordered raw supply-temperature assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot {
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
    pub predecessor_active_guard_false_fallthrough: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub supply_temperature_assignment_executed: bool,
    pub zone_cooling_setpoint_load_read: bool,
    pub zone_cooling_setpoint_load_w: Option<f64>,
    pub cp_air_read: bool,
    pub cp_air_j_per_kg_k: Option<f64>,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub cp_air_times_supply_mass_flow_rate_calculated: bool,
    pub cp_air_times_supply_mass_flow_rate_w_per_k: Option<f64>,
    pub zone_cooling_setpoint_load_over_denominator_calculated: bool,
    pub zone_cooling_setpoint_load_over_denominator_c: Option<f64>,
    pub zone_node_temperature_read: bool,
    pub zone_node_temperature_c: Option<f64>,
    pub supply_temperature_calculated: bool,
    pub calculated_supply_temperature_c: Option<f64>,
    pub supply_temperature_assigned: bool,
    pub supply_temperature_c: Option<f64>,
}

/// Final selected-unit CP332 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP332 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_temperature_assignment
                .clone(),
        },
    )
}
