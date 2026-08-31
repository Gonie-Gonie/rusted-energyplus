//! Persistent CP435 heating outdoor-air maximum-flow-guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot as Snapshot;
use super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRetainedRoute as Route;

/// Persistent bounded state and exact CP434/CP435 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub heating_outdoor_air_maximum_flow_guard_evaluation_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts: [usize; 36],
    pub maximum_heating_flow_body_entry_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub heating_limit_flow_rate_comparison_count: usize,
    pub heating_limit_flow_rate_match_count: usize,
    pub heating_limit_flow_rate_and_capacity_comparison_count: usize,
    pub heating_limit_flow_rate_and_capacity_match_count: usize,
    pub heating_flow_limit_selector_rejection_count: usize,
    pub cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count: usize,
    pub outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count: usize,
    pub maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count: usize,
    pub outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count: usize,
    pub outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count:
        usize,
    pub maximum_heating_flow_body_entry_count: usize,
    pub heating_outdoor_air_maximum_flow_guard_false_fallthrough_count: usize,
    pub cp434_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp434_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp434_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardRuntimeState {
    /// Creates zeroed CP435 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            heating_outdoor_air_maximum_flow_guard_evaluation_count: 0,
            predecessor_route_counts: [0; 36],
            heating_outdoor_air_maximum_flow_guard_false_fallthrough_route_counts: [0; 36],
            maximum_heating_flow_body_entry_route_counts: [0; 36],
            source_site_execution_count: 0,
            heating_limit_flow_rate_comparison_count: 0,
            heating_limit_flow_rate_match_count: 0,
            heating_limit_flow_rate_and_capacity_comparison_count: 0,
            heating_limit_flow_rate_and_capacity_match_count: 0,
            heating_flow_limit_selector_rejection_count: 0,
            cp311_same_call_outdoor_air_mass_flow_rate_bit_corroboration_count: 0,
            outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit_count: 0,
            maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit_count: 0,
            outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_count: 0,
            outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate_count: 0,
            maximum_heating_flow_body_entry_count: 0,
            heating_outdoor_air_maximum_flow_guard_false_fallthrough_count: 0,
            cp434_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp434_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp434_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
