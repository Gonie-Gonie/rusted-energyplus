//! Bounded Humidistat dehumidification-control case-entry evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_humidistat_case_entry_is_consistent;
pub(in crate::ideal_loads) use release::cooling_humidistat_case_entry_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingHumidistatCaseEntryError,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingHumidistatCaseEntryRetainedRoute;
pub use state::PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_humidistat_case_entry_state;

/// EnergyPlus source statement represented by CP358.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2228";
/// First lexically subsequent executable source statement excluded after CP358.
///
/// Line 2229 is the first statement in the `Humidistat` case body. A completed
/// constant-SHR case has already continued at line 2245 instead of falling
/// through this label. Neither continuation is represented by CP358.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2229";
/// Sole Humidistat case-entry source site represented by CP358.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER: &[&str] =
    &["enter-purchased-air-dehumidification-control-humidistat-case"];

/// One CP357-to-CP358 source-ordered Humidistat case-entry witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot {
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break:
        bool,
    pub predecessor_dehumidification_control_humidistat_case_selected_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_case_entered: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: bool,
}

/// Final selected-unit CP358 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState,
}

/// Returns the bounded selected-unit CP358 lifecycle summary.
pub fn purchased_air_calc_cooling_humidistat_case_entry_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatCaseEntryError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingHumidistatCaseEntryError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_humidistat_case_entry.clone(),
    })
}
