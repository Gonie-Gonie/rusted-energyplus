//! Bounded Cooling capacity-limit sensible-output guard from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
pub(in crate::ideal_loads::calc) mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent;
pub(in crate::ideal_loads::calc) use release::cooling_positive_supply_capacity_limit_sensible_output_guard_committed_latest_maximum_total_cooling_capacity;
pub(in crate::ideal_loads::calc) use release::snapshots_match_bit_exact as cooling_positive_supply_capacity_limit_sensible_output_guard_snapshots_match_bit_exact;
pub(in crate::ideal_loads) use release::cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardActiveInput,
    advance_cooling_positive_supply_capacity_limit_sensible_output_guard_state,
};

/// EnergyPlus source statement represented by CP340.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2198";
/// First executable statement deliberately excluded after CP340.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2199";
/// Exact four textual source sites represented by CP340.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER:
    &[&str] = &[
    "read-retained-cooling-sensible-output-for-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-sensible-output-comparison",
    "compare-cooling-sensible-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
    "enter-cooling-capacity-adjustment-body-if-comparison-satisfied",
];

/// One CP339-to-CP340 source-ordered sensible-output guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
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
    pub predecessor_capacity_limit_guard_evaluated: bool,
    pub predecessor_capacity_limit_body_entered: bool,
    pub predecessor_active_capacity_limit_guard_false_fallthrough: bool,
    pub predecessor_capacity_limit_cp_air_assignment_executed: bool,
    pub predecessor_capacity_limit_sensible_output_assignment_executed: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_sensible_output_guard_evaluated: bool,
    pub cooling_sensible_output_read: bool,
    pub cooling_sensible_output_w: Option<f64>,
    pub maximum_total_cooling_capacity_read: bool,
    pub maximum_total_cooling_capacity_w: Option<f64>,
    pub cooling_sensible_output_maximum_capacity_comparison_evaluated: bool,
    pub cooling_sensible_output_at_or_above_maximum_capacity: Option<bool>,
    pub capacity_limit_sensible_output_guard_false_fallthrough: bool,
    pub capacity_limit_sensible_output_adjustment_body_entered: bool,
}

/// Final selected-unit CP340 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState,
}

/// Returns the bounded selected-unit CP340 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
                .clone(),
        },
    )
}
