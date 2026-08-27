//! Persistent CP428 zero-flow sensible-output positive-zero assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot as Snapshot;
use super::transition::PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRetainedRoute as Route;

/// Persistent bounded state and exact CP427/CP428 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub zero_supply_mass_flow_sensible_output_positive_zero_assignment_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp427_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp427_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp427_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp428_cooling_sensible_output_state_owner_count: usize,
    pub cooling_sensible_output_assignment_write_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentRuntimeState {
    /// Creates zeroed CP428 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            zero_supply_mass_flow_sensible_output_positive_zero_assignment_count: 0,
            predecessor_route_counts: [0; 36],
            zero_supply_mass_flow_sensible_output_positive_zero_assignment_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp427_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp427_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp427_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp428_cooling_sensible_output_state_owner_count: 0,
            cooling_sensible_output_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
