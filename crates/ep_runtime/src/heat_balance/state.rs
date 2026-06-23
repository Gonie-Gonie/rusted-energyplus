//! Heat-balance trace and diagnostic state value types.

use super::algorithm::HeatBalanceZoneAirAlgorithm;
use ep_model::{SimulationModel, ZoneId};

const ENERGYPLUS_ZONE_INITIAL_TEMP_C: f64 = 23.0;

/// EnergyPlus zone-air temperature coefficient snapshot.
///
/// These fields mirror the predictor/corrector coefficient names in
/// `ZoneTempPredictorCorrector.cc`. They are diagnostic state until the full
/// zone-air predictor is wired into the heat-balance timestep.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneAirTemperatureCoefficients {
    /// EnergyPlus `TempDepCoef` in W/K.
    pub temp_dependent_coefficient_w_per_k: f64,
    /// EnergyPlus `TempIndCoef` in W.
    pub temp_independent_coefficient_w: f64,
    /// EnergyPlus `AirPowerCap = C_air / dt` in W/K.
    pub air_power_cap_w_per_k: f64,
    /// EnergyPlus third-order `TempHistoryTerm` in W.
    pub third_order_history_term_w: f64,
    /// EnergyPlus third-order `tempDepLoad` in W/K.
    pub third_order_temp_dependent_load_w_per_k: f64,
    /// EnergyPlus third-order `tempIndLoad` in W.
    pub third_order_temp_independent_load_w: f64,
}

impl ZoneAirTemperatureCoefficients {
    pub(crate) const ZERO: Self = Self {
        temp_dependent_coefficient_w_per_k: 0.0,
        temp_independent_coefficient_w: 0.0,
        air_power_cap_w_per_k: 0.0,
        third_order_history_term_w: 0.0,
        third_order_temp_dependent_load_w_per_k: 0.0,
        third_order_temp_independent_load_w: 0.0,
    };
}

/// One per-slot CTF history contribution captured for a heat-balance timestep.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceCtfHistorySlotSample {
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// One-based CTF history slot index.
    pub slot_index: usize,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// CTF outside/X history coefficient for this slot in W/m2-K.
    pub outside_history_coefficient_w_per_m2_k: f64,
    /// CTF cross/Y history coefficient for this slot in W/m2-K.
    pub cross_history_coefficient_w_per_m2_k: f64,
    /// CTF inside/Z history coefficient for this slot in W/m2-K.
    pub inside_history_coefficient_w_per_m2_k: f64,
    /// CTF flux history coefficient for this slot.
    pub flux_history_coefficient: f64,
    /// Previous outside face temperature in C for this slot.
    pub outside_temperature_history_c: f64,
    /// Previous inside face temperature in C for this slot.
    pub inside_temperature_history_c: f64,
    /// Previous outside conduction flux in W/m2 for this slot.
    pub outside_flux_history_w_per_m2: f64,
    /// Previous inside conduction flux in W/m2 for this slot.
    pub inside_flux_history_w_per_m2: f64,
    /// Inside-face temperature-history contribution in W.
    pub inside_temperature_term_w: f64,
    /// Inside-face flux-history contribution in W.
    pub inside_flux_term_w: f64,
    /// Inside-face total history contribution in W.
    pub inside_total_term_w: f64,
    /// Outside-face temperature-history contribution in reported W sign.
    pub outside_temperature_term_w: f64,
    /// Outside-face flux-history contribution in reported W sign.
    pub outside_flux_term_w: f64,
    /// Outside-face total history contribution in reported W sign.
    pub outside_total_term_w: f64,
}

