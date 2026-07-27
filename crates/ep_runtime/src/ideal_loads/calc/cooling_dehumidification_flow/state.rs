//! Persistent state for the CP319 cooling dehumidification-flow calculation.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingDehumidificationFlowSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute {
    UnitOff,
    NonCooling,
    CoolingAvailabilityOff,
    DehumidificationControlInactive,
    DeltaHumidityRatioFallthrough,
    MoistureDemandFallthrough,
    CandidateAssigned,
}

/// Persistent bounded state for one system's CP319 transitions.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingDehumidificationFlowRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP318 snapshots consumed.
    pub transition_count: usize,
    /// Cooling bodies entered.
    pub cooling_body_entry_count: usize,
    /// Unit-off skips.
    pub unit_off_skip_count: usize,
    /// Active non-cooling skips.
    pub non_cooling_skip_count: usize,
    /// Dehumidification-flow zero resets.
    pub supply_mass_flow_rate_for_dehumidification_reset_assignment_count: usize,
    /// Retained `CoolOn` reads.
    pub cooling_on_read_count: usize,
    /// `CoolOn` true-body entries.
    pub cooling_on_body_entry_count: usize,
    /// `CoolOn` false fallthroughs.
    pub cooling_on_fallthrough_count: usize,
    /// Dehumidification-control enum reads.
    pub dehumidification_control_type_read_count: usize,
    /// Humidistat selector matches.
    pub dehumidification_control_type_humidistat_count: usize,
    /// Non-Humidistat selector fallthroughs.
    pub dehumidification_control_type_fallthrough_count: usize,
    /// Humidistat dehumidification-body entries.
    pub dehumidification_control_body_entry_count: usize,
    /// Zone dehumidifying setpoint-demand reads.
    pub zone_dehumidifying_setpoint_moisture_demand_read_count: usize,
    /// Local dehumidifying setpoint-demand assignments.
    pub zone_dehumidifying_setpoint_moisture_demand_assignment_count: usize,
    /// Minimum cooling supply-air humidity-ratio reads.
    pub minimum_cooling_supply_air_humidity_ratio_read_count: usize,
    /// Zone humidity-ratio reads.
    pub zone_humidity_ratio_read_count: usize,
    /// Delta humidity-ratio calculations.
    pub delta_humidity_ratio_calculation_count: usize,
    /// Local delta humidity-ratio assignments.
    pub delta_humidity_ratio_assignment_count: usize,
    /// Delta humidity-ratio reads for the first predicate.
    pub delta_humidity_ratio_for_gate_read_count: usize,
    /// Strict delta humidity-ratio comparisons.
    pub delta_humidity_ratio_comparison_count: usize,
    /// Satisfied delta humidity-ratio comparisons.
    pub delta_humidity_ratio_comparison_satisfied_count: usize,
    /// Delta humidity-ratio predicate fallthroughs.
    pub delta_humidity_ratio_fallthrough_count: usize,
    /// Demand reads for the short-circuited second predicate.
    pub zone_dehumidifying_setpoint_moisture_demand_for_gate_read_count: usize,
    /// Strict negative-demand comparisons.
    pub zone_dehumidifying_setpoint_moisture_demand_comparison_count: usize,
    /// Satisfied negative-demand comparisons.
    pub zone_dehumidifying_setpoint_moisture_demand_comparison_satisfied_count: usize,
    /// Negative-demand predicate fallthroughs.
    pub zone_dehumidifying_setpoint_moisture_demand_fallthrough_count: usize,
    /// Dehumidification-flow division-body entries.
    pub dehumidification_flow_body_entry_count: usize,
    /// Demand reads for the source division.
    pub zone_dehumidifying_setpoint_moisture_demand_for_division_read_count: usize,
    /// Delta humidity-ratio reads for the source division.
    pub delta_humidity_ratio_for_division_read_count: usize,
    /// Dehumidification-flow calculations.
    pub supply_mass_flow_rate_for_dehumidification_calculation_count: usize,
    /// Dehumidification-flow assignments.
    pub supply_mass_flow_rate_for_dehumidification_assignment_count: usize,
    /// Latest bounded snapshot.
    pub latest: Option<PurchasedAirCalcCoolingDehumidificationFlowSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingDehumidificationFlowRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingDehumidificationFlowRuntimeState {
    /// Creates zeroed CP319 state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            supply_mass_flow_rate_for_dehumidification_reset_assignment_count: 0,
            cooling_on_read_count: 0,
            cooling_on_body_entry_count: 0,
            cooling_on_fallthrough_count: 0,
            dehumidification_control_type_read_count: 0,
            dehumidification_control_type_humidistat_count: 0,
            dehumidification_control_type_fallthrough_count: 0,
            dehumidification_control_body_entry_count: 0,
            zone_dehumidifying_setpoint_moisture_demand_read_count: 0,
            zone_dehumidifying_setpoint_moisture_demand_assignment_count: 0,
            minimum_cooling_supply_air_humidity_ratio_read_count: 0,
            zone_humidity_ratio_read_count: 0,
            delta_humidity_ratio_calculation_count: 0,
            delta_humidity_ratio_assignment_count: 0,
            delta_humidity_ratio_for_gate_read_count: 0,
            delta_humidity_ratio_comparison_count: 0,
            delta_humidity_ratio_comparison_satisfied_count: 0,
            delta_humidity_ratio_fallthrough_count: 0,
            zone_dehumidifying_setpoint_moisture_demand_for_gate_read_count: 0,
            zone_dehumidifying_setpoint_moisture_demand_comparison_count: 0,
            zone_dehumidifying_setpoint_moisture_demand_comparison_satisfied_count: 0,
            zone_dehumidifying_setpoint_moisture_demand_fallthrough_count: 0,
            dehumidification_flow_body_entry_count: 0,
            zone_dehumidifying_setpoint_moisture_demand_for_division_read_count: 0,
            delta_humidity_ratio_for_division_read_count: 0,
            supply_mass_flow_rate_for_dehumidification_calculation_count: 0,
            supply_mass_flow_rate_for_dehumidification_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
