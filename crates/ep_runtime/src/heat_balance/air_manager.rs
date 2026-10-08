//! HeatBalanceAirManager source-order stage contract.

use crate::execution_plan::{EnergyPlusCompatibilityStage, ExecutionStageKind};
use crate::heat_balance::algorithm::HeatBalanceRuntimeConfig;
use crate::heat_balance::state::{HeatBalanceState, ZoneHeatBalanceState};
use crate::heat_balance::zone_air_initialization::{
    ZoneAirCallerTemperatureInputs, ZoneAirInitializationState,
};
use crate::heat_balance::zone_air_initialization_trace::{self, ZoneAirInitializationObservation};
use crate::psychrometrics::energyplus_zone_air_heat_capacity_j_per_k;
use crate::weather::{
    HeatBalanceWeatherContext, WeatherTimestepSeries,
    energyplus_weather_atmospheric_pressure_for_context, weather_context_outdoor_humidity_ratio,
};
use ep_model::FirstHourInterpolationStartingValues;

/// EnergyPlus `HeatBalanceAirManager::ManageAirHeatBalance`.
#[must_use]
pub const fn manage_air_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    EnergyPlusCompatibilityStage {
        kind: ExecutionStageKind::ManageAirHeatBalance,
        stage_name: "manage-air-heat-balance",
        source_file: "src/EnergyPlus/HeatBalanceAirManager.cc",
        source_routine: "ManageAirHeatBalance",
    }
}

/// EnergyPlus `HeatBalanceAirManager::ManageAirHeatBalance` source-order wrapper.
pub(crate) fn manage_air_heat_balance_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// Compatibility alias for EnergyPlus `GetAirHeatBalanceInput` ownership.
pub(crate) fn get_air_heat_balance_input_compat<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// Compatibility alias for EnergyPlus air heat-balance initialization.
pub(crate) fn init_air_heat_balance_compat<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// Compatibility alias for EnergyPlus air heat-balance calculation.
pub(crate) fn calc_heat_balance_air_compat<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// Compatibility alias for zone mean air temperature reporting.
pub(crate) fn report_zone_mean_air_temp_compat<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// Compatibility alias for EnergyPlus `HeatBalanceAirManager::ManageAirHeatBalance`.
pub(crate) fn manage_air_heat_balance_compat<T>(execute: impl FnOnce() -> T) -> T {
    manage_air_heat_balance_source_order_path(execute)
}

pub(crate) fn weather_proxy_zone_air_heat_capacity_j_per_k(
    zone: &ZoneHeatBalanceState,
    context: Option<HeatBalanceWeatherContext<'_>>,
    _fallback_dry_bulb_c: f64,
) -> Option<f64> {
    weather_context_zone_air_heat_capacity_j_per_k(zone, context)
}

pub(crate) fn weather_context_zone_air_heat_capacity_j_per_k(
    zone: &ZoneHeatBalanceState,
    context: Option<HeatBalanceWeatherContext<'_>>,
) -> Option<f64> {
    let context = context?;
    let record = context.records.get(context.record_index)?;
    let atmospheric_pressure_pa = energyplus_weather_atmospheric_pressure_for_context(
        context,
        record.atmospheric_pressure_pa,
    );

    energyplus_zone_air_heat_capacity_j_per_k(
        zone.volume_m3,
        atmospheric_pressure_pa,
        zone.mean_air_temperature_c,
        zone.air_humidity_ratio,
    )
}

pub(crate) fn update_single_zone_air_heat_capacity_from_weather_context(
    zone: &mut ZoneHeatBalanceState,
    context: Option<HeatBalanceWeatherContext<'_>>,
    _fallback_dry_bulb_c: f64,
) {
    if let Some(air_heat_capacity_j_per_k) =
        crate::psychrometrics::production_trace::in_phase("zone_air_heat_capacity_update", || {
            weather_context_zone_air_heat_capacity_j_per_k(zone, context)
        })
    {
        zone.air_heat_capacity_j_per_k = air_heat_capacity_j_per_k;
    }
}