/// First reported hourly sample CTF history contribution averaged by slot.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceCtfHistorySlotFirstSample {
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// One-based CTF history slot index.
    pub slot_index: usize,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// Number of zone timesteps averaged into the first hourly sample.
    pub timestep_count: usize,
    /// CTF outside/X history coefficient for this slot in W/m2-K.
    pub outside_history_coefficient_w_per_m2_k: f64,
    /// CTF cross/Y history coefficient for this slot in W/m2-K.
    pub cross_history_coefficient_w_per_m2_k: f64,
    /// CTF inside/Z history coefficient for this slot in W/m2-K.
    pub inside_history_coefficient_w_per_m2_k: f64,
    /// CTF flux history coefficient for this slot.
    pub flux_history_coefficient: f64,
    /// Average previous outside face temperature in C for this slot.
    pub outside_temperature_history_c: f64,
    /// Average previous inside face temperature in C for this slot.
    pub inside_temperature_history_c: f64,
    /// Average previous outside conduction flux in W/m2 for this slot.
    pub outside_flux_history_w_per_m2: f64,
    /// Average previous inside conduction flux in W/m2 for this slot.
    pub inside_flux_history_w_per_m2: f64,
    /// Average inside-face temperature-history contribution in W.
    pub inside_temperature_term_w: f64,
    /// Average inside-face flux-history contribution in W.
    pub inside_flux_term_w: f64,
    /// Average inside-face total history contribution in W.
    pub inside_total_term_w: f64,
    /// Average outside-face temperature-history contribution in reported W sign.
    pub outside_temperature_term_w: f64,
    /// Average outside-face flux-history contribution in reported W sign.
    pub outside_flux_term_w: f64,
    /// Average outside-face total history contribution in reported W sign.
    pub outside_total_term_w: f64,
}

/// One reported hourly sample CTF history contribution averaged by slot.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceCtfHistorySlotHourlySample {
    /// Zero-based hourly sample index.
    pub sample_index: usize,
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// One-based CTF history slot index.
    pub slot_index: usize,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// Number of zone timesteps averaged into the hourly sample.
    pub timestep_count: usize,
    /// CTF outside/X history coefficient for this slot in W/m2-K.
    pub outside_history_coefficient_w_per_m2_k: f64,
    /// CTF cross/Y history coefficient for this slot in W/m2-K.
    pub cross_history_coefficient_w_per_m2_k: f64,
    /// CTF inside/Z history coefficient for this slot in W/m2-K.
    pub inside_history_coefficient_w_per_m2_k: f64,
    /// CTF flux history coefficient for this slot.
    pub flux_history_coefficient: f64,
    /// Average previous outside face temperature in C for this slot.
    pub outside_temperature_history_c: f64,
    /// Average previous inside face temperature in C for this slot.
    pub inside_temperature_history_c: f64,
    /// Average previous outside conduction flux in W/m2 for this slot.
    pub outside_flux_history_w_per_m2: f64,
    /// Average previous inside conduction flux in W/m2 for this slot.
    pub inside_flux_history_w_per_m2: f64,
    /// Average inside-face temperature-history contribution in W.
    pub inside_temperature_term_w: f64,
    /// Average inside-face flux-history contribution in W.
    pub inside_flux_term_w: f64,
    /// Average inside-face total history contribution in W.
    pub inside_total_term_w: f64,
    /// Average outside-face temperature-history contribution in reported W sign.
    pub outside_temperature_term_w: f64,
    /// Average outside-face flux-history contribution in reported W sign.
    pub outside_flux_term_w: f64,
    /// Average outside-face total history contribution in reported W sign.
    pub outside_total_term_w: f64,
}

/// One surface-state sample captured after a zone timestep in the first reported hour.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSurfaceFirstSampleTrace {
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// One-based zone timestep within the first reported hourly sample.
    pub timestep_index: u32,
    /// Outdoor dry-bulb temperature used by this timestep in C.
    pub outdoor_dry_bulb_c: f64,
    /// Owning-zone mean air temperature after the timestep in C.
    pub zone_mean_air_temperature_c: f64,
    /// Inside face temperature after the timestep in C.
    pub inside_face_temperature_c: f64,
    /// Inside face temperature used to calculate inside hconv in C.
    pub inside_convection_input_inside_face_temperature_c: f64,
    /// Reference air temperature used to calculate inside hconv in C.
    pub inside_convection_input_reference_air_temperature_c: f64,
    /// Reported outside face temperature after the timestep in C.
    pub outside_face_temperature_c: f64,
    /// Inside-face convection heat gain rate in W.
    pub inside_convection_heat_gain_rate_w: f64,
    /// Inside-face net longwave heat gain rate in W.
    pub inside_net_surface_thermal_radiation_heat_gain_rate_w: f64,
    /// Inside-face conduction heat transfer rate in W.
    pub inside_conduction_rate_w: f64,
    /// Outside-face conduction heat transfer rate in W.
    pub outside_conduction_rate_w: f64,
    /// Surface heat storage rate in W.
    pub heat_storage_rate_w: f64,
    /// Outside-face convection heat gain rate in W.
    pub outside_convection_heat_gain_rate_w: f64,
    /// Outside-face net thermal radiation heat gain rate in W.
    pub outside_net_thermal_radiation_heat_gain_rate_w: f64,
    /// Outside-face solar radiation heat gain rate in W.
    pub outside_solar_radiation_heat_gain_rate_w: f64,
}

