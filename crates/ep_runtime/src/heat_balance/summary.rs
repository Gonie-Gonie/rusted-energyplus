//! Heat-balance simulation summary value types.

use crate::ResultStore;
use crate::heat_balance::state::{
    HeatBalanceCtfHistorySlotFirstSample, HeatBalanceCtfHistorySlotHourlySample,
    HeatBalanceCtfHistorySlotSample, HeatBalanceCtfInitialHistoryPolicy, HeatBalanceState,
    HeatBalanceSurfaceFirstSampleTrace, HeatBalanceSurfaceIterationFirstSampleTrace,
    HeatBalanceSurfaceIterationSampleTrace, HeatBalanceSurfaceLoopZoneAirCorrection,
    HeatBalanceWarmupDayEndZoneAirStateSample, HeatBalanceZoneAirFirstSampleTrace,
    HeatBalanceZoneAirReportSampling, HeatBalanceZoneAirStateSample,
    HeatBalanceZoneConductionReportSource,
};
/// Summary of the executed heat-balance warmup loop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceWarmupSummary {
    /// Whether warmup was requested.
    pub enabled: bool,
    /// Number of warmup days actually executed.
    pub day_count: u32,
    /// Number of timesteps executed during warmup.
    pub timestep_count: usize,
    /// Number of weather hours repeated for one warmup day.
    pub hours_per_day: usize,
    /// Whether the repeated-day end state converged before max days.
    pub converged: bool,
    /// Final max zone air temperature delta between repeated-day end states.
    pub final_max_zone_temperature_delta_c: f64,
}

impl HeatBalanceWarmupSummary {
    /// Creates a disabled warmup summary.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            day_count: 0,
            timestep_count: 0,
            hours_per_day: 0,
            converged: false,
            final_max_zone_temperature_delta_c: 0.0,
        }
    }
}

/// Summary for the heat-balance zone-air diagnostic trace.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSimulationSummary {
    /// Hourly output sample count.
    pub samples: usize,
    /// Number of executed zone timesteps.
    pub timestep_count: usize,
    /// Number of reported run-period zone timesteps.
    pub run_period_timestep_count: usize,
    /// Warmup execution summary.
    pub warmup: HeatBalanceWarmupSummary,
    /// Number of zones represented in the state.
    pub zone_count: usize,
    /// Number of surfaces represented in the state.
    pub surface_count: usize,
    /// Number of surface-balance passes used per zone timestep.
    pub surface_iteration_count: u32,
    /// Optional frozen inside-convection coefficient re-evaluation interval.
    pub inside_hconv_reevaluation_interval: Option<u32>,
    /// Initial CTF temperature/flux history seeding policy.
    pub ctf_initial_history_policy: HeatBalanceCtfInitialHistoryPolicy,
    /// Source used for zone opaque conduction report variables.
    pub zone_conduction_report_source: HeatBalanceZoneConductionReportSource,
    /// Sampling mode used for zone air heat-balance report variables.
    pub zone_air_report_sampling: HeatBalanceZoneAirReportSampling,
    /// Timing for zone-air correction during interleaved surface-balance probes.
    pub surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
    /// Per-zone zone-air state after warmup and before the run period starts.
    pub run_period_initial_zone_air_states: Vec<HeatBalanceZoneAirStateSample>,
    /// Per-zone day-end state captured during run-period warmup.
    pub warmup_day_end_zone_air_states: Vec<HeatBalanceWarmupDayEndZoneAirStateSample>,
    /// Per-slot CTF history terms after optional warmup, before the run period starts.
    pub run_period_initial_ctf_history_slots: Vec<HeatBalanceCtfHistorySlotSample>,
    /// Per-slot CTF history terms averaged over the first reported hourly sample.
    pub first_sample_ctf_history_slots: Vec<HeatBalanceCtfHistorySlotFirstSample>,
    /// Per-slot CTF history terms averaged for each reported hourly sample before history advance.
    pub hourly_ctf_history_slots: Vec<HeatBalanceCtfHistorySlotHourlySample>,
    /// Per-slot CTF history terms averaged for each reported hourly sample after history advance.
    pub hourly_ctf_history_slots_after_advance: Vec<HeatBalanceCtfHistorySlotHourlySample>,
    /// Per-surface timestep states captured across the first reported hourly sample.
    pub surface_first_sample_trace: Vec<HeatBalanceSurfaceFirstSampleTrace>,
    /// Per-zone timestep states captured across the first reported hourly sample.
    pub zone_air_first_sample_trace: Vec<HeatBalanceZoneAirFirstSampleTrace>,
    /// Per-timestep inside-surface iteration summary for the first reported hourly sample.
    pub surface_iteration_first_sample_trace: Vec<HeatBalanceSurfaceIterationFirstSampleTrace>,
    /// Per-timestep inside-surface iteration summary for every reported hourly sample.
    pub surface_iteration_sample_trace: Vec<HeatBalanceSurfaceIterationSampleTrace>,
}

/// Result of the heat-balance zone-air diagnostic trace.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSimulation {
    /// Final heat-balance state.
    pub state: HeatBalanceState,
    /// Native output results.
    pub results: ResultStore,
    /// Trace summary.
    pub summary: HeatBalanceSimulationSummary,
}
