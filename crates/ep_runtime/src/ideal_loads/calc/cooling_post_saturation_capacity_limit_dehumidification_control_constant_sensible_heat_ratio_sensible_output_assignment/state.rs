//! Persistent CP388 sensible-output assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot;
use super::transition::routes::RetainedRoute;

/// Persistent bounded state and exact CP387/CP388 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count:
        usize,
    pub predecessor_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cooling_total_output_owned_read_count: usize,
    pub cooling_total_output_bit_corroboration_count: usize,
    pub cooling_sensible_heat_ratio_read_count: usize,
    pub cooling_sensible_output_calculation_count: usize,
    pub cooling_sensible_output_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
    >,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState {
    /// Creates zeroed CP388 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count: 0,
            predecessor_route_counts: [0; 30],
            source_site_execution_count: 0,
            cooling_total_output_owned_read_count: 0,
            cooling_total_output_bit_corroboration_count: 0,
            cooling_sensible_heat_ratio_read_count: 0,
            cooling_sensible_output_calculation_count: 0,
            cooling_sensible_output_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
