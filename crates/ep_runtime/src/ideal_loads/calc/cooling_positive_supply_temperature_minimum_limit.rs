//! Bounded Cooling positive-supply temperature minimum limit from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_temperature_minimum_limit_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitActiveInput,
    advance_cooling_positive_supply_temperature_minimum_limit_state,
    source_shaped_two_argument_maximum,
};

/// EnergyPlus source statement represented by CP333.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2187";
/// First executable statement deliberately excluded after CP333.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2189";
/// Exact four textual source sites represented by CP333.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-supply-temperature-for-maximum",
    "reread-minimum-cooling-supply-air-temperature-for-maximum",
    "apply-source-shaped-two-argument-maximum",
    "assign-purchased-air-supply-temperature",
];

/// One CP332-to-CP333 source-ordered minimum-temperature-limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot {
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
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub supply_temperature_minimum_limit_executed: bool,
    pub supply_temperature_for_maximum_read: bool,
    pub supply_temperature_before_minimum_limit_c: Option<f64>,
    pub minimum_cooling_supply_air_temperature_for_maximum_read: bool,
    pub minimum_cooling_supply_air_temperature_c: Option<f64>,
    pub source_shaped_two_argument_maximum_evaluated: bool,
    pub maximum_supply_temperature_c: Option<f64>,
    pub supply_temperature_assignment_performed: bool,
    pub assigned_supply_temperature_c: Option<f64>,
}

/// Final selected-unit CP333 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState,
}

/// Returns the bounded selected-unit CP333 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_temperature_minimum_limit
                .clone(),
        },
    )
}
