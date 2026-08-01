//! Bounded cooling humidification supply-humidity-ratio maximum-limit evidence.

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
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use release::active_operands_from_selected_typed_owner_for_test;
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_links_to_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitActiveOperands,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_state,
};

/// EnergyPlus source statement represented by CP374.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2250";
/// First lexically subsequent executable source statement excluded after CP374.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2251";
/// Exact four dependency-ordered source sites represented by CP374.
///
/// The side-effect-free reads do not claim C++ operand evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE_ORDER: &[&str] = &[
    "read-local-supply-humidity-ratio-for-humidification-for-maximum-limit-minimum",
    "read-purchased-air-maximum-heating-supply-air-humidity-ratio-for-humidification-maximum-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-humidification-maximum-limit",
    "assign-local-supply-humidity-ratio-for-humidification-for-maximum-limit",
];

/// One CP373-to-CP374 source-ordered local maximum-limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot {
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
    pub predecessor_dehumidification_control_type_first_read: bool,
    pub predecessor_first_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_dehumidification_control_type_humidistat: Option<bool>,
    pub predecessor_dehumidification_control_type_second_read: bool,
    pub predecessor_second_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_dehumidification_control_type_none: Option<bool>,
    pub predecessor_dehumidification_control_body_entered: bool,
    pub predecessor_dehumidification_control_guard_false_fallthrough: bool,
    pub predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed: bool,
    pub predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed: bool,
    pub predecessor_resulting_supply_humidity_ratio_for_humidification: Option<f64>,
    pub dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed: bool,
    pub dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed: bool,
    pub supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read: bool,
    pub supply_humidity_ratio_for_humidification_before_maximum_limit: Option<f64>,
    pub maximum_heating_supply_air_humidity_ratio_for_minimum_read: bool,
    pub maximum_heating_supply_air_humidity_ratio: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_supply_humidity_ratio_for_humidification: Option<f64>,
    pub supply_humidity_ratio_for_humidification_assignment_performed: bool,
    pub assigned_supply_humidity_ratio_for_humidification: Option<f64>,
    pub resulting_supply_humidity_ratio_for_humidification: Option<f64>,
}

/// Final selected-unit CP374 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState,
}

/// Returns the bounded selected-unit CP374 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitError::UnknownSystem { system },
    )?;
    Ok(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit.clone(),
    })
}
