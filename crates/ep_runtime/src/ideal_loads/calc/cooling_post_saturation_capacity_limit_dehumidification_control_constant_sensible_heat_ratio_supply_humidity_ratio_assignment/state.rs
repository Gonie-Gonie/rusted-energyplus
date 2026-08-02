//! Persistent CP392 supply-humidity-ratio assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot;
use super::transition::routes::RetainedRoute;

/// Persistent bounded state and exact CP391/CP392 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count:
        usize,
    pub predecessor_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cp391_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp391_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub supply_temperature_owned_read_count: usize,
    pub supply_temperature_for_humidity_ratio_inversion_read_count: usize,
    pub supply_enthalpy_owned_read_count: usize,
    pub supply_enthalpy_for_humidity_ratio_inversion_read_count: usize,
    pub psychrometric_supply_humidity_ratio_evaluation_count: usize,
    pub supply_humidity_ratio_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
    >,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentRuntimeState {
    /// Creates zeroed CP392 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count: 0,
            predecessor_route_counts: [0; 30],
            source_site_execution_count: 0,
            cp391_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp391_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            supply_temperature_owned_read_count: 0,
            supply_temperature_for_humidity_ratio_inversion_read_count: 0,
            supply_enthalpy_owned_read_count: 0,
            supply_enthalpy_for_humidity_ratio_inversion_read_count: 0,
            psychrometric_supply_humidity_ratio_evaluation_count: 0,
            supply_humidity_ratio_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
