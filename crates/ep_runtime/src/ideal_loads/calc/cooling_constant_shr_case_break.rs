//! Bounded constant-SHR dehumidification-control case-break evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub(in crate::ideal_loads) use release::cooling_constant_shr_case_break_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingConstantShrCaseBreakError,
    advance_direct_no_oa_calc_cooling_constant_shr_case_break,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_constant_shr_case_break_is_consistent,
    cooling_constant_shr_case_break_snapshots_match_bit_exact,
};
pub(in crate::ideal_loads::calc) use release::{
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingConstantShrCaseBreakRetainedRoute;
pub use state::PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_constant_shr_case_break_state;

/// EnergyPlus source statement represented by CP357.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2227";
/// First lexically subsequent executable source statement excluded after CP357.
///
/// The line 2228 `Humidistat` label is CP358's next control checkpoint, while
/// an active constant-SHR break dynamically continues at line 2245. Neither
/// continuation is represented by CP357.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2229";
/// Sole constant-SHR case-break source site represented by CP357.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER: &[&str] =
    &["exit-purchased-air-dehumidification-control-constant-sensible-heat-ratio-case-via-break"];

/// One CP356-to-CP357 source-ordered case-break witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot {
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
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub predecessor_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_dehumidification_control_none_case_completed_skip: bool,
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed:
        bool,
    pub predecessor_dehumidification_control_humidistat_case_selected_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: bool,
    pub dehumidification_control_humidistat_case_selected_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: bool,
}

/// Final selected-unit CP357 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantShrCaseBreakLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState,
}

/// Returns the bounded selected-unit CP357 lifecycle summary.
pub fn purchased_air_calc_cooling_constant_shr_case_break_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingConstantShrCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrCaseBreakError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingConstantShrCaseBreakError::UnknownSystem { system })?;
    Ok(
        PurchasedAirCalcCoolingConstantShrCaseBreakLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_constant_shr_case_break.clone(),
        },
    )
}
