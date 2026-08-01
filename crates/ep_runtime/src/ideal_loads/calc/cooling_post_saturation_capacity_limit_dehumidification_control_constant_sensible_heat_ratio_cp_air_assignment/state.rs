//! Persistent CP387 case-entry and `CpAir` assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot;
use super::transition::routes::RetainedRoute;

/// Persistent bounded state and exact CP386/CP387 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count: usize,
    pub predecessor_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_case_entry_count: usize,
    pub mixed_air_humidity_ratio_read_count: usize,
    pub psychrometric_cp_air_evaluation_count: usize,
    pub cp_air_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    >,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState {
    /// Creates zeroed CP387 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count: 0,
            predecessor_route_counts: [0; 30],
            source_site_execution_count: 0,
            dehumidification_control_constant_sensible_heat_ratio_case_entry_count: 0,
            mixed_air_humidity_ratio_read_count: 0,
            psychrometric_cp_air_evaluation_count: 0,
            cp_air_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
