//! Bounded post-saturation dehumidification-control switch dispatch.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchError,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads) use release::{
    cooling_post_saturation_capacity_limit_dehumidification_control_switch_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_switch_characterization,
};
#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_switch_is_consistent;
pub(in crate::ideal_loads::calc) use release::cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact;
pub(in crate::ideal_loads::calc) use release::cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshots_match_bit_exact;
pub use state::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState;
pub(in crate::ideal_loads) use transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state;

/// EnergyPlus source statement represented by CP386.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2272";
/// First excluded lexical source line after CP386.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2273";
/// First executable source statement deliberately excluded after CP386.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2277";
/// Exact selector-read and symbolic-dispatch sites represented by CP386.
pub const PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-dehumidification-control-type",
    "dispatch-dehumidification-control-switch",
];

/// One CP385-to-CP386 source-ordered switch-dispatch witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot
{
    pub source: &'static str,
    pub first_excluded_lexical_source: &'static str,
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
    pub predecessor_supply_enthalpy_assignment_executed: bool,
    pub predecessor_resulting_supply_enthalpy_j_per_kg: Option<f64>,
    pub dehumidification_control_type_read: bool,
    pub dehumidification_control_type: Option<DehumidificationControlType>,
    pub dehumidification_control_switch_dispatched: bool,
    pub resulting_supply_enthalpy_j_per_kg: Option<f64>,
}

/// Final selected-unit CP386 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First excluded lexical source line.
    pub first_excluded_lexical_source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState,
}

/// Returns the bounded selected-unit CP386 lifecycle summary.
pub fn purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchError::UnknownSystem {
            system,
        },
    )?;
    Ok(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
        first_excluded_lexical_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_LEXICAL_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
        state: unit
            .calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch
            .clone(),
    })
}
