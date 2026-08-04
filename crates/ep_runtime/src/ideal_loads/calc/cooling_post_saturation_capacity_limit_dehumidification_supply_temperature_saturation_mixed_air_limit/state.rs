//! Persistent CP415 saturation-temperature mixed-air-limit state.

use ep_model::IdealLoadsAirSystemId;

use super::transition::RetainedRoute;
use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot;

/// Persistent bounded state and exact CP414/CP415 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_supply_temperature_saturation_assignment_count: usize,
    pub supply_temperature_saturation_mixed_air_limit_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 36],
    pub predecessor_guard_body_entry_route_counts: [usize; 36],
    pub predecessor_supply_temperature_saturation_assignment_route_counts: [usize; 36],
    pub supply_temperature_mixed_air_limit_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp414_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp414_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp414_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp415_mixed_air_limited_supply_temperature_state_owner_count: usize,
    pub cp414_retained_supply_temperature_owned_read_count: usize,
    pub supply_temperature_for_minimum_read_count: usize,
    pub cp329_retained_mixed_air_temperature_owned_read_count: usize,
    pub mixed_air_temperature_for_minimum_read_count: usize,
    pub source_shaped_two_argument_minimum_evaluation_count: usize,
    pub supply_temperature_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot,
    >,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState {
    /// Creates zeroed CP415 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_supply_temperature_saturation_assignment_count: 0,
            supply_temperature_saturation_mixed_air_limit_count: 0,
            predecessor_route_counts: [0; 36],
            predecessor_guard_false_fallthrough_route_counts: [0; 36],
            predecessor_guard_body_entry_route_counts: [0; 36],
            predecessor_supply_temperature_saturation_assignment_route_counts: [0; 36],
            supply_temperature_mixed_air_limit_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp414_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp414_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp414_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp415_mixed_air_limited_supply_temperature_state_owner_count: 0,
            cp414_retained_supply_temperature_owned_read_count: 0,
            supply_temperature_for_minimum_read_count: 0,
            cp329_retained_mixed_air_temperature_owned_read_count: 0,
            mixed_air_temperature_for_minimum_read_count: 0,
            source_shaped_two_argument_minimum_evaluation_count: 0,
            supply_temperature_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
