//! Fixed-timestep direct-Zone heat-balance/PurchasedAir release runtime.

use std::fmt::{Display, Formatter};

use ep_model::{AutosizeOrNumber, IdealLoadsLimit, SimulationModel};

use crate::error::RuntimeError;
use crate::heat_balance::air_manager::seed_zone_air_humidity_ratios_from_weather_series;
use crate::heat_balance::algorithm::{
    HeatBalanceRuntimeConfig, direct_zone_purchased_air_fixed_step_runtime_config,
};
use crate::heat_balance::initialization::initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache_profiled;
use crate::heat_balance::manager::init_heat_balance_source_order_path;
use crate::heat_balance::reports::{
    HeatBalanceResultSeriesTraces, heat_balance_result_store_from_traces,
};
use crate::heat_balance::run_period::sample_heat_balance_run_period_with_step_driver;
use crate::heat_balance::state::{
    HeatBalanceCtfInitialHistoryPolicy, HeatBalanceSimulationOptions, HeatBalanceState,
};
use crate::heat_balance::surface_boundary::{
    seed_energyplus_initial_surface_ctf_histories, seed_initial_surface_ctf_boundary_histories,
};
use crate::heat_balance::timestep::advance_heat_balance_state_one_timestep_with_direct_zone_purchased_air;
use crate::heat_balance::trace::HeatBalanceRunPeriodSamples;
use crate::schedules::{
    HeatBalanceInternalGainScheduleOperationProfile, ScheduleSeriesCache,
    precompute_hour_only_internal_gain_schedule_cache_profiled,
};
use crate::time_axis::run_period_first_hour_interpolation_starting_values;
use crate::weather::WeatherTimestepSeries;
use crate::{ResultStore, ZoneSensibleDemandInputKind};

use super::{
    DirectZonePurchasedAirBindingError, DirectZonePurchasedAirHourlyOutputError,
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirRuntimeStepError,
    IdealLoadsPurchasedAirBranch, PURCHASED_AIR_CALC_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER, PurchasedAirAvailabilityStatus,
    PurchasedAirCalcEntryError, PurchasedAirCalcEntryLifecycleSummary,
    PurchasedAirCalcEntrySnapshot, PurchasedAirCalcMinimumOaPrefixError,
    PurchasedAirCalcMinimumOaPrefixLifecycleSummary, PurchasedAirHardSizeLegacyRoute,
    PurchasedAirInitError, PurchasedAirInitLifecycleSummary, PurchasedAirRecirculationSource,
    PurchasedAirRuntimeState, PurchasedAirSizedLimits,
    append_direct_zone_purchased_air_hourly_output_series, bind_direct_zone_purchased_air_model,
    purchased_air_calc_entry_lifecycle_summary,
    purchased_air_calc_minimum_oa_prefix_lifecycle_summary, purchased_air_init_lifecycle_summary,
};

mod minimum_oa_validation;

const SECONDS_PER_HOUR: f64 = 3_600.0;

/// Stable release-loop demand provenance for the bounded direct-Zone runtime.
pub const DIRECT_ZONE_PURCHASED_AIR_DEMAND_SOURCE: &str =
    "rust-predictor-source-setpoint-thresholds";

/// Stable recirculation-state provenance for the source-valid single-return subset.
pub const DIRECT_ZONE_PURCHASED_AIR_RECIRCULATION_SOURCE: &str =
    "rust-direct-zone-return-projection";

/// Actual coupled source-order stages executed once per nominal system step.
pub const DIRECT_ZONE_PURCHASED_AIR_COUPLED_SOURCE_ORDER: &[&str] = &[
    "predict-system-loads",
    "init-purchased-air",
    "calc-purch-air-loads",
    "update-purchased-air",
    "report-purchased-air",
    "correct-zone-air-temps",
];

/// Options for the bounded fixed-timestep direct-Zone coupled runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectZonePurchasedAirCoupledOptions {
    /// Number of hourly run-period samples to report.
    pub sample_count: usize,
    /// Initial Zone mean air temperature.
    pub initial_zone_air_temperature_c: f64,
}

