//! Bounded cooling supply-mass-flow EMS override body from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub(in crate::ideal_loads::calc) use release::completed_direct_cooling_supply_mass_flow_ems_override_body_is_consistent;
pub(in crate::ideal_loads) use release::cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState;
pub(in crate::ideal_loads::calc) use transition::advance_cooling_supply_mass_flow_ems_override_body_state;

/// EnergyPlus source slice represented by CP324.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2158-2159";
/// First executable source statement deliberately excluded.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2161";
/// Exact six lexical source sites represented by CP324.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER: &[&str] = &[
    "read-ems-supply-mass-flow-override-value",
    "assign-supply-mass-flow-rate-from-ems-override",
    "read-outdoor-air-mass-flow-rate-for-minimum",
    "read-supply-mass-flow-rate-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-outdoor-air-mass-flow-rate",
];

/// Pre-sampled values used only by internal true-body characterization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput {
    pub ems_supply_mass_flow_override_value_kg_per_s: f64,
    pub outdoor_air_mass_flow_rate_before_override_kg_per_s: f64,
}

/// One CP323-to-CP324 source-ordered body witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub source_order: &'static [&'static str],
    pub system: IdealLoadsAirSystemId,
    pub parent_call_ordinal: usize,
    pub controlled_zone: ZoneId,
    pub unit_body_entered: bool,
    pub predecessor_cooling_body_entered: bool,
    pub predecessor_ems_supply_mass_flow_override_body_entered: bool,
    pub predecessor_ems_supply_mass_flow_override_guard_false_fallthrough: bool,
    pub unit_off_skipped: bool,
    pub non_cooling_skipped: bool,
    pub cooling_body_entered: bool,
    pub body_skipped: bool,
    pub ems_disabled_fallthrough: bool,
    pub ems_supply_mass_flow_override_value_read: bool,
    pub ems_supply_mass_flow_override_value_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_override_assignment_performed: bool,
    pub assigned_supply_mass_flow_rate_kg_per_s: Option<f64>,
    pub outdoor_air_mass_flow_rate_for_minimum_read: bool,
    pub outdoor_air_mass_flow_rate_before_override_kg_per_s: Option<f64>,
    pub supply_mass_flow_rate_for_minimum_read: bool,
    pub supply_mass_flow_rate_for_minimum_kg_per_s: Option<f64>,
    pub source_shaped_two_argument_minimum_evaluated: bool,
    pub minimum_outdoor_air_mass_flow_rate_kg_per_s: Option<f64>,
    pub outdoor_air_mass_flow_rate_assignment_performed: bool,
    pub assigned_outdoor_air_mass_flow_rate_kg_per_s: Option<f64>,
}

/// Final selected-unit CP324 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
}

/// Returns the bounded selected-unit CP324 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
            state: unit.calc_cooling_supply_mass_flow_ems_override_body.clone(),
        },
    )
}
