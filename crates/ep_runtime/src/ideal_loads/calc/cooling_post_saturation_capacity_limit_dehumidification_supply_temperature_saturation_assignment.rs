//! Bounded post-saturation supply-temperature saturation assignment.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization,
};
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_state;

/// EnergyPlus source statement represented by CP414.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2316";
/// First lexically subsequent executable source statement excluded after CP414.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2319";
/// Exact source/dependency order represented by CP414.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-cp413-retained-supply-enthalpy-for-saturation-temperature",
    "read-environment-outdoor-barometric-pressure-for-saturation-temperature",
    "evaluate-psy-tsat-fn-h-pb",
    "assign-purchased-air-supply-temperature-to-saturation-temperature",
];

/// One CP413-to-CP414 source-ordered saturation-temperature assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot
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
    pub resulting_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub resulting_supply_temperature_c: Option<f64>,
}

/// Final selected-unit CP414 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP414 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment.clone(),
    })
}