impl DirectZonePurchasedAirCoupledOptions {
    /// Creates fixed-timestep options for an hourly result prefix.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            initial_zone_air_temperature_c: 23.0,
        }
    }
}

/// Summary of one bounded coupled heat-balance/PurchasedAir run.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectZonePurchasedAirCoupledSummary {
    /// Number of hourly samples reported.
    pub samples: usize,
    /// Number of nominal system/Zone timesteps executed.
    pub timestep_count: usize,
    /// Number of Zone timesteps per reporting hour.
    pub zone_timesteps_per_hour: u32,
    /// Fixed nominal timestep duration.
    pub timestep_seconds: f64,
    /// Number of successful CP301 calls.
    pub coupling_call_count: usize,
    /// Bound IdealLoads system name.
    pub system_name: String,
    /// Bound supply-node name.
    pub supply_node_name: String,
    /// Bound Zone return-node name used by blank-exhaust PurchasedAir.
    pub return_node_name: String,
    /// PurchasedAir branch enforced by the binding.
    pub branch: IdealLoadsPurchasedAirBranch,
    /// Zone-demand provenance used by every call.
    pub zone_demand_source: &'static str,
    /// Whether the oracle/default active-split constructor was used.
    pub fixture_demand_injection_used: bool,
    /// Provenance of the state projected onto the bound direct return node.
    pub recirculation_state_source: &'static str,
    /// Actual nested predictor/HVAC/corrector order.
    pub actual_coupled_source_order: &'static [&'static str],
    /// Persistent bounded `InitPurchasedAir` lifecycle report.
    pub init_lifecycle: PurchasedAirInitLifecycleSummary,
    /// Persistent bounded `CalcPurchAirLoads` entry-prefix lifecycle report.
    pub calc_entry_lifecycle: PurchasedAirCalcEntryLifecycleSummary,
    /// Persistent bounded minimum-outdoor-air prefix lifecycle report.
    pub calc_minimum_oa_prefix_lifecycle: PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
}

/// Result of the bounded coupled release runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectZonePurchasedAirCoupledSimulation {
    /// Final heat-balance state after reported run-period samples.
    pub state: HeatBalanceState,
    /// Combined heat-balance and PurchasedAir result series.
    pub results: ResultStore,
    /// Bounded runtime summary and provenance.
    pub summary: DirectZonePurchasedAirCoupledSummary,
    /// Deterministic internal-gain schedule operation counts.
    pub internal_gain_schedule_cache_profile: HeatBalanceInternalGainScheduleOperationProfile,
}

