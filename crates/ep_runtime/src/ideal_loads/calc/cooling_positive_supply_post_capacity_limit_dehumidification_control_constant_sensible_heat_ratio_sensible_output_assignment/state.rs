//! Persistent CP350 constant-SHR sensible-output assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    DehumidificationControlNoneCaseCompletedSkip,
    DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned,
    DehumidificationControlHumidistatCaseSelectedSkip,
    DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub dehumidification_control_none_case_completed_skip_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count:
        usize,
    pub dehumidification_control_humidistat_case_selected_skip_count: usize,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count: usize,
    pub source_site_execution_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub cp_air_read_count: usize,
    pub supply_mass_flow_rate_times_cp_air_calculation_count: usize,
    pub mixed_air_temperature_read_count: usize,
    pub supply_temperature_read_count: usize,
    pub mixed_air_minus_supply_temperature_calculation_count: usize,
    pub cooling_sensible_output_calculation_count: usize,
    pub cooling_sensible_output_assignment_write_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
    >,
    pub(super) latest_route: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRetainedRoute,
    >,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_dehumidification_control_none_case_completed_skip_count: usize,
    pub(super)
        witnessed_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count:
        usize,
    pub(super) witnessed_dehumidification_control_humidistat_case_selected_skip_count: usize,
    pub(super)
        witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count:
        usize,
}

impl
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState
{
    /// Creates zeroed CP350 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            dehumidification_control_none_case_completed_skip_count: 0,
            dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count:
                0,
            dehumidification_control_humidistat_case_selected_skip_count: 0,
            dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count: 0,
            source_site_execution_count: 0,
            supply_mass_flow_rate_read_count: 0,
            cp_air_read_count: 0,
            supply_mass_flow_rate_times_cp_air_calculation_count: 0,
            mixed_air_temperature_read_count: 0,
            supply_temperature_read_count: 0,
            mixed_air_minus_supply_temperature_calculation_count: 0,
            cooling_sensible_output_calculation_count: 0,
            cooling_sensible_output_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_dehumidification_control_none_case_completed_skip_count: 0,
            witnessed_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count:
                0,
            witnessed_dehumidification_control_humidistat_case_selected_skip_count: 0,
            witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count:
                0,
        }
    }
}
