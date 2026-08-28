//! Persistent CP431 heating-mode-guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcHeatingModeGuardSnapshot as Snapshot;
use super::transition::PurchasedAirCalcHeatingModeGuardRetainedRoute as Route;

/// Persistent bounded state and exact CP430/CP431 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcHeatingModeGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub heating_mode_guard_evaluation_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub heating_mode_guard_evaluation_route_counts: [usize; 36],
    pub heating_operating_mode_body_entry_route_counts: [usize; 36],
    pub heating_mode_guard_false_fallthrough_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp430_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp430_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp430_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count: usize,
    pub cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count: usize,
    pub minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count: usize,
    pub cp310_retained_heating_setpoint_demand_owner_read_count: usize,
    pub heating_setpoint_demand_for_heating_mode_guard_read_count: usize,
    pub minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count: usize,
    pub minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count:
        usize,
    pub prevalidated_temperature_control_type_owner_read_count: usize,
    pub temperature_control_type_read_after_sensible_comparison_short_circuit_count: usize,
    pub temperature_control_type_single_cool_comparison_count: usize,
    pub temperature_control_type_permits_heating_count: usize,
    pub single_cool_block_count: usize,
    pub heating_operating_mode_body_entry_count: usize,
    pub heating_mode_guard_false_fallthrough_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcHeatingModeGuardRuntimeState {
    /// Creates zeroed CP431 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            heating_mode_guard_evaluation_count: 0,
            predecessor_route_counts: [0; 36],
            heating_mode_guard_evaluation_route_counts: [0; 36],
            heating_operating_mode_body_entry_route_counts: [0; 36],
            heating_mode_guard_false_fallthrough_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp430_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp430_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp430_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp311_retained_minimum_outdoor_air_sensible_output_owner_read_count: 0,
            cp312_same_call_minimum_outdoor_air_sensible_output_bit_corroboration_count: 0,
            minimum_outdoor_air_sensible_output_for_heating_mode_guard_read_count: 0,
            cp310_retained_heating_setpoint_demand_owner_read_count: 0,
            heating_setpoint_demand_for_heating_mode_guard_read_count: 0,
            minimum_outdoor_air_sensible_output_heating_setpoint_demand_comparison_count: 0,
            minimum_outdoor_air_sensible_output_strictly_less_than_heating_setpoint_demand_count: 0,
            prevalidated_temperature_control_type_owner_read_count: 0,
            temperature_control_type_read_after_sensible_comparison_short_circuit_count: 0,
            temperature_control_type_single_cool_comparison_count: 0,
            temperature_control_type_permits_heating_count: 0,
            single_cool_block_count: 0,
            heating_operating_mode_body_entry_count: 0,
            heating_mode_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
