//! Persistent CP324 cooling supply-mass-flow EMS override body state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRetainedRoute
{
    UnitOff,
    NonCooling,
    EmsDisabledFallthrough,
    OverrideApplied,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub body_entry_count: usize,
    pub body_skip_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub ems_disabled_fallthrough_count: usize,
    pub ems_supply_mass_flow_override_value_read_count: usize,
    pub supply_mass_flow_rate_override_assignment_count: usize,
    pub outdoor_air_mass_flow_rate_for_minimum_read_count: usize,
    pub supply_mass_flow_rate_for_minimum_read_count: usize,
    pub source_shaped_two_argument_minimum_evaluation_count: usize,
    pub outdoor_air_mass_flow_rate_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState {
    /// Creates zeroed CP324 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            body_entry_count: 0,
            body_skip_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            ems_disabled_fallthrough_count: 0,
            ems_supply_mass_flow_override_value_read_count: 0,
            supply_mass_flow_rate_override_assignment_count: 0,
            outdoor_air_mass_flow_rate_for_minimum_read_count: 0,
            supply_mass_flow_rate_for_minimum_read_count: 0,
            source_shaped_two_argument_minimum_evaluation_count: 0,
            outdoor_air_mass_flow_rate_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
