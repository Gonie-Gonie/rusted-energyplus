//! Bounded Cooling supply-humidity-ratio humidification-control Humidistat guard evidence.

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
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_is_consistent,
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_route,
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshots_match_exact,
    private_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_counterfactual_links_to_direct_release,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_state;

/// EnergyPlus source statement represented by CP370.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2246";
/// First lexically subsequent executable source statement excluded after CP370.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2247";
/// Exact three textual source sites represented by CP370.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-humidification-control-type-for-cooling-supply-humidity-ratio-humidification-guard",
    "compare-purchased-air-humidification-control-type-equal-to-humidistat",
    "enter-cooling-supply-humidity-ratio-humidification-control-body-if-humidistat",
];

/// One CP369-to-CP370 source-ordered, numeric-free Humidistat-guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot
{
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
        bool,
    pub predecessor_dehumidification_control_humidistat_case_completed_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
        bool,
    pub predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_case_completed_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: bool,
    pub predecessor_heating_on_read: bool,
    pub predecessor_heating_on: Option<bool>,
    pub predecessor_cooling_supply_humidity_ratio_humidification_body_entered: bool,
    pub predecessor_heating_on_guard_false_fallthrough: bool,
    pub humidification_control_type_read: bool,
    pub humidification_control_type: Option<HumidificationControlType>,
    pub humidification_control_type_humidistat: Option<bool>,
    pub humidification_control_body_entered: bool,
    pub humidification_control_guard_false_fallthrough: bool,
}

/// Final selected-unit CP370 lifecycle summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState,
}

/// Returns the bounded selected-unit CP370 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError::UnknownSystem { system },
    )?;
    Ok(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_FIRST_EXCLUDED_SOURCE,
        state: unit
            .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard
            .clone(),
    })
}
