//! Persistent CP321 cooling-capacity-zero reset state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute {
    UnitOff,
    NonCooling,
    CoolingLimitRejected,
    MaximumCoolingCapacityNonZero,
    CandidatesZeroed,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub first_cooling_limit_read_count: usize,
    pub cooling_limit_capacity_count: usize,
    pub second_cooling_limit_read_count: usize,
    pub cooling_limit_flow_rate_and_capacity_count: usize,
    pub cooling_limit_rejected_count: usize,
    pub maximum_total_cooling_capacity_read_count: usize,
    pub maximum_total_cooling_capacity_comparison_count: usize,
    pub maximum_total_cooling_capacity_zero_count: usize,
    pub maximum_total_cooling_capacity_nonzero_count: usize,
    pub zero_cooling_capacity_body_entry_count: usize,
    pub supply_mass_flow_rate_for_cool_zero_assignment_count: usize,
    pub supply_mass_flow_rate_for_dehumidification_zero_assignment_count: usize,
    pub supply_mass_flow_rate_for_humidification_zero_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState {
    /// Creates zeroed CP321 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            first_cooling_limit_read_count: 0,
            cooling_limit_capacity_count: 0,
            second_cooling_limit_read_count: 0,
            cooling_limit_flow_rate_and_capacity_count: 0,
            cooling_limit_rejected_count: 0,
            maximum_total_cooling_capacity_read_count: 0,
            maximum_total_cooling_capacity_comparison_count: 0,
            maximum_total_cooling_capacity_zero_count: 0,
            maximum_total_cooling_capacity_nonzero_count: 0,
            zero_cooling_capacity_body_entry_count: 0,
            supply_mass_flow_rate_for_cool_zero_assignment_count: 0,
            supply_mass_flow_rate_for_dehumidification_zero_assignment_count: 0,
            supply_mass_flow_rate_for_humidification_zero_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
