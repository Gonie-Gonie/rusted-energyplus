//! Bounded cooling supply-mass-flow maximum from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::super::PurchasedAirRuntimeState;

mod ems_override_guard;
mod flow_limit_guard;
pub(in crate::ideal_loads::calc) mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub use ems_override_guard::*;
pub use flow_limit_guard::*;
pub use release::*;
pub(super) use state::PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState;
pub(super) use transition::advance_cooling_supply_mass_flow_maximum_state;

/// EnergyPlus source slice represented by CP322.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2155";
/// First executable source statement deliberately excluded.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2157";
/// Exact six lexical source sites represented by CP322.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER: &[&str] = &[
    "read-outdoor-air-mass-flow-rate",
    "read-supply-mass-flow-rate-for-cool",
    "read-supply-mass-flow-rate-for-dehumidification",
    "read-supply-mass-flow-rate-for-humidification",
    "apply-source-shaped-five-argument-maximum-with-positive-zero-floor",
    "assign-supply-mass-flow-rate",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyMassFlowMaximumInput {
    pub outdoor_air_mass_flow_rate_kg_per_s: f64,
}

/// One operand identity retained by the exact Objexx-style maximum tree.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand {
    /// Leading positive-zero floor.
    PositiveZeroFloor,
    /// Current outdoor-air mass flow.
    OutdoorAir,
    /// CP321 cooling candidate.
    Cooling,
    /// CP321 dehumidification candidate.
    Dehumidification,
    /// CP321 humidification candidate.
    Humidification,
}

/// One CP321-to-CP322 source-ordered witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot {
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
    pub outdoor_air_mass_flow_rate_read: bool,
    pub outdoor_air_mass_flow_rate_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_for_cool_read: bool,
    pub supply_mass_flow_rate_for_cool_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_for_dehumidification_read: bool,
    pub supply_mass_flow_rate_for_dehumidification_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_for_humidification_read: bool,
    pub supply_mass_flow_rate_for_humidification_kg_per_s: Option<f64>,
    pub positive_zero_vs_outdoor_air_comparison_evaluated: bool,
    pub positive_zero_less_than_outdoor_air: Option<bool>,
    pub positive_zero_outdoor_air_winner:
        Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand>,
    pub positive_zero_outdoor_air_maximum_kg_per_s: Option<f64>,
    pub cooling_vs_dehumidification_comparison_evaluated: bool,
    pub cooling_less_than_dehumidification: Option<bool>,
    pub cooling_dehumidification_winner:
        Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand>,
    pub cooling_dehumidification_maximum_kg_per_s: Option<f64>,
    pub leading_vs_candidate_pair_comparison_evaluated: bool,
    pub leading_less_than_candidate_pair: Option<bool>,
    pub leading_candidate_pair_winner: Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand>,
    pub leading_candidate_pair_maximum_kg_per_s: Option<f64>,
    pub leading_vs_humidification_comparison_evaluated: bool,
    pub leading_less_than_humidification: Option<bool>,
    pub final_winner: Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand>,
    pub maximum_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_assigned: bool,
    pub assigned_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub resulting_supply_mass_flow_rate_kg_per_s: Option<f64>,
}

/// Final selected-unit CP322 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
}

/// Returns the bounded selected-unit CP322 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingSupplyMassFlowMaximumError::UnknownSystem { system })?;
    Ok(
        PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_supply_mass_flow_maximum.clone(),
        },
    )
}
