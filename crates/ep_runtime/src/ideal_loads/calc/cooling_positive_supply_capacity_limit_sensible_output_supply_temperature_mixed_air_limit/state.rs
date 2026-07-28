//! Persistent CP344 Cooling capacity-limit supply-temperature mixed-air-limit state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    ActiveCapacityLimitGuardFalseFallthrough,
    CapacityLimitSensibleOutputGuardFalseFallthrough,
    CapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitExecuted,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_sensible_output_guard_false_fallthrough_count: usize,
    pub capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count: usize,
    pub source_site_execution_count: usize,
    pub supply_temperature_for_minimum_read_count: usize,
    pub mixed_air_temperature_for_minimum_read_count: usize,
    pub source_shaped_two_argument_minimum_evaluation_count: usize,
    pub supply_temperature_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    >,
    pub(super) latest_route: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedRoute,
    >,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count: usize,
    pub(super) witnessed_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count:
        usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState {
    /// Creates zeroed CP344 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            capacity_limit_guard_false_fallthrough_skip_count: 0,
            capacity_limit_sensible_output_guard_false_fallthrough_count: 0,
            capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count: 0,
            source_site_execution_count: 0,
            supply_temperature_for_minimum_read_count: 0,
            mixed_air_temperature_for_minimum_read_count: 0,
            source_shaped_two_argument_minimum_evaluation_count: 0,
            supply_temperature_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count: 0,
            witnessed_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count: 0,
        }
    }
}
