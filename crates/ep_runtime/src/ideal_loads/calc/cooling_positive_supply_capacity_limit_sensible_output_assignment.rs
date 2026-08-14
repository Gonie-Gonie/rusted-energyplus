//! Bounded Cooling capacity-limit sensible-output assignment from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
pub(in crate::ideal_loads::calc) mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent;
pub(in crate::ideal_loads::calc) use release::cooling_positive_supply_capacity_limit_sensible_output_assignment_committed_latest_snapshot_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentInput,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentActiveInput,
    advance_cooling_positive_supply_capacity_limit_sensible_output_assignment_state,
};

/// EnergyPlus source statement represented by CP339.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2197";
/// First executable statement deliberately excluded after CP339.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2198";
/// Exact six textual source sites represented by CP339.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
    "read-retained-supply-mass-flow-rate-for-sensible-output-product",
    "read-retained-mixed-air-enthalpy-for-sensible-output-difference",
    "read-retained-supply-enthalpy-for-sensible-output-difference",
    "calculate-mixed-air-enthalpy-minus-supply-enthalpy",
    "calculate-supply-mass-flow-rate-times-enthalpy-difference",
    "assign-local-cooling-sensible-output",
];

/// One CP338-to-CP339 source-ordered sensible-output assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_body_entered: bool,
    pub predecessor_cooling_body_entered: bool,
    pub predecessor_no_outdoor_air_fallback_entered: bool,
    pub predecessor_positive_supply_mass_flow_body_entered: bool,
    pub predecessor_active_guard_false_fallthrough: bool,
    pub predecessor_capacity_limit_guard_evaluated: bool,
    pub predecessor_capacity_limit_body_entered: bool,
    pub predecessor_active_capacity_limit_guard_false_fallthrough: bool,
    pub predecessor_capacity_limit_cp_air_assignment_executed: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_sensible_output_assignment_executed: bool,
    pub supply_mass_flow_rate_read: bool,
    pub supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub mixed_air_enthalpy_read: bool,
    pub mixed_air_enthalpy_j_per_kg: Option<f64>,
    pub supply_enthalpy_read: bool,
    pub supply_enthalpy_j_per_kg: Option<f64>,
    pub enthalpy_difference_calculated: bool,
    pub mixed_air_minus_supply_enthalpy_j_per_kg: Option<f64>,
    pub cooling_sensible_output_calculated: bool,
    pub calculated_cooling_sensible_output_w: Option<f64>,
    pub cooling_sensible_output_assigned: bool,
    pub cooling_sensible_output_w: Option<f64>,
}

/// Final selected-unit CP339 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP339 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
                .clone(),
        },
    )
}