/// Fail-closed error from the bounded coupled release runtime.
#[derive(Debug, PartialEq)]
pub enum DirectZonePurchasedAirCoupledRuntimeError {
    /// Static CP301 topology/model binding failed.
    Binding(DirectZonePurchasedAirBindingError),
    /// Heat-balance initialization or weather input failed.
    HeatBalance(RuntimeError),
    /// A release run with no system timestep cannot execute initialization.
    NoTimestepsRequested,
    /// The active zone-timestep cache cannot cover the requested prefix.
    ScheduleCacheCoverage {
        /// Required zone-timestep samples.
        required: usize,
        /// Available cache samples.
        available: usize,
    },
    /// Requested hourly and Zone-timestep counts overflowed `usize`.
    TimestepCountOverflow,
    /// One live predictor-bound CP301 call failed.
    RuntimeStep(DirectZonePurchasedAirRuntimeStepError),
    /// Final lifecycle summary could not resolve the bound unit.
    InitLifecycle(PurchasedAirInitError),
    /// Final Calc-entry lifecycle summary could not resolve the bound unit.
    CalcEntryLifecycle(PurchasedAirCalcEntryError),
    /// Final minimum-outdoor-air prefix summary could not resolve the bound unit.
    CalcMinimumOaPrefixLifecycle(PurchasedAirCalcMinimumOaPrefixError),
    /// A lifecycle transition count did not match the single-environment run.
    InitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Calc-entry lifecycle transition did not match the executed run.
    CalcEntryLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A minimum-outdoor-air prefix lifecycle invariant did not match the run.
    CalcMinimumOaPrefixLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Calc call did not retain the exact persistent initialization flags.
    UnexpectedInitializationFlags {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Calc-entry prefix snapshot did not match its bound release call.
    UnexpectedCalculationEntry {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A minimum-outdoor-air prefix snapshot did not match its bound release call.
    UnexpectedCalculationMinimumOutdoorAir {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A successful CP301 call did not retain source-setpoint demand provenance.
    UnexpectedDemandInputKind {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
        /// Unexpected demand kind.
        actual: ZoneSensibleDemandInputKind,
    },
    /// A timestep dispatched a PurchasedAir branch different from the immutable binding.
    UnexpectedPurchasedAirBranch {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
        /// Branch retained by the immutable binding.
        expected: IdealLoadsPurchasedAirBranch,
        /// Branch returned by the generic PurchasedAir wrapper.
        actual: IdealLoadsPurchasedAirBranch,
    },
    /// Hourly PurchasedAir aggregation rejected the collected outputs.
    HourlyOutput(DirectZonePurchasedAirHourlyOutputError),
}

impl Display for DirectZonePurchasedAirCoupledRuntimeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binding(error) => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir binding failed: {error:?}"
                )
            }
            Self::HeatBalance(error) => Display::fmt(error, formatter),
            Self::NoTimestepsRequested => write!(
                formatter,
                "direct-Zone PurchasedAir requires at least one system timestep"
            ),
            Self::ScheduleCacheCoverage {
                required,
                available,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir schedule cache requires {required} zone-timestep samples but contains {available}"
            ),
            Self::TimestepCountOverflow => write!(
                formatter,
                "direct-Zone PurchasedAir requested timestep count overflowed usize"
            ),
            Self::RuntimeStep(error) => write!(
                formatter,
                "direct-Zone PurchasedAir predictor step failed: {error:?}"
            ),
            Self::InitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir lifecycle summary failed: {error:?}"
            ),
            Self::CalcEntryLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Calc-entry lifecycle summary failed: {error:?}"
            ),
            Self::CalcMinimumOaPrefixLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir minimum-OA prefix lifecycle summary failed: {error:?}"
            ),
            Self::InitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcEntryLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Calc-entry lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcMinimumOaPrefixLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir minimum-OA prefix lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::UnexpectedInitializationFlags { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not consume its persistent initialization flags"
            ),
            Self::UnexpectedCalculationEntry { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its bound Calc-entry prefix"
            ),
            Self::UnexpectedCalculationMinimumOutdoorAir { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its minimum-OA prefix"
            ),
            Self::UnexpectedDemandInputKind {
                timestep_index,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} produced unexpected demand input kind {actual:?}"
            ),
            Self::UnexpectedPurchasedAirBranch {
                timestep_index,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} dispatched branch {actual:?}, expected bound branch {expected:?}"
            ),
            Self::HourlyOutput(error) => write!(
                formatter,
                "direct-Zone PurchasedAir hourly output aggregation failed: {error:?}"
            ),
        }
    }
}

impl std::error::Error for DirectZonePurchasedAirCoupledRuntimeError {}

