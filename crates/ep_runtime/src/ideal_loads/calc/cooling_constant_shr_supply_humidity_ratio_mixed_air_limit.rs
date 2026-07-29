//! Bounded constant-SHR cooling supply-humidity-ratio mixed-air limit.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub(in crate::ideal_loads) use release::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitError,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_is_consistent,
    private_active_counterfactual_from_direct_release,
    private_active_counterfactual_links_to_direct_release,
    snapshots_match_bit_exact as cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
};
pub(super) use state::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRetainedRoute;
pub use state::PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use tests::completed_cp355_case;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitActiveOperands,
    advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_state,
};

/// EnergyPlus source statement represented by CP356.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2226";
/// First executable source statement deliberately excluded after CP356.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2227";
/// Exact four source sites represented by CP356.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER:
    &[&str] = &[
        "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-mixed-air-limit-minimum",
        "read-purchased-air-mixed-air-humidity-ratio-for-constant-sensible-heat-ratio-mixed-air-limit-minimum",
        "apply-source-shaped-two-argument-minimum-for-constant-sensible-heat-ratio-mixed-air-limit",
        "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-mixed-air-limit",
    ];

/// One CP355-to-CP356 source-ordered mixed-air-limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot {
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed:
        bool,
    pub predecessor_dehumidification_control_humidistat_case_selected_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed:
        bool,
    pub dehumidification_control_humidistat_case_selected_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: bool,
    pub supply_humidity_ratio_for_mixed_air_limit_minimum_read: bool,
    pub supply_humidity_ratio_before_mixed_air_limit: Option<f64>,
    pub mixed_air_humidity_ratio_for_minimum_read: bool,
    pub mixed_air_humidity_ratio: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_supply_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_assignment_performed: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_humidity_ratio: Option<f64>,
}

/// Final selected-unit CP356 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
}

/// Returns the bounded selected-unit CP356 lifecycle summary.
pub fn purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit
                .clone(),
        },
    )
}
