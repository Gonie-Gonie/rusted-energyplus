//! Persistent CP401 shared-case latent-output assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot;
use super::transition::routes::RetainedRoute;

/// Persistent bounded state and exact CP400/CP401 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count:
        usize,
    pub predecessor_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cp400_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp400_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp400_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cooling_total_output_owned_read_count: usize,
    pub cooling_total_output_bit_corroboration_count: usize,
    pub cooling_total_output_read_count: usize,
    pub cooling_sensible_output_owned_read_count: usize,
    pub cooling_sensible_output_read_count: usize,
    pub cooling_latent_output_calculation_count: usize,
    pub cooling_latent_output_assignment_write_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState {
    /// Creates zeroed CP401 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count: 0,
            predecessor_route_counts: [0; 30],
            source_site_execution_count: 0,
            cp400_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp400_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp400_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cooling_total_output_owned_read_count: 0,
            cooling_total_output_bit_corroboration_count: 0,
            cooling_total_output_read_count: 0,
            cooling_sensible_output_owned_read_count: 0,
            cooling_sensible_output_read_count: 0,
            cooling_latent_output_calculation_count: 0,
            cooling_latent_output_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
