//! Persistent CP343 Cooling capacity-limit supply-temperature assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    ActiveCapacityLimitGuardFalseFallthrough,
    CapacityLimitSensibleOutputGuardFalseFallthrough,
    CapacityLimitSensibleOutputSupplyTemperatureAssigned,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub capacity_limit_sensible_output_guard_false_fallthrough_count: usize,
    pub capacity_limit_sensible_output_supply_temperature_assignment_count: usize,
    pub source_site_execution_count: usize,
    pub supply_enthalpy_for_dry_bulb_inversion_read_count: usize,
    pub supply_humidity_ratio_for_dry_bulb_inversion_read_count: usize,
    pub psychrometric_supply_temperature_evaluation_count: usize,
    pub supply_temperature_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    >,
    pub(super) latest_route: Option<
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedRoute,
    >,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count: usize,
    pub(super) witnessed_capacity_limit_sensible_output_supply_temperature_assignment_count: usize,
}

impl PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState {
    /// Creates zeroed CP343 state for one system.
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
            capacity_limit_sensible_output_supply_temperature_assignment_count: 0,
            source_site_execution_count: 0,
            supply_enthalpy_for_dry_bulb_inversion_read_count: 0,
            supply_humidity_ratio_for_dry_bulb_inversion_read_count: 0,
            psychrometric_supply_temperature_evaluation_count: 0,
            supply_temperature_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_guard_false_fallthrough_skip_count: 0,
            witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count: 0,
            witnessed_capacity_limit_sensible_output_supply_temperature_assignment_count: 0,
        }
    }
}
