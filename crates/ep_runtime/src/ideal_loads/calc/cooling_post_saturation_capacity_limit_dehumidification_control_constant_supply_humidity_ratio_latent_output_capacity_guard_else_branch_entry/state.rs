//! Persistent CP406 shared-case latent-output capacity-guard else-entry state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot;
use super::transition::RetainedRoute;

/// Persistent bounded state and exact CP405/CP406 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_guard_false_fallthrough_count: usize,
    pub predecessor_maximum_capacity_assignment_count: usize,
    pub dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count:
        usize,
    pub predecessor_route_counts: [usize; 30],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 30],
    pub predecessor_maximum_capacity_assignment_route_counts: [usize; 30],
    pub else_branch_entry_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState {
    /// Creates zeroed CP406 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_guard_false_fallthrough_count: 0,
            predecessor_maximum_capacity_assignment_count: 0,
            dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count: 0,
            predecessor_route_counts: [0; 30],
            predecessor_guard_false_fallthrough_route_counts: [0; 30],
            predecessor_maximum_capacity_assignment_route_counts: [0; 30],
            else_branch_entry_route_counts: [0; 30],
            source_site_execution_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
