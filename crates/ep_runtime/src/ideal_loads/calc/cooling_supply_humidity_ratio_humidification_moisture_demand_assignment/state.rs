//! Persistent CP372 humidifying-setpoint moisture-demand assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRetainedRoute {
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthrough,
    DehumidificationControlHumidistatMoistureDemandAssignmentExecuted,
    DehumidificationControlNoneMoistureDemandAssignmentExecuted,
    DehumidificationControlGuardFalseFallthrough,
}

/// Persistent bounded state and exact source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub heating_availability_guard_false_fallthrough_count: usize,
    pub humidification_control_guard_false_fallthrough_count: usize,
    pub dehumidification_control_humidistat_moisture_demand_assignment_count: usize,
    pub dehumidification_control_none_moisture_demand_assignment_count: usize,
    pub dehumidification_control_guard_false_fallthrough_count: usize,
    pub humidification_moisture_demand_assignment_count: usize,
    pub source_site_execution_count: usize,
    pub zone_humidifying_setpoint_moisture_demand_read_count: usize,
    pub zone_humidifying_setpoint_moisture_demand_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState {
    /// Creates zeroed CP372 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            heating_availability_guard_false_fallthrough_count: 0,
            humidification_control_guard_false_fallthrough_count: 0,
            dehumidification_control_humidistat_moisture_demand_assignment_count: 0,
            dehumidification_control_none_moisture_demand_assignment_count: 0,
            dehumidification_control_guard_false_fallthrough_count: 0,
            humidification_moisture_demand_assignment_count: 0,
            source_site_execution_count: 0,
            zone_humidifying_setpoint_moisture_demand_read_count: 0,
            zone_humidifying_setpoint_moisture_demand_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
