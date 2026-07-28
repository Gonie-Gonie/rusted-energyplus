//! Bounded cooling supply-mass-flow limit guard from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod body;
mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use body::*;
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_supply_mass_flow_limit_guard_is_consistent;
pub(in crate::ideal_loads) use release::cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_mass_flow_limit_guard_state;

/// EnergyPlus source slice represented by CP325.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2161-2162";
/// First executable source statement deliberately excluded.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2163";
/// Exact seven lexical source sites represented by CP325.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-cooling-limit-for-flow-rate-comparison",
    "compare-cooling-limit-equal-to-flow-rate",
    "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "read-maximum-cooling-air-mass-flow-rate-after-limit-condition-true",
    "compare-maximum-cooling-air-mass-flow-rate-strictly-above-zero",
    "enter-supply-mass-flow-limit-body-if-compound-condition-satisfied",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyMassFlowLimitGuardInput {
    pub cooling_limit: IdealLoadsLimit,
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
}

/// One CP324-to-CP325 source-ordered guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
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
    pub first_cooling_limit_read: bool,
    pub first_cooling_limit: Option<IdealLoadsLimit>,
    pub cooling_limit_flow_rate_comparison_evaluated: bool,
    pub cooling_limit_flow_rate: Option<bool>,
    pub second_cooling_limit_read: bool,
    pub second_cooling_limit: Option<IdealLoadsLimit>,
    pub cooling_limit_flow_rate_and_capacity_comparison_evaluated: bool,
    pub cooling_limit_flow_rate_and_capacity: Option<bool>,
    pub cooling_limit_condition_satisfied: Option<bool>,
    pub maximum_cooling_air_mass_flow_rate_read: bool,
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: Option<f64>,
    pub maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: bool,
    pub maximum_cooling_air_mass_flow_rate_strictly_positive: Option<bool>,
    pub supply_mass_flow_limit_body_entered: bool,
    pub active_guard_false_fallthrough: bool,
}

/// Final selected-unit CP325 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
}

/// Returns the bounded selected-unit CP325 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError::UnknownSystem { system })?;
    Ok(
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_supply_mass_flow_limit_guard.clone(),
        },
    )
}
