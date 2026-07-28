//! Persistent CP338 Cooling positive-supply capacity-limit `CpAir` assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    ActiveCapacityLimitGuardFalseFallthrough,
    CapacityLimitCpAirAssigned,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_cp_air_assignment_count: usize,
    pub source_site_execution_count: usize,
    pub mixed_air_humidity_ratio_read_count: usize,
    pub psychrometric_cp_air_evaluation_count: usize,
    pub cp_air_assignment_write_count: usize,
    pub latest:
        Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_cp_air_assignment_count: usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState {
    /// Creates zeroed CP338 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            capacity_limit_guard_false_fallthrough_skip_count: 0,
            capacity_limit_cp_air_assignment_count: 0,
            source_site_execution_count: 0,
            mixed_air_humidity_ratio_read_count: 0,
            psychrometric_cp_air_evaluation_count: 0,
            cp_air_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_cp_air_assignment_count: 0,
        }
    }
}
