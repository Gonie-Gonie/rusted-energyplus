//! Persistent CP333 Cooling positive-supply temperature-minimum-limit state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    SupplyTemperatureMinimumLimitExecuted,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub supply_temperature_minimum_limit_count: usize,
    pub source_site_execution_count: usize,
    pub supply_temperature_for_maximum_read_count: usize,
    pub minimum_cooling_supply_air_temperature_for_maximum_read_count: usize,
    pub source_shaped_two_argument_maximum_evaluation_count: usize,
    pub supply_temperature_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_supply_temperature_minimum_limit_count: usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState {
    /// Creates zeroed CP333 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            supply_temperature_minimum_limit_count: 0,
            source_site_execution_count: 0,
            supply_temperature_for_maximum_read_count: 0,
            minimum_cooling_supply_air_temperature_for_maximum_read_count: 0,
            source_shaped_two_argument_maximum_evaluation_count: 0,
            supply_temperature_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_supply_temperature_minimum_limit_count: 0,
        }
    }
}
