//! Bounded cooling-capacity-zero candidate reset from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};

use super::super::PurchasedAirRuntimeState;

pub(in crate::ideal_loads::calc) mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use release::*;
pub(super) use state::PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute;
pub use state::PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState;
pub(super) use transition::advance_cooling_capacity_zero_flow_reset_state;

/// EnergyPlus source slice represented by CP321.
pub const PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2147-2152";
/// First executable source statement deliberately excluded.
pub const PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2155";
/// Exact ten source-order sites represented by CP321.
pub const PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER: &[&str] = &[
    "read-cooling-limit-for-capacity-comparison",
    "compare-cooling-limit-equal-to-capacity",
    "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "read-maximum-total-cooling-capacity-after-limit-condition-true",
    "compare-maximum-total-cooling-capacity-equal-to-zero",
    "enter-zero-cooling-capacity-body-if-compound-condition-satisfied",
    "assign-supply-mass-flow-rate-for-cooling-zero",
    "assign-supply-mass-flow-rate-for-dehumidification-zero",
    "assign-supply-mass-flow-rate-for-humidification-zero",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingCapacityZeroFlowResetInput {
    pub cooling_limit: IdealLoadsLimit,
    pub maximum_total_cooling_capacity_w: f64,
    pub supply_mass_flow_rate_for_cool_kg_per_s: f64,
    pub supply_mass_flow_rate_for_dehumidification_kg_per_s: f64,
}

/// One CP320-to-CP321 source-ordered witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_body_entered: bool,
    pub predecessor_cooling_body_entered: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub cooling_body_entered: bool,
    pub first_cooling_limit_read: bool,
    pub first_cooling_limit: Option<IdealLoadsLimit>,
    pub cooling_limit_capacity: Option<bool>,
    pub second_cooling_limit_read: bool,
    pub second_cooling_limit: Option<IdealLoadsLimit>,
    pub cooling_limit_flow_rate_and_capacity: Option<bool>,
    pub cooling_limit_condition_satisfied: Option<bool>,
    pub maximum_total_cooling_capacity_read: bool,
    pub maximum_total_cooling_capacity_w: Option<f64>,
    pub maximum_total_cooling_capacity_comparison_evaluated: bool,
    pub maximum_total_cooling_capacity_equal_to_zero: Option<bool>,
    pub zero_cooling_capacity_body_entered: bool,
    pub predecessor_supply_mass_flow_rate_for_cool_kg_per_s: Option<f64>,
    pub predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s: Option<f64>,
    pub predecessor_supply_mass_flow_rate_for_humidification_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_for_cool_zero_assigned: bool,
    pub assigned_supply_mass_flow_rate_for_cool_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_for_dehumidification_zero_assigned: bool,
    pub assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_for_humidification_zero_assigned: bool,
    pub assigned_supply_mass_flow_rate_for_humidification_kg_per_s: Option<f64>,
    pub resulting_supply_mass_flow_rate_for_cool_kg_per_s: Option<f64>,
    pub resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s: Option<f64>,
    pub resulting_supply_mass_flow_rate_for_humidification_kg_per_s: Option<f64>,
}

/// Final selected-unit CP321 lifecycle summary.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub state: PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
}

/// Returns the bounded selected-unit CP321 lifecycle summary.
pub fn purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    PurchasedAirCalcCoolingCapacityZeroFlowResetError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingCapacityZeroFlowResetError::UnknownSystem { system })?;
    Ok(
        PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_capacity_zero_flow_reset.clone(),
        },
    )
}
