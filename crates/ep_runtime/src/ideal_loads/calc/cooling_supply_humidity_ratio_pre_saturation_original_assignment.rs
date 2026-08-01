//! Bounded cooling supply-humidity-ratio pre-saturation original assignment.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_pre_saturation_original_assignment_characterization,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState;
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner;
pub(in crate::ideal_loads::calc) use transition::{
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput,
};

/// EnergyPlus source statement represented by CP376.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2258";
/// First lexically subsequent executable source statement excluded after CP376.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2259";
/// Exact read-then-local-assignment source order represented by CP376.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-supply-humidity-ratio-before-saturation-limit",
    "assign-local-original-supply-humidity-ratio-before-saturation-limit",
];

/// One CP375-to-CP376 source-ordered local-copy witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub heating_availability_guard_false_fallthrough: bool,
    pub humidification_control_guard_false_fallthrough: bool,
    pub dehumidification_control_humidistat_maximum_assignment_executed: bool,
    pub dehumidification_control_none_maximum_assignment_executed: bool,
    pub dehumidification_control_guard_false_fallthrough: bool,
    pub predecessor_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_purchased_air_supply_humidity_ratio_assignment_performed: bool,
    pub predecessor_resulting_supply_humidity_ratio: Option<f64>,
    pub cp375_maximum_assignment_owned_read: bool,
    pub cp347_none_case_owned_read: bool,
    pub cp356_constant_shr_owned_read: bool,
    pub cp362_humidistat_owned_read: bool,
    pub cp365_constant_supply_humidity_ratio_owned_read: bool,
    pub purchased_air_supply_humidity_ratio_read: bool,
    pub purchased_air_supply_humidity_ratio_before_saturation_check: Option<f64>,
    pub local_supply_humidity_ratio_original_assignment_performed: bool,
    pub assigned_supply_humidity_ratio_original: Option<f64>,
    pub resulting_supply_humidity_ratio_original: Option<f64>,
}

/// Final selected-unit CP376 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP376 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
                .clone(),
        },
    )
}
