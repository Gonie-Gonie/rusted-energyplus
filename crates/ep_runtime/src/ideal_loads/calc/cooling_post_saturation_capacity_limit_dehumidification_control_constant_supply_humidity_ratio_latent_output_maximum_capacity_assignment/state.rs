//! Persistent CP405 shared-case latent-output maximum-capacity assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot;
use super::transition::RetainedRoute;

/// Persistent bounded state and exact CP404/CP405 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_guard_false_fallthrough_count: usize,
    pub cooling_latent_output_maximum_capacity_assignment_count: usize,
    pub predecessor_route_counts: [usize; 30],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 30],
    pub cooling_latent_output_maximum_capacity_assignment_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cp404_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp404_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp404_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp404_retained_maximum_total_cooling_capacity_owned_read_count: usize,
    pub maximum_total_cooling_capacity_read_count: usize,
    pub cooling_latent_output_assignment_write_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentRuntimeState {
    /// Creates zeroed CP405 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_guard_false_fallthrough_count: 0,
            cooling_latent_output_maximum_capacity_assignment_count: 0,
            predecessor_route_counts: [0; 30],
            predecessor_guard_false_fallthrough_route_counts: [0; 30],
            cooling_latent_output_maximum_capacity_assignment_route_counts: [0; 30],
            source_site_execution_count: 0,
            cp404_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp404_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp404_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp404_retained_maximum_total_cooling_capacity_owned_read_count: 0,
            maximum_total_cooling_capacity_read_count: 0,
            cooling_latent_output_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
