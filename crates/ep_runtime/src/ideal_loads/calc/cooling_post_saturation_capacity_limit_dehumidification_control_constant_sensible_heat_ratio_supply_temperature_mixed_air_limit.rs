//! Bounded post-saturation constant-SHR supply-temperature mixed-air limit.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_characterization,
};
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state;

/// EnergyPlus source statement represented by CP390.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2281";
/// First executable source statement deliberately excluded after CP390.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2283";
/// Exact dependency-ordered reads, operation, and assignment represented by CP390.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];

/// One CP389-to-CP390 source-ordered mixed-air temperature-limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed:
        bool,
    pub predecessor_mixed_air_humidity_ratio_read: bool,
    pub predecessor_mixed_air_humidity_ratio: Option<f64>,
    pub predecessor_psychrometric_cp_air_evaluated: bool,
    pub predecessor_psychrometric_cp_air_result_j_per_kg_k: Option<f64>,
    pub predecessor_cp_air_assigned: bool,
    pub predecessor_cp_air_j_per_kg_k: Option<f64>,
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed:
        bool,
    pub predecessor_cp384_retained_cooling_total_output_owned_read: bool,
    pub predecessor_cp385_cooling_total_output_bit_corroborated: bool,
    pub predecessor_cooling_total_output_read: bool,
    pub predecessor_cooling_total_output_w: Option<f64>,
    pub predecessor_cooling_sensible_heat_ratio_read: bool,
    pub predecessor_cooling_sensible_heat_ratio: Option<f64>,
    pub predecessor_cooling_sensible_output_calculated: bool,
    pub predecessor_calculated_cooling_sensible_output_w: Option<f64>,
    pub predecessor_cooling_sensible_output_assigned: bool,
    pub predecessor_cooling_sensible_output_w: Option<f64>,
    pub predecessor_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed:
        bool,
    pub predecessor_cp379_retained_supply_temperature_state_owned: bool,
    pub predecessor_preexisting_supply_temperature_c: Option<f64>,
    pub predecessor_cp329_retained_mixed_air_temperature_owned_read: bool,
    pub predecessor_mixed_air_temperature_read: bool,
    pub predecessor_mixed_air_temperature_c: Option<f64>,
    pub predecessor_cp388_retained_cooling_sensible_output_owned_read: bool,
    pub predecessor_cooling_sensible_output_read: bool,
    pub predecessor_cp389_cooling_sensible_output_w: Option<f64>,
    pub predecessor_cp387_retained_cp_air_owned_read: bool,
    pub predecessor_cp_air_read: bool,
    pub predecessor_cp389_cp_air_j_per_kg_k: Option<f64>,
    pub predecessor_cp330_retained_supply_mass_flow_rate_owned_read: bool,
    pub predecessor_cp329_supply_mass_flow_rate_bit_corroborated: bool,
    pub predecessor_supply_mass_flow_rate_read: bool,
    pub predecessor_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub predecessor_cp_air_times_supply_mass_flow_rate_calculated: bool,
    pub predecessor_cp_air_times_supply_mass_flow_rate_w_per_k: Option<f64>,
    pub predecessor_cooling_sensible_output_over_air_capacity_rate_calculated: bool,
    pub predecessor_cooling_sensible_output_over_air_capacity_rate_k: Option<f64>,
    pub predecessor_supply_temperature_calculated: bool,
    pub predecessor_calculated_supply_temperature_c: Option<f64>,
    pub predecessor_supply_temperature_assigned: bool,
    pub predecessor_assigned_supply_temperature_c: Option<f64>,
    pub predecessor_resulting_supply_temperature_c: Option<f64>,
    pub resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed:
        bool,
    pub cp389_retained_supply_temperature_state_owned: bool,
    pub preexisting_supply_temperature_c: Option<f64>,
    pub cp389_retained_supply_temperature_owned_read: bool,
    pub supply_temperature_for_minimum_read: bool,
    pub supply_temperature_before_mixed_air_limit_c: Option<f64>,
    pub cp329_retained_mixed_air_temperature_owned_read: bool,
    pub cp389_mixed_air_temperature_bit_corroborated: bool,
    pub mixed_air_temperature_for_minimum_read: bool,
    pub mixed_air_temperature_c: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_supply_temperature_c: Option<f64>,
    pub supply_temperature_assignment_performed: bool,
    pub assigned_supply_temperature_c: Option<f64>,
    pub resulting_supply_temperature_c: Option<f64>,
}

/// Final selected-unit CP390 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState,
}

/// Returns the bounded selected-unit CP390 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitLifecycleSummary, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitError>{
    let unit = runtime.units.get(&system).ok_or(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit.clone(),
    })
}
