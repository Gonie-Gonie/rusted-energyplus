//! Persistent CP427 zero-flow supply-temperature assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot as Snapshot;
use super::transition::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentRetainedRoute as Route;

/// Persistent bounded state and exact CP426/CP427 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub zero_supply_mass_flow_supply_temperature_mixed_air_assignment_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp426_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp426_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp426_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp427_supply_temperature_state_owner_count: usize,
    pub cp329_retained_mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_owned_read_count: usize,
    pub mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_read_count: usize,
    pub supply_temperature_assignment_write_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentRuntimeState {
    /// Creates zeroed CP427 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            zero_supply_mass_flow_supply_temperature_mixed_air_assignment_count: 0,
            predecessor_route_counts: [0; 36],
            zero_supply_mass_flow_supply_temperature_mixed_air_assignment_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp426_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp426_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp426_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp427_supply_temperature_state_owner_count: 0,
            cp329_retained_mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_owned_read_count: 0,
            mixed_air_temperature_for_zero_supply_mass_flow_supply_temperature_assignment_read_count: 0,
            supply_temperature_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
