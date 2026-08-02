//! Bounded post-saturation shared-case latent-output capacity guard.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_route,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization,
};
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState;
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardActiveInput;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_state;

/// EnergyPlus source statement represented by CP402.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2297";
/// First executable statement deliberately excluded after CP402.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2298";
/// Exact dependency-ordered sites represented by CP402.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-retained-cooling-latent-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-comparison",
    "compare-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cooling-latent-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
    "enter-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-body-if-comparison-satisfied",
];

/// One CP401-to-CP402 source-ordered latent-output guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot {
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
    pub predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: bool,
    pub predecessor_dehumidification_control_humidistat_case_entered: bool,
    pub predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: bool,
    pub predecessor_dehumidification_control_humidistat_case_exited_via_break: bool,
    pub predecessor_cp397_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp397_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp397_resulting_supply_temperature_c: Option<f64>,
    pub predecessor_dehumidification_control_none_case_entered: bool,
    pub predecessor_cp398_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp398_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp398_resulting_supply_temperature_c: Option<f64>,
    pub predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: bool,
    pub predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed: bool,
    pub predecessor_mixed_air_humidity_ratio_read: bool,
    pub predecessor_mixed_air_humidity_ratio: Option<f64>,
    pub predecessor_psychrometric_cp_air_evaluated: bool,
    pub predecessor_psychrometric_cp_air_result_j_per_kg_k: Option<f64>,
    pub predecessor_cp_air_assigned: bool,
    pub predecessor_cp_air_j_per_kg_k: Option<f64>,
    pub predecessor_cp399_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp399_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp399_resulting_supply_temperature_c: Option<f64>,
    pub predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed: bool,
    pub predecessor_cp399_retained_supply_humidity_ratio_state_owned: bool,
    pub predecessor_cp399_retained_supply_enthalpy_state_owned: bool,
    pub predecessor_cp399_retained_supply_temperature_state_owned: bool,
    pub predecessor_cp330_retained_supply_mass_flow_rate_owned_read: bool,
    pub predecessor_cp329_supply_mass_flow_rate_bit_corroborated: bool,
    pub predecessor_supply_mass_flow_rate_read: bool,
    pub predecessor_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub predecessor_cp399_retained_cp_air_owned_read: bool,
    pub predecessor_cp_air_read: bool,
    pub predecessor_cp400_cp_air_j_per_kg_k: Option<f64>,
    pub predecessor_supply_mass_flow_rate_times_cp_air_calculated: bool,
    pub predecessor_supply_mass_flow_rate_times_cp_air_w_per_k: Option<f64>,
    pub predecessor_cp329_retained_mixed_air_temperature_owned_read: bool,
    pub predecessor_mixed_air_temperature_read: bool,
    pub predecessor_mixed_air_temperature_c: Option<f64>,
    pub predecessor_cp399_retained_supply_temperature_owned_read: bool,
    pub predecessor_supply_temperature_read: bool,
    pub predecessor_supply_temperature_c: Option<f64>,
    pub predecessor_mixed_air_minus_supply_temperature_calculated: bool,
    pub predecessor_mixed_air_minus_supply_temperature_k: Option<f64>,
    pub predecessor_cooling_sensible_output_calculated: bool,
    pub predecessor_calculated_cooling_sensible_output_w: Option<f64>,
    pub predecessor_cooling_sensible_output_assigned: bool,
    pub predecessor_cooling_sensible_output_w: Option<f64>,
    pub predecessor_cp400_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp400_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp400_resulting_supply_temperature_c: Option<f64>,
    pub predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed: bool,
    pub predecessor_cp400_retained_supply_humidity_ratio_state_owned: bool,
    pub predecessor_cp400_retained_supply_enthalpy_state_owned: bool,
    pub predecessor_cp400_retained_supply_temperature_state_owned: bool,
    pub predecessor_cp384_retained_cooling_total_output_owned_read: bool,
    pub predecessor_cp385_cooling_total_output_bit_corroborated: bool,
    pub predecessor_cooling_total_output_read: bool,
    pub predecessor_cooling_total_output_w: Option<f64>,
    pub predecessor_cp400_retained_cooling_sensible_output_owned_read: bool,
    pub predecessor_cp401_cooling_sensible_output_read: bool,
    pub predecessor_cp401_cooling_sensible_output_w: Option<f64>,
    pub predecessor_cooling_latent_output_calculated: bool,
    pub predecessor_calculated_cooling_latent_output_w: Option<f64>,
    pub predecessor_cooling_latent_output_assigned: bool,
    pub predecessor_cooling_latent_output_w: Option<f64>,
    pub predecessor_cp401_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp401_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp401_resulting_supply_temperature_c: Option<f64>,
    pub dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated: bool,
    pub cp401_retained_cooling_latent_output_owned_read: bool,
    pub cooling_latent_output_read: bool,
    pub cooling_latent_output_w: Option<f64>,
    pub cp321_maximum_total_cooling_capacity_owned_read: bool,
    pub cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: bool,
    pub maximum_total_cooling_capacity_read: bool,
    pub maximum_total_cooling_capacity_w: Option<f64>,
    pub cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated: bool,
    pub cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity: Option<bool>,
    pub dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered: bool,
    pub dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: bool,
    pub cp401_retained_supply_humidity_ratio_state_owned: bool,
    pub cp401_retained_supply_enthalpy_state_owned: bool,
    pub cp401_retained_supply_temperature_state_owned: bool,
    pub resulting_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub resulting_supply_temperature_c: Option<f64>,
}

/// Final selected-unit CP402 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState,
}

/// Returns the bounded selected-unit CP402 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleSummary, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardError> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardError::UnknownSystem { system },
    )?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard.clone(),
    })
}
