//! Bounded post-saturation capacity-limit dehumidification guard.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_guard_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_characterization,
};
pub(super) use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState;
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state;

/// EnergyPlus source statement represented by CP381.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2266";
/// First lexically subsequent executable source statement excluded after CP381.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2267";
/// Exact source-ordered operand, comparison, and conditional-entry sites represented by CP381.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER:
    &[&str] = &[
    "read-retained-purchased-air-supply-humidity-ratio-for-post-saturation-dehumidification-comparison",
    "read-retained-purchased-air-mixed-air-humidity-ratio-for-post-saturation-dehumidification-comparison",
    "compare-purchased-air-supply-humidity-ratio-strictly-less-than-mixed-air-humidity-ratio-for-post-saturation-dehumidification-guard",
    "enter-post-saturation-capacity-limit-dehumidification-body-if-comparison-satisfied",
];

/// One CP380-to-CP381 source-ordered dehumidification-guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub heating_availability_guard_false_fallthrough: bool,
    pub humidification_control_guard_false_fallthrough: bool,
    pub dehumidification_control_humidistat_maximum_assignment_executed: bool,
    pub dehumidification_control_none_maximum_assignment_executed: bool,
    pub dehumidification_control_guard_false_fallthrough: bool,
    pub predecessor_capacity_limit_guard_evaluated: bool,
    pub predecessor_capacity_limit_body_entered: bool,
    pub predecessor_active_capacity_limit_guard_false_fallthrough: bool,
    pub dehumidification_guard_evaluated: bool,
    pub cp378_supply_humidity_ratio_saturation_limit_owned_read: bool,
    pub cp379_same_call_supply_humidity_ratio_bit_corroborated: bool,
    pub purchased_air_supply_humidity_ratio_read: bool,
    pub supply_humidity_ratio: Option<f64>,
    pub cp329_mixed_air_humidity_ratio_owned_read: bool,
    pub purchased_air_mixed_air_humidity_ratio_read: bool,
    pub mixed_air_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated: bool,
    pub supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio: Option<bool>,
    pub dehumidification_body_entered: bool,
    pub dehumidification_guard_false_fallthrough: bool,
}

/// Final selected-unit CP381 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState,
}

/// Returns the bounded selected-unit CP381 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardLifecycleSummary,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardError::
            UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_post_saturation_capacity_limit_dehumidification_guard
                .clone(),
        },
    )
}
