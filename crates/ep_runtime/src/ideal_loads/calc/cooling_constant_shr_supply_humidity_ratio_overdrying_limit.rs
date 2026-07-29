//! Bounded constant-SHR cooling supply-humidity-ratio overdrying limit.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[cfg(test)]
pub(in crate::ideal_loads::calc) use tests::completed_cp353_case;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_is_consistent,
    private_active_counterfactual_from_direct_release,
    private_active_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use release::cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit,
};
pub(super) use state::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRetainedRoute;
pub use state::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitActiveOperands,
    advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_state,
};

/// EnergyPlus source statement represented by CP354.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2222";
/// First executable source statement deliberately excluded after CP354.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2224";
/// Exact six source sites represented by CP354.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER:
    &[&str] = &[
        "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit-minimum",
        "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
        "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
        "evaluate-psy-w-fn-tdb-h-for-constant-sensible-heat-ratio-overdrying-limit",
        "apply-source-shaped-two-argument-minimum-for-constant-sensible-heat-ratio-overdrying-limit",
        "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
    ];

/// One CP353-to-CP354 source-ordered supply-humidity-ratio limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot {
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed:
        bool,
    pub predecessor_dehumidification_control_humidistat_case_selected_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed:
        bool,
    pub dehumidification_control_humidistat_case_selected_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: bool,
    pub supply_humidity_ratio_for_overdrying_limit_minimum_read: bool,
    pub supply_humidity_ratio_before_overdrying_limit: Option<f64>,
    pub supply_temperature_for_humidity_ratio_inversion_read: bool,
    pub supply_temperature_c: Option<f64>,
    pub supply_enthalpy_for_humidity_ratio_inversion_read: bool,
    pub supply_enthalpy_j_per_kg: Option<f64>,
    pub psychrometric_supply_humidity_ratio_evaluated: bool,
    pub psychrometric_supply_humidity_ratio: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_supply_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_assignment_performed: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_humidity_ratio: Option<f64>,
}

/// Final selected-unit CP354 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRuntimeState,
}

/// Returns the bounded selected-unit CP354 lifecycle summary.
pub fn purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit
                .clone(),
        },
    )
}
