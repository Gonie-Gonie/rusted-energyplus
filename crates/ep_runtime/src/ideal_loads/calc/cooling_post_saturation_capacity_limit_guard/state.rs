//! Persistent CP380 post-saturation capacity-limit guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthroughBodyEntered,
    HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthroughBodyEntered,
    HumidificationControlGuardFalseFallthroughGuardFalseFallthrough,
    DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered,
    DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough,
    DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered,
    DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough,
    DehumidificationControlGuardFalseFallthroughBodyEntered,
    DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough,
}

/// Persistent bounded state and exact source/lineage counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub heating_availability_guard_false_fallthrough_count: usize,
    pub humidification_control_guard_false_fallthrough_count: usize,
    pub dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count: usize,
    pub dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count: usize,
    pub dehumidification_control_guard_false_fallthrough_count: usize,
    pub heating_availability_guard_false_fallthrough_body_entry_count: usize,
    pub heating_availability_guard_false_fallthrough_capacity_guard_false_count: usize,
    pub humidification_control_guard_false_fallthrough_body_entry_count: usize,
    pub humidification_control_guard_false_fallthrough_capacity_guard_false_count: usize,
    pub dehumidification_control_humidistat_maximum_assignment_body_entry_count: usize,
    pub dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count: usize,
    pub dehumidification_control_none_maximum_assignment_body_entry_count: usize,
    pub dehumidification_control_none_maximum_assignment_capacity_guard_false_count: usize,
    pub dehumidification_control_guard_false_fallthrough_body_entry_count: usize,
    pub dehumidification_control_guard_false_fallthrough_capacity_guard_false_count: usize,
    pub capacity_limit_guard_evaluation_count: usize,
    pub source_site_execution_count: usize,
    pub configured_cooling_limit_owned_read_count: usize,
    pub cp337_same_call_selector_lineage_corroboration_count: usize,
    pub first_cooling_limit_read_count: usize,
    pub cooling_limit_capacity_comparison_count: usize,
    pub cooling_limit_capacity_match_count: usize,
    pub second_cooling_limit_read_count: usize,
    pub cooling_limit_flow_rate_and_capacity_comparison_count: usize,
    pub cooling_limit_flow_rate_and_capacity_match_count: usize,
    pub cooling_limit_rejected_count: usize,
    pub capacity_limit_body_entry_count: usize,
    pub active_guard_false_fallthrough_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState {
    /// Creates zeroed CP380 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            heating_availability_guard_false_fallthrough_count: 0,
            humidification_control_guard_false_fallthrough_count: 0,
            dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count: 0,
            dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count: 0,
            dehumidification_control_guard_false_fallthrough_count: 0,
            heating_availability_guard_false_fallthrough_body_entry_count: 0,
            heating_availability_guard_false_fallthrough_capacity_guard_false_count: 0,
            humidification_control_guard_false_fallthrough_body_entry_count: 0,
            humidification_control_guard_false_fallthrough_capacity_guard_false_count: 0,
            dehumidification_control_humidistat_maximum_assignment_body_entry_count: 0,
            dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count: 0,
            dehumidification_control_none_maximum_assignment_body_entry_count: 0,
            dehumidification_control_none_maximum_assignment_capacity_guard_false_count: 0,
            dehumidification_control_guard_false_fallthrough_body_entry_count: 0,
            dehumidification_control_guard_false_fallthrough_capacity_guard_false_count: 0,
            capacity_limit_guard_evaluation_count: 0,
            source_site_execution_count: 0,
            configured_cooling_limit_owned_read_count: 0,
            cp337_same_call_selector_lineage_corroboration_count: 0,
            first_cooling_limit_read_count: 0,
            cooling_limit_capacity_comparison_count: 0,
            cooling_limit_capacity_match_count: 0,
            second_cooling_limit_read_count: 0,
            cooling_limit_flow_rate_and_capacity_comparison_count: 0,
            cooling_limit_flow_rate_and_capacity_match_count: 0,
            cooling_limit_rejected_count: 0,
            capacity_limit_body_entry_count: 0,
            active_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
