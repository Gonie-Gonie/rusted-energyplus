//! Persistent CP323 cooling supply-mass-flow EMS override guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute
{
    UnitOff,
    NonCooling,
    OverrideBodyEntered,
    OverrideGuardFalseFallthrough,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub ems_supply_mass_flow_override_flag_read_count: usize,
    pub ems_supply_mass_flow_override_guard_evaluation_count: usize,
    pub ems_supply_mass_flow_override_body_entry_count: usize,
    pub ems_supply_mass_flow_override_guard_false_fallthrough_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState {
    /// Creates zeroed CP323 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            ems_supply_mass_flow_override_flag_read_count: 0,
            ems_supply_mass_flow_override_guard_evaluation_count: 0,
            ems_supply_mass_flow_override_body_entry_count: 0,
            ems_supply_mass_flow_override_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
