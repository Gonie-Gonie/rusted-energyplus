//! Bounded post-saturation constant-SHR supply-humidity-ratio assignment.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
pub(in crate::ideal_loads::calc) mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_committed_latest_snapshot_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_route,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_characterization,
};
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_state;

/// EnergyPlus source statement represented by CP392.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2284";
/// First executable source statement deliberately excluded after CP392.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2285";
/// Exact four dependency-ordered sites represented by CP392.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
        "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
        "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
        "evaluate-psy-w-fn-tdb-h-for-constant-sensible-heat-ratio-overdrying-limit",
        "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
    ];

/// One CP391-to-CP392 source-ordered supply-humidity-ratio assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot
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
    pub predecessor_cp390_resulting_supply_enthalpy_j_per_kg: Option<f64>,
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
    pub predecessor_cp390_resulting_supply_temperature_c: Option<f64>,
    pub dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed: bool,
    pub cp390_retained_supply_enthalpy_state_owned: bool,
    pub preexisting_supply_enthalpy_j_per_kg: Option<f64>,
    pub cp390_retained_supply_enthalpy_owned_read: bool,
    pub supply_enthalpy_for_overdrying_limit_maximum_read: bool,
    pub supply_enthalpy_before_overdrying_limit_j_per_kg: Option<f64>,
    pub cp390_retained_supply_temperature_owned_read: bool,
    pub supply_temperature_for_minimum_humidity_ratio_enthalpy_read: bool,
    pub predecessor_cp391_supply_temperature_c: Option<f64>,
    pub psychrometric_minimum_supply_enthalpy_evaluated: bool,
    pub psychrometric_minimum_supply_enthalpy_j_per_kg: Option<f64>,
    pub source_shaped_two_argument_maximum_evaluated: bool,
    pub maximum_supply_enthalpy_j_per_kg: Option<f64>,
    pub supply_enthalpy_assignment_performed: bool,
    pub assigned_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp391_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp391_resulting_supply_temperature_c: Option<f64>,
    pub dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed:
        bool,
    pub cp391_retained_supply_temperature_state_owned: bool,
    pub cp391_retained_supply_enthalpy_state_owned: bool,
    pub cp391_retained_supply_temperature_owned_read: bool,
    pub supply_temperature_for_humidity_ratio_inversion_read: bool,
    pub supply_temperature_c: Option<f64>,
    pub cp391_retained_supply_enthalpy_owned_read: bool,
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

/// Final selected-unit CP392 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP392 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentLifecycleSummary, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentError>{
    let unit = runtime.units.get(&system).ok_or(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment.clone(),
    })
}
