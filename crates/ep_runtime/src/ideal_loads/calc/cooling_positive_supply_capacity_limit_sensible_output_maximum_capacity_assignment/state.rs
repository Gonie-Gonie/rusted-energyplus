//! Persistent CP341 Cooling sensible-output maximum-capacity assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    ActiveCapacityLimitGuardFalseFallthrough,
    CapacityLimitSensibleOutputGuardFalseFallthrough,
    CapacityLimitSensibleOutputMaximumCapacityAssigned,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_sensible_output_guard_false_fallthrough_count: usize,
    pub capacity_limit_sensible_output_maximum_capacity_assignment_count: usize,
    pub source_site_execution_count: usize,
    pub maximum_total_cooling_capacity_read_count: usize,
    pub cooling_sensible_output_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    >,
    pub(super) latest_route: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRetainedRoute,
    >,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count: usize,
    pub(super) witnessed_capacity_limit_sensible_output_maximum_capacity_assignment_count: usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState {
    /// Creates zeroed CP341 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            capacity_limit_guard_false_fallthrough_skip_count: 0,
            capacity_limit_sensible_output_guard_false_fallthrough_count: 0,
            capacity_limit_sensible_output_maximum_capacity_assignment_count: 0,
            source_site_execution_count: 0,
            maximum_total_cooling_capacity_read_count: 0,
            cooling_sensible_output_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count: 0,
            witnessed_capacity_limit_sensible_output_maximum_capacity_assignment_count: 0,
        }
    }
}
