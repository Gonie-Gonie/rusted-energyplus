//! Persistent CP377 saturation-assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthrough,
    DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
    DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
    DehumidificationControlGuardFalseFallthrough,
}

/// Persistent bounded state and exact source/owner counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState {
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
    pub purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count: usize,
    pub environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count: usize,
    pub psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count: usize,
    pub local_saturation_supply_humidity_ratio_assignment_count: usize,
    pub cp334_supply_temperature_mixed_air_limit_owner_count: usize,
    pub cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count: usize,
    pub environment_outdoor_barometric_pressure_owner_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState {
    /// Creates zeroed CP377 state for one system.
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
            purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count: 0,
            environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count: 0,
            psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count: 0,
            local_saturation_supply_humidity_ratio_assignment_count: 0,
            cp334_supply_temperature_mixed_air_limit_owner_count: 0,
            cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count: 0,
            environment_outdoor_barometric_pressure_owner_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
