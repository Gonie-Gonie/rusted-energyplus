//! Bounded constant-SHR cooling supply-humidity-ratio minimum limit.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[cfg(test)]
pub(in crate::ideal_loads::calc) use tests::completed_cp354_case;

pub(in crate::ideal_loads) use release::cooling_constant_shr_supply_humidity_ratio_minimum_limit_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitError,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_constant_shr_supply_humidity_ratio_minimum_limit_is_consistent,
    private_active_counterfactual_from_direct_release,
    private_active_counterfactual_links_to_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRetainedRoute;
pub use state::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitActiveOperands,
    advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit_state,
};

/// EnergyPlus source statement represented by CP355.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2224";
/// First executable source statement deliberately excluded after CP355.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2226";
/// Exact four source sites represented by CP355.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER:
    &[&str] = &[
        "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit-maximum",
        "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit-maximum",
        "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-minimum-limit",
        "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit",
    ];

/// One CP354-to-CP355 source-ordered minimum-limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot {
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed:
        bool,
    pub predecessor_dehumidification_control_humidistat_case_selected_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed:
        bool,
    pub dehumidification_control_humidistat_case_selected_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: bool,
    pub supply_humidity_ratio_for_minimum_limit_maximum_read: bool,
    pub supply_humidity_ratio_before_minimum_limit: Option<f64>,
    pub minimum_cooling_supply_air_humidity_ratio_for_maximum_read: bool,
    pub minimum_cooling_supply_air_humidity_ratio: Option<f64>,
    pub source_shaped_two_argument_maximum_evaluated: bool,
    pub maximum_supply_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_assignment_performed: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_humidity_ratio: Option<f64>,
}

/// Final selected-unit CP355 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRuntimeState,
}

/// Returns the bounded selected-unit CP355 lifecycle summary.
pub fn purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit
                .clone(),
        },
    )
}