/// Executes the exact one-Zone/no-OA sensible subset through a shared fixed
/// ThirdOrder heat-balance and PurchasedAir loop.
///
/// The caller must supply the zone-timestep schedule cache built from the same
/// `SimulationModel` and active environment axis. Binding is performed once;
/// CP301 is then called exactly once inside each `PredictSystemLoads` step, and
/// the existing corrector consumes the committed `SumSysMCp`/`SumSysMCpT` in
/// that same timestep.
pub fn simulate_direct_zone_purchased_air_coupled_heat_balance(
    model: &SimulationModel,
    weather_series: &WeatherTimestepSeries,
    coupling_schedule_cache: &ScheduleSeriesCache,
    options: DirectZonePurchasedAirCoupledOptions,
) -> Result<DirectZonePurchasedAirCoupledSimulation, DirectZonePurchasedAirCoupledRuntimeError> {
    let weather_dry_bulb_c = weather_series.hourly_dry_bulb_c();
    if weather_dry_bulb_c.is_empty() {
        return Err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance(
            RuntimeError::NoWeatherData,
        ));
    }
    if options.sample_count == 0 {
        return Err(DirectZonePurchasedAirCoupledRuntimeError::NoTimestepsRequested);
    }
    if options.sample_count > weather_dry_bulb_c.len() {
        return Err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance(
            RuntimeError::SampleCountExceedsWeather {
                requested: options.sample_count,
                available: weather_dry_bulb_c.len(),
            },
        ));
    }
    if model.typed.zones.is_empty() {
        return Err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance(
            RuntimeError::NoZones,
        ));
    }

    let binding = bind_direct_zone_purchased_air_model(model)
        .map_err(DirectZonePurchasedAirCoupledRuntimeError::Binding)?;
    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour;
    let required_timestep_count = options
        .sample_count
        .checked_mul(zone_steps_per_hour as usize)
        .ok_or(DirectZonePurchasedAirCoupledRuntimeError::TimestepCountOverflow)?;
    if coupling_schedule_cache.sample_count() < required_timestep_count {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::ScheduleCacheCoverage {
                required: required_timestep_count,
                available: coupling_schedule_cache.sample_count(),
            },
        );
    }
    let seconds_per_timestep = SECONDS_PER_HOUR / f64::from(zone_steps_per_hour);
    let first_hour_interpolation_starting_values =
        run_period_first_hour_interpolation_starting_values(&model.typed);
    let runtime_config = direct_zone_purchased_air_fixed_step_runtime_config();
    validate_fixed_runtime_config(runtime_config);

    let heat_balance_options = HeatBalanceSimulationOptions {
        sample_count: options.sample_count,
        initial_zone_air_temperature_c: options.initial_zone_air_temperature_c,
        ..HeatBalanceSimulationOptions::hourly_samples(options.sample_count)
    };
    let (mut state, internal_gain_schedule_cache, mut internal_gain_schedule_cache_profile) =
        init_heat_balance_source_order_path(|| {
            let (schedule_cache, mut schedule_cache_profile) =
                precompute_hour_only_internal_gain_schedule_cache_profiled(&model.typed)
                    .map_err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance)?;
            let mut state =
                initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache_profiled(
                    model,
                    options.initial_zone_air_temperature_c,
                    &[],
                    &schedule_cache,
                    &mut schedule_cache_profile,
                )
                .map_err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance)?;
            seed_zone_air_humidity_ratios_from_weather_series(
                &mut state,
                Some(weather_series),
                weather_dry_bulb_c[0],
                zone_steps_per_hour,
                first_hour_interpolation_starting_values,
            );
            match heat_balance_options.ctf_initial_history_policy {
                HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue => {
                    seed_initial_surface_ctf_boundary_histories(&mut state, weather_dry_bulb_c[0]);
                }
                HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial => {
                    seed_energyplus_initial_surface_ctf_histories(
                        &mut state,
                        options.initial_zone_air_temperature_c,
                        weather_dry_bulb_c[0],
                    );
                }
            }
            Ok::<_, DirectZonePurchasedAirCoupledRuntimeError>((
                state,
                schedule_cache,
                schedule_cache_profile,
            ))
        })?;
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();

    let (samples, timestep_outputs) = sample_heat_balance_run_period_with_step_driver(
        model,
        &mut state,
        weather_dry_bulb_c,
        Some(weather_series.hourly_records()),
        Some(weather_series),
        heat_balance_options,
        runtime_config,
        zone_steps_per_hour,
        seconds_per_timestep,
        first_hour_interpolation_starting_values,
        |state, input, weather_context, hour_index, substep| {
            let sample_index =
                hour_index * zone_steps_per_hour as usize + (substep.saturating_sub(1) as usize);
            advance_heat_balance_state_one_timestep_with_direct_zone_purchased_air(
                &model.typed,
                &internal_gain_schedule_cache,
                &mut internal_gain_schedule_cache_profile.run_period,
                state,
                input,
                weather_context,
                runtime_config,
                heat_balance_options.surface_iteration_count,
                heat_balance_options.inside_hconv_reevaluation_interval,
                heat_balance_options.surface_loop_zone_air_correction,
                &binding,
                &mut purchased_air_runtime_state,
                sample_index == 0,
                coupling_schedule_cache,
                sample_index,
            )
            .map_err(DirectZonePurchasedAirCoupledRuntimeError::RuntimeStep)
        },
    )?;

    for (timestep_index, output) in timestep_outputs.iter().enumerate() {
        if !calc_entry_snapshot_matches_release(output, timestep_index + 1, &binding) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedCalculationEntry {
                    timestep_index,
                },
            );
        }
        if !minimum_oa_validation::snapshot_matches_release(output, timestep_index + 1, &binding) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedCalculationMinimumOutdoorAir {
                    timestep_index,
                },
            );
        }
        if !output.initialization.flags.state_machine_used
            || output.coupling.purchased_air.init_flags != output.initialization.flags
        {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedInitializationFlags {
                    timestep_index,
                },
            );
        }
        let actual_branch = output.coupling.purchased_air.branch;
        if actual_branch != binding.branch {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedPurchasedAirBranch {
                    timestep_index,
                    expected: binding.branch,
                    actual: actual_branch,
                },
            );
        }
        let actual = output
            .coupling
            .purchased_air
            .trace
            .demand
            .sensible_input_kind;
        if actual != ZoneSensibleDemandInputKind::SourceSetpointThresholds {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedDemandInputKind {
                    timestep_index,
                    actual,
                },
            );
        }
    }
    let init_lifecycle = purchased_air_init_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .map_err(DirectZonePurchasedAirCoupledRuntimeError::InitLifecycle)?;
    validate_init_lifecycle(&init_lifecycle, timestep_outputs.len(), &binding)?;
    let calc_entry_lifecycle = purchased_air_calc_entry_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycle)?;
    let latest_output = timestep_outputs.last().ok_or(
        DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycleInvariant {
            field: "latest_output_present",
            expected: 1,
            actual: 0,
        },
    )?;
    validate_calc_entry_lifecycle(
        &calc_entry_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_minimum_oa_prefix_lifecycle = purchased_air_calc_minimum_oa_prefix_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcMinimumOaPrefixLifecycle)?;
    minimum_oa_validation::validate_lifecycle(
        &calc_minimum_oa_prefix_lifecycle,
        &calc_entry_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;

    let HeatBalanceRunPeriodSamples {
        zone_temperatures,
        zone_humidity_ratios,
        zone_conduction_rates,
        inside_surface_iteration_counts,
        zone_air_heat_balance_rates,
        zone_air_debug_traces,
        surface_temperatures,
        outdoor_temperatures,
        outdoor_wet_bulb_temperatures,
        sky_temperatures,
        horizontal_infrared_radiation_rates,
        rain_statuses,
        ..
    } = samples;
    let mut results = heat_balance_result_store_from_traces(HeatBalanceResultSeriesTraces {
        zone_temperatures,
        zone_humidity_ratios,
        zone_conduction_rates,
        inside_surface_iteration_counts,
        zone_air_heat_balance_rates,
        zone_air_debug_traces,
        surface_temperatures,
        outdoor_temperatures,
        outdoor_wet_bulb_temperatures,
        sky_temperatures,
        horizontal_infrared_radiation_rates,
        rain_statuses,
    });
    let supply_node_name = node_name(model, binding.supply_node);
    let return_node_name = node_name(model, binding.return_node);
    let zone_name = zone_name(model, binding.zone);
    append_direct_zone_purchased_air_hourly_output_series(
        &mut results,
        binding.system,
        &zone_name,
        binding.supply_node,
        &supply_node_name,
        binding.limit_context,
        &timestep_outputs,
        zone_steps_per_hour,
        seconds_per_timestep,
    )
    .map_err(DirectZonePurchasedAirCoupledRuntimeError::HourlyOutput)?;

    Ok(DirectZonePurchasedAirCoupledSimulation {
        summary: DirectZonePurchasedAirCoupledSummary {
            samples: options.sample_count,
            timestep_count: state.timestep_index,
            zone_timesteps_per_hour: zone_steps_per_hour,
            timestep_seconds: seconds_per_timestep,
            coupling_call_count: timestep_outputs.len(),
            system_name: binding.system.name.0.clone(),
            supply_node_name,
            return_node_name,
            branch: binding.branch,
            zone_demand_source: DIRECT_ZONE_PURCHASED_AIR_DEMAND_SOURCE,
            fixture_demand_injection_used: false,
            recirculation_state_source: DIRECT_ZONE_PURCHASED_AIR_RECIRCULATION_SOURCE,
            actual_coupled_source_order: DIRECT_ZONE_PURCHASED_AIR_COUPLED_SOURCE_ORDER,
            init_lifecycle,
            calc_entry_lifecycle,
            calc_minimum_oa_prefix_lifecycle,
        },
        state,
        results,
        internal_gain_schedule_cache_profile,
    })
}