/// One inside-surface iteration sample captured after a zone timestep in the first reported hour.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSurfaceIterationFirstSampleTrace {
    /// One-based zone timestep within the first reported hourly sample.
    pub timestep_index: u32,
    /// Number of inside-surface heat-balance iterations executed in this timestep.
    pub inside_surface_iteration_count: u32,
    /// Final max inside-surface temperature change in C.
    pub max_inside_surface_delta_c: f64,
    /// Surface that controlled the final max inside-surface temperature change.
    pub max_delta_surface_name: Option<String>,
}

/// One inside-surface iteration sample captured after a zone timestep.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSurfaceIterationSampleTrace {
    /// Zero-based hourly output sample index.
    pub sample_index: usize,
    /// One-based zone timestep within the hourly sample.
    pub timestep_index: u32,
    /// Number of inside-surface heat-balance iterations executed in this timestep.
    pub inside_surface_iteration_count: u32,
    /// Final max inside-surface temperature change in C.
    pub max_inside_surface_delta_c: f64,
    /// Surface that controlled the final max inside-surface temperature change.
    pub max_delta_surface_name: Option<String>,
}

/// Per-zone zone-air state captured at a diagnostic boundary.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceZoneAirStateSample {
    /// Zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// Current mean air temperature in C.
    pub mean_air_temperature_c: f64,
    /// Last zone-timestep average mean air temperature in C.
    pub zone_timestep_average_air_temperature_c: f64,
    /// Previous zone-timestep mean-air-temperature history in C.
    pub previous_mean_air_temperatures_c: [f64; 3],
    /// Previous system-timestep mean-air-temperature history in C.
    pub previous_system_mean_air_temperatures_c: [f64; 3],
    /// Adaptive system timestep count used in the previous zone timestep.
    pub previous_system_timestep_count: u32,
    /// Current zone air humidity ratio in kgWater/kgDryAir.
    pub air_humidity_ratio: f64,
    /// Last zone-timestep average humidity ratio in kgWater/kgDryAir.
    pub zone_timestep_average_air_humidity_ratio: f64,
    /// Previous zone-timestep humidity-ratio history in kgWater/kgDryAir.
    pub previous_air_humidity_ratios: [f64; 3],
    /// Previous system-timestep humidity-ratio history in kgWater/kgDryAir.
    pub previous_system_air_humidity_ratios: [f64; 3],
    /// Zone air heat capacity in J/K.
    pub air_heat_capacity_j_per_k: f64,
    /// Latest zone-air coefficient snapshot.
    pub zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients,
}

/// Per-zone zone-air state captured at the end of one warmup day.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceWarmupDayEndZoneAirStateSample {
    /// One-based warmup day index.
    pub day_index: u32,
    /// Per-zone state at the end of the warmup day.
    pub state: HeatBalanceZoneAirStateSample,
}

