//! Bounded Cooling positive supply-mass-flow guard from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent;
pub(in crate::ideal_loads::calc) use release::cooling_supply_mass_flow_positive_guard_committed_latest_snapshot_is_consistent;
pub(in crate::ideal_loads::calc) use release::cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate;
pub(in crate::ideal_loads) use release::cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release;
pub(in crate::ideal_loads::calc) use release::positive_guard_links_to_mixed_air_call;
pub(in crate::ideal_loads::calc) use release::snapshots_match_bit_exact as cooling_supply_mass_flow_positive_guard_snapshots_match_bit_exact;
pub use release::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_mass_flow_positive_guard_state;

/// EnergyPlus source slice represented by CP330.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2183";
/// First executable statement deliberately excluded after the guard.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2185";
/// Exact three textual source sites represented by CP330.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-retained-supply-mass-flow-rate",
    "compare-supply-mass-flow-rate-strictly-greater-than-positive-zero",
    "enter-positive-supply-mass-flow-body-if-satisfied",
];

/// One CP329-to-CP330 source-ordered guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_body_entered: bool,
    pub predecessor_cooling_call_executed: bool,
    pub predecessor_zero_flow_reset_body_entered: bool,
    pub predecessor_active_guard_false_fallthrough: bool,
    pub predecessor_no_outdoor_air_fallback_entered: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub cooling_body_entered: bool,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_strictly_positive_comparison_evaluated: bool,
    pub supply_mass_flow_rate_strictly_positive: Option<bool>,
    pub positive_supply_mass_flow_body_entered: bool,
    pub active_guard_false_fallthrough: bool,
}

/// Final selected-unit CP330 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
}

/// Returns the bounded selected-unit CP330 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError::UnknownSystem { system })?;
    Ok(
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_supply_mass_flow_positive_guard.clone(),
        },
    )
}
