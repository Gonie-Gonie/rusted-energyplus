//! Persistent state for the CP317 cooling economizer true body.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingEconomizerBodySnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingEconomizerBodyRetainedRoute {
    UnitOff,
    NonCooling,
    MaximumCoolingFlowBodySibling,
    NoEconomizerOuterGuardFallthrough,
    EconomizerConditionFallthrough,
    Executed,
}

/// Persistent bounded state for one system's CP317 transitions.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEconomizerBodyRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP316 snapshots consumed.
    pub transition_count: usize,
    /// CP317 bodies executed.
    pub body_execution_count: usize,
    /// Unit-off skips.
    pub unit_off_skip_count: usize,
    /// Active non-cooling skips.
    pub non_cooling_skip_count: usize,
    /// CP313 maximum-flow sibling skips.
    pub maximum_cooling_flow_body_sibling_skip_count: usize,
    /// False CP315 outer-guard skips.
    pub no_economizer_outer_guard_fallthrough_skip_count: usize,
    /// False CP316 condition skips.
    pub economizer_condition_fallthrough_skip_count: usize,
    /// Zone humidity-ratio reads.
    pub zone_humidity_ratio_read_count: usize,
    /// `PsyCpAirFnW` evaluations.
    pub psychrometric_cp_air_evaluation_count: usize,
    /// Local `CpAir` assignments.
    pub cp_air_assignment_count: usize,
    /// Outdoor-air temperature reads.
    pub outdoor_air_temperature_read_count: usize,
    /// Zone temperature reads.
    pub zone_temperature_read_count: usize,
    /// Outdoor-minus-Zone calculations.
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
    pub supply_mass_flow_rate_calculation_count: usize,
    /// Initial supply-flow assignments.
    pub initial_supply_mass_flow_rate_assignment_count: usize,
    /// First `CoolingLimit` reads.
    pub cooling_limit_flow_rate_read_count: usize,
    /// First cooling-limit comparisons.
    pub cooling_limit_flow_rate_comparison_count: usize,
    /// First cooling-limit matches.
    pub cooling_limit_flow_rate_match_count: usize,
    /// Second `CoolingLimit` reads.
    pub cooling_limit_flow_rate_and_capacity_read_count: usize,
    /// Second cooling-limit comparisons.
    pub cooling_limit_flow_rate_and_capacity_comparison_count: usize,
    /// Second cooling-limit matches.
    pub cooling_limit_flow_rate_and_capacity_match_count: usize,
    /// Maximum-flow guard reads.
    pub maximum_cooling_air_mass_flow_rate_read_count: usize,
    /// Maximum-flow positive comparisons.
    pub maximum_cooling_air_mass_flow_rate_positive_comparison_count: usize,
    /// Satisfied maximum-flow positive comparisons.
    pub maximum_cooling_air_mass_flow_rate_positive_count: usize,
    /// Clamp-body entries.
    pub maximum_flow_clamp_body_entry_count: usize,
    /// Supply-flow reads for the inner maximum.
    pub supply_mass_flow_rate_for_clamp_read_count: usize,
    /// Inner maximum evaluations.
    pub inner_max_evaluation_count: usize,
    /// Maximum-flow clamp-upper-bound re-reads.
    pub maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count: usize,
    /// Outer minimum evaluations.
    pub outer_min_evaluation_count: usize,
    /// Legacy completed-clamp count.
    pub supply_mass_flow_rate_clamp_count: usize,
    /// Clamped supply-flow assignments.
    pub clamped_supply_mass_flow_rate_assignment_count: usize,
    /// Supply-flow reads for the final comparison.
    pub resulting_supply_mass_flow_rate_read_count: usize,
    /// Outdoor-air-flow reads for the final comparison.
    pub outdoor_air_mass_flow_rate_read_count: usize,
    /// Final strict mass-flow comparisons.
    pub supply_above_outdoor_air_mass_flow_comparison_count: usize,
    /// Satisfied final mass-flow comparisons.
    pub supply_above_outdoor_air_mass_flow_comparison_satisfied_count: usize,
    /// Economizer activation-body entries.
    pub economizer_activation_body_entry_count: usize,
    /// False final-comparison fallthroughs.
    pub outdoor_air_mass_flow_comparison_fallthrough_count: usize,
    /// `EconoOn` assignments.
    pub economizer_on_assignment_count: usize,
    /// Supply-flow re-reads for outdoor-air assignment.
    pub supply_mass_flow_rate_for_outdoor_air_assignment_read_count: usize,
    /// Outdoor-air-flow assignments.
    pub outdoor_air_mass_flow_rate_assignment_count: usize,
    /// `TimeStepSys` reads.
    pub system_time_step_read_count: usize,
    /// `TimeEconoActive` assignments.
    pub economizer_active_time_assignment_count: usize,
    /// Latest bounded snapshot.
    pub latest: Option<PurchasedAirCalcCoolingEconomizerBodySnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingEconomizerBodyRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingEconomizerBodyRuntimeState {
    /// Creates zeroed CP317 state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            body_execution_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            maximum_cooling_flow_body_sibling_skip_count: 0,
            no_economizer_outer_guard_fallthrough_skip_count: 0,
            economizer_condition_fallthrough_skip_count: 0,
            zone_humidity_ratio_read_count: 0,
            psychrometric_cp_air_evaluation_count: 0,
            cp_air_assignment_count: 0,
            outdoor_air_temperature_read_count: 0,
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
            supply_mass_flow_rate_calculation_count: 0,
            initial_supply_mass_flow_rate_assignment_count: 0,
            cooling_limit_flow_rate_read_count: 0,
            cooling_limit_flow_rate_comparison_count: 0,
            cooling_limit_flow_rate_match_count: 0,
            cooling_limit_flow_rate_and_capacity_read_count: 0,
            cooling_limit_flow_rate_and_capacity_comparison_count: 0,
            cooling_limit_flow_rate_and_capacity_match_count: 0,
            maximum_cooling_air_mass_flow_rate_read_count: 0,
            maximum_cooling_air_mass_flow_rate_positive_comparison_count: 0,
            maximum_cooling_air_mass_flow_rate_positive_count: 0,
            maximum_flow_clamp_body_entry_count: 0,
            supply_mass_flow_rate_for_clamp_read_count: 0,
            inner_max_evaluation_count: 0,
            maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count: 0,
            outer_min_evaluation_count: 0,
            supply_mass_flow_rate_clamp_count: 0,
            clamped_supply_mass_flow_rate_assignment_count: 0,
            resulting_supply_mass_flow_rate_read_count: 0,
            outdoor_air_mass_flow_rate_read_count: 0,
            supply_above_outdoor_air_mass_flow_comparison_count: 0,
            supply_above_outdoor_air_mass_flow_comparison_satisfied_count: 0,
            economizer_activation_body_entry_count: 0,
            outdoor_air_mass_flow_comparison_fallthrough_count: 0,
            economizer_on_assignment_count: 0,
            supply_mass_flow_rate_for_outdoor_air_assignment_read_count: 0,
            outdoor_air_mass_flow_rate_assignment_count: 0,
            system_time_step_read_count: 0,
            economizer_active_time_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
