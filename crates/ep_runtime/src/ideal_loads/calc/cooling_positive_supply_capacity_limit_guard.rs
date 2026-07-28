//! Bounded Cooling positive-supply capacity-limit guard from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardActiveInput,
    advance_cooling_positive_supply_capacity_limit_guard_state,
};

/// EnergyPlus source statement represented by CP337.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2195";
/// First executable statement deliberately excluded after CP337.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2196";
/// Exact five textual source sites represented by CP337.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-cooling-limit-for-capacity-comparison",
    "compare-cooling-limit-equal-to-capacity",
    "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "enter-capacity-limit-body-if-compound-condition-satisfied",
];

/// One CP336-to-CP337 source-ordered capacity-limit guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot {
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
    pub capacity_limit_guard_evaluated: bool,
    pub first_cooling_limit_read: bool,
    pub first_cooling_limit: Option<IdealLoadsLimit>,
    pub cooling_limit_capacity_comparison_evaluated: bool,
    pub cooling_limit_capacity: Option<bool>,
    pub second_cooling_limit_read: bool,
    pub second_cooling_limit: Option<IdealLoadsLimit>,
    pub cooling_limit_flow_rate_and_capacity_comparison_evaluated: bool,
    pub cooling_limit_flow_rate_and_capacity: Option<bool>,
    pub cooling_limit_condition_satisfied: Option<bool>,
    pub cooling_limit_rejected: bool,
    pub capacity_limit_body_entered: bool,
    pub active_guard_false_fallthrough: bool,
}

/// Final selected-unit CP337 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
}

/// Returns the bounded selected-unit CP337 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_capacity_limit_guard
                .clone(),
        },
    )
}
