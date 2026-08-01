//! Bounded cooling supply-humidity-ratio saturation assignment.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_humidity_ratio_saturation_assignment_is_consistent,
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_route,
    cooling_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_humidity_ratio_saturation_assignment_latest_metadata_is_consistent,
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_saturation_assignment_characterization,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState;
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput,
    advance_cooling_supply_humidity_ratio_saturation_assignment_state,
};

/// EnergyPlus source statement represented by CP377.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2259";
/// First lexically subsequent executable source statement excluded after CP377.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2260";
/// Exact source order represented by CP377.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-supply-temperature-for-saturation-humidity-ratio",
    "read-environment-outdoor-barometric-pressure-for-saturation-humidity-ratio",
    "evaluate-psy-w-fn-tdb-rh-pb-at-unity-relative-humidity",
    "assign-local-saturation-supply-humidity-ratio",
];

/// One CP376-to-CP377 source-ordered saturation-humidity-ratio witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot {
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
    pub cp334_supply_temperature_mixed_air_limit_owned_read: bool,
    pub cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read: bool,
    pub environment_outdoor_barometric_pressure_owned_read: bool,
    pub purchased_air_supply_temperature_for_saturation_humidity_ratio_read: bool,
    pub supply_temperature_for_saturation_humidity_ratio_c: Option<f64>,
    pub environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: bool,
    pub outdoor_barometric_pressure_pa: Option<f64>,
    pub psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: bool,
    pub saturation_supply_humidity_ratio: Option<f64>,
    pub local_saturation_supply_humidity_ratio_assignment_performed: bool,
    pub assigned_saturation_supply_humidity_ratio: Option<f64>,
    pub resulting_saturation_supply_humidity_ratio: Option<f64>,
}

/// Final selected-unit CP377 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP377 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_supply_humidity_ratio_saturation_assignment
                .clone(),
        },
    )
}
