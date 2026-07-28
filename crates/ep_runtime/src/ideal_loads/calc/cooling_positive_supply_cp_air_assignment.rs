//! Bounded Cooling positive-supply `CpAir` assignment from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_cp_air_assignment_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentInput,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentActiveInput,
    advance_cooling_positive_supply_cp_air_assignment_state,
};

/// EnergyPlus source statement represented by CP331.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2185";
/// First executable statement deliberately excluded after CP331.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2186";
/// Exact three textual source sites represented by CP331.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-controlled-zone-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air",
];

/// One CP330-to-CP331 source-ordered `CpAir` assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot {
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
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub cp_air_assignment_executed: bool,
    pub zone_humidity_ratio_read: bool,
    pub zone_humidity_ratio: Option<f64>,
    pub psychrometric_cp_air_evaluated: bool,
    pub psychrometric_cp_air_result_j_per_kg_k: Option<f64>,
    pub cp_air_assigned: bool,
    pub cp_air_j_per_kg_k: Option<f64>,
}

/// Final selected-unit CP331 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP331 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_positive_supply_cp_air_assignment.clone(),
        },
    )
}
