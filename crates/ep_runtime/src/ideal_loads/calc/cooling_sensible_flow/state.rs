//! Persistent state for the CP318 cooling sensible-flow calculation.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSensibleFlowSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSensibleFlowRetainedRoute {
    UnitOff,
    NonCooling,
    CoolingAvailabilityOff,
    DeltaTemperatureFallthrough,
    CandidateAssigned,
}

/// Persistent bounded state for one system's CP318 transitions.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSensibleFlowRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP317 snapshots consumed.
    pub transition_count: usize,
    /// Cooling bodies entered.
    pub cooling_body_entry_count: usize,
    /// Unit-off skips.
    pub unit_off_skip_count: usize,
    /// Active non-cooling skips.
    pub non_cooling_skip_count: usize,
    /// Cooling-body zero assignments.
    pub supply_mass_flow_rate_for_cool_reset_assignment_count: usize,
    /// Retained `CoolOn` reads.
    pub cooling_on_read_count: usize,
    /// `CoolOn` true-body entries.
    pub cooling_on_body_entry_count: usize,
    /// `CoolOn` false fallthroughs.
    pub cooling_on_fallthrough_count: usize,
    /// Zone humidity-ratio reads.
    pub zone_humidity_ratio_read_count: usize,
    /// `PsyCpAirFnW` evaluations.
    pub psychrometric_cp_air_evaluation_count: usize,
    /// Local `CpAir` assignments.
    pub cp_air_assignment_count: usize,
    /// Minimum cooling supply-air temperature reads.
    pub minimum_cooling_supply_air_temperature_read_count: usize,
    /// Zone temperature reads.
    pub zone_temperature_read_count: usize,
    /// Minimum-supply-minus-Zone calculations.
    pub delta_temperature_calculation_count: usize,
    /// Local `DeltaT` assignments.
    pub delta_temperature_assignment_count: usize,
    /// `DeltaT` reads for the strict gate.
    pub delta_temperature_for_gate_read_count: usize,
    /// Strict delta-temperature comparisons.
    pub delta_temperature_comparison_count: usize,
    /// Satisfied delta-temperature comparisons.
    pub delta_temperature_comparison_satisfied_count: usize,
    /// True delta-temperature body entries.
    pub delta_temperature_body_entry_count: usize,
    /// False delta-temperature fallthroughs.
    pub delta_temperature_fallthrough_count: usize,
    /// `QZnCoolSP` reads.
    pub zone_cooling_setpoint_load_read_count: usize,
    /// `CpAir` reads for the first division.
    pub cp_air_for_first_division_read_count: usize,
    /// `QZnCoolSP / CpAir` calculations.
    pub zone_cooling_setpoint_load_over_cp_air_calculation_count: usize,
    /// `DeltaT` reads for the second division.
    pub delta_temperature_for_second_division_read_count: usize,
    /// Second supply-flow divisions.
    pub supply_mass_flow_rate_for_cool_calculation_count: usize,
    /// Final supply-flow assignments.
    pub supply_mass_flow_rate_for_cool_assignment_count: usize,
    /// Latest bounded snapshot.
    pub latest: Option<PurchasedAirCalcCoolingSensibleFlowSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingSensibleFlowRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSensibleFlowRuntimeState {
    /// Creates zeroed CP318 state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_body_entry_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            supply_mass_flow_rate_for_cool_reset_assignment_count: 0,
            cooling_on_read_count: 0,
            cooling_on_body_entry_count: 0,
            cooling_on_fallthrough_count: 0,
            zone_humidity_ratio_read_count: 0,
            psychrometric_cp_air_evaluation_count: 0,
            cp_air_assignment_count: 0,
            minimum_cooling_supply_air_temperature_read_count: 0,
            zone_temperature_read_count: 0,
            delta_temperature_calculation_count: 0,
            delta_temperature_assignment_count: 0,
            delta_temperature_for_gate_read_count: 0,
            delta_temperature_comparison_count: 0,
            delta_temperature_comparison_satisfied_count: 0,
            delta_temperature_body_entry_count: 0,
            delta_temperature_fallthrough_count: 0,
            zone_cooling_setpoint_load_read_count: 0,
            cp_air_for_first_division_read_count: 0,
            zone_cooling_setpoint_load_over_cp_air_calculation_count: 0,
            delta_temperature_for_second_division_read_count: 0,
            supply_mass_flow_rate_for_cool_calculation_count: 0,
            supply_mass_flow_rate_for_cool_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
