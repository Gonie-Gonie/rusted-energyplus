//! Runtime state, heat-balance execution, weather, and trace helpers.

#[cfg(test)]
pub(crate) use crate::diagnostics::*;
pub use crate::error::*;
pub use crate::first_zone::*;
pub use crate::geometry::{surface_area_m2, surface_geometry_summaries, zone_geometry_summaries};
#[cfg(test)]
pub(crate) use crate::geometry::{surface_azimuth_deg, surface_tilt_deg};
use crate::heat_balance::air_manager::seed_zone_air_humidity_ratios_from_weather_records;
#[cfg(test)]
use crate::heat_balance::air_manager::{
    update_zone_air_heat_capacities_from_weather_context, zone_air_heat_balance_air_storage_rate_w,
};
#[cfg(test)]
use crate::heat_balance::convection::{
    ENERGYPLUS_DEFAULT_WEATHER_FILE_TEMPERATURE_SENSOR_HEIGHT_M,
    ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K,
    energyplus_ashrae_tarp_natural_convection_w_per_m2_k,
    energyplus_doe2_outside_convection_coefficient_w_per_m2_k,
    energyplus_surface_outdoor_air_temperature_c, energyplus_surface_outside_wind_speed_m_per_s,
    energyplus_tarp_inside_convection_coefficient_w_per_m2_k,
    heat_balance_uses_doe2_outside_convection,
};
pub use crate::heat_balance::ctf::ConstructionCtfCoefficientOverride;
use crate::heat_balance::ctf::heat_balance_ctf_history_slot_samples;
#[cfg(test)]
use crate::heat_balance::ctf::{
    CtfInsideFaceBalanceInput, CtfOutsideFaceBalanceInput, CtfOutsideQuickConductionBalanceInput,
    advance_surface_ctf_histories, advance_surface_ctf_histories_with_outside_temperature_override,
    energyplus_ctf_inside_face_temperature_c, energyplus_ctf_outside_face_temperature_c,
    energyplus_ctf_outside_face_temperature_quick_conduction_c, surface_ctf_history_slot_samples,
    surface_heat_storage_rate_w, surface_inside_conduction_flux_w_per_m2,
    surface_inside_conduction_rate_w, surface_outside_conduction_flux_w_per_m2,
    surface_outside_conduction_rate_w, update_surface_ctf_history_constants,
};
#[cfg(test)]
pub(crate) use crate::heat_balance::initialization::initialize_heat_balance_state;
use crate::heat_balance::initialization::initialize_heat_balance_state_with_ctf_coefficients;
#[cfg(test)]
use crate::heat_balance::inside_convection::{
    surface_inside_convection_heat_gain_rate_per_area_w_per_m2,
    surface_inside_convection_report_coefficient_w_per_m2_k,
    zone_air_heat_balance_surface_convection_rate_at_air_temperature_w,
    zone_air_heat_balance_surface_convection_rate_from_balance_w,
    zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w,
    zone_air_heat_balance_surface_convection_rate_w,
};
#[cfg(test)]
use crate::heat_balance::longwave::{
    energyplus_exterior_longwave_terms, energyplus_linearized_radiation_coefficient_w_per_m2_k,
    horizontal_infrared_sky_temperature_c,
};
#[cfg(test)]
use crate::heat_balance::radiation::{
    InteriorLongwaveExchangeProbe, InteriorLongwaveSurfaceSnapshot, KELVIN_OFFSET,
    STEFAN_BOLTZMANN_W_PER_M2_K4, energyplus_approximate_view_factors,
    energyplus_scriptf_from_view_factors, fix_energyplus_approximate_view_factors,
    update_surface_inside_longwave_exchange_probe,
    update_surface_inside_scriptf_longwave_exchange_probe,
};
#[cfg(test)]
use crate::heat_balance::reports::zone_surface_report_conduction_rates_w;
use crate::heat_balance::reports::{
    HeatBalanceResultSeriesTraces, heat_balance_result_store_from_traces,
};
use crate::heat_balance::run_period::sample_heat_balance_run_period;
#[cfg(test)]
pub(crate) use crate::heat_balance::solar::surface_incident_solar_radiation_for_weather_context_w_per_m2;
#[cfg(test)]
use crate::heat_balance::solar::{
    append_surface_incident_solar_radiation_series,
    surface_incident_solar_components_hourly_average_w_per_m2,
};
pub use crate::heat_balance::state::*;
#[cfg(test)]
pub(crate) use crate::heat_balance::state::{
    SurfaceExteriorReportTerms, SurfaceOutsideBalanceDiagnostics,
};
pub use crate::heat_balance::summary::*;
#[cfg(test)]
use crate::heat_balance::surface_balance::{
    QuickOutsideConductionContext, exterior_surface_energy_balance, surface_exterior_report_terms,
    surface_inside_ctf_source_terms_w_per_m2,
};
#[cfg(test)]
use crate::heat_balance::surface_boundary::{
    ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C,
    inside_ctf_outside_temperature_history_commit_override_c, surface_steady_u_value_w_per_m2_k,
};
use crate::heat_balance::surface_boundary::{
    seed_energyplus_initial_surface_ctf_histories, seed_initial_surface_ctf_boundary_histories,
};
#[cfg(test)]
pub(crate) use crate::heat_balance::surface_loop::run_surface_balance_passes;
#[cfg(test)]
use crate::heat_balance::surface_weather::{
    energyplus_exterior_wet_context_fraction, energyplus_exterior_wet_timestep_fraction,
    energyplus_weather_record_is_rain_at_timestep,
};
#[cfg(test)]
pub(crate) use crate::heat_balance::timestep::advance_heat_balance_state_one_timestep;
pub(crate) use crate::heat_balance::timestep::advance_heat_balance_state_one_timestep_internal;
pub(crate) use crate::heat_balance::trace::*;
pub(crate) use crate::heat_balance::warmup::run_heat_balance_run_period_warmup;
#[cfg(test)]
use crate::heat_balance::zone_air_correction::{
    apply_energyplus_adaptive_system_timestep_zone_air_correction,
    energyplus_down_interpolate_three_history_values,
    zone_air_system_timestep_storage_report_rate_w,
};
#[cfg(test)]
use crate::heat_balance::{
    energyplus_analytical_zone_air_temperature_c, energyplus_anisotropic_sky_multiplier,
    energyplus_average_solar_coefficients, energyplus_daily_solar_coefficients,
    energyplus_shadowing_period_solar_coefficients, energyplus_third_order_zone_air_temperature_c,
    energyplus_weather_record_day_of_year, energyplus_zone_air_temperature_coefficients,
    heat_balance_uses_balance_surface_convection_report,
    heat_balance_uses_surface_reference_air_convection_report,
    heat_balance_uses_surface_reference_air_surface_convection_report,
    heat_balance_zone_air_algorithm_execution_variant, solar_position_rad_at_local_hour,
    solar_weather_interpolation_weights,
};
#[cfg(test)]
pub(crate) use crate::heat_balance::{surface_air_sky_radiation_split, surface_sky_view_factor};
#[cfg(test)]
pub(crate) use crate::psychrometrics::energyplus_outdoor_wet_bulb_c;
pub use crate::psychrometrics::{
    energyplus_moist_air_density_kg_per_m3, energyplus_moist_air_specific_heat_j_per_kg_k,
    energyplus_psychrometric_humidity_ratio_from_rh, energyplus_water_vapor_gas_enthalpy_j_per_kg,
    energyplus_zone_air_heat_capacity_j_per_k,
};
#[cfg(test)]
use crate::schedules::update_surface_radiant_internal_gain_source_terms;
pub use crate::schedules::{
    ScheduleTrace, ZoneInternalGainTrace, simulate_constant_schedules, simulate_schedule_values,
    simulate_zone_internal_convective_gains,
};
use crate::time_axis::run_period_first_hour_interpolation_starting_values;
pub use crate::weather::*;
#[cfg(test)]
use crate::weather::{
    energyplus_weather_atmospheric_pressure_at_timestep, energyplus_weather_dry_bulb_at_timestep,
    energyplus_weather_dry_bulb_at_timestep_with_starting_values,
    energyplus_weather_horizontal_infrared_at_timestep,
    energyplus_weather_relative_humidity_at_timestep,
    energyplus_weather_wind_direction_at_timestep, energyplus_weather_wind_speed_at_timestep,
};
#[cfg(test)]
pub(crate) use crate::{OutputSeries, ResultStore};
#[cfg(test)]
use crate::{SimulationMode, SimulationState};
use ep_model::SimulationModel;
const SECONDS_PER_HOUR: f64 = 3600.0;
#[cfg(test)]
const ENERGYPLUS_ZONE_INITIAL_TEMP_C: f64 = 23.0;

