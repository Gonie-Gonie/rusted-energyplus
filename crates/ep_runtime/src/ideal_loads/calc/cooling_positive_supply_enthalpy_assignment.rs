//! Bounded Cooling positive-supply enthalpy assignment from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError,
    advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentActiveInput,
    advance_cooling_positive_supply_enthalpy_assignment_state,
};

/// EnergyPlus source statement represented by CP336.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2191";
/// First executable statement deliberately excluded after CP336.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2195";
/// Exact four textual source sites represented by CP336.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER: &[&str] = &[
    "read-purchased-air-supply-temperature-for-enthalpy",
    "read-purchased-air-supply-humidity-ratio-for-enthalpy",
    "evaluate-psy-h-fn-tdb-w",
    "assign-local-supply-enthalpy",
];

/// One CP335-to-CP336 source-ordered supply-enthalpy assignment witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot {
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
    pub supply_enthalpy_assignment_executed: bool,
    pub supply_temperature_for_enthalpy_read: bool,
    pub supply_temperature_c: Option<f64>,
    pub supply_humidity_ratio_for_enthalpy_read: bool,
    pub supply_humidity_ratio: Option<f64>,
    pub psychrometric_supply_enthalpy_evaluated: bool,
    pub psychrometric_supply_enthalpy_result_j_per_kg: Option<f64>,
    pub supply_enthalpy_assigned: bool,
    pub supply_enthalpy_j_per_kg: Option<f64>,
}

/// Final selected-unit CP336 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP336 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_enthalpy_assignment
                .clone(),
        },
    )
}
