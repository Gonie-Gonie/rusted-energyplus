//! Bounded Cooling positive-supply capacity-limit `CpAir` assignment from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_capacity_limit_cp_air_assignment_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentInput,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentActiveInput,
    advance_cooling_positive_supply_capacity_limit_cp_air_assignment_state,
};

/// EnergyPlus source statement represented by CP338.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2196";
/// First executable statement deliberately excluded after CP338.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2197";
/// Exact three textual source sites represented by CP338.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-mixed-air-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air",
];

/// One CP337-to-CP338 source-ordered capacity-limit `CpAir` assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot {
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
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_cp_air_assignment_executed: bool,
    pub mixed_air_humidity_ratio_read: bool,
    pub mixed_air_humidity_ratio: Option<f64>,
    pub psychrometric_cp_air_evaluated: bool,
    pub psychrometric_cp_air_result_j_per_kg_k: Option<f64>,
    pub cp_air_assigned: bool,
    pub cp_air_j_per_kg_k: Option<f64>,
}

/// Final selected-unit CP338 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP338 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_capacity_limit_cp_air_assignment
                .clone(),
        },
    )
}
