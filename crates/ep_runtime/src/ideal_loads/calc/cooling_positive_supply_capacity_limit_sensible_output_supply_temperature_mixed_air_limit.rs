//! Bounded Cooling capacity-limit supply-temperature mixed-air limit.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

#[allow(unused_imports)]
pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent;
pub(in crate::ideal_loads) use release::cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit,
};
pub(super) use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute;
pub use state::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState;
pub(in crate::ideal_loads::calc) use transition::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitActiveOperands,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput,
    advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_state,
};

/// EnergyPlus source statement represented by CP344.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2203";
/// First executable statement deliberately excluded after CP344.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2208";
/// Exact four textual source sites represented by CP344.
pub const PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER:
    &[&str] = &[
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature",
];

/// One CP343-to-CP344 source-ordered mixed-air temperature-limit witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot
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
    pub predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed: bool,
    pub predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed: bool,
    pub predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub positive_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_guard_false_fallthrough_skipped: bool,
    pub capacity_limit_sensible_output_guard_false_fallthrough: bool,
    pub capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed: bool,
    pub preexisting_supply_temperature_c: Option<f64>,
    pub supply_temperature_for_minimum_read: bool,
    pub supply_temperature_before_mixed_air_limit_c: Option<f64>,
    pub mixed_air_temperature_for_minimum_read: bool,
    pub mixed_air_temperature_c: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_supply_temperature_c: Option<f64>,
    pub supply_temperature_assignment_performed: bool,
    pub assigned_supply_temperature_c: Option<f64>,
    pub resulting_supply_temperature_c: Option<f64>,
}

/// Final selected-unit CP344 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary
{
    /// EnergyPlus source statement.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
}

/// Returns the bounded selected-unit CP344 lifecycle summary.
pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
                .clone(),
        },
    )
}