/// Simulates hourly zone mean air temperatures through the heat-balance state
/// shell without making a conformance claim.
///
/// This diagnostic trace runs every configured zone timestep, samples hourly
/// MAT values, and stores EnergyPlus-style result series for all zones.
pub fn simulate_heat_balance_zone_air_temperatures(
    model: &SimulationModel,
    weather_dry_bulb_c: &[f64],
    options: HeatBalanceSimulationOptions,
) -> Result<HeatBalanceSimulation, RuntimeError> {
    simulate_heat_balance_zone_air_temperatures_internal(
        model,
        weather_dry_bulb_c,
        None,
        options,
        &[],
    )
}

/// Simulates hourly zone mean air temperatures with full EPW records available
/// for diagnostic exterior surface forcing.
pub fn simulate_heat_balance_zone_air_temperatures_with_weather_records(
    model: &SimulationModel,
    weather_records: &[EpwRecord],
    options: HeatBalanceSimulationOptions,
) -> Result<HeatBalanceSimulation, RuntimeError> {
    simulate_heat_balance_zone_air_temperatures_with_weather_records_and_ctf_coefficients(
        model,
        weather_records,
        options,
        &[],
    )
}

/// Simulates hourly zone mean air temperatures with diagnostic CTF coefficient rows.
///
/// The coefficient rows are intended for diagnostic isolation with EnergyPlus
/// `eplusout.eio` CTF output. Conformance paths should use the default
/// simulation entry points until native coefficient generation is ported.
pub fn simulate_heat_balance_zone_air_temperatures_with_weather_records_and_ctf_coefficients(
    model: &SimulationModel,
    weather_records: &[EpwRecord],
    options: HeatBalanceSimulationOptions,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
) -> Result<HeatBalanceSimulation, RuntimeError> {
    let weather_dry_bulb_c = weather_records
        .iter()
        .map(|record| record.dry_bulb_c)
        .collect::<Vec<_>>();
    simulate_heat_balance_zone_air_temperatures_internal(
        model,
        &weather_dry_bulb_c,
        Some(weather_records),
        options,
        ctf_coefficients,
    )
}

