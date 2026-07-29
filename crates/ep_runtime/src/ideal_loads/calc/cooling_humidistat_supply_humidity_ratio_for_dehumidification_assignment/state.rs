//! Persistent CP360 Humidistat local dehumidification supply-humidity-ratio state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    DehumidificationControlNoneCaseCompletedSkip,
    DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,
    DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted,
    DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
}

/// Persistent bounded state and exact source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub dehumidification_control_none_case_completed_skip_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count: usize,
    pub dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count:
        usize,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count: usize,
    pub source_site_execution_count: usize,
    pub zone_dehumidifying_setpoint_moisture_demand_read_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub moisture_demand_derived_supply_humidity_ratio_calculation_count: usize,
    pub zone_node_humidity_ratio_read_count: usize,
    pub supply_humidity_ratio_for_dehumidification_calculation_count: usize,
    pub supply_humidity_ratio_for_dehumidification_assignment_count: usize,
    pub latest:
        Option<PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot>,
    pub(super) latest_route:
        Option<PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
    pub(super) witnessed_positive_guard_false_fallthrough_skip_count: usize,
    pub(super) witnessed_dehumidification_control_none_case_completed_skip_count: usize,
    pub(super) witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count:
        usize,
    pub(super) witnessed_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count:
        usize,
    pub(super) witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count:
        usize,
}

impl PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState {
    /// Creates zeroed CP360 state for one system.
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
            dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count: 0,
            dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count: 0,
            source_site_execution_count: 0,
            zone_dehumidifying_setpoint_moisture_demand_read_count: 0,
            supply_mass_flow_rate_read_count: 0,
            moisture_demand_derived_supply_humidity_ratio_calculation_count: 0,
            zone_node_humidity_ratio_read_count: 0,
            supply_humidity_ratio_for_dehumidification_calculation_count: 0,
            supply_humidity_ratio_for_dehumidification_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
            witnessed_positive_guard_false_fallthrough_skip_count: 0,
            witnessed_dehumidification_control_none_case_completed_skip_count: 0,
            witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count: 0,
            witnessed_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count: 0,
            witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count: 0,
        }
    }
}