fn validate_calc_entry_lifecycle(
    lifecycle: &PurchasedAirCalcEntryLifecycleSummary,
    timestep_count: usize,
    latest_output: &super::DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), DirectZonePurchasedAirCoupledRuntimeError> {
    let state = &lifecycle.state;
    for (field, expected, actual) in [
        ("call_count", timestep_count, state.call_count),
        ("reset_count", timestep_count, state.reset_count),
        ("demand_read_count", timestep_count, state.demand_read_count),
        (
            "overall_availability_read_count",
            timestep_count,
            state.overall_availability_read_count,
        ),
        (
            "heating_availability_read_count",
            timestep_count,
            state.heating_availability_read_count,
        ),
        (
            "cooling_availability_read_count",
            timestep_count,
            state.cooling_availability_read_count,
        ),
        (
            "availability_manager_read_count",
            timestep_count,
            state.availability_manager_read_count,
        ),
        (
            "availability_manager_zone_write_count",
            timestep_count,
            state.availability_manager_zone_write_count,
        ),
        (
            "availability_status_copy_count",
            timestep_count,
            state.availability_status_copy_count,
        ),
        ("force_off_count", 0, state.force_off_count),
        ("heating_on_count", timestep_count, state.heating_on_count),
        ("cooling_on_count", timestep_count, state.cooling_on_count),
        (
            "unit_on_off_partition",
            timestep_count,
            state.unit_body_entry_count + state.unit_off_count,
        ),
        (
            "overall_gate_partition",
            timestep_count,
            state.unit_body_entry_count + state.overall_schedule_off_count,
        ),
    ] {
        if actual != expected {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycleInvariant {
                    field,
                    expected,
                    actual,
                },
            );
        }
    }
    let latest = state.latest.as_ref().ok_or(
        DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycleInvariant {
            field: "latest_snapshot_present",
            expected: 1,
            actual: 0,
        },
    )?;
    let ready = lifecycle.source == PURCHASED_AIR_CALC_ENTRY_SOURCE
        && state.system == binding.ideal_loads_air_system
        && state.availability_manager_zone == Some(binding.zone)
        && state.availability_status == PurchasedAirAvailabilityStatus::NoAction
        && state.minimum_outdoor_air_mass_flow_rate_kg_per_s == 0.0
        && state.economizer_active_time_hours == 0.0
        && state.heat_recovery_active_time_hours == 0.0
        && latest == &latest_output.calculation_entry;
    if !ready {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycleInvariant {
                field: "latest_release_snapshot_ready",
                expected: 1,
                actual: 0,
            },
        );
    }
    Ok(())
}

