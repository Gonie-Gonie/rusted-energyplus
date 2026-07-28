//! Persistent CP326 cooling supply-mass-flow limit-body state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRetainedRoute {
    UnitOff,
    NonCooling,
    ActiveGuardFalseFallthrough,
    SupplyMassFlowLimitApplied,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub supply_mass_flow_limit_body_entry_count: usize,
    pub body_skip_count: usize,
    pub active_guard_false_fallthrough_count: usize,
    pub supply_mass_flow_rate_for_minimum_read_count: usize,
    pub maximum_cooling_air_mass_flow_rate_for_minimum_read_count: usize,
    pub source_shaped_two_argument_minimum_evaluation_count: usize,
    pub supply_mass_flow_rate_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState {
    /// Creates zeroed CP326 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            supply_mass_flow_limit_body_entry_count: 0,
            body_skip_count: 0,
            active_guard_false_fallthrough_count: 0,
            supply_mass_flow_rate_for_minimum_read_count: 0,
            maximum_cooling_air_mass_flow_rate_for_minimum_read_count: 0,
            source_shaped_two_argument_minimum_evaluation_count: 0,
            supply_mass_flow_rate_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
