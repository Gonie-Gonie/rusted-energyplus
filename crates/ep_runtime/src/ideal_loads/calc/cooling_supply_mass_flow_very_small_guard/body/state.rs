//! Persistent CP328 cooling supply-mass-flow positive-zero reset-body state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveZeroAssigned,
    ActiveGuardFalseFallthrough,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub zero_flow_reset_body_entry_count: usize,
    pub body_skip_count: usize,
    pub active_guard_false_fallthrough_count: usize,
    pub supply_mass_flow_rate_positive_zero_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_zero_flow_reset_body_entry_count: usize,
    pub(super) witnessed_active_guard_false_fallthrough_count: usize,
}

impl PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState {
    /// Creates zeroed CP328 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            zero_flow_reset_body_entry_count: 0,
            body_skip_count: 0,
            active_guard_false_fallthrough_count: 0,
            supply_mass_flow_rate_positive_zero_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_zero_flow_reset_body_entry_count: 0,
            witnessed_active_guard_false_fallthrough_count: 0,
        }
    }
}
