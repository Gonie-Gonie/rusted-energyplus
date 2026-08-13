//! Persistent CP423 sensible-output supply-temperature assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Snapshot;
use super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRetainedRoute as Route;

/// Persistent bounded state and exact CP422/CP423 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_guard_false_fallthrough_count: usize,
    pub cooling_sensible_output_supply_temperature_assignment_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 36],
    pub cooling_sensible_output_supply_temperature_assignment_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp422_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp422_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp422_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp423_sensible_output_supply_temperature_state_owner_count: usize,
    pub cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read_count: usize,
    pub mixed_air_temperature_for_sensible_output_supply_temperature_read_count: usize,
    pub cp422_retained_cooling_sensible_output_owned_read_count: usize,
    pub cooling_sensible_output_for_supply_temperature_read_count: usize,
    pub cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read_count: usize,
    pub cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroboration_count: usize,
    pub supply_mass_flow_rate_for_sensible_output_supply_temperature_read_count: usize,
    pub cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read_count: usize,
    pub cp_air_for_sensible_output_supply_temperature_read_count: usize,
    pub supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculation_count: usize,
    pub cooling_sensible_output_over_air_capacity_rate_calculation_count: usize,
    pub sensible_output_supply_temperature_calculation_count: usize,
    pub sensible_output_supply_temperature_assignment_write_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState {
    /// Creates zeroed CP423 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_guard_false_fallthrough_count: 0,
            cooling_sensible_output_supply_temperature_assignment_count: 0,
            predecessor_route_counts: [0; 36],
            predecessor_guard_false_fallthrough_route_counts: [0; 36],
            cooling_sensible_output_supply_temperature_assignment_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp422_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp422_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp422_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp423_sensible_output_supply_temperature_state_owner_count: 0,
            cp329_retained_mixed_air_temperature_for_sensible_output_supply_temperature_owned_read_count: 0,
            mixed_air_temperature_for_sensible_output_supply_temperature_read_count: 0,
            cp422_retained_cooling_sensible_output_owned_read_count: 0,
            cooling_sensible_output_for_supply_temperature_read_count: 0,
            cp330_retained_supply_mass_flow_rate_for_sensible_output_supply_temperature_owned_read_count: 0,
            cp329_supply_mass_flow_rate_for_sensible_output_supply_temperature_bit_corroboration_count: 0,
            supply_mass_flow_rate_for_sensible_output_supply_temperature_read_count: 0,
            cp419_retained_cp_air_for_sensible_output_supply_temperature_owned_read_count: 0,
            cp_air_for_sensible_output_supply_temperature_read_count: 0,
            supply_mass_flow_rate_times_cp_air_for_sensible_output_supply_temperature_calculation_count: 0,
            cooling_sensible_output_over_air_capacity_rate_calculation_count: 0,
            sensible_output_supply_temperature_calculation_count: 0,
            sensible_output_supply_temperature_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
