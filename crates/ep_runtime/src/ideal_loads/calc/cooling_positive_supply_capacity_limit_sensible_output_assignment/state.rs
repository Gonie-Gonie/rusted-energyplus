//! Persistent CP339 Cooling capacity-limit sensible-output assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    ActiveCapacityLimitGuardFalseFallthrough,
    CapacityLimitSensibleOutputAssigned,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_sensible_output_assignment_count: usize,
    pub source_site_execution_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub mixed_air_enthalpy_read_count: usize,
    pub supply_enthalpy_read_count: usize,
    pub enthalpy_difference_calculation_count: usize,
    pub cooling_sensible_output_calculation_count: usize,
    pub cooling_sensible_output_assignment_write_count: usize,
    pub latest:
        Option<PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot>,
    pub(super) latest_route: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRetainedRoute,
    >,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_sensible_output_assignment_count: usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState {
    /// Creates zeroed CP339 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            capacity_limit_guard_false_fallthrough_skip_count: 0,
            capacity_limit_sensible_output_assignment_count: 0,
            source_site_execution_count: 0,
            supply_mass_flow_rate_read_count: 0,
            mixed_air_enthalpy_read_count: 0,
            supply_enthalpy_read_count: 0,
            enthalpy_difference_calculation_count: 0,
            cooling_sensible_output_calculation_count: 0,
            cooling_sensible_output_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_sensible_output_assignment_count: 0,
        }
    }
}
