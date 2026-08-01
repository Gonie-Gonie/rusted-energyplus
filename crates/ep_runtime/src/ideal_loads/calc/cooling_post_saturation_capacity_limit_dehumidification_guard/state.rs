//! Persistent CP381 post-saturation capacity-limit dehumidification-guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered,
    HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered,
    HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
    DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
    DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered,
    DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
    DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
    DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered,
    DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
    DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
    DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered,
    DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
}

/// Persistent bounded state and exact source/lineage counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState {
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
    pub heating_availability_guard_false_fallthrough_dehumidification_body_entry_count: usize,
    pub heating_availability_guard_false_fallthrough_dehumidification_guard_false_count: usize,
    pub humidification_control_guard_false_fallthrough_dehumidification_body_entry_count: usize,
    pub humidification_control_guard_false_fallthrough_dehumidification_guard_false_count: usize,
    pub dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count:
        usize,
    pub dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count:
        usize,
    pub dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count: usize,
    pub dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count: usize,
    pub dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count: usize,
    pub dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count: usize,
    pub dehumidification_guard_evaluation_count: usize,
    pub source_site_execution_count: usize,
    pub cp378_supply_humidity_ratio_saturation_limit_owned_read_count: usize,
    pub cp379_same_call_supply_humidity_ratio_bit_corroboration_count: usize,
    pub purchased_air_supply_humidity_ratio_read_count: usize,
    pub cp329_mixed_air_humidity_ratio_owned_read_count: usize,
    pub purchased_air_mixed_air_humidity_ratio_read_count: usize,
    pub supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count: usize,
    pub supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count: usize,
    pub dehumidification_body_entry_count: usize,
    pub dehumidification_guard_false_fallthrough_count: usize,
    pub latest:
        Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot>,
    pub(super) latest_route: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRetainedRoute,
    >,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState {
    /// Creates zeroed CP381 state for one system.
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
            heating_availability_guard_false_fallthrough_dehumidification_body_entry_count: 0,
            heating_availability_guard_false_fallthrough_dehumidification_guard_false_count: 0,
            humidification_control_guard_false_fallthrough_dehumidification_body_entry_count: 0,
            humidification_control_guard_false_fallthrough_dehumidification_guard_false_count: 0,
            dehumidification_control_humidistat_maximum_assignment_dehumidification_body_entry_count: 0,
            dehumidification_control_humidistat_maximum_assignment_dehumidification_guard_false_count: 0,
            dehumidification_control_none_maximum_assignment_dehumidification_body_entry_count: 0,
            dehumidification_control_none_maximum_assignment_dehumidification_guard_false_count: 0,
            dehumidification_control_guard_false_fallthrough_dehumidification_body_entry_count: 0,
            dehumidification_control_guard_false_fallthrough_dehumidification_guard_false_count: 0,
            dehumidification_guard_evaluation_count: 0,
            source_site_execution_count: 0,
            cp378_supply_humidity_ratio_saturation_limit_owned_read_count: 0,
            cp379_same_call_supply_humidity_ratio_bit_corroboration_count: 0,
            purchased_air_supply_humidity_ratio_read_count: 0,
            cp329_mixed_air_humidity_ratio_owned_read_count: 0,
            purchased_air_mixed_air_humidity_ratio_read_count: 0,
            supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count: 0,
            supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count: 0,
            dehumidification_body_entry_count: 0,
            dehumidification_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
