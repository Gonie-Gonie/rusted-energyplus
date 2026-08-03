//! Persistent CP412 saturation-humidity-ratio assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::transition::routes::RetainedRoute;
use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot;

/// Persistent bounded state and exact CP411/CP412 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_guard_false_fallthrough_count: usize,
    pub predecessor_maximum_capacity_assignment_count: usize,
    pub predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count: usize,
    pub supply_humidity_ratio_saturation_assignment_count: usize,
    pub predecessor_route_counts: [usize; 30],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 30],
    pub predecessor_maximum_capacity_assignment_route_counts: [usize; 30],
    pub predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts:
        [usize; 30],
    pub supply_humidity_ratio_saturation_assignment_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cp411_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp411_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp411_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp411_retained_supply_temperature_owned_read_count: usize,
    pub purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count: usize,
    pub environment_outdoor_barometric_pressure_owner_count: usize,
    pub environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count: usize,
    pub psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count: usize,
    pub local_saturation_supply_humidity_ratio_assignment_write_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState {
    /// Creates zeroed CP412 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_guard_false_fallthrough_count: 0,
            predecessor_maximum_capacity_assignment_count: 0,
            predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count: 0,
            supply_humidity_ratio_saturation_assignment_count: 0,
            predecessor_route_counts: [0; 30],
            predecessor_guard_false_fallthrough_route_counts: [0; 30],
            predecessor_maximum_capacity_assignment_route_counts: [0; 30],
            predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts:
                [0; 30],
            supply_humidity_ratio_saturation_assignment_route_counts: [0; 30],
            source_site_execution_count: 0,
            cp411_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp411_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp411_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp411_retained_supply_temperature_owned_read_count: 0,
            purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count: 0,
            environment_outdoor_barometric_pressure_owner_count: 0,
            environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count: 0,
            psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count: 0,
            local_saturation_supply_humidity_ratio_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
