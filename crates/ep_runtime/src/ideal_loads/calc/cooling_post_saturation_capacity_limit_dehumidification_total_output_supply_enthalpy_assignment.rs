//! Bounded post-saturation capacity-limited dehumidification supply-enthalpy assignment.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshots_match_bit_exact,
};
pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_characterization,
};
pub(super) use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState;
pub(in crate::ideal_loads) use transition::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentActiveOperands,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedInput,
};
pub(in crate::ideal_loads::calc) use transition::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state,
};

/// EnergyPlus source statement represented by CP385.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2270";
/// First executable statement deliberately excluded after CP385.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2272";
/// Exact source-ordered reads, calculations, and assignment represented by CP385.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-retained-mixed-air-enthalpy-for-post-saturation-capacity-limited-dehumidification-supply-enthalpy-difference",
    "read-retained-cooling-total-output-for-post-saturation-capacity-limited-dehumidification-specific-cooling-output-division",
    "read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limited-dehumidification-specific-cooling-output-division",
    "calculate-cooling-total-output-divided-by-supply-mass-flow-rate-for-post-saturation-capacity-limited-dehumidification-supply-enthalpy",
    "calculate-mixed-air-enthalpy-minus-specific-cooling-output-for-post-saturation-capacity-limited-dehumidification-supply-enthalpy",
    "assign-local-supply-enthalpy-after-post-saturation-capacity-limited-dehumidification-total-output-adjustment",
];

/// One CP384-to-CP385 source-ordered supply-enthalpy assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot {
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
    pub supply_enthalpy_assignment_executed: bool,
    pub preexisting_supply_enthalpy_j_per_kg: Option<f64>,
    pub cp379_retained_supply_enthalpy_owned_read: bool,
    pub cp329_retained_mixed_air_enthalpy_owned_read: bool,
    pub mixed_air_enthalpy_read: bool,
    pub mixed_air_enthalpy_j_per_kg: Option<f64>,
    pub cp384_retained_cooling_total_output_owned_read: bool,
    pub cooling_total_output_read: bool,
    pub cooling_total_output_w: Option<f64>,
    pub cp330_retained_supply_mass_flow_rate_owned_read: bool,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub specific_cooling_output_calculated: bool,
    pub specific_cooling_output_j_per_kg: Option<f64>,
    pub supply_enthalpy_difference_calculated: bool,
    pub calculated_supply_enthalpy_j_per_kg: Option<f64>,
    pub supply_enthalpy_assigned: bool,
    pub assigned_supply_enthalpy_j_per_kg: Option<f64>,
    pub resulting_supply_enthalpy_j_per_kg: Option<f64>,
}

/// Final selected-unit CP385 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP385 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentError> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentError::UnknownSystem { system },
    )?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment.clone(),
    })
}
