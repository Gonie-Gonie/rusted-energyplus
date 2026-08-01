//! Bounded post-saturation cooling supply-enthalpy assignment.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError,
    advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_cooling_supply_enthalpy_post_saturation_assignment_is_consistent,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact,
    cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_supply_enthalpy_post_saturation_assignment_latest_metadata_is_consistent,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_enthalpy_post_saturation_assignment_characterization,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState;
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_enthalpy_post_saturation_assignment_state;

/// EnergyPlus source statement represented by CP379.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2261";
/// First lexically subsequent executable source statement excluded after CP379.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2264";
/// Exact dependency-ordered source sites represented by CP379.
///
/// The two side-effect-free reads do not claim C++ argument evaluation order.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-supply-temperature-for-post-saturation-enthalpy",
    "read-purchased-air-supply-humidity-ratio-for-post-saturation-enthalpy",
    "evaluate-psy-h-fn-tdb-w-for-post-saturation-enthalpy",
    "assign-local-supply-enthalpy-after-saturation-limit",
];

/// One CP378-to-CP379 source-ordered post-saturation enthalpy witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot {
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
    pub predecessor_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_supply_humidity_ratio_saturation_limit_assignment_performed: bool,
    pub predecessor_resulting_supply_humidity_ratio: Option<f64>,
    pub cp377_supply_temperature_owned_read: bool,
    pub cp334_supply_temperature_mixed_air_limit_owned_read: bool,
    pub cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read: bool,
    pub cp378_supply_humidity_ratio_saturation_limit_owned_read: bool,
    pub purchased_air_supply_temperature_for_post_saturation_enthalpy_read: bool,
    pub supply_temperature_c: Option<f64>,
    pub purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read: bool,
    pub supply_humidity_ratio: Option<f64>,
    pub psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated: bool,
    pub psychrometric_supply_enthalpy_j_per_kg: Option<f64>,
    pub local_supply_enthalpy_after_saturation_limit_assignment_performed: bool,
    pub assigned_supply_enthalpy_j_per_kg: Option<f64>,
    pub resulting_supply_enthalpy_j_per_kg: Option<f64>,
}

/// Final selected-unit CP379 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary {
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First lexically subsequent executable source statement excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState,
}

/// Returns the bounded selected-unit CP379 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_supply_enthalpy_post_saturation_assignment
                .clone(),
        },
    )
}
