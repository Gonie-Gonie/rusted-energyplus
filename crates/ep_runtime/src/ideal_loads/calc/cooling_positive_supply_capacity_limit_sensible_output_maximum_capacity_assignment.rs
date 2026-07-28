//! Bounded Cooling sensible-output maximum-capacity assignment from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_state;

/// EnergyPlus source statement represented by CP341.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2199";
/// First executable statement deliberately excluded after CP341.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2200";
/// Exact two textual source sites represented by CP341.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
    "read-retained-maximum-total-cooling-capacity-for-sensible-output-assignment",
    "assign-local-cooling-sensible-output-from-maximum-total-cooling-capacity",
];

/// One CP340-to-CP341 source-ordered maximum-capacity assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot
{
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
    pub predecessor_capacity_limit_sensible_output_assignment_executed: bool,
    pub predecessor_capacity_limit_sensible_output_guard_evaluated: bool,
    pub predecessor_capacity_limit_sensible_output_guard_false_fallthrough: bool,
    pub predecessor_capacity_limit_sensible_output_adjustment_body_entered: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_sensible_output_guard_false_fallthrough: bool,
    pub capacity_limit_sensible_output_maximum_capacity_assignment_executed: bool,
    pub preexisting_cooling_sensible_output_w: Option<f64>,
    pub maximum_total_cooling_capacity_read: bool,
    pub maximum_total_cooling_capacity_w: Option<f64>,
    pub cooling_sensible_output_assigned: bool,
    pub assigned_cooling_sensible_output_w: Option<f64>,
    pub resulting_cooling_sensible_output_w: Option<f64>,
}

/// Final selected-unit CP341 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP341 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment
                .clone(),
        },
    )
}
