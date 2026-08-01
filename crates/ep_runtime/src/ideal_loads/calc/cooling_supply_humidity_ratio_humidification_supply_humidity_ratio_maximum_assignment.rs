//! Bounded cooling humidification purchased-air supply-humidity-ratio maximum assignment.

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
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
};
#[cfg(test)]
pub(in crate::ideal_loads::calc) use release::{
    active_humidistat_operands_from_cp362_counterfactual,
    active_none_operands_from_retained_cp345_for_test,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_links_to_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentActiveOperands,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_state,
};

/// EnergyPlus source statement represented by CP375.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2251";
/// First lexically subsequent executable source statement excluded after CP375.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2258";
/// Exact four dependency-ordered source sites represented by CP375.
///
/// The side-effect-free reads do not claim C++ operand evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-supply-humidity-ratio-for-humidification-supply-maximum",
    "read-local-supply-humidity-ratio-for-humidification-for-supply-maximum",
    "apply-source-shaped-two-argument-maximum-for-humidification-supply-maximum",
    "assign-purchased-air-supply-humidity-ratio-for-humidification-supply-maximum",
];

/// One CP374-to-CP375 source-ordered result-store assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot {
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
    pub predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed: bool,
    pub predecessor_dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed: bool,
    pub predecessor_resulting_supply_humidity_ratio_for_humidification: Option<f64>,
    pub dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed: bool,
    pub dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed: bool,
    pub purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read: bool,
    pub purchased_air_supply_humidity_ratio_before_humidification_supply_maximum: Option<f64>,
    pub supply_humidity_ratio_for_humidification_for_supply_maximum_read: bool,
    pub supply_humidity_ratio_for_humidification_for_supply_maximum: Option<f64>,
    pub source_shaped_two_argument_maximum_evaluated: bool,
    pub maximum_supply_humidity_ratio: Option<f64>,
    pub purchased_air_supply_humidity_ratio_assignment_performed: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_humidity_ratio: Option<f64>,
}

/// Final selected-unit CP375 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP375 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentError::UnknownSystem { system },
    )?;
    Ok(PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment.clone(),
    })
}
