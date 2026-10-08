//! Source-order `UpdateWeatherData`, WeatherManager.cc:1994–2033 at the frozen pin.

use super::state::WeatherDayState;

/// Copies the complete tomorrow owners and updates only the source current fields.
///
/// Environment schedule ordinal and tomorrow dates are preserved. The caller
/// owns any subsequent prefetch, `InitializeWeather` writes, raw reads, and physics.
pub fn update_weather_data(state: &mut WeatherDayState) {
    state.today_variables = state.tomorrow_variables;

    if state.global.begin_envrn_flag {
        state.global.previous_hour = 24;
    }

    state.today_values = state.tomorrow_values.clone();

    state.environment.day_of_year = state.today_variables.day_of_year;
    state.environment.year = state.today_variables.year;
    state.environment.month = state.today_variables.month;
    state.environment.day_of_month = state.today_variables.day_of_month;
    state.environment.day_of_week = state.today_variables.day_of_week;
    state.environment.holiday_index = state.today_variables.holiday_index;
    state.weather.rpt_day_type = if state.environment.holiday_index > 0 {
        state.environment.holiday_index
    } else {
        state.environment.day_of_week
    };
    state.environment.dst_indicator = state.today_variables.daylight_saving_index;
    state.environment.equation_of_time = state.today_variables.equation_of_time;
    state.environment.cos_solar_declin_angle = state.today_variables.cos_solar_declin_angle;
    state.environment.sin_solar_declin_angle = state.today_variables.sin_solar_declin_angle;
}
