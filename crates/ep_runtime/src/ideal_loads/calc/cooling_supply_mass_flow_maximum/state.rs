//! Persistent CP322 cooling supply-mass-flow maximum state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute {
    UnitOff,
    NonCooling,
    CoolingMaximumAssigned,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub outdoor_air_mass_flow_rate_read_count: usize,
    pub supply_mass_flow_rate_for_cool_read_count: usize,
    pub supply_mass_flow_rate_for_dehumidification_read_count: usize,
    pub supply_mass_flow_rate_for_humidification_read_count: usize,
    pub positive_zero_vs_outdoor_air_comparison_count: usize,
    pub cooling_vs_dehumidification_comparison_count: usize,
    pub leading_vs_candidate_pair_comparison_count: usize,
    pub leading_vs_humidification_comparison_count: usize,
    pub maximum_evaluation_count: usize,
    pub supply_mass_flow_rate_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState {
    /// Creates zeroed CP322 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            outdoor_air_mass_flow_rate_read_count: 0,
            supply_mass_flow_rate_for_cool_read_count: 0,
            supply_mass_flow_rate_for_dehumidification_read_count: 0,
            supply_mass_flow_rate_for_humidification_read_count: 0,
            positive_zero_vs_outdoor_air_comparison_count: 0,
            cooling_vs_dehumidification_comparison_count: 0,
            leading_vs_candidate_pair_comparison_count: 0,
            leading_vs_humidification_comparison_count: 0,
            maximum_evaluation_count: 0,
            supply_mass_flow_rate_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
