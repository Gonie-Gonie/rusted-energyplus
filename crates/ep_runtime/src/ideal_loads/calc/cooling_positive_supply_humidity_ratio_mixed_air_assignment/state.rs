//! Persistent CP335 Cooling positive-supply mixed-air humidity-ratio assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    SupplyHumidityRatioMixedAirAssigned,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub supply_humidity_ratio_mixed_air_assignment_count: usize,
    pub source_site_execution_count: usize,
    pub mixed_air_humidity_ratio_read_count: usize,
    pub supply_humidity_ratio_assignment_count: usize,
    pub latest:
        Option<PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_supply_humidity_ratio_mixed_air_assignment_count: usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState {
    /// Creates zeroed CP335 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            supply_humidity_ratio_mixed_air_assignment_count: 0,
            source_site_execution_count: 0,
            mixed_air_humidity_ratio_read_count: 0,
            supply_humidity_ratio_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_supply_humidity_ratio_mixed_air_assignment_count: 0,
        }
    }
}
