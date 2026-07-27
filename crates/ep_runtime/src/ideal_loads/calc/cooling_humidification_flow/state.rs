//! Persistent CP320 cooling humidification-flow state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingHumidificationFlowSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingHumidificationFlowRetainedRoute {
    UnitOff,
    NonCooling,
    HeatingAvailabilityOff,
    HumidificationControlInactive,
    DehumidificationControlRejected,
    DeltaHumidityRatioFallthrough,
    MoistureDemandFallthrough,
    CandidateAssigned,
}

/// Persistent bounded state and exact source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidificationFlowRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_body_entry_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub reset_assignment_count: usize,
    pub heating_on_read_count: usize,
    pub heating_on_body_entry_count: usize,
    pub heating_on_fallthrough_count: usize,
    pub humidification_control_type_read_count: usize,
    pub humidification_control_type_humidistat_count: usize,
    pub humidification_control_type_fallthrough_count: usize,
    pub humidification_control_body_entry_count: usize,
    pub dehumidification_control_type_first_read_count: usize,
    pub dehumidification_control_type_humidistat_count: usize,
    pub dehumidification_control_type_second_read_count: usize,
    pub dehumidification_control_type_none_count: usize,
    pub dehumidification_control_type_rejected_count: usize,
    pub admitted_control_body_entry_count: usize,
    pub moisture_demand_read_count: usize,
    pub moisture_demand_assignment_count: usize,
    pub maximum_heating_supply_humidity_ratio_read_count: usize,
    pub zone_humidity_ratio_read_count: usize,
    pub delta_calculation_count: usize,
    pub delta_assignment_count: usize,
    pub delta_gate_read_count: usize,
    pub delta_comparison_count: usize,
    pub delta_comparison_satisfied_count: usize,
    pub delta_fallthrough_count: usize,
    pub moisture_demand_gate_read_count: usize,
    pub moisture_demand_comparison_count: usize,
    pub moisture_demand_comparison_satisfied_count: usize,
    pub moisture_demand_fallthrough_count: usize,
    pub humidification_flow_body_entry_count: usize,
    pub moisture_demand_division_read_count: usize,
    pub delta_division_read_count: usize,
    pub calculation_count: usize,
    pub assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingHumidificationFlowSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingHumidificationFlowRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingHumidificationFlowRuntimeState {
    /// Creates zeroed CP320 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            reset_assignment_count: 0,
            heating_on_read_count: 0,
            heating_on_body_entry_count: 0,
            heating_on_fallthrough_count: 0,
            humidification_control_type_read_count: 0,
            humidification_control_type_humidistat_count: 0,
            humidification_control_type_fallthrough_count: 0,
            humidification_control_body_entry_count: 0,
            dehumidification_control_type_first_read_count: 0,
            dehumidification_control_type_humidistat_count: 0,
            dehumidification_control_type_second_read_count: 0,
            dehumidification_control_type_none_count: 0,
            dehumidification_control_type_rejected_count: 0,
            admitted_control_body_entry_count: 0,
            moisture_demand_read_count: 0,
            moisture_demand_assignment_count: 0,
            maximum_heating_supply_humidity_ratio_read_count: 0,
            zone_humidity_ratio_read_count: 0,
            delta_calculation_count: 0,
            delta_assignment_count: 0,
            delta_gate_read_count: 0,
            delta_comparison_count: 0,
            delta_comparison_satisfied_count: 0,
            delta_fallthrough_count: 0,
            moisture_demand_gate_read_count: 0,
            moisture_demand_comparison_count: 0,
            moisture_demand_comparison_satisfied_count: 0,
            moisture_demand_fallthrough_count: 0,
            humidification_flow_body_entry_count: 0,
            moisture_demand_division_read_count: 0,
            delta_division_read_count: 0,
            calculation_count: 0,
            assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
