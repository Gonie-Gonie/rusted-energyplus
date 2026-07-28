//! Bounded cooling supply-mass-flow EMS override guard from `CalcPurchAirLoads`.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use crate::ideal_loads::PurchasedAirRuntimeState;

mod release;
mod state;
#[cfg(test)]
mod tests;
mod transition;

pub(in crate::ideal_loads) use release::cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release;
pub use release::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
};
pub(super) use state::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute;
pub use state::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState;
pub(super) use transition::advance_cooling_supply_mass_flow_ems_override_guard_state;

/// EnergyPlus source slice represented by CP323.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2157";
/// First executable source statement deliberately excluded.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE:
    &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2158";
/// Exact three lexical source sites represented by CP323.
pub const PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-ems-supply-mass-flow-override-flag",
    "evaluate-ems-supply-mass-flow-override-guard",
    "enter-ems-supply-mass-flow-override-body-if-enabled",
];

/// One CP322-to-CP323 source-ordered guard witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
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
    pub ems_supply_mass_flow_override_flag_read: bool,
    pub ems_supply_mass_flow_override_enabled: Option<bool>,
    pub ems_supply_mass_flow_override_guard_evaluated: bool,
    pub ems_supply_mass_flow_override_body_entered: bool,
    pub ems_supply_mass_flow_override_guard_false_fallthrough: bool,
}

/// Final selected-unit CP323 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
}

/// Returns the bounded selected-unit CP323 lifecycle summary.
pub fn purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError::UnknownSystem { system },
    )?;
    Ok(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_supply_mass_flow_ems_override_guard
                .clone(),
        },
    )
}
