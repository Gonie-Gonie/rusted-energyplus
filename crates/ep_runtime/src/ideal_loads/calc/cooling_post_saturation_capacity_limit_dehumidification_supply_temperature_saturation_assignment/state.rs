//! Persistent CP414 saturation-temperature-assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::transition::RetainedRoute;
use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot;

/// Persistent bounded state and exact CP413/CP414 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub saturation_supply_temperature_assignment_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 36],
    pub predecessor_guard_body_entry_route_counts: [usize; 36],
    pub supply_temperature_saturation_assignment_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp413_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp413_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp413_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp414_saturation_supply_temperature_state_owner_count: usize,
    pub cp413_retained_supply_enthalpy_owned_read_count: usize,
    pub supply_enthalpy_for_saturation_temperature_read_count: usize,
    pub environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count: usize,
    pub environment_outdoor_barometric_pressure_for_saturation_temperature_read_count: usize,
    pub psy_tsat_fn_h_pb_evaluation_count: usize,
    pub purchased_air_supply_temperature_saturation_assignment_write_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState {
    /// Creates zeroed CP414 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            saturation_supply_temperature_assignment_count: 0,
            predecessor_route_counts: [0; 36],
            predecessor_guard_false_fallthrough_route_counts: [0; 36],
            predecessor_guard_body_entry_route_counts: [0; 36],
            supply_temperature_saturation_assignment_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp413_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp413_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp413_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp414_saturation_supply_temperature_state_owner_count: 0,
            cp413_retained_supply_enthalpy_owned_read_count: 0,
            supply_enthalpy_for_saturation_temperature_read_count: 0,
            environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count: 0,
            environment_outdoor_barometric_pressure_for_saturation_temperature_read_count: 0,
            psy_tsat_fn_h_pb_evaluation_count: 0,
            purchased_air_supply_temperature_saturation_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
