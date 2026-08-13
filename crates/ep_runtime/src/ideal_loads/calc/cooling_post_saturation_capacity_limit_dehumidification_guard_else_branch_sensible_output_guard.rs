//! Bounded post-saturation sensible-output maximum-capacity guard.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshots_match_bit_exact,
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_characterization,
};
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState;
#[allow(unused_imports)]
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardActiveInput;
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use transition::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state_with_validated_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_route_from_committed_predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute,
};

/// EnergyPlus source statement represented by CP421.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2332";
/// First lexically subsequent executable source statement excluded after CP421.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2333";
/// Exact dependency-ordered reads, comparison, and conditional body entry.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-retained-cooling-sensible-output-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-comparison",
    "compare-post-saturation-capacity-limit-dehumidification-guard-else-branch-cooling-sensible-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
    "enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-capacity-adjustment-body-if-comparison-satisfied",
];

/// One CP420-to-CP421 source-ordered maximum-capacity guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot
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
    pub predecessor_dehumidification_control_humidistat_case_entered: bool,
    pub predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed:
        bool,
    pub predecessor_dehumidification_control_humidistat_case_exited_via_break: bool,
    pub predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered:
        bool,
    pub predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough:
        bool,
    pub predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed:
        bool,
    pub predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break:
        bool,
    pub predecessor_cp409_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp409_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp409_resulting_supply_temperature_c: Option<f64>,
    pub predecessor_dehumidification_control_default_case_exited_via_break: bool,
    pub predecessor_cp410_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp410_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp410_resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed:
        bool,
    pub cp410_retained_supply_humidity_ratio_state_owned: bool,
    pub cp410_retained_supply_enthalpy_state_owned: bool,
    pub cp410_retained_supply_temperature_state_owned: bool,
    pub cp410_retained_supply_humidity_ratio_owned_read: bool,
    pub purchased_air_supply_humidity_ratio_read: bool,
    pub purchased_air_supply_humidity_ratio_before_saturation_check: Option<f64>,
    pub local_supply_humidity_ratio_original_assignment_performed: bool,
    pub assigned_supply_humidity_ratio_original: Option<f64>,
    pub resulting_supply_humidity_ratio_original: Option<f64>,
    pub predecessor_cp411_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp411_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp411_resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed:
        bool,
    pub cp411_retained_supply_humidity_ratio_state_owned: bool,
    pub cp411_retained_supply_enthalpy_state_owned: bool,
    pub cp411_retained_supply_temperature_state_owned: bool,
    pub cp411_retained_supply_temperature_owned_read: bool,
    pub purchased_air_supply_temperature_for_saturation_humidity_ratio_read: bool,
    pub supply_temperature_for_saturation_humidity_ratio_c: Option<f64>,
    pub environment_outdoor_barometric_pressure_owned_read: bool,
    pub environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read: bool,
    pub outdoor_barometric_pressure_pa: Option<f64>,
    pub psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated: bool,
    pub saturation_supply_humidity_ratio: Option<f64>,
    pub local_saturation_supply_humidity_ratio_assignment_performed: bool,
    pub assigned_saturation_supply_humidity_ratio: Option<f64>,
    pub resulting_saturation_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp412_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp412_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp412_resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated:
        bool,
    pub cp412_saturation_supply_humidity_ratio_owned_read: bool,
    pub saturation_supply_humidity_ratio_for_guard_read: bool,
    pub saturation_supply_humidity_ratio_for_guard: Option<f64>,
    pub cp411_original_supply_humidity_ratio_owned_read: bool,
    pub cp412_same_call_original_supply_humidity_ratio_bit_corroborated: bool,
    pub original_supply_humidity_ratio_for_guard_read: bool,
    pub original_supply_humidity_ratio_for_guard: Option<f64>,
    pub saturation_original_supply_humidity_ratio_comparison_evaluated: bool,
    pub saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio:
        Option<bool>,
    pub saturation_supply_humidity_ratio_guard_body_entered: bool,
    pub saturation_supply_humidity_ratio_guard_false_fallthrough: bool,
    pub cp412_retained_supply_humidity_ratio_state_owned: bool,
    pub cp412_retained_supply_enthalpy_state_owned: bool,
    pub cp412_retained_supply_temperature_state_owned: bool,
    pub predecessor_cp413_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp413_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp413_resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed:
        bool,
    pub cp413_retained_supply_humidity_ratio_state_owned: bool,
    pub cp413_retained_supply_enthalpy_state_owned: bool,
    pub cp413_retained_supply_temperature_state_owned: bool,
    pub cp413_retained_supply_enthalpy_owned_read: bool,
    pub supply_enthalpy_for_saturation_temperature_read: bool,
    pub supply_enthalpy_for_saturation_temperature_j_per_kg: Option<f64>,
    pub environment_outdoor_barometric_pressure_for_saturation_temperature_owned_read: bool,
    pub environment_outdoor_barometric_pressure_for_saturation_temperature_read: bool,
    pub outdoor_barometric_pressure_for_saturation_temperature_pa: Option<f64>,
    pub psy_tsat_fn_h_pb_evaluated: bool,
    pub psychrometric_saturation_supply_temperature_result_c: Option<f64>,
    pub purchased_air_supply_temperature_saturation_assignment_performed: bool,
    pub assigned_saturation_supply_temperature_c: Option<f64>,
    pub predecessor_cp414_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp414_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp414_resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed:
        bool,
    pub cp414_retained_supply_temperature_state_owned: bool,
    pub preexisting_supply_temperature_c: Option<f64>,
    pub cp414_retained_supply_temperature_owned_read: bool,
    pub supply_temperature_for_minimum_read: bool,
    pub supply_temperature_before_mixed_air_limit_c: Option<f64>,
    pub cp329_retained_mixed_air_temperature_owned_read: bool,
    pub mixed_air_temperature_for_minimum_read: bool,
    pub mixed_air_temperature_c: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_supply_temperature_c: Option<f64>,
    pub supply_temperature_assignment_performed: bool,
    pub assigned_supply_temperature_c: Option<f64>,
    pub predecessor_cp415_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp415_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp415_resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed:
        bool,
    pub cp415_retained_supply_humidity_ratio_state_owned: bool,
    pub cp415_retained_supply_enthalpy_state_owned: bool,
    pub cp415_retained_supply_temperature_state_owned: bool,
    pub cp415_retained_supply_temperature_owned_read: bool,
    pub supply_temperature_for_humidity_ratio_inversion_read: bool,
    pub supply_temperature_c: Option<f64>,
    pub cp415_retained_supply_enthalpy_owned_read: bool,
    pub supply_enthalpy_for_humidity_ratio_inversion_read: bool,
    pub supply_enthalpy_j_per_kg: Option<f64>,
    pub psychrometric_supply_humidity_ratio_evaluated: bool,
    pub psychrometric_supply_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_assignment_performed: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp416_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp416_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp416_resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed: bool,
    pub cp416_retained_supply_humidity_ratio_state_owned: bool,
    pub cp416_retained_supply_enthalpy_state_owned: bool,
    pub cp416_retained_supply_temperature_state_owned: bool,
    pub cp416_retained_supply_temperature_owned_read: bool,
    pub supply_temperature_for_enthalpy_read: bool,
    pub supply_temperature_for_enthalpy_c: Option<f64>,
    pub cp416_retained_supply_humidity_ratio_owned_read: bool,
    pub supply_humidity_ratio_for_enthalpy_read: bool,
    pub supply_humidity_ratio_for_enthalpy: Option<f64>,
    pub psychrometric_supply_enthalpy_evaluated: bool,
    pub psychrometric_supply_enthalpy_j_per_kg: Option<f64>,
    pub supply_enthalpy_assignment_performed: bool,
    pub assigned_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp418_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp418_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp418_resulting_supply_temperature_c: Option<f64>,
    pub predecessor_post_saturation_capacity_limit_dehumidification_guard_else_branch_entered: bool,
    pub post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed:
        bool,
    pub cp329_retained_mixed_air_humidity_ratio_owned_read: bool,
    pub mixed_air_humidity_ratio_for_cp_air_read: bool,
    pub mixed_air_humidity_ratio_for_cp_air: Option<f64>,
    pub psychrometric_cp_air_evaluated: bool,
    pub psychrometric_cp_air_result_j_per_kg_k: Option<f64>,
    pub cp_air_assigned: bool,
    pub cp_air_j_per_kg_k: Option<f64>,
    pub predecessor_cp419_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp419_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp419_resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed:
        bool,
    pub cp419_retained_supply_humidity_ratio_state_owned: bool,
    pub cp419_retained_supply_enthalpy_state_owned: bool,
    pub cp419_retained_supply_temperature_state_owned: bool,
    pub cp330_retained_supply_mass_flow_rate_owned_read: bool,
    pub cp329_supply_mass_flow_rate_bit_corroborated: bool,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub cp419_retained_cp_air_owned_read: bool,
    pub cp_air_read: bool,
    pub cp419_cp_air_for_sensible_output_j_per_kg_k: Option<f64>,
    pub supply_mass_flow_rate_times_cp_air_calculated: bool,
    pub supply_mass_flow_rate_times_cp_air_w_per_k: Option<f64>,
    pub cp329_retained_mixed_air_temperature_for_sensible_output_owned_read: bool,
    pub mixed_air_temperature_read: bool,
    pub mixed_air_temperature_for_sensible_output_c: Option<f64>,
    pub cp419_retained_supply_temperature_owned_read: bool,
    pub supply_temperature_read: bool,
    pub supply_temperature_for_sensible_output_c: Option<f64>,
    pub mixed_air_minus_supply_temperature_calculated: bool,
    pub mixed_air_minus_supply_temperature_k: Option<f64>,
    pub cooling_sensible_output_calculated: bool,
    pub calculated_cooling_sensible_output_w: Option<f64>,
    pub cooling_sensible_output_assigned: bool,
    pub cooling_sensible_output_w: Option<f64>,
    pub predecessor_cp420_resulting_supply_humidity_ratio: Option<f64>,
    pub predecessor_cp420_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_cp420_resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated: bool,
    pub cp420_retained_cooling_sensible_output_owned_read: bool,
    pub cooling_sensible_output_read: bool,
    pub cp420_cooling_sensible_output_for_capacity_guard_w: Option<f64>,
    pub cp321_maximum_total_cooling_capacity_owned_read: bool,
    pub cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: bool,
    pub maximum_total_cooling_capacity_read: bool,
    pub maximum_total_cooling_capacity_w: Option<f64>,
    pub cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated: bool,
    pub cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity: Option<bool>,
    pub post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered: bool,
    pub post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough: bool,
    pub cp420_retained_supply_humidity_ratio_state_owned: bool,
    pub cp420_retained_supply_enthalpy_state_owned: bool,
    pub cp420_retained_supply_temperature_state_owned: bool,
    pub resulting_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub resulting_supply_temperature_c: Option<f64>,
}

/// Final selected-unit CP421 lifecycle summary.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleSummary
{
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState,
}

/// Returns the bounded selected-unit CP421 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleSummary,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardError::UnknownSystem {
            system,
        },
    )?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard.clone(),
    })
}
