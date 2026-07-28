//! Persistent CP325 cooling supply-mass-flow limit-guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute {
    UnitOff,
    NonCooling,
    CoolingLimitRejected,
    MaximumCoolingMassFlowNotPositive,
    FlowLimitBodyEntered,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub first_cooling_limit_read_count: usize,
    pub cooling_limit_flow_rate_comparison_count: usize,
    pub cooling_limit_flow_rate_match_count: usize,
    pub second_cooling_limit_read_count: usize,
    pub cooling_limit_flow_rate_and_capacity_comparison_count: usize,
    pub cooling_limit_flow_rate_and_capacity_match_count: usize,
    pub cooling_limit_rejected_count: usize,
    pub maximum_cooling_air_mass_flow_rate_read_count: usize,
    pub maximum_cooling_air_mass_flow_rate_positive_comparison_count: usize,
    pub maximum_cooling_air_mass_flow_rate_strictly_positive_count: usize,
    pub maximum_cooling_air_mass_flow_rate_not_positive_count: usize,
    pub supply_mass_flow_limit_body_entry_count: usize,
    pub active_guard_false_fallthrough_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState {
    /// Creates zeroed CP325 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            first_cooling_limit_read_count: 0,
            cooling_limit_flow_rate_comparison_count: 0,
            cooling_limit_flow_rate_match_count: 0,
            second_cooling_limit_read_count: 0,
            cooling_limit_flow_rate_and_capacity_comparison_count: 0,
            cooling_limit_flow_rate_and_capacity_match_count: 0,
            cooling_limit_rejected_count: 0,
            maximum_cooling_air_mass_flow_rate_read_count: 0,
            maximum_cooling_air_mass_flow_rate_positive_comparison_count: 0,
            maximum_cooling_air_mass_flow_rate_strictly_positive_count: 0,
            maximum_cooling_air_mass_flow_rate_not_positive_count: 0,
            supply_mass_flow_limit_body_entry_count: 0,
            active_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
