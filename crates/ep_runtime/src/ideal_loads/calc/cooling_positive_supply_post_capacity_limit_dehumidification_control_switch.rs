//! Bounded post-capacity-limit dehumidification-control switch dispatch.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state,
};

/// EnergyPlus source statement represented by CP346.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2209";
/// First executable statement deliberately excluded after CP346.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2211";
/// Exact selector-read and switch-dispatch sites represented by CP346.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-dehumidification-control-type",
    "dispatch-dehumidification-control-switch",
];

/// One CP345-to-CP346 source-ordered switch-dispatch witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot
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
    pub dehumidification_control_type_read: bool,
    pub dehumidification_control_type: Option<DehumidificationControlType>,
    pub dehumidification_control_switch_dispatched: bool,
}

/// Final selected-unit CP346 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState,
}

/// Returns the bounded selected-unit CP346 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError,
>{
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
                .clone(),
        },
    )
}
