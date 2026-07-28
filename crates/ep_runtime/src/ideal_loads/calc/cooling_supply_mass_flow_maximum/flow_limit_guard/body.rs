//! Bounded cooling supply-mass-flow limit body from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub(in crate::ideal_loads) use release::cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_mass_flow_limit_body_state;

/// EnergyPlus source slice represented by CP326.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2163";
/// First executable source statement deliberately excluded.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2166";
/// Exact four lexical source sites represented by CP326.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER: &[&str] = &[
    "read-supply-mass-flow-rate-for-minimum",
    "reread-maximum-cooling-air-mass-flow-rate-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-supply-mass-flow-rate",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyMassFlowLimitBodyInput {
    pub supply_mass_flow_rate_before_limit_kg_per_s: Option<f64>,
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
}

/// One CP325-to-CP326 source-ordered body witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_body_entered: bool,
    pub predecessor_cooling_body_entered: bool,
    pub predecessor_ems_supply_mass_flow_override_body_entered: bool,
    pub predecessor_ems_supply_mass_flow_override_body_skipped: bool,
    pub predecessor_ems_disabled_fallthrough: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub cooling_body_entered: bool,
    pub supply_mass_flow_limit_body_entered: bool,
    pub body_skipped: bool,
    pub active_guard_false_fallthrough: bool,
    pub supply_mass_flow_rate_for_minimum_read: bool,
    pub supply_mass_flow_rate_before_limit_kg_per_s: Option<f64>,
    pub maximum_cooling_air_mass_flow_rate_for_minimum_read: bool,
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_assignment_performed: bool,
    pub assigned_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub resulting_supply_mass_flow_rate_kg_per_s: Option<f64>,
}

/// Final selected-unit CP326 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
}

/// Returns the bounded selected-unit CP326 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError::UnknownSystem { system })?;
    Ok(
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_supply_mass_flow_limit_body.clone(),
        },
    )
}
