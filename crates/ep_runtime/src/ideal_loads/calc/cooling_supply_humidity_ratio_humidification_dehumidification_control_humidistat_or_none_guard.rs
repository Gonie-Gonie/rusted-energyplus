//! Bounded Cooling supply-humidity-ratio nested dehumidification-control guard evidence.

use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId, ZoneId,
};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardError,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_is_consistent,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshots_match_exact,
    private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_links_to_direct_release,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state;

/// EnergyPlus source statement represented by CP371.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2247";
/// First lexically subsequent executable source statement excluded after CP371.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2248";
/// Exact five textual sites, identical to CP320's nested selector slice.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-dehumidification-control-type-for-humidistat-comparison",
    "compare-dehumidification-control-type-equal-to-humidistat",
    "read-dehumidification-control-type-for-none-comparison-after-first-false",
    "compare-dehumidification-control-type-equal-to-none",
    "enter-admitted-humidification-body-if-control-condition-satisfied",
];

/// One CP370-to-CP371 source-ordered, numeric-free nested-control witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot {
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
    pub predecessor_dehumidification_control_humidistat_case_completed_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: bool,
    pub predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break: bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_case_completed_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: bool,
    pub predecessor_heating_on_read: bool,
    pub predecessor_heating_on: Option<bool>,
    pub predecessor_cooling_supply_humidity_ratio_humidification_body_entered: bool,
    pub predecessor_heating_on_guard_false_fallthrough: bool,
    pub predecessor_humidification_control_type_read: bool,
    pub predecessor_humidification_control_type: Option<HumidificationControlType>,
    pub predecessor_humidification_control_type_humidistat: Option<bool>,
    pub predecessor_humidification_control_body_entered: bool,
    pub predecessor_humidification_control_guard_false_fallthrough: bool,
    pub dehumidification_control_type_first_read: bool,
    pub first_dehumidification_control_type: Option<DehumidificationControlType>,
    pub dehumidification_control_type_humidistat: Option<bool>,
    pub dehumidification_control_type_second_read: bool,
    pub second_dehumidification_control_type: Option<DehumidificationControlType>,
    pub dehumidification_control_type_none: Option<bool>,
    pub dehumidification_control_body_entered: bool,
    pub dehumidification_control_guard_false_fallthrough: bool,
}

/// Final selected-unit CP371 lifecycle summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState,
}

/// Returns the bounded selected-unit CP371 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardError::UnknownSystem { system },
    )?;
    Ok(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_FIRST_EXCLUDED_SOURCE,
        state: unit
            .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
            .clone(),
    })
}