//! Persistent CP342 Cooling capacity-limit supply-enthalpy assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    ActiveCapacityLimitGuardFalseFallthrough,
    CapacityLimitSensibleOutputGuardFalseFallthrough,
    CapacityLimitSensibleOutputSupplyEnthalpyAssigned,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_sensible_output_guard_false_fallthrough_count: usize,
    pub capacity_limit_sensible_output_supply_enthalpy_assignment_count: usize,
    pub source_site_execution_count: usize,
    pub mixed_air_enthalpy_read_count: usize,
    pub cooling_sensible_output_read_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub specific_cooling_output_calculation_count: usize,
    pub supply_enthalpy_calculation_count: usize,
    pub supply_enthalpy_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    >,
    pub(super) latest_route: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRetainedRoute,
    >,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count: usize,
    pub(super) witnessed_capacity_limit_sensible_output_supply_enthalpy_assignment_count: usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState {
    /// Creates zeroed CP342 state for one system.
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
            capacity_limit_sensible_output_supply_enthalpy_assignment_count: 0,
            source_site_execution_count: 0,
            mixed_air_enthalpy_read_count: 0,
            cooling_sensible_output_read_count: 0,
            supply_mass_flow_rate_read_count: 0,
            specific_cooling_output_calculation_count: 0,
            supply_enthalpy_calculation_count: 0,
            supply_enthalpy_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count: 0,
            witnessed_capacity_limit_sensible_output_supply_enthalpy_assignment_count: 0,
        }
    }
}
