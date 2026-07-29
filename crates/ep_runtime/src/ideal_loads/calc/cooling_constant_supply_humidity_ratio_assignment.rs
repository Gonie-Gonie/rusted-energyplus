//! Bounded constant-supply-humidity-ratio supply-humidity-ratio assignment evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_constant_supply_humidity_ratio_assignment_is_consistent,
    cooling_constant_supply_humidity_ratio_assignment_snapshots_match_bit_exact,
    private_constant_supply_humidity_ratio_assignment_counterfactual_from_direct_release,
    private_constant_supply_humidity_ratio_assignment_counterfactual_links_to_direct_release,
};
pub(in crate::ideal_loads) use release::{
    cooling_constant_supply_humidity_ratio_assignment_latest_metadata_is_consistent,
    cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    cooling_constant_supply_humidity_ratio_assignment_snapshot_links_to_predecessor,
};
pub(super) use state::PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_constant_supply_humidity_ratio_assignment_state;

/// EnergyPlus source statement represented by CP365.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2235";
/// First lexically subsequent executable source statement excluded after CP365.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2236";
/// Exact two dependency-ordered source sites represented by CP365.
pub const PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-constant-supply-humidity-ratio-assignment",
    "assign-purchased-air-supply-humidity-ratio-for-constant-supply-humidity-ratio-case",
];

/// One CP364-to-CP365 source-ordered supply-humidity-ratio assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot {
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
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered: bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_case_completed_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_assignment_executed: bool,
    pub minimum_cooling_supply_air_humidity_ratio_read: bool,
    pub minimum_cooling_supply_air_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_assigned: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_humidity_ratio: Option<f64>,
}

/// Final selected-unit CP365 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP365 lifecycle summary.
pub fn purchased_air_calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_constant_supply_humidity_ratio_assignment
                .clone(),
        },
    )
}