/// Per-zone zone-air state captured for one timestep in the first reported hour.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceZoneAirFirstSampleTrace {
    /// Zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// One-based zone timestep within the first reported hourly sample.
    pub timestep_index: u32,
    /// Interpolated outdoor dry-bulb temperature in C used for the timestep.
    pub outdoor_dry_bulb_c: f64,
    /// Zone-timestep length in seconds.
    pub timestep_seconds: f64,
    /// Current mean air temperature in C.
    pub mean_air_temperature_c: f64,
    /// Zone-timestep average mean air temperature in C.
    pub zone_timestep_average_air_temperature_c: f64,
    /// Previous zone-timestep mean-air-temperature history in C.
    pub previous_mean_air_temperatures_c: [f64; 3],
    /// Previous system-timestep mean-air-temperature history in C.
    pub previous_system_mean_air_temperatures_c: [f64; 3],
    /// Adaptive system timestep count used in the previous zone timestep.
    pub previous_system_timestep_count: u32,
    /// Current zone air humidity ratio in kgWater/kgDryAir.
    pub air_humidity_ratio: f64,
    /// Zone-timestep average humidity ratio in kgWater/kgDryAir.
    pub zone_timestep_average_air_humidity_ratio: f64,
    /// Zone air heat capacity in J/K.
    pub air_heat_capacity_j_per_k: f64,
    /// Zone air power capacity recomputed from the active zone timestep.
    pub zone_timestep_air_power_cap_w_per_k: f64,
    /// Latest zone-air coefficient snapshot.
    pub zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients,
    /// Third-order temperature solution numerator in W.
    pub third_order_solution_numerator_w: f64,
    /// Third-order temperature solution denominator in W/K.
    pub third_order_solution_denominator_w_per_k: f64,
    /// Third-order temperature solution in C from the stored coefficients.
    pub third_order_solution_temperature_c: f64,
}

/// Inputs for advancing the first heat-balance timestep shell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceStepInput {
    /// Outdoor dry-bulb temperature in C for this timestep.
    pub outdoor_dry_bulb_c: f64,
    /// EnergyPlus-style hour ending, 1-24.
    pub hour_ending: u32,
    /// Timestep duration in seconds.
    pub timestep_seconds: f64,
}

/// Initial CTF temperature/flux history seeding used by diagnostic heat-balance traces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceCtfInitialHistoryPolicy {
    /// Existing Rust diagnostic seed: current boundary temperature and steady U-value flux.
    BoundaryTemperatureAndUValue,
    /// EnergyPlus 26.1 style InitHeatBalance seed: SurfInitialTemp inside
    /// histories, boundary outside histories, and steady U-value flux histories.
    EnergyPlusSurfInitial,
}

/// Source used for zone opaque conduction report variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceZoneConductionReportSource {
    /// Use the zone heat-balance state values captured during correction.
    ZoneState,
    /// Sum the same per-surface report rates used by surface conduction outputs.
    SurfaceReport,
}

/// Sampling mode used for zone air heat-balance report variables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceZoneAirReportSampling {
    /// Average reported values over the zone timesteps in each hour.
    Average,
    /// Report the last system state in each hour for source-order probes.
    LastSystemState,
}

/// Timing for zone-air correction during interleaved surface-balance probes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceSurfaceLoopZoneAirCorrection {
    /// Correct zone air after every surface loop pass.
    EachSurfaceIteration,
    /// Correct zone air once after the inside surface loop converges.
    AfterSurfaceLoop,
}

/// Options for the heat-balance zone-air diagnostic trace.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceSimulationOptions {
    /// Number of hourly weather samples to execute.
    pub sample_count: usize,
    /// Initial zone mean air temperature in C.
    pub initial_zone_air_temperature_c: f64,
    /// Optional run-period warmup loop.
    pub warmup: HeatBalanceWarmupOptions,
    /// Zone-air temperature algorithm for diagnostic probes.
    pub zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    /// Number of inside/outside surface-balance passes per zone timestep.
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
}

/// Warmup settings for heat-balance diagnostic traces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceWarmupOptions {
    /// Whether to execute a warmup loop before reported samples are recorded.
    pub enabled: bool,
    /// Minimum number of repeated warmup days.
    pub minimum_days: u32,
    /// Maximum number of repeated warmup days.
    pub maximum_days: u32,
    /// Zone end-state convergence tolerance in delta C.
    pub temperature_convergence_tolerance_delta_c: f64,
}

impl HeatBalanceWarmupOptions {
    /// Creates disabled warmup settings.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            minimum_days: 0,
            maximum_days: 0,
            temperature_convergence_tolerance_delta_c: 0.0,
        }
    }
}

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

