//! Bounded constant-supply-humidity-ratio case-entry evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent,
    cooling_constant_supply_humidity_ratio_case_entry_snapshots_match_bit_exact,
    private_constant_supply_humidity_ratio_case_entry_counterfactual_from_direct_release,
    private_constant_supply_humidity_ratio_case_entry_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use release::{
    cooling_constant_supply_humidity_ratio_case_entry_latest_metadata_is_consistent,
    cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release,
    cooling_constant_supply_humidity_ratio_case_entry_snapshot_links_to_predecessor,
};
pub(super) use state::PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRetainedRoute;
pub use state::PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_constant_supply_humidity_ratio_case_entry_state;

/// EnergyPlus source construct represented by CP364.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2234";
/// First lexically subsequent executable source statement excluded after CP364.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2235";
/// Sole constant-supply-humidity-ratio case-entry source site represented by CP364.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER:
    &[&str] = &["enter-purchased-air-dehumidification-control-constant-supply-humidity-ratio-case"];

/// One CP363-to-CP364 source-ordered case-entry witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot {
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub predecessor_dehumidification_control_humidistat_case_exited_via_break: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_case_completed_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_entered: bool,
}

/// Final selected-unit CP364 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary {
    /// EnergyPlus source construct.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState,
}

/// Returns the bounded selected-unit CP364 lifecycle summary.
pub fn purchased_air_calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_constant_supply_humidity_ratio_case_entry
                .clone(),
        },
    )
}
