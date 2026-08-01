//! Bounded final cooling supply-humidity-ratio saturation-limit assignment.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_saturation_limit_assignment_is_consistent,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_saturation_limit_assignment_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_saturation_limit_assignment_characterization,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state;

/// EnergyPlus source statement represented by CP378.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2260";
/// First lexically subsequent executable source statement excluded after CP378.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2261";
/// Exact dependency-ordered source sites represented by CP378.
///
/// The two side-effect-free reads do not claim C++ operand evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-local-original-supply-humidity-ratio-for-saturation-limit-minimum",
    "read-local-saturation-supply-humidity-ratio-for-saturation-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-saturation-limit",
    "assign-purchased-air-supply-humidity-ratio-for-saturation-limit",
];

/// One CP377-to-CP378 final purchased-air humidity-ratio assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot {
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
    pub predecessor_local_supply_humidity_ratio_original_assignment_performed: bool,
    pub predecessor_resulting_supply_humidity_ratio_original: Option<f64>,
    pub predecessor_local_saturation_supply_humidity_ratio_assignment_performed: bool,
    pub predecessor_resulting_saturation_supply_humidity_ratio: Option<f64>,
    pub cp376_original_supply_humidity_ratio_owned_read: bool,
    pub cp377_saturation_supply_humidity_ratio_owned_read: bool,
    pub local_original_supply_humidity_ratio_for_saturation_limit_minimum_read: bool,
    pub original_supply_humidity_ratio_before_saturation_limit: Option<f64>,
    pub local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read: bool,
    pub saturation_supply_humidity_ratio_for_limit: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_supply_humidity_ratio_after_saturation_limit: Option<f64>,
    pub purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_humidity_ratio: Option<f64>,
}

/// Final selected-unit CP378 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP378 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_saturation_limit_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
                .clone(),
        },
    )
}
