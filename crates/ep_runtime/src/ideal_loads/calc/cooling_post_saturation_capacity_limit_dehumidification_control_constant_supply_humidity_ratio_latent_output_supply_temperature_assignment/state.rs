//! Persistent CP407 shared-case supply-temperature assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot;
use super::transition::RetainedRoute;

/// Persistent bounded state and exact CP406/CP407 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_guard_false_fallthrough_count: usize,
    pub predecessor_maximum_capacity_assignment_count: usize,
    pub predecessor_else_branch_entry_count: usize,
    pub dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count:
        usize,
    pub predecessor_route_counts: [usize; 30],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 30],
    pub predecessor_maximum_capacity_assignment_route_counts: [usize; 30],
    pub predecessor_else_branch_entry_route_counts: [usize; 30],
    pub supply_temperature_assignment_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cp385_retained_supply_enthalpy_owned_read_count: usize,
    pub cp406_same_call_supply_enthalpy_bit_corroboration_count: usize,
    pub supply_enthalpy_for_dry_bulb_inversion_read_count: usize,
    pub cp378_retained_supply_humidity_ratio_owned_read_count: usize,
    pub supply_humidity_ratio_for_dry_bulb_inversion_read_count: usize,
    pub psychrometric_supply_temperature_evaluation_count: usize,
    pub supply_temperature_assignment_write_count: usize,
    pub cp406_preexisting_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState {
    /// Creates zeroed CP407 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_guard_false_fallthrough_count: 0,
            predecessor_maximum_capacity_assignment_count: 0,
            predecessor_else_branch_entry_count: 0,
            dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count: 0,
            predecessor_route_counts: [0; 30],
            predecessor_guard_false_fallthrough_route_counts: [0; 30],
            predecessor_maximum_capacity_assignment_route_counts: [0; 30],
            predecessor_else_branch_entry_route_counts: [0; 30],
            supply_temperature_assignment_route_counts: [0; 30],
            source_site_execution_count: 0,
            cp385_retained_supply_enthalpy_owned_read_count: 0,
            cp406_same_call_supply_enthalpy_bit_corroboration_count: 0,
            supply_enthalpy_for_dry_bulb_inversion_read_count: 0,
            cp378_retained_supply_humidity_ratio_owned_read_count: 0,
            supply_humidity_ratio_for_dry_bulb_inversion_read_count: 0,
            psychrometric_supply_temperature_evaluation_count: 0,
            supply_temperature_assignment_write_count: 0,
            cp406_preexisting_supply_temperature_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
