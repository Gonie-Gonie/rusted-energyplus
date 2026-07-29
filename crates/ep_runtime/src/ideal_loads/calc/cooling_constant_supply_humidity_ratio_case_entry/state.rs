//! Persistent CP364 constant-supply-humidity-ratio case-entry state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    DehumidificationControlNoneCaseCompletedSkip,
    DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,
    DehumidificationControlHumidistatCaseCompletedSkip,
    DehumidificationControlConstantSupplyHumidityRatioCaseEntered,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub dehumidification_control_none_case_completed_skip_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count: usize,
    pub dehumidification_control_humidistat_case_completed_skip_count: usize,
    pub dehumidification_control_constant_supply_humidity_ratio_case_entry_count: usize,
    pub source_site_execution_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_dehumidification_control_none_case_completed_skip_count: usize,
    pub(super) witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count:
        usize,
    pub(super) witnessed_dehumidification_control_humidistat_case_completed_skip_count: usize,
    pub(super) witnessed_dehumidification_control_constant_supply_humidity_ratio_case_entry_count:
        usize,
}

impl PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState {
    /// Creates zeroed CP364 state for one system.
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
            dehumidification_control_constant_supply_humidity_ratio_case_entry_count: 0,
            source_site_execution_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_dehumidification_control_none_case_completed_skip_count: 0,
            witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count:
                0,
            witnessed_dehumidification_control_humidistat_case_completed_skip_count: 0,
            witnessed_dehumidification_control_constant_supply_humidity_ratio_case_entry_count: 0,
        }
    }
}