fn calc_entry_snapshot_matches_release(
    output: &super::DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let entry: PurchasedAirCalcEntrySnapshot = output.calculation_entry;
    let demand = output.coupling.prediction.zone_demand;
    entry.source == PURCHASED_AIR_CALC_ENTRY_SOURCE
        && entry.source_order == PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER
        && entry.system == binding.ideal_loads_air_system
        && entry.call_ordinal == call_ordinal
        && entry.controlled_zone == binding.zone
        && entry.supply_node == binding.supply_node
        && entry.zone_node == binding.zone_air_node
        && entry.outdoor_air_node.is_none()
        && entry.recirculation_node == binding.return_node
        && entry.reset.all_zero()
        && entry.demand.zone == demand.zone
        && entry.demand.sensible_input_kind == demand.sensible_input_kind
        && entry.demand.remaining_output_req_to_heat_sp_w
            == demand.remaining_output_req_to_heat_sp_w
        && entry.demand.remaining_output_req_to_cool_sp_w
            == demand.remaining_output_req_to_cool_sp_w
        && entry.unit_defaulted_on
        && !entry.economizer_defaulted_on
        && entry.availability_manager_read_site_visited
        && entry.availability_manager_zone_written
        && entry.copied_availability_status == Some(PurchasedAirAvailabilityStatus::NoAction)
        && !entry.force_off_applied
        && entry.overall_availability_read_site_visited
        && entry.heating_availability_read_site_visited
        && entry.cooling_availability_read_site_visited
        && entry.overall_availability == output.schedules.overall_availability
        && entry.heating_availability == 1.0
        && entry.cooling_availability == 1.0
        && entry.unit_on == output.schedules.unit_available
        && entry.heating_on
        && entry.cooling_on
        && entry.unit_body_entered == entry.unit_on
}

