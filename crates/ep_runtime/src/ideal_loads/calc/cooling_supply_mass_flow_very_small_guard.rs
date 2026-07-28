//! Bounded cooling supply-mass-flow very-small guard from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod body;
mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use body::*;
pub(in crate::ideal_loads) use release::cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard,
};
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_mass_flow_very_small_guard_is_consistent,
    snapshots_match_bit_exact as cooling_supply_mass_flow_very_small_guard_snapshots_match_bit_exact,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_mass_flow_very_small_guard_state;

/// EnergyPlus source slice represented by CP327.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2166";
/// First executable source statement deliberately excluded.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2167";
/// EnergyPlus declaration that supplies `HVAC::VerySmallMassFlow`.
pub const ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE: &str =
    "EnergyPlus 26.1 DataHVACGlobals.hh:89";
/// Exact EnergyPlus 26.1 `HVAC::VerySmallMassFlow` value in kg/s.
pub const ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S: f64 = 1.0e-30;
/// Exact four textual source sites represented by CP327.
///
/// This is a lexical inventory and makes no claim about C++ operand
/// evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-retained-supply-mass-flow-rate",
    "read-hvac-very-small-mass-flow",
    "compare-supply-mass-flow-rate-less-than-or-equal-to-hvac-very-small-mass-flow",
    "enter-zero-flow-reset-body-if-at-or-below-threshold",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardInput {
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
}

/// One CP326-to-CP327 source-ordered guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
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
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub cooling_body_entered: bool,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub hvac_very_small_mass_flow_read: bool,
    pub hvac_very_small_mass_flow_source: Option<&'static str>,
    pub hvac_very_small_mass_flow_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated: bool,
    pub supply_mass_flow_rate_at_or_below_very_small_mass_flow: Option<bool>,
    pub zero_flow_reset_body_entered: bool,
    pub active_guard_false_fallthrough: bool,
}

/// Final selected-unit CP327 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
}

/// Returns the bounded selected-unit CP327 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_supply_mass_flow_very_small_guard.clone(),
        },
    )
}
