//! Persistent CP373 humidification supply-humidity-ratio assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRetainedRoute {
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthrough,
    DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted,
    DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted,
    DehumidificationControlGuardFalseFallthrough,
}

/// Persistent bounded state and exact source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub heating_availability_guard_false_fallthrough_count: usize,
    pub humidification_control_guard_false_fallthrough_count: usize,
    pub dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count: usize,
    pub dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count: usize,
    pub dehumidification_control_guard_false_fallthrough_count: usize,
    pub source_site_execution_count: usize,
    pub zone_humidifying_setpoint_moisture_demand_read_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub moisture_demand_derived_supply_humidity_ratio_calculation_count: usize,
    pub zone_node_humidity_ratio_read_count: usize,
    pub supply_humidity_ratio_for_humidification_calculation_count: usize,
    pub supply_humidity_ratio_for_humidification_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState {
    /// Creates zeroed CP373 state for one system.
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
            dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count: 0,
            dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count: 0,
            dehumidification_control_guard_false_fallthrough_count: 0,
            source_site_execution_count: 0,
            zone_humidifying_setpoint_moisture_demand_read_count: 0,
            supply_mass_flow_rate_read_count: 0,
            moisture_demand_derived_supply_humidity_ratio_calculation_count: 0,
            zone_node_humidity_ratio_read_count: 0,
            supply_humidity_ratio_for_humidification_calculation_count: 0,
            supply_humidity_ratio_for_humidification_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
