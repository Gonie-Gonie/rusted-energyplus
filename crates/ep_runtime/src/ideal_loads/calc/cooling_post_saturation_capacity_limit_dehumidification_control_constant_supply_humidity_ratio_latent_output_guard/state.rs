//! Persistent CP402 shared-case latent-output capacity-guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot;
use super::transition::routes::RetainedRoute;

/// Persistent bounded state and exact CP401/CP402 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count: usize,
    pub predecessor_route_counts: [usize; 30],
    pub guard_false_fallthrough_route_counts: [usize; 30],
    pub adjustment_body_entry_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cp401_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp401_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp401_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp401_cooling_latent_output_owned_read_count: usize,
    pub cooling_latent_output_read_count: usize,
    pub cp321_maximum_total_cooling_capacity_owned_read_count: usize,
    pub cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count: usize,
    pub maximum_total_cooling_capacity_read_count: usize,
    pub cooling_latent_output_maximum_total_cooling_capacity_comparison_count: usize,
    pub cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count: usize,
    pub dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count: usize,
    pub dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState {
    /// Creates zeroed CP402 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count: 0,
            predecessor_route_counts: [0; 30],
            guard_false_fallthrough_route_counts: [0; 30],
            adjustment_body_entry_route_counts: [0; 30],
            source_site_execution_count: 0,
            cp401_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp401_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp401_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp401_cooling_latent_output_owned_read_count: 0,
            cooling_latent_output_read_count: 0,
            cp321_maximum_total_cooling_capacity_owned_read_count: 0,
            cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count: 0,
            maximum_total_cooling_capacity_read_count: 0,
            cooling_latent_output_maximum_total_cooling_capacity_comparison_count: 0,
            cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count: 0,
            dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count: 0,
            dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
