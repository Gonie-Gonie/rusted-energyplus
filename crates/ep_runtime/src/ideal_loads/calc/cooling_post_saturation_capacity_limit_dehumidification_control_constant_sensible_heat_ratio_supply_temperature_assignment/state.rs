//! Persistent CP389 supply-temperature assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot;
use super::transition::routes::RetainedRoute;

/// Persistent bounded state and exact CP388/CP389 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count:
        usize,
    pub predecessor_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cp379_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub mixed_air_temperature_owned_read_count: usize,
    pub cooling_sensible_output_owned_read_count: usize,
    pub cp_air_owned_read_count: usize,
    pub supply_mass_flow_rate_owned_read_count: usize,
    pub supply_mass_flow_rate_bit_corroboration_count: usize,
    pub air_capacity_rate_calculation_count: usize,
    pub sensible_temperature_drop_calculation_count: usize,
    pub supply_temperature_calculation_count: usize,
    pub supply_temperature_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot,
    >,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState {
    /// Creates zeroed CP389 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count: 0,
            predecessor_route_counts: [0; 30],
            source_site_execution_count: 0,
            cp379_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            mixed_air_temperature_owned_read_count: 0,
            cooling_sensible_output_owned_read_count: 0,
            cp_air_owned_read_count: 0,
            supply_mass_flow_rate_owned_read_count: 0,
            supply_mass_flow_rate_bit_corroboration_count: 0,
            air_capacity_rate_calculation_count: 0,
            sensible_temperature_drop_calculation_count: 0,
            supply_temperature_calculation_count: 0,
            supply_temperature_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