fn simulate_heat_balance_zone_air_temperatures_internal(
    model: &SimulationModel,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    options: HeatBalanceSimulationOptions,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
) -> Result<HeatBalanceSimulation, RuntimeError> {
    if weather_dry_bulb_c.is_empty() {
        return Err(RuntimeError::NoWeatherData);
    }
    if options.sample_count > weather_dry_bulb_c.len() {
        return Err(RuntimeError::SampleCountExceedsWeather {
            requested: options.sample_count,
            available: weather_dry_bulb_c.len(),
        });
    }
    if model.typed.zones.is_empty() {
        return Err(RuntimeError::NoZones);
    }

    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    let seconds_per_timestep = SECONDS_PER_HOUR / f64::from(zone_steps_per_hour);
    let first_hour_interpolation_starting_values =
        run_period_first_hour_interpolation_starting_values(&model.typed);
    let mut state = initialize_heat_balance_state_with_ctf_coefficients(
        model,
        options.initial_zone_air_temperature_c,
        ctf_coefficients,
    )?;
    seed_zone_air_humidity_ratios_from_weather_records(
        &mut state,
        weather_records,
        weather_dry_bulb_c[0],
        zone_steps_per_hour,
        first_hour_interpolation_starting_values,
    );
    match options.ctf_initial_history_policy {
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
    let mut warmup_day_end_zone_air_states = Vec::new();
    let warmup = run_heat_balance_run_period_warmup(
        &model.typed,
        &mut state,
        weather_dry_bulb_c,
        weather_records,
        zone_steps_per_hour,
        seconds_per_timestep,
        options.warmup,
        options.zone_air_algorithm,
        options.surface_iteration_count,
        options.inside_hconv_reevaluation_interval,
        options.surface_loop_zone_air_correction,
        first_hour_interpolation_starting_values,
        &mut warmup_day_end_zone_air_states,
        advance_heat_balance_state_one_timestep_internal,
    );
    let run_period_initial_zone_air_states = state
        .zones
        .iter()
        .map(heat_balance_zone_air_state_sample)
        .collect::<Vec<_>>();
    let run_period_initial_ctf_history_slots =
        heat_balance_ctf_history_slot_samples(&state.surfaces);
    let run_period_timestep_start = state.timestep_index;
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
        first_sample_ctf_history_slot_accumulators,
        hourly_ctf_history_slots,
        hourly_ctf_history_slots_after_advance,
        surface_first_sample_trace,
        zone_air_first_sample_trace,
        surface_iteration_first_sample_trace,
        surface_iteration_sample_trace,
    } = sample_heat_balance_run_period(
        model,
        &mut state,
        weather_dry_bulb_c,
        weather_records,
        options,
        zone_steps_per_hour,
        seconds_per_timestep,
        first_hour_interpolation_starting_values,
    );
    let results = heat_balance_result_store_from_traces(HeatBalanceResultSeriesTraces {
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
    let summary = HeatBalanceSimulationSummary {
        samples: options.sample_count,
        timestep_count: state.timestep_index,
        run_period_timestep_count: state.timestep_index - run_period_timestep_start,
        warmup,
        zone_count: state.zones.len(),
        surface_count: state.surfaces.len(),
        surface_iteration_count: options.surface_iteration_count,
        inside_hconv_reevaluation_interval: options.inside_hconv_reevaluation_interval,
        ctf_initial_history_policy: options.ctf_initial_history_policy,
        zone_conduction_report_source: options.zone_conduction_report_source,
        zone_air_report_sampling: options.zone_air_report_sampling,
        surface_loop_zone_air_correction: options.surface_loop_zone_air_correction,
        run_period_initial_zone_air_states,
        warmup_day_end_zone_air_states,
        run_period_initial_ctf_history_slots,
        first_sample_ctf_history_slots: first_sample_ctf_history_slot_accumulators
            .into_values()
            .map(HeatBalanceCtfHistorySlotFirstSampleAccumulator::finalize)
            .collect(),
        hourly_ctf_history_slots,
        hourly_ctf_history_slots_after_advance,
        surface_first_sample_trace,
        zone_air_first_sample_trace,
        surface_iteration_first_sample_trace,
        surface_iteration_sample_trace,
    };

    Ok(HeatBalanceSimulation {
        state,
        results,
        summary,
    })
}

#[cfg(test)]
mod tests;
