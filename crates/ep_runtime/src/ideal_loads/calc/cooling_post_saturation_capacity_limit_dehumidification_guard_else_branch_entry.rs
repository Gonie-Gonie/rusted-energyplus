//! Bounded post-saturation not-dehumidifying else-branch entry.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_route,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_has_exact_cp417_prefix_and_marker,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_characterization,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshots_match_bit_exact,
};
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState;
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use transition::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_state_with_validated_route,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_route_from_validated_predecessor,
};

/// EnergyPlus source statement represented by CP418.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2327";
/// First lexically subsequent executable source statement excluded after CP418.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2330";
/// Exact source/dependency order represented by CP418.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER: &[&str] = &[
    "enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-after-guard-false-fallthrough",
];

/// One CP417-to-CP418 source-ordered not-dehumidifying else-entry witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot
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
    pub resulting_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub resulting_supply_temperature_c: Option<f64>,
    pub post_saturation_capacity_limit_dehumidification_guard_else_branch_entered: bool,
}

/// Final selected-unit CP418 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState,
}

/// Returns the bounded selected-unit CP418 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryLifecycleSummary,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError,
>{
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryError::UnknownSystem {
            system,
        },
    )?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry.clone(),
    })
}
