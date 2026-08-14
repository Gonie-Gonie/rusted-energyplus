//! Bounded post-saturation dehumidifying cooling-total-output assignment.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent;
pub(in crate::ideal_loads::calc) use release::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_committed_latest_snapshot_is_consistent;
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshots_match_bit_exact;
pub(in crate::ideal_loads::calc) use release::snapshot_route as cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_route;
pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentInput,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_characterization,
};
pub(super) use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState;
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_state;

/// EnergyPlus source statement represented by CP382.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2267";
/// First executable statement deliberately excluded after CP382.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2268";
/// Exact dependency-ordered source sites represented by CP382.
///
/// The three side-effect-free reads do not claim C++ operand evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-retained-supply-mass-flow-rate-for-post-saturation-dehumidification-total-output-product",
    "read-retained-mixed-air-enthalpy-for-post-saturation-dehumidification-total-output-difference",
    "read-retained-supply-enthalpy-for-post-saturation-dehumidification-total-output-difference",
    "calculate-mixed-air-enthalpy-minus-supply-enthalpy-for-post-saturation-dehumidification-total-output",
    "calculate-supply-mass-flow-rate-times-enthalpy-difference-for-post-saturation-dehumidification-total-output",
    "assign-local-cooling-total-output-for-post-saturation-dehumidification",
];

/// One CP381-to-CP382 source-ordered total-output assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot
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
    pub dehumidification_total_output_assignment_executed: bool,
    pub cp330_supply_mass_flow_rate_owned_read: bool,
    pub cp329_same_call_supply_mass_flow_rate_bit_corroborated: bool,
    pub cp339_same_call_supply_mass_flow_rate_bit_corroborated: bool,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub cp329_mixed_air_enthalpy_owned_read: bool,
    pub cp329_same_call_recirculation_enthalpy_bit_corroborated: bool,
    pub cp339_same_call_mixed_air_enthalpy_bit_corroborated: bool,
    pub mixed_air_enthalpy_read: bool,
    pub mixed_air_enthalpy_j_per_kg: Option<f64>,
    pub cp379_post_saturation_supply_enthalpy_owned_read: bool,
    pub cp379_same_call_supply_enthalpy_bits_corroborated: bool,
    pub supply_enthalpy_read: bool,
    pub supply_enthalpy_j_per_kg: Option<f64>,
    pub enthalpy_difference_calculated: bool,
    pub mixed_air_minus_supply_enthalpy_j_per_kg: Option<f64>,
    pub cooling_total_output_calculated: bool,
    pub calculated_cooling_total_output_w: Option<f64>,
    pub cooling_total_output_assigned: bool,
    pub cooling_total_output_w: Option<f64>,
}

/// Final selected-unit CP382 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP382 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError,
>{
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment
            .clone(),
    })
}
