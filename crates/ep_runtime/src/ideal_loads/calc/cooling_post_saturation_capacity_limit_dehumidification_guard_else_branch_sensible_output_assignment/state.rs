//! Persistent CP420 not-dehumidifying sensible-output assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot;
use super::transition::RetainedRoute;

/// Persistent bounded state and exact CP419/CP420 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_supply_temperature_saturation_assignment_count: usize,
    pub predecessor_supply_temperature_saturation_mixed_air_limit_count: usize,
    pub predecessor_supply_humidity_ratio_assignment_count: usize,
    pub predecessor_supply_enthalpy_assignment_count: usize,
    pub predecessor_dehumidification_guard_else_branch_entry_count: usize,
    pub predecessor_dehumidification_guard_else_branch_cp_air_assignment_count: usize,
    pub dehumidification_guard_else_branch_sensible_output_assignment_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 36],
    pub predecessor_guard_body_entry_route_counts: [usize; 36],
    pub predecessor_supply_temperature_saturation_assignment_route_counts: [usize; 36],
    pub predecessor_supply_temperature_mixed_air_limit_route_counts: [usize; 36],
    pub predecessor_supply_humidity_ratio_assignment_route_counts: [usize; 36],
    pub predecessor_supply_enthalpy_assignment_route_counts: [usize; 36],
    pub predecessor_dehumidification_guard_else_branch_entry_route_counts: [usize; 36],
    pub predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts: [usize; 36],
    pub dehumidification_guard_else_branch_sensible_output_assignment_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp419_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp419_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp419_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub supply_mass_flow_rate_owned_read_count: usize,
    pub supply_mass_flow_rate_bit_corroboration_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub cp_air_owned_read_count: usize,
    pub cp_air_read_count: usize,
    pub supply_mass_flow_rate_times_cp_air_calculation_count: usize,
    pub mixed_air_temperature_owned_read_count: usize,
    pub mixed_air_temperature_read_count: usize,
    pub supply_temperature_owned_read_count: usize,
    pub supply_temperature_read_count: usize,
    pub mixed_air_minus_supply_temperature_calculation_count: usize,
    pub cooling_sensible_output_calculation_count: usize,
    pub cooling_sensible_output_assignment_write_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState {
    /// Creates zeroed CP420 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_supply_temperature_saturation_assignment_count: 0,
            predecessor_supply_temperature_saturation_mixed_air_limit_count: 0,
            predecessor_supply_humidity_ratio_assignment_count: 0,
            predecessor_supply_enthalpy_assignment_count: 0,
            predecessor_dehumidification_guard_else_branch_entry_count: 0,
            predecessor_dehumidification_guard_else_branch_cp_air_assignment_count: 0,
            dehumidification_guard_else_branch_sensible_output_assignment_count: 0,
            predecessor_route_counts: [0; 36],
            predecessor_guard_false_fallthrough_route_counts: [0; 36],
            predecessor_guard_body_entry_route_counts: [0; 36],
            predecessor_supply_temperature_saturation_assignment_route_counts: [0; 36],
            predecessor_supply_temperature_mixed_air_limit_route_counts: [0; 36],
            predecessor_supply_humidity_ratio_assignment_route_counts: [0; 36],
            predecessor_supply_enthalpy_assignment_route_counts: [0; 36],
            predecessor_dehumidification_guard_else_branch_entry_route_counts: [0; 36],
            predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts: [0; 36],
            dehumidification_guard_else_branch_sensible_output_assignment_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp419_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp419_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp419_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            supply_mass_flow_rate_owned_read_count: 0,
            supply_mass_flow_rate_bit_corroboration_count: 0,
            supply_mass_flow_rate_read_count: 0,
            cp_air_owned_read_count: 0,
            cp_air_read_count: 0,
            supply_mass_flow_rate_times_cp_air_calculation_count: 0,
            mixed_air_temperature_owned_read_count: 0,
            mixed_air_temperature_read_count: 0,
            supply_temperature_owned_read_count: 0,
            supply_temperature_read_count: 0,
            mixed_air_minus_supply_temperature_calculation_count: 0,
            cooling_sensible_output_calculation_count: 0,
            cooling_sensible_output_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
