//! Bounded Humidistat purchased-air supply-humidity-ratio mixed-air limit.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

mod lifecycle;
mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use lifecycle::*;
pub use release::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
};
pub(in crate::ideal_loads) use release::{
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_metadata_is_consistent,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent,
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads::calc) use state::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRetainedRoute;
pub use state::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitActiveOperands,
    advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_state,
};

/// EnergyPlus source statement represented by CP362.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2232";
/// First lexically subsequent executable source statement excluded after CP362.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2233";
/// Exact dependency-ordered source sites represented by CP362.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER:
    &[&str] = &[
        "read-purchased-air-mixed-air-humidity-ratio-for-humidistat-mixed-air-limit-minimum",
        "read-local-supply-humidity-ratio-for-dehumidification-for-humidistat-mixed-air-limit-minimum",
        "apply-source-shaped-two-argument-minimum-for-humidistat-mixed-air-limit",
        "assign-purchased-air-supply-humidity-ratio-for-humidistat-mixed-air-limit",
    ];

/// One CP361-to-CP362 source-ordered mixed-air-limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot {
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
    pub predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed:
        bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub predecessor_resulting_supply_humidity_ratio_for_dehumidification: Option<f64>,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: bool,
    pub mixed_air_humidity_ratio_for_minimum_read: bool,
    pub mixed_air_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read: bool,
    pub supply_humidity_ratio_for_dehumidification_before_mixed_air_limit: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_supply_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_assignment_performed: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_humidity_ratio: Option<f64>,
}
