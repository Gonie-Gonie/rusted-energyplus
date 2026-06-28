//! Exterior surface weather forcing helpers for CalcHeatBalanceOutsideSurf.

use crate::psychrometrics::energyplus_outdoor_wet_bulb_c;
use crate::weather::{
    EpwRecord, HeatBalanceWeatherContext, energyplus_weather_atmospheric_pressure_for_context,
    energyplus_weather_dry_bulb_at_timestep_with_starting_values,
    energyplus_weather_interpolation_weight, energyplus_weather_relative_humidity_for_context,
    previous_weather_record, previous_weather_record_with_first_hour_starting_values,
};
use ep_model::{FirstHourInterpolationStartingValues, Surface, WindExposure};

/// EnergyPlus source-order owner for exterior wet-surface weather forcing.
pub const EXTERIOR_SURFACE_WEATHER_OWNER_STAGE: &str = "CalcHeatBalanceOutsideSurf";

const ENERGYPLUS_HOURLY_RAIN_THRESHOLD_MM: f64 = 0.8;

pub(crate) fn energyplus_exterior_wet_timestep_fraction(
    records: &[EpwRecord],
    record_index: usize,
    zone_steps_per_hour: u32,
    typed_surface: &Surface,
) -> f64 {
    if typed_surface.wind_exposure != WindExposure::WindExposed {
        return 0.0;
    }

    let steps = zone_steps_per_hour.max(1);
    let wet_steps = (1..=steps)
        .filter(|timestep| {
            energyplus_weather_record_is_rain_at_timestep(records, record_index, *timestep, steps)
        })
        .count();
    wet_steps as f64 / f64::from(steps)
}

pub(crate) fn energyplus_exterior_wet_context_fraction(
    context: HeatBalanceWeatherContext<'_>,
    typed_surface: &Surface,
) -> f64 {
    if typed_surface.wind_exposure != WindExposure::WindExposed {
        return 0.0;
    }

    let steps = context.zone_steps_per_hour.max(1);
    if let Some(sample) = context.sample {
        return if sample.liquid_precipitation_depth_mm >= ENERGYPLUS_HOURLY_RAIN_THRESHOLD_MM {
            1.0
        } else {
            0.0
        };
    }
    if let Some(timestep) = context.zone_timestep {
        return if energyplus_weather_record_is_rain_at_timestep_with_starting_values(
            context.records,
            context.record_index,
            timestep,
            steps,
            context.first_hour_interpolation_starting_values,
        ) {
            1.0
        } else {
            0.0
        };
    }

    energyplus_exterior_wet_timestep_fraction(
        context.records,
        context.record_index,
        steps,
        typed_surface,
    )
}

pub(crate) fn energyplus_weather_record_is_rain_at_timestep(
    records: &[EpwRecord],
    record_index: usize,
    timestep: u32,
    zone_steps_per_hour: u32,
) -> bool {
    let Some(record) = records.get(record_index) else {
        return false;
    };
    let previous = previous_weather_record(records, record_index);
    let steps = zone_steps_per_hour.max(1);
    let interpolation_weight = energyplus_weather_interpolation_weight(steps, timestep);
    let interpolated_precipitation_depth_mm = previous.liquid_precipitation_depth_mm
        * (1.0 - interpolation_weight)
        + record.liquid_precipitation_depth_mm * interpolation_weight;

    interpolated_precipitation_depth_mm >= ENERGYPLUS_HOURLY_RAIN_THRESHOLD_MM
}

pub(crate) fn energyplus_weather_record_is_rain_at_timestep_with_starting_values(
    records: &[EpwRecord],
    record_index: usize,
    timestep: u32,
    zone_steps_per_hour: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> bool {
    let Some(record) = records.get(record_index) else {
        return false;
    };
    let previous = previous_weather_record_with_first_hour_starting_values(
        records,
        record_index,
        first_hour_interpolation_starting_values,
    );
    let steps = zone_steps_per_hour.max(1);
    let interpolation_weight = energyplus_weather_interpolation_weight(steps, timestep);
    let interpolated_precipitation_depth_mm = previous.liquid_precipitation_depth_mm
        * (1.0 - interpolation_weight)
        + record.liquid_precipitation_depth_mm * interpolation_weight;

    interpolated_precipitation_depth_mm >= ENERGYPLUS_HOURLY_RAIN_THRESHOLD_MM
}

pub(crate) fn energyplus_exterior_wet_reference_temperature_c(
    context: HeatBalanceWeatherContext<'_>,
    fallback_dry_bulb_c: f64,
) -> f64 {
    if let Some(sample) = context.sample {
        return sample.wet_bulb_c;
    }
    let Some(record) = context.records.get(context.record_index) else {
        return fallback_dry_bulb_c;
    };
    let dry_bulb_c = context
        .zone_timestep
        .map(|timestep| {
            energyplus_weather_dry_bulb_at_timestep_with_starting_values(
                Some(context.records),
                context.record_index,
                fallback_dry_bulb_c,
                context.zone_steps_per_hour,
                timestep,
                context.first_hour_interpolation_starting_values,
            )
        })
        .unwrap_or(fallback_dry_bulb_c);
    let relative_humidity_percent =
        energyplus_weather_relative_humidity_for_context(context, record.relative_humidity_percent);
    let atmospheric_pressure_pa = energyplus_weather_atmospheric_pressure_for_context(
        context,
        record.atmospheric_pressure_pa,
    );

    energyplus_outdoor_wet_bulb_c(
        dry_bulb_c,
        relative_humidity_percent,
        atmospheric_pressure_pa,
    )
    .unwrap_or(dry_bulb_c)
}
