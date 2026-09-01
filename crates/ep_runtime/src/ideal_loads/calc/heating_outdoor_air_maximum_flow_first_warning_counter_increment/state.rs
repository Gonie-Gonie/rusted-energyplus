//! Persistent CP438 heating outdoor-air maximum-flow first-warning-counter-increment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot as Snapshot;
use super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRetainedRoute as Route;

/// Persistent bounded state and exact CP437/CP438 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub outdoor_air_flow_maximum_heating_output_error_count_increment_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 36],
    pub predecessor_guard_body_entry_route_counts: [usize; 36],
    pub predecessor_volume_flow_assignment_route_counts: [usize; 36],
    pub predecessor_first_warning_guard_false_fallthrough_route_counts: [usize; 36],
    pub predecessor_first_warning_branch_entry_route_counts: [usize; 36],
    pub heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp437_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp437_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp437_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count: usize,
    pub outdoor_air_flow_maximum_heating_output_error_count_increment_write_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementRuntimeState {
    /// Creates zeroed CP438 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            outdoor_air_flow_maximum_heating_output_error_count_increment_count: 0,
            predecessor_route_counts: [0; 36],
            predecessor_guard_false_fallthrough_route_counts: [0; 36],
            predecessor_guard_body_entry_route_counts: [0; 36],
            predecessor_volume_flow_assignment_route_counts: [0; 36],
            predecessor_first_warning_guard_false_fallthrough_route_counts: [0; 36],
            predecessor_first_warning_branch_entry_route_counts: [0; 36],
            heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp437_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp437_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp437_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count: 0,
            outdoor_air_flow_maximum_heating_output_error_count_increment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
