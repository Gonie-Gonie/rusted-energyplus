//! Persistent CP421 sensible-output maximum-capacity guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Snapshot;
use super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRetainedRoute as Route;

/// Persistent bounded state and exact CP420/CP421 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub guard_false_fallthrough_route_counts: [usize; 36],
    pub adjustment_body_entry_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp420_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp420_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp420_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp420_cooling_sensible_output_owned_read_count: usize,
    pub cooling_sensible_output_read_count: usize,
    pub cp321_maximum_total_cooling_capacity_owned_read_count: usize,
    pub cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count: usize,
    pub maximum_total_cooling_capacity_read_count: usize,
    pub cooling_sensible_output_maximum_total_cooling_capacity_comparison_count: usize,
    pub cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count: usize,
    pub post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count: usize,
    pub post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState {
    /// Creates zeroed CP421 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluation_count: 0,
            predecessor_route_counts: [0; 36],
            guard_false_fallthrough_route_counts: [0; 36],
            adjustment_body_entry_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp420_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp420_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp420_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp420_cooling_sensible_output_owned_read_count: 0,
            cooling_sensible_output_read_count: 0,
            cp321_maximum_total_cooling_capacity_owned_read_count: 0,
            cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count: 0,
            maximum_total_cooling_capacity_read_count: 0,
            cooling_sensible_output_maximum_total_cooling_capacity_comparison_count: 0,
            cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count: 0,
            post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entry_count: 0,
            post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
