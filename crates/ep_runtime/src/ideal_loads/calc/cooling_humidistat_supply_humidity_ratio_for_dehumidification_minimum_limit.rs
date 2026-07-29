//! Bounded Humidistat local dehumidification humidity-ratio minimum-limit evidence.

use crate::ideal_loads::PurchasedAirRuntimeState;
use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[cfg(test)]
pub(in crate::ideal_loads::calc) use release::active_operands_from_retained_owners_for_test;
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent;
pub(in crate::ideal_loads) use release::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitError,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRetainedRoute;
pub use state::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitActiveOperands,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_state,
};

/// EnergyPlus source statement represented by CP361.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2231";
/// First lexically subsequent executable source statement excluded after CP361.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2232";
/// Exact four dependency-ordered source sites represented by CP361.
///
/// The side-effect-free reads do not claim C++ operand evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER:
    &[&str] = &[
    "read-local-supply-humidity-ratio-for-dehumidification-for-humidistat-minimum-limit-maximum",
    "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-humidistat-minimum-limit-maximum",
    "apply-source-shaped-two-argument-maximum-for-humidistat-minimum-limit",
    "assign-local-supply-humidity-ratio-for-dehumidification-for-humidistat-minimum-limit",
];

/// One CP360-to-CP361 source-ordered local minimum-limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot
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
    pub predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed:
        bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
        bool,
    pub predecessor_resulting_supply_humidity_ratio_for_dehumidification: Option<f64>,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed:
        bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: bool,
    pub supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read: bool,
    pub supply_humidity_ratio_for_dehumidification_before_minimum_limit: Option<f64>,
    pub minimum_cooling_supply_air_humidity_ratio_for_maximum_read: bool,
    pub minimum_cooling_supply_air_humidity_ratio: Option<f64>,
    pub source_shaped_two_argument_maximum_evaluated: bool,
    pub maximum_supply_humidity_ratio_for_dehumidification: Option<f64>,
    pub supply_humidity_ratio_for_dehumidification_assignment_performed: bool,
    pub assigned_supply_humidity_ratio_for_dehumidification: Option<f64>,
    pub resulting_supply_humidity_ratio_for_dehumidification: Option<f64>,
}

/// Final selected-unit CP361 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState,
}

/// Returns the bounded selected-unit CP361 lifecycle summary.
pub fn purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitError,
>{
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
                .clone(),
        },
    )
}