fn validate_init_lifecycle(
    lifecycle: &PurchasedAirInitLifecycleSummary,
    timestep_count: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), DirectZonePurchasedAirCoupledRuntimeError> {
    for (field, expected, actual) in [
        ("init_call_count", timestep_count, lifecycle.init_call_count),
        (
            "module_initialization_count",
            1,
            lifecycle.module_initialization_count,
        ),
        (
            "equipment_list_check_count",
            1,
            lifecycle.equipment_list_check_count,
        ),
        (
            "declared_system_count",
            1,
            lifecycle.declared_system_order.len(),
        ),
        (
            "equipment_list_scanned_unit_count",
            1,
            lifecycle.equipment_list_scanned_unit_count,
        ),
        (
            "equipment_list_missing_unit_count",
            0,
            lifecycle.equipment_list_missing_unit_count,
        ),
        (
            "equipment_list_diagnostic_count",
            0,
            lifecycle.equipment_list_diagnostics.len(),
        ),
        (
            "one_time_initialization_count",
            1,
            lifecycle.one_time_initialization_count,
        ),
        (
            "topology_completion_count",
            1,
            lifecycle.topology_completion_count,
        ),
        ("sizing_attempt_count", 1, lifecycle.sizing_attempt_count),
        ("sizing_check_count", 1, lifecycle.sizing_check_count),
        (
            "environment_initialization_count",
            1,
            lifecycle.environment_initialization_count,
        ),
        (
            "environment_rearm_count",
            usize::from(timestep_count > 1),
            lifecycle.environment_rearm_count,
        ),
    ] {
        if actual != expected {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::InitLifecycleInvariant {
                    field,
                    expected,
                    actual,
                },
            );
        }
    }
    let flags = lifecycle.flags;
    let ready = flags.state_machine_used
        && flags.one_time_checked
        && flags.topology_ready
        && flags.environment_initialized
        && flags.sizing_checked
        && flags.equipment_list_checked
        && flags.return_plenum_inactive
        && lifecycle.equipment_list_scan_order == lifecycle.declared_system_order
        && lifecycle.declared_system_order == vec![binding.system.id]
        && lifecycle.equipment_list_scan_ordinal == Some(1)
        && lifecycle.first_matching_equipment_list == Some(binding.equipment_list)
        && lifecycle.equipment_list_membership_found == Some(true)
        && lifecycle.controlled_zone == Some(binding.zone)
        && lifecycle.equipment_list == Some(binding.equipment_list)
        && lifecycle.supply_node == Some(binding.supply_node)
        && lifecycle.recirculation_node == Some(binding.return_node)
        && lifecycle.recirculation_source
            == Some(PurchasedAirRecirculationSource::SingleZoneReturn)
        && lifecycle.rejected_exhaust_node.is_none()
        && lifecycle.reported_first_return_node.is_none()
        && lifecycle.topology_diagnostics.is_empty()
        && lifecycle.topology_failure.is_none()
        && lifecycle.economizer_flow_limit_warning_count == 0
        && lifecycle.supply_temperature_registered_recurring_diagnostic_count == 0
        && lifecycle.supply_temperature_diagnostic_event_count == 0
        && lifecycle.supply_temperature_characterized_severe_error_count_increment == 0
        && lifecycle.cooling_supply_temperature_error_index == 0
        && lifecycle.heating_supply_temperature_error_index == 0
        && lifecycle.cooling_supply_temperature_first_diagnostic_count == 0
        && lifecycle.heating_supply_temperature_first_diagnostic_count == 0
        && lifecycle.supply_temperature_diagnostics.is_empty()
        && lifecycle.cooling_supply_temperature_warning_count == 0
        && lifecycle.heating_supply_temperature_warning_count == 0
        && flags.environment_initialization_needed == (timestep_count > 1);
    if !ready {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::InitLifecycleInvariant {
                field: "final_flags_ready",
                expected: 1,
                actual: 0,
            },
        );
    }
    let expected_sized_limits = PurchasedAirSizedLimits::from_system(binding.system);
    let sizing_ready = lifecycle.sized_limits == Some(expected_sized_limits)
        && lifecycle.sizing_outcome.is_some_and(|outcome| {
            outcome.route == PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun
                && outcome.sized_limits == expected_sized_limits
        });
    if !sizing_ready {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::InitLifecycleInvariant {
                field: "sizing_overlay_ready",
                expected: 1,
                actual: 0,
            },
        );
    }
    let density = lifecycle.standard_air_density_kg_per_m3;
    let density_valid = density.is_some_and(|value| value.is_finite() && value > 0.0);
    let caches_valid = lifecycle
        .maximum_heating_air_mass_flow_rate_kg_per_s
        .is_finite()
        && lifecycle.maximum_heating_air_mass_flow_rate_kg_per_s >= 0.0
        && lifecycle
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_finite()
        && lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s >= 0.0;
    let expected_mass_flow = |limit: IdealLoadsLimit, volume_flow: Option<AutosizeOrNumber>| {
        if matches!(
            limit,
            IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
        ) {
            match (volume_flow, density) {
                (Some(AutosizeOrNumber::Value(volume_flow)), Some(density)) => {
                    Some(volume_flow * density)
                }
                _ => None,
            }
        } else {
            Some(0.0)
        }
    };
    let flow_caches_match_sizing = expected_mass_flow(
        binding.system.heating_limit,
        expected_sized_limits.maximum_heating_air_flow_rate_m3_per_s,
    )
    .is_some_and(|expected| {
        (lifecycle.maximum_heating_air_mass_flow_rate_kg_per_s - expected).abs() <= 1.0e-12
    }) && expected_mass_flow(
        binding.system.cooling_limit,
        expected_sized_limits.maximum_cooling_air_flow_rate_m3_per_s,
    )
    .is_some_and(|expected| {
        (lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s - expected).abs() <= 1.0e-12
    });
    if !density_valid || !caches_valid || !flow_caches_match_sizing {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::InitLifecycleInvariant {
                field: "environment_cache_valid",
                expected: 1,
                actual: 0,
            },
        );
    }
    Ok(())
}

fn validate_fixed_runtime_config(runtime_config: HeatBalanceRuntimeConfig) {
    debug_assert!(runtime_config.use_third_order_zone_air_correction);
    debug_assert!(!runtime_config.use_energyplus_adaptive_system_timestep_zone_air_correction);
}

fn node_name(model: &SimulationModel, node: ep_model::NodeId) -> String {
    model
        .typed
        .nodes
        .iter()
        .find(|candidate| candidate.id == node)
        .map(|candidate| candidate.name.0.clone())
        .unwrap_or_else(|| format!("NODE {}", node.0))
}

#[cfg(test)]
#[path = "coupled_runtime_tests.rs"]
mod tests;

fn zone_name(model: &SimulationModel, zone: ep_model::ZoneId) -> String {
    model
        .typed
        .zones
        .iter()
        .find(|candidate| candidate.id == zone)
        .map(|candidate| candidate.name.0.clone())
        .unwrap_or_else(|| format!("ZONE {}", zone.0))
}
