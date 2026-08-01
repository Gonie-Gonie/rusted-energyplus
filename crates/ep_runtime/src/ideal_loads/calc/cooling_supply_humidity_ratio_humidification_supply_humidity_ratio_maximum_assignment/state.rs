//! Persistent CP375 humidification supply-humidity-ratio maximum-assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRetainedRoute {
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthrough,
    DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
    DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
    DehumidificationControlGuardFalseFallthrough,
}

/// Persistent bounded state and exact source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub heating_availability_guard_false_fallthrough_count: usize,
    pub humidification_control_guard_false_fallthrough_count: usize,
    pub dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count: usize,
    pub dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count: usize,
    pub dehumidification_control_guard_false_fallthrough_count: usize,
    pub source_site_execution_count: usize,
    pub purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count: usize,
    pub supply_humidity_ratio_for_humidification_for_supply_maximum_read_count: usize,
    pub source_shaped_two_argument_maximum_evaluation_count: usize,
    pub purchased_air_supply_humidity_ratio_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState {
    /// Creates zeroed CP375 state for one system.
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
            dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count: 0,
            dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count: 0,
            dehumidification_control_guard_false_fallthrough_count: 0,
            source_site_execution_count: 0,
            purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count: 0,
            supply_humidity_ratio_for_humidification_for_supply_maximum_read_count: 0,
            source_shaped_two_argument_maximum_evaluation_count: 0,
            purchased_air_supply_humidity_ratio_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
