//! Persistent CP330 Cooling positive supply-mass-flow guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveSupplyMassFlowBodyEntered,
    ActiveGuardFalseFallthrough,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub source_site_execution_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub supply_mass_flow_rate_strictly_positive_comparison_count: usize,
    pub positive_supply_mass_flow_body_entry_count: usize,
    pub active_guard_false_fallthrough_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_supply_mass_flow_body_entry_count: usize,
    pub(super) witnessed_active_guard_false_fallthrough_count: usize,
}

impl PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState {
    /// Creates zeroed CP330 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            source_site_execution_count: 0,
            supply_mass_flow_rate_read_count: 0,
            supply_mass_flow_rate_strictly_positive_comparison_count: 0,
            positive_supply_mass_flow_body_entry_count: 0,
            active_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_supply_mass_flow_body_entry_count: 0,
            witnessed_active_guard_false_fallthrough_count: 0,
        }
    }
}
