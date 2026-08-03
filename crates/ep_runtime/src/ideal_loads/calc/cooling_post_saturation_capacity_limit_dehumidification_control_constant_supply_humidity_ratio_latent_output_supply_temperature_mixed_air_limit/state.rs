//! Persistent CP408 shared-case supply-temperature mixed-air-limit state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot;
use super::transition::RetainedRoute;

/// Persistent bounded state and exact CP407/CP408 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_guard_false_fallthrough_count: usize,
    pub predecessor_maximum_capacity_assignment_count: usize,
    pub predecessor_else_branch_entry_count: usize,
    pub predecessor_supply_temperature_assignment_count: usize,
    pub dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count:
        usize,
    pub predecessor_route_counts: [usize; 30],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 30],
    pub predecessor_maximum_capacity_assignment_route_counts: [usize; 30],
    pub predecessor_else_branch_entry_route_counts: [usize; 30],
    pub predecessor_supply_temperature_assignment_route_counts: [usize; 30],
    pub supply_temperature_mixed_air_limit_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cp407_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp407_retained_supply_temperature_owned_read_count: usize,
    pub supply_temperature_for_minimum_read_count: usize,
    pub cp329_retained_mixed_air_temperature_owned_read_count: usize,
    pub mixed_air_temperature_for_minimum_read_count: usize,
    pub source_shaped_two_argument_minimum_evaluation_count: usize,
    pub supply_temperature_assignment_write_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState {
    /// Creates zeroed CP408 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_guard_false_fallthrough_count: 0,
            predecessor_maximum_capacity_assignment_count: 0,
            predecessor_else_branch_entry_count: 0,
            predecessor_supply_temperature_assignment_count: 0,
            dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count: 0,
            predecessor_route_counts: [0; 30],
            predecessor_guard_false_fallthrough_route_counts: [0; 30],
            predecessor_maximum_capacity_assignment_route_counts: [0; 30],
            predecessor_else_branch_entry_route_counts: [0; 30],
            predecessor_supply_temperature_assignment_route_counts: [0; 30],
            supply_temperature_mixed_air_limit_route_counts: [0; 30],
            source_site_execution_count: 0,
            cp407_supply_temperature_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp407_retained_supply_temperature_owned_read_count: 0,
            supply_temperature_for_minimum_read_count: 0,
            cp329_retained_mixed_air_temperature_owned_read_count: 0,
            mixed_air_temperature_for_minimum_read_count: 0,
            source_shaped_two_argument_minimum_evaluation_count: 0,
            supply_temperature_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
