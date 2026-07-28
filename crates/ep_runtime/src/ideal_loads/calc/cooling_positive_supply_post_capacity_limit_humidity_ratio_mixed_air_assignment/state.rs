//! Persistent CP345 post-capacity-limit humidity-ratio assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    SupplyHumidityRatioMixedAirAssigned,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count: usize,
    pub assignment_after_capacity_limit_guard_false_fallthrough_count: usize,
    pub assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count: usize,
    pub assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count:
        usize,
    pub source_site_execution_count: usize,
    pub mixed_air_humidity_ratio_read_count: usize,
    pub supply_humidity_ratio_assignment_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    >,
    pub(super) latest_route: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRetainedRoute,
    >,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count:
        usize,
    pub(super) witnessed_assignment_after_capacity_limit_guard_false_fallthrough_count: usize,
    pub(super) witnessed_assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count:
        usize,
    pub(super) witnessed_assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count:
        usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState {
    /// Creates zeroed CP345 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count: 0,
            assignment_after_capacity_limit_guard_false_fallthrough_count: 0,
            assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count: 0,
            assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count:
                0,
            source_site_execution_count: 0,
            mixed_air_humidity_ratio_read_count: 0,
            supply_humidity_ratio_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count: 0,
            witnessed_assignment_after_capacity_limit_guard_false_fallthrough_count: 0,
            witnessed_assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count:
                0,
            witnessed_assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count:
                0,
        }
    }
}
