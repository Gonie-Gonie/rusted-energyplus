//! Bounded post-capacity-limit dehumidification-control `None` case.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state,
};

/// EnergyPlus source statements represented by CP347.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2210-2212";
/// First executable statement deliberately excluded after CP347.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2216";
/// Exact `None` case entry, read, assignment, and break sites represented by CP347.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER:
    &[&str] = &[
    "enter-purchased-air-dehumidification-control-none-case",
    "read-purchased-air-mixed-air-humidity-ratio-for-none-case",
    "assign-purchased-air-supply-humidity-ratio-in-none-case",
    "exit-purchased-air-dehumidification-control-none-case-via-break",
];

/// One CP346-to-CP347 source-ordered `None`-case witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot
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
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub predecessor_capacity_limit_guard_false_fallthrough: bool,
    pub predecessor_capacity_limit_sensible_output_guard_false_fallthrough: bool,
    pub predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed:
        bool,
    pub predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed: bool,
    pub predecessor_assigned_supply_humidity_ratio: Option<f64>,
    pub predecessor_dehumidification_control_type_read: bool,
    pub predecessor_dehumidification_control_type: Option<DehumidificationControlType>,
    pub predecessor_dehumidification_control_switch_dispatched: bool,
    pub dehumidification_control_none_case_entered: bool,
    pub mixed_air_humidity_ratio_read: bool,
    pub mixed_air_humidity_ratio: Option<f64>,
    pub supply_humidity_ratio_assignment_performed: bool,
    pub assigned_supply_humidity_ratio: Option<f64>,
    pub resulting_supply_humidity_ratio: Option<f64>,
    pub dehumidification_control_none_case_exited_via_break: bool,
}

/// Final selected-unit CP347 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary
{
    /// EnergyPlus source statements.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
}

/// Returns the bounded selected-unit CP347 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError,
>{
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
                .clone(),
        },
    )
}
