//! Bounded Cooling supply-humidity-ratio humidification heating-availability guard evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_humidification_heating_availability_guard_is_consistent,
    cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact,
    private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_links_to_direct_release,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard_state;

/// EnergyPlus source statement represented by CP369.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2245";
/// First lexically subsequent executable source statement excluded after CP369.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2246";
/// Exact two source sites represented by CP369.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-local-heating-on-for-cooling-humidification-guard",
    "enter-cooling-supply-humidity-ratio-humidification-body-if-heating-on",
];

/// One CP368-to-CP369 source-ordered, numeric-free heating-availability guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub predecessor_dehumidification_control_humidistat_case_completed_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
        bool,
    pub predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_case_completed_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: bool,
    pub heating_on_read: bool,
    pub heating_on: Option<bool>,
    pub cooling_supply_humidity_ratio_humidification_body_entered: bool,
    pub heating_on_guard_false_fallthrough: bool,
}

/// Final selected-unit CP369 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState,
}

/// Returns the bounded selected-unit CP369 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError::UnknownSystem { system },
    )?;
    Ok(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        state: unit
            .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
            .clone(),
    })
}