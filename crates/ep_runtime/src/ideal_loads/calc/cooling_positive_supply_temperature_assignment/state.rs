//! Persistent CP332 Cooling positive-supply temperature-assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    SupplyTemperatureAssigned,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub supply_temperature_assignment_count: usize,
    pub source_site_execution_count: usize,
    pub zone_cooling_setpoint_load_read_count: usize,
    pub cp_air_read_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub cp_air_times_supply_mass_flow_rate_calculation_count: usize,
    pub zone_cooling_setpoint_load_over_denominator_calculation_count: usize,
    pub zone_node_temperature_read_count: usize,
    pub supply_temperature_calculation_count: usize,
    pub supply_temperature_assignment_write_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_supply_temperature_assignment_count: usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState {
    /// Creates zeroed CP332 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            supply_temperature_assignment_count: 0,
            source_site_execution_count: 0,
            zone_cooling_setpoint_load_read_count: 0,
            cp_air_read_count: 0,
            supply_mass_flow_rate_read_count: 0,
            cp_air_times_supply_mass_flow_rate_calculation_count: 0,
            zone_cooling_setpoint_load_over_denominator_calculation_count: 0,
            zone_node_temperature_read_count: 0,
            supply_temperature_calculation_count: 0,
            supply_temperature_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_supply_temperature_assignment_count: 0,
        }
    }
}
