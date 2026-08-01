//! Bounded post-saturation cooling capacity-limit guard.

use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[cfg(test)]
pub(in crate::ideal_loads::calc) use tests::{
    active_input as active_input_for_cp381_test,
    completed_cp380_case as completed_cp380_case_for_cp381_test,
    predecessor_for_route as predecessor_for_cp381_test,
};

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_post_saturation_capacity_limit_guard_is_consistent,
    cooling_post_saturation_capacity_limit_guard_snapshots_match_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_guard_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_guard_characterization,
};
pub(super) use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState;
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardActiveInput;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_guard_state;

/// EnergyPlus source statement represented by CP380.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2264";
/// First lexically subsequent executable source statement excluded after CP380.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2266";
/// Exact sequenced and short-circuited source sites represented by CP380.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-cooling-limit-for-post-saturation-capacity-comparison",
    "compare-cooling-limit-equal-to-capacity-for-post-saturation-capacity-guard",
    "read-cooling-limit-for-post-saturation-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity-for-post-saturation-capacity-guard",
    "enter-post-saturation-capacity-limit-body-if-compound-condition-satisfied",
];

/// One CP379-to-CP380 source-ordered post-saturation capacity-limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot {
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
    pub predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed: bool,
    pub capacity_limit_guard_evaluated: bool,
    pub configured_cooling_limit_owned_read: bool,
    pub cp337_same_call_selector_lineage_corroborated: bool,
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

/// Final selected-unit CP380 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState,
}

/// Returns the bounded selected-unit CP380 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_post_saturation_capacity_limit_guard
                .clone(),
        },
    )
}
