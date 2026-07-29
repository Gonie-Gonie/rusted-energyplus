//! Persistent CP366 constant-supply-humidity-ratio case-break state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    DehumidificationControlNoneCaseCompletedSkip,
    DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,
    DehumidificationControlHumidistatCaseCompletedSkip,
    DehumidificationControlConstantSupplyHumidityRatioCaseBreak,
}

/// Persistent bounded state and exact source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub dehumidification_control_none_case_completed_skip_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count: usize,
    pub dehumidification_control_humidistat_case_completed_skip_count: usize,
    pub dehumidification_control_constant_supply_humidity_ratio_case_break_count: usize,
    pub source_site_execution_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_dehumidification_control_none_case_completed_skip_count: usize,
    pub(super) witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count:
        usize,
    pub(super) witnessed_dehumidification_control_humidistat_case_completed_skip_count: usize,
    pub(super) witnessed_dehumidification_control_constant_supply_humidity_ratio_case_break_count:
        usize,
}

impl PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRuntimeState {
    /// Creates zeroed CP366 state for one system.
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
            dehumidification_control_constant_supply_humidity_ratio_case_break_count: 0,
            source_site_execution_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_dehumidification_control_none_case_completed_skip_count: 0,
            witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count:
                0,
            witnessed_dehumidification_control_humidistat_case_completed_skip_count: 0,
            witnessed_dehumidification_control_constant_supply_humidity_ratio_case_break_count: 0,
        }
    }
}
