//! Bounded post-saturation dehumidifying total-output capacity guard.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use tests::{
    active_input as active_input_for_cp384_test,
    completed_cp382_case as completed_cp382_case_for_cp384_test,
    predecessor_for_route as predecessor_for_cp384_test,
};
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_is_consistent,
    snapshot_route as cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_route,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshots_match_bit_exact,
};
pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_characterization,
};
pub(super) use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState;
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardActiveInput;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state;

/// EnergyPlus source statement represented by CP383.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2268";
/// First executable statement deliberately excluded after CP383.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2269";
/// Exact dependency-ordered source sites represented by CP383.
///
/// The two side-effect-free reads do not claim C++ operand evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-retained-cooling-total-output-for-post-saturation-dehumidification-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-dehumidification-total-output-comparison",
    "compare-post-saturation-dehumidification-cooling-total-output-strictly-greater-than-maximum-total-cooling-capacity",
    "enter-post-saturation-dehumidification-total-output-capacity-adjustment-body-if-comparison-satisfied",
];

/// One CP382-to-CP383 source-ordered total-output capacity-guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot
{
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
    pub predecessor_dehumidification_guard_evaluated: bool,
    pub predecessor_dehumidification_body_entered: bool,
    pub predecessor_dehumidification_guard_false_fallthrough: bool,
    pub predecessor_dehumidification_total_output_assignment_executed: bool,
    pub dehumidification_total_output_capacity_guard_evaluated: bool,
    pub cp382_cooling_total_output_owned_read: bool,
    pub cooling_total_output_read: bool,
    pub cooling_total_output_w: Option<f64>,
    pub cp321_maximum_total_cooling_capacity_owned_read: bool,
    pub cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: bool,
    pub maximum_total_cooling_capacity_read: bool,
    pub maximum_total_cooling_capacity_w: Option<f64>,
    pub cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated: bool,
    pub cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity: Option<bool>,
    pub dehumidification_total_output_capacity_adjustment_body_entered: bool,
    pub dehumidification_total_output_capacity_guard_false_fallthrough: bool,
}

/// Final selected-unit CP383 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState,
}

/// Returns the bounded selected-unit CP383 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardLifecycleSummary,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardError::UnknownSystem {
            system,
        },
    )?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        state: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard
            .clone(),
    })
}
