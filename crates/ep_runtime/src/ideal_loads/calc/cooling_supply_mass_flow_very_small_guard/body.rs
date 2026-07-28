//! Bounded cooling supply-mass-flow positive-zero reset body from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub(in crate::ideal_loads) use release::cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_mass_flow_very_small_guard_body_state;

/// EnergyPlus source slice represented by CP328.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2167";
/// First executable source statement deliberately excluded.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2171";
/// Exact single lexical source site represented by CP328.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER:
    &[&str] = &["assign-supply-mass-flow-rate-positive-zero"];

/// One CP327-to-CP328 source-ordered body witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
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
    pub predecessor_supply_mass_flow_limit_body_entered: bool,
    pub predecessor_supply_mass_flow_limit_body_skipped: bool,
    pub predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: bool,
    pub predecessor_zero_flow_reset_body_entered: bool,
    pub predecessor_active_guard_false_fallthrough: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub cooling_body_entered: bool,
    pub zero_flow_reset_body_entered: bool,
    pub body_skipped: bool,
    pub active_guard_false_fallthrough: bool,
    pub predecessor_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_positive_zero_assignment_performed: bool,
    pub assigned_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub resulting_supply_mass_flow_rate_kg_per_s: Option<f64>,
}

/// Final selected-unit CP328 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
}

/// Returns the bounded selected-unit CP328 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_supply_mass_flow_very_small_guard_body
                .clone(),
        },
    )
}