impl HeatBalanceSimulationOptions {
    /// Creates options with a fixed hourly sample count.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            initial_zone_air_temperature_c: ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            warmup: HeatBalanceWarmupOptions::disabled(),
            zone_air_algorithm: HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            surface_iteration_count: 1,
            inside_hconv_reevaluation_interval: None,
            ctf_initial_history_policy:
                HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue,
            zone_conduction_report_source: HeatBalanceZoneConductionReportSource::ZoneState,
            zone_air_report_sampling: HeatBalanceZoneAirReportSampling::Average,
            surface_loop_zone_air_correction:
                HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
        }
    }

    /// Creates options with a run-period warmup loop based on typed Building settings.
    #[must_use]
    pub fn hourly_samples_with_model_warmup(model: &SimulationModel, sample_count: usize) -> Self {
        let Some(building) = model.typed.building.as_ref() else {
            return Self::hourly_samples(sample_count);
        };
        let minimum_days = building.minimum_number_of_warmup_days;
        let maximum_days = building.maximum_number_of_warmup_days.max(minimum_days);
        Self {
            sample_count,
            initial_zone_air_temperature_c: ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            warmup: HeatBalanceWarmupOptions {
                enabled: maximum_days > 0,
                minimum_days,
                maximum_days,
                temperature_convergence_tolerance_delta_c: building
                    .temperature_convergence_tolerance_delta_c,
            },
            zone_air_algorithm: HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            surface_iteration_count: 1,
            inside_hconv_reevaluation_interval: None,
            ctf_initial_history_policy:
                HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue,
            zone_conduction_report_source: HeatBalanceZoneConductionReportSource::ZoneState,
            zone_air_report_sampling: HeatBalanceZoneAirReportSampling::Average,
            surface_loop_zone_air_correction:
                HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
        }
    }

    /// Returns options with an explicit zone-air diagnostic algorithm.
    #[must_use]
    pub const fn with_zone_air_algorithm(
        mut self,
        zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    ) -> Self {
        self.zone_air_algorithm = zone_air_algorithm;
        self
    }

    /// Returns options with an elevated warmup minimum day count for diagnostics.
    #[must_use]
    pub fn with_warmup_minimum_days(mut self, minimum_days: u32) -> Self {
        if self.warmup.enabled {
            self.warmup.minimum_days = minimum_days;
            self.warmup.maximum_days = self.warmup.maximum_days.max(minimum_days);
        }
        self
    }

    /// Returns options with an explicit surface-balance iteration count.
    #[must_use]
    pub const fn with_surface_iteration_count(mut self, surface_iteration_count: u32) -> Self {
        self.surface_iteration_count = if surface_iteration_count == 0 {
            1
        } else {
            surface_iteration_count
        };
        self
    }

    /// Returns options with a frozen inside-convection coefficient re-evaluation interval.
    #[must_use]
    pub const fn with_inside_hconv_reevaluation_interval(
        mut self,
        inside_hconv_reevaluation_interval: Option<u32>,
    ) -> Self {
        self.inside_hconv_reevaluation_interval = match inside_hconv_reevaluation_interval {
            Some(0) => None,
            interval => interval,
        };
        self
    }

    /// Returns options with an explicit initial CTF history seed policy.
    #[must_use]
    pub const fn with_ctf_initial_history_policy(
        mut self,
        ctf_initial_history_policy: HeatBalanceCtfInitialHistoryPolicy,
    ) -> Self {
        self.ctf_initial_history_policy = ctf_initial_history_policy;
        self
    }

    /// Returns options with an explicit zone opaque conduction report source.
    #[must_use]
    pub const fn with_zone_conduction_report_source(
        mut self,
        zone_conduction_report_source: HeatBalanceZoneConductionReportSource,
    ) -> Self {
        self.zone_conduction_report_source = zone_conduction_report_source;
        self
    }

    /// Returns options with an explicit zone air heat-balance report sampling mode.
    #[must_use]
    pub const fn with_zone_air_report_sampling(
        mut self,
        zone_air_report_sampling: HeatBalanceZoneAirReportSampling,
    ) -> Self {
        self.zone_air_report_sampling = zone_air_report_sampling;
        self
    }

    /// Returns options with explicit zone-air correction timing in the surface loop.
    #[must_use]
    pub const fn with_surface_loop_zone_air_correction(
        mut self,
        surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
    ) -> Self {
        self.surface_loop_zone_air_correction = surface_loop_zone_air_correction;
        self
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
