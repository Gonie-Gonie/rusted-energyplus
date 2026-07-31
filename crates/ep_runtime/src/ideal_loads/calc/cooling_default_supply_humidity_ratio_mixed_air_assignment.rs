//! Bounded default supply-humidity-ratio mixed-air assignment evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError,
    advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_default_supply_humidity_ratio_mixed_air_assignment_is_consistent,
    cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshots_match_exact,
    private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_from_direct_release,
    private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_links_to_direct_release,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_metadata_is_consistent,
    cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
};
pub(super) use state::PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_default_supply_humidity_ratio_mixed_air_assignment_state;

/// EnergyPlus source statement represented by CP367.
pub const PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2238";
/// First lexically subsequent executable source statement excluded after CP367.
pub const PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2239";
/// Dependency-ordered default assignment source sites represented by CP367.
pub const PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-mixed-air-humidity-ratio-for-dehumidification-control-default-assignment",
    "assign-purchased-air-supply-humidity-ratio-for-dehumidification-control-default-case",
];

/// One CP366-to-CP367 source-ordered, numeric-free default-assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot {
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
        bool,
    pub predecessor_dehumidification_control_humidistat_case_completed_skip: bool,
    pub predecessor_dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break:
        bool,
    pub dehumidification_control_none_case_completed_skip: bool,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: bool,
    pub dehumidification_control_humidistat_case_completed_skip: bool,
    pub dehumidification_control_constant_supply_humidity_ratio_case_completed_skip: bool,
    pub dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed: bool,
}

/// Final selected-unit CP367 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP367 lifecycle summary.
pub fn purchased_air_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_default_supply_humidity_ratio_mixed_air_assignment
                .clone(),
        },
    )
}
