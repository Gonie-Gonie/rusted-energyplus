//! Persistent CP432 heating operating-mode Heat-assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot as Snapshot;
use super::transition::PurchasedAirCalcHeatingOperatingModeHeatAssignmentRetainedRoute as Route;

/// Persistent bounded state and exact CP431/CP432 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_heating_mode_guard_evaluation_count: usize,
    pub predecessor_heating_mode_guard_false_fallthrough_count: usize,
    pub heating_operating_mode_heat_assignment_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub predecessor_heating_mode_guard_evaluation_route_counts: [usize; 36],
    pub predecessor_heating_mode_guard_false_fallthrough_route_counts: [usize; 36],
    pub heating_operating_mode_heat_assignment_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp431_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp431_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp431_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp432_heating_operating_mode_state_owner_count: usize,
    pub heating_operating_mode_assignment_write_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcHeatingOperatingModeHeatAssignmentRuntimeState {
    /// Creates zeroed CP432 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_heating_mode_guard_evaluation_count: 0,
            predecessor_heating_mode_guard_false_fallthrough_count: 0,
            heating_operating_mode_heat_assignment_count: 0,
            predecessor_route_counts: [0; 36],
            predecessor_heating_mode_guard_evaluation_route_counts: [0; 36],
            predecessor_heating_mode_guard_false_fallthrough_route_counts: [0; 36],
            heating_operating_mode_heat_assignment_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp431_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp431_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp431_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp432_heating_operating_mode_state_owner_count: 0,
            heating_operating_mode_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
