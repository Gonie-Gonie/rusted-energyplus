//! Selected hourly solar preprocessing and stored-weight branch from the pin.
//!
//! Raw records remain separate. Hourly warning/negative handling and the later
//! sentinel/Ignore writes surround the default sky call in source order.
//! Current solar position, nighttime masking and surface equations are separate.

use super::{WeatherDayState, WeatherEnvironmentState, WeatherVars};

/// Early source warning/count/negative phase, WeatherManager.cc:2751-2766.
pub(super) fn preprocess_hourly_solar(value: &mut WeatherVars, state: &mut WeatherDayState) {
    let mut beam = value.beam_solar_rad;
    let mut diffuse = value.dif_solar_rad;
    if state.environment.display_weather_missing_data_warnings {
        if beam >= 9999.0 {
            state.missed_counts.beam_solar_rad += 1;
        }
        if diffuse >= 9999.0 {
            state.missed_counts.dif_solar_rad = state.missed_counts.beam_solar_rad + 1;
        }
        if beam < 0.0 {
            beam = 9999.0;
            state.out_of_range_counts.beam_solar_rad += 1;
        }
        if diffuse < 0.0 {
            diffuse = 9999.0;
            state.out_of_range_counts.dif_solar_rad += 1;
        }
    }
    value.beam_solar_rad = beam;
    value.dif_solar_rad = diffuse;
}

/// Later source sentinel/Ignore phase, after sky, WeatherManager.cc:2978-3009.
pub(super) fn finalize_hourly_solar(
    value: &mut WeatherVars,
    environment: &WeatherEnvironmentState,
) {
    let mut beam = value.beam_solar_rad;
    let mut diffuse = value.dif_solar_rad;
    if beam >= 9999.0 {
        beam = 0.0;
    }
    if diffuse >= 9999.0 {
        diffuse = 0.0;
    }
    if environment.ignore_solar_radiation {
        beam = 0.0;
        diffuse = 0.0;
    }
    if environment.ignore_beam_radiation {
        beam = 0.0;
    }
    if environment.ignore_diffuse_radiation {
        diffuse = 0.0;
    }
    value.beam_solar_rad = beam;
    value.dif_solar_rad = diffuse;
}

// Existing solar unit cases exercise the same two phases without an intervening
// sky computation. Production calls the phases at their separate source loci.
#[cfg(test)]
fn process_hourly_solar(value: &mut WeatherVars, state: &mut WeatherDayState) {
    preprocess_hourly_solar(value, state);
    finalize_hourly_solar(value, &state.environment);
}

/// Active multi-step branch, WeatherManager.cc:3083-3100.
/// The caller owns the stored weight and already admits the allocated timestep.
pub(super) fn interpolation_weights(
    current_weight: f64,
    timestep: u32,
    time_step_fraction: f64,
) -> (f64, f64) {
    if current_weight == 1.0 {
        (0.0, 0.0)
    } else if f64::from(timestep) * time_step_fraction < 0.5 {
        (1.0 - current_weight, 0.0)
    } else {
        (0.0, 1.0 - current_weight)
    }
}

#[cfg(test)]
#[path = "solar_tests.rs"]
mod tests;
