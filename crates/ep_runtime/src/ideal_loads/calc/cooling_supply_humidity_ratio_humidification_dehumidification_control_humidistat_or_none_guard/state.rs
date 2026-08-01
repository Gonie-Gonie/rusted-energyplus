//! Persistent CP371 nested dehumidification-control guard state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthrough,
    DehumidificationControlHumidistatBodyEntered,
    DehumidificationControlNoneBodyEntered,
    DehumidificationControlGuardFalseFallthrough,
}

/// Persistent bounded state and exact source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub dehumidification_control_none_case_completed_skip_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count: usize,
    pub dehumidification_control_humidistat_case_completed_skip_count: usize,
    pub dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count: usize,
    pub heating_on_read_count: usize,
    pub heating_on_body_entry_count: usize,
    pub heating_on_guard_false_fallthrough_count: usize,
    pub humidification_control_type_read_count: usize,
    pub humidification_control_type_humidistat_comparison_count: usize,
    pub humidification_control_body_entry_count: usize,
    pub humidification_control_guard_false_fallthrough_count: usize,
    pub dehumidification_control_type_first_read_count: usize,
    pub dehumidification_control_type_humidistat_comparison_count: usize,
    pub dehumidification_control_type_humidistat_match_count: usize,
    pub dehumidification_control_type_second_read_count: usize,
    pub dehumidification_control_type_none_comparison_count: usize,
    pub dehumidification_control_type_none_match_count: usize,
    pub dehumidification_control_body_entry_count: usize,
    pub dehumidification_control_guard_false_fallthrough_count: usize,
    pub source_site_execution_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState {
    /// Creates zeroed CP371 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            dehumidification_control_none_case_completed_skip_count: 0,
            dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count: 0,
            dehumidification_control_humidistat_case_completed_skip_count: 0,
            dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count: 0,
            heating_on_read_count: 0,
            heating_on_body_entry_count: 0,
            heating_on_guard_false_fallthrough_count: 0,
            humidification_control_type_read_count: 0,
            humidification_control_type_humidistat_comparison_count: 0,
            humidification_control_body_entry_count: 0,
            humidification_control_guard_false_fallthrough_count: 0,
            dehumidification_control_type_first_read_count: 0,
            dehumidification_control_type_humidistat_comparison_count: 0,
            dehumidification_control_type_humidistat_match_count: 0,
            dehumidification_control_type_second_read_count: 0,
            dehumidification_control_type_none_comparison_count: 0,
            dehumidification_control_type_none_match_count: 0,
            dehumidification_control_body_entry_count: 0,
            dehumidification_control_guard_false_fallthrough_count: 0,
            source_site_execution_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
