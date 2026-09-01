//! Persistent CP436 heating outdoor-air volume-flow-assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRetainedRoute as Route;
use super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot as Snapshot;

/// Persistent bounded state and exact CP435/CP436 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub outdoor_air_volume_flow_assignment_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 36],
    pub predecessor_guard_body_entry_route_counts: [usize; 36],
    pub heating_outdoor_air_volume_flow_assignment_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp435_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp435_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp435_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp435_outdoor_air_mass_flow_rate_owned_read_count: usize,
    pub outdoor_air_mass_flow_rate_for_volume_flow_division_read_count: usize,
    pub begin_environment_standard_air_density_owner_count: usize,
    pub standard_air_density_for_volume_flow_division_read_count: usize,
    pub outdoor_air_mass_flow_rate_standard_air_density_division_count: usize,
    pub local_outdoor_air_volume_flow_rate_assignment_write_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState {
    /// Creates zeroed CP436 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            outdoor_air_volume_flow_assignment_count: 0,
            predecessor_route_counts: [0; 36],
            predecessor_guard_false_fallthrough_route_counts: [0; 36],
            predecessor_guard_body_entry_route_counts: [0; 36],
            heating_outdoor_air_volume_flow_assignment_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp435_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp435_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp435_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp435_outdoor_air_mass_flow_rate_owned_read_count: 0,
            outdoor_air_mass_flow_rate_for_volume_flow_division_read_count: 0,
            begin_environment_standard_air_density_owner_count: 0,
            standard_air_density_for_volume_flow_division_read_count: 0,
            outdoor_air_mass_flow_rate_standard_air_density_division_count: 0,
            local_outdoor_air_volume_flow_rate_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
