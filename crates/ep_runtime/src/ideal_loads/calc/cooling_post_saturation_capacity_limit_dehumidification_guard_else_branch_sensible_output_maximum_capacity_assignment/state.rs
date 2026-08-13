//! Persistent CP422 sensible-output maximum-capacity assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Snapshot;
use super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRetainedRoute as Route;

/// Persistent bounded state and exact CP421/CP422 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_guard_false_fallthrough_count: usize,
    pub cooling_sensible_output_maximum_capacity_assignment_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 36],
    pub cooling_sensible_output_maximum_capacity_assignment_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp421_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp421_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp421_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp421_retained_maximum_total_cooling_capacity_owned_read_count: usize,
    pub maximum_total_cooling_capacity_for_sensible_output_assignment_read_count: usize,
    pub cooling_sensible_output_maximum_capacity_assignment_write_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState {
    /// Creates zeroed CP422 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_guard_false_fallthrough_count: 0,
            cooling_sensible_output_maximum_capacity_assignment_count: 0,
            predecessor_route_counts: [0; 36],
            predecessor_guard_false_fallthrough_route_counts: [0; 36],
            cooling_sensible_output_maximum_capacity_assignment_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp421_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp421_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp421_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp421_retained_maximum_total_cooling_capacity_owned_read_count: 0,
            maximum_total_cooling_capacity_for_sensible_output_assignment_read_count: 0,
            cooling_sensible_output_maximum_capacity_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