pub(crate) fn update_zone_air_heat_capacities_from_weather_context(
    zones: &mut [ZoneHeatBalanceState],
    context: Option<HeatBalanceWeatherContext<'_>>,
    _fallback_dry_bulb_c: f64,
) {
    for zone in zones {
        if let Some(air_heat_capacity_j_per_k) = crate::psychrometrics::production_trace::in_phase(
            "zone_air_heat_capacity_update",
            || weather_context_zone_air_heat_capacity_j_per_k(zone, context),
        ) {
            zone.air_heat_capacity_j_per_k = air_heat_capacity_j_per_k;
        }
    }
}

pub(crate) fn seed_zone_air_humidity_ratios_from_weather_series(
    state: &mut HeatBalanceState,
    weather_series: Option<&WeatherTimestepSeries>,
    fallback_dry_bulb_c: f64,
    zone_steps_per_hour: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) {
    let Some(series) = weather_series else {
        return;
    };
    let Some(humidity_ratio) = weather_context_outdoor_humidity_ratio(
        HeatBalanceWeatherContext {
            records: series.hourly_records(),
            sample: series.sample_for(0, 1),
            record_index: 0,
            zone_steps_per_hour,
            zone_timestep: Some(1),
            first_hour_interpolation_starting_values,
        },
        fallback_dry_bulb_c,
    ) else {
        return;
    };

    // This existing weather-entry path starts one environment for either A or B.
    // Weather interpolation remains its own producer. The selected global guard
    // prevents a repeated entry from reconstructing active solver histories.
    if !state.zone_air_environment_guard.my_environment_flag {
        return;
    }
    let capturing = zone_air_initialization_trace::is_active();
    let mut owners = Vec::with_capacity(state.zones.len());
    let mut preparations = capturing.then(|| Vec::with_capacity(state.zones.len()));
    for zone in &state.zones {
        let mut owner = ZoneAirInitializationState::default();
        let constructor = if capturing { Some(owner) } else { None };
        owner.bulk_reconstruct_and_current_w_seed(humidity_ratio);
        let after_bulk = if capturing { Some(owner) } else { None };
        let caller_temperature_inputs = ZoneAirCallerTemperatureInputs::from_legacy_zone(zone);
        owner.prepare_caller_temperature_inputs(&caller_temperature_inputs);
        if let (Some(preparations), Some(constructor), Some(after_bulk)) =
            (&mut preparations, constructor, after_bulk)
        {
            preparations.push((constructor, after_bulk, caller_temperature_inputs, owner));
        }
        owners.push(owner);
    }
    let invocation = state
        .zone_air_environment_guard
        .apply(true, humidity_ratio, &mut owners);
    for (zone, owner) in state.zones.iter_mut().zip(&owners) {
        owner.project_to_legacy_zone(zone);
    }
    if let Some(preparations) = preparations {
        for ((zone, owner), (constructor, after_bulk, caller_temperature_inputs, before_begin)) in
            state.zones.iter().zip(owners).zip(preparations)
        {
            zone_air_initialization_trace::record(ZoneAirInitializationObservation {
                zone_id: zone.zone_id,
                zone_name: zone.zone_name.clone(),
                out_hum_rat: humidity_ratio,
                caller_temperature_inputs,
                constructor,
                after_bulk,
                before_begin,
                after_begin: owner,
                guard: invocation,
                handoff: zone_air_initialization_trace::capture_projection(zone),
            });
        }
    }
}

pub(crate) fn zone_air_heat_balance_air_storage_rate_w(
    zone_state: &ZoneHeatBalanceState,
    timestep_seconds: f64,
    runtime_config: HeatBalanceRuntimeConfig,
    third_order_report_air_heat_capacity_j_per_k: Option<f64>,
) -> f64 {
    if runtime_config.use_third_order_zone_air_correction {
        if timestep_seconds > 0.0 {
            third_order_report_air_heat_capacity_j_per_k
                .unwrap_or(zone_state.air_heat_capacity_j_per_k)
                * (zone_state.mean_air_temperature_c
                    - zone_state.previous_mean_air_temperatures_c[0])
                / timestep_seconds
        } else {
            0.0
        }
    } else {
        zone_state
            .zone_air_temperature_coefficients
            .temp_independent_coefficient_w
            - zone_state
                .zone_air_temperature_coefficients
                .temp_dependent_coefficient_w_per_k
                * zone_state.mean_air_temperature_c
    }
}
