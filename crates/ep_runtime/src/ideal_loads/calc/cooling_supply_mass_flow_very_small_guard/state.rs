//! Persistent CP327 cooling supply-mass-flow very-small-guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRetainedRoute
{
    UnitOff,
    NonCooling,
    ZeroFlowResetBodyEntered,
    ActiveGuardFalseFallthrough,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub hvac_very_small_mass_flow_read_count: usize,
    pub supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count: usize,
    pub zero_flow_reset_body_entry_count: usize,
    pub active_guard_false_fallthrough_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_zero_flow_reset_body_entry_count: usize,
    pub(super) witnessed_active_guard_false_fallthrough_count: usize,
}

impl PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState {
    /// Creates zeroed CP327 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            supply_mass_flow_rate_read_count: 0,
            hvac_very_small_mass_flow_read_count: 0,
            supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count: 0,
            zero_flow_reset_body_entry_count: 0,
            active_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_zero_flow_reset_body_entry_count: 0,
            witnessed_active_guard_false_fallthrough_count: 0,
        }
    }
}
