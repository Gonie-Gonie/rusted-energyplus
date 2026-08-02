//! Bounded post-saturation Humidistat supply-humidity-ratio assignment.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
pub(in crate::ideal_loads::calc) mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_route,
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_characterization,
};
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state;

/// EnergyPlus source statement represented by CP395.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2288";
/// First executable source statement deliberately excluded after CP395.
///
/// CP396 owns line 2289's Humidistat-case `break`; the dynamic continuation
/// after the switch at line 2313 is also outside CP395.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2289";
/// Exact four dependency-ordered sites represented by CP395.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
        "read-purchased-air-supply-temperature-for-humidistat-humidity-ratio-inversion",
        "read-local-supply-enthalpy-for-humidistat-humidity-ratio-inversion",
        "evaluate-psy-w-fn-tdb-h-for-humidistat-capacity-limit",
        "assign-purchased-air-supply-humidity-ratio-for-humidistat-capacity-limit",
    ];

/// One CP394-to-CP395 source-ordered Humidistat humidity-ratio assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot
{
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
    pub predecessor_capacity_limit_guard_evaluated: bool,
    pub predecessor_capacity_limit_body_entered: bool,
    pub predecessor_active_capacity_limit_guard_false_fallthrough: bool,
    pub predecessor_dehumidification_guard_evaluated: bool,
    pub predecessor_dehumidification_body_entered: bool,
    pub predecessor_dehumidification_guard_false_fallthrough: bool,
    pub predecessor_dehumidification_total_output_assignment_executed: bool,
    pub predecessor_dehumidification_total_output_capacity_guard_evaluated: bool,
    pub predecessor_dehumidification_total_output_capacity_adjustment_body_entered: bool,
    pub predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: bool,
    pub dehumidification_total_output_capacity_guard_false_fallthrough: bool,
    pub dehumidification_total_output_maximum_capacity_assignment_executed: bool,
    pub predecessor_supply_enthalpy_assignment_executed: bool,
    pub predecessor_dehumidification_control_type_read: bool,
    pub predecessor_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_dehumidification_control_switch_dispatched: bool,
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: bool,
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break:
        bool,
    pub predecessor_cp393_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp393_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp393_resulting_supply_temperature_c: Option<f64>,
    pub predecessor_dehumidification_control_humidistat_case_entered: bool,
    pub predecessor_cp394_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp394_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp394_resulting_supply_temperature_c: Option<f64>,
    pub dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: bool,
    pub cp394_retained_supply_humidity_ratio_state_owned: bool,
    pub cp394_retained_supply_temperature_state_owned: bool,
    pub cp394_retained_supply_enthalpy_state_owned: bool,
    pub cp394_retained_supply_temperature_owned_read: bool,
    pub supply_temperature_for_humidity_ratio_inversion_read: bool,
    pub supply_temperature_c: Option<f64>,
    pub cp394_retained_supply_enthalpy_owned_read: bool,
    pub supply_enthalpy_for_humidity_ratio_inversion_read: bool,
    pub supply_enthalpy_j_per_kg: Option<f64>,
    pub psychrometric_supply_humidity_ratio_evaluated: bool,
    pub psychrometric_supply_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_assignment_performed: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub resulting_supply_temperature_c: Option<f64>,
}

/// Final selected-unit CP395 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP395 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentLifecycleSummary, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentError>{
    let unit = runtime.units.get(&system).ok_or(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment.clone(),
    })
}
