//! Selected native global/environment/weather context carried by daily weather.
//!
//! Defaults follow DataGlobals.hh, DataEnvironment.hh and WeatherManager.hh;
//! this is selected storage, not a complete EnergyPlus state constructor.

use super::super::CurrentWeatherState;

/// Selected global caller flags and clocks, including fields the handoff preserves.
#[derive(Clone, Debug, PartialEq)]
pub struct WeatherGlobalState {
    /// Source begin-simulation flag.
    pub begin_sim_flag: bool,
    /// Source begin-environment flag.
    pub begin_envrn_flag: bool,
    /// Source begin-day flag.
    pub begin_day_flag: bool,
    /// Source begin-hour flag.
    pub begin_hour_flag: bool,
    /// Source begin-timestep flag.
    pub begin_time_step_flag: bool,
    /// Source end-day flag.
    pub end_day_flag: bool,
    /// Source end-hour flag.
    pub end_hour_flag: bool,
    /// Source end-environment flag.
    pub end_envrn_flag: bool,
    /// Source end-design-day-environments flag.
    pub end_design_day_envrns_flag: bool,
    /// Source warmup flag.
    pub warmup_flag: bool,
    /// Source simulation-day counter.
    pub day_of_sim: i32,
    /// Source calendar year.
    pub calendar_year: i32,
    /// Previous source hour, set to 24 only for a begin-environment handoff.
    pub previous_hour: i32,
    /// Source current hour.
    pub hour_of_day: i32,
    /// Source number of environment days.
    pub num_of_day_in_envrn: i32,
    /// Source timesteps per hour.
    pub time_steps_in_hour: i32,
    /// Source one-based timestep number.
    pub time_step: i32,
    /// Native KindOfSim integer representation; constructor Invalid is -1.
    pub kind_of_sim: i32,
    /// Source weather-simulation switch.
    pub do_weath_sim: bool,
    /// Source design-day-simulation switch.
    pub do_des_day_sim: bool,
    /// Source reporting switch.
    pub do_output_reporting: bool,
    /// Source zone timestep duration, carried without arithmetic.
    pub time_step_zone: f64,
    /// Zone timestep duration in seconds, prepared by the actual caller.
    pub time_step_zone_sec: f64,
    /// Integer minutes in the declared zone timestep.
    pub minutes_in_time_step: i32,
    /// Current stored interpolation weight.
    pub weight_now: f64,
    /// Previous-hour interpolation weight.
    pub weight_previous_hour: f64,
    /// Source zone-step clock in hours.
    pub current_time: f64,
    /// Source one-based cumulative zone-step number.
    pub sim_time_steps: i32,
    /// Source report string for simulation day; native constructor is "0".
    pub day_of_sim_chr: String,
    /// Source report string for calendar year.
    pub calendar_year_chr: String,
}

impl Default for WeatherGlobalState {
    fn default() -> Self {
        Self {
            begin_sim_flag: false,
            begin_envrn_flag: false,
            begin_day_flag: false,
            begin_hour_flag: false,
            begin_time_step_flag: false,
            end_day_flag: false,
            end_hour_flag: false,
            end_envrn_flag: false,
            end_design_day_envrns_flag: false,
            warmup_flag: false,
            day_of_sim: 0,
            calendar_year: 0,
            previous_hour: 0,
            hour_of_day: 0,
            num_of_day_in_envrn: 0,
            time_steps_in_hour: 0,
            time_step: 0,
            kind_of_sim: -1,
            do_weath_sim: false,
            do_des_day_sim: false,
            do_output_reporting: false,
            time_step_zone: 0.0,
            time_step_zone_sec: 0.0,
            minutes_in_time_step: 0,
            weight_now: 0.0,
            weight_previous_hour: 0.0,
            current_time: 0.0,
            sim_time_steps: 0,
            day_of_sim_chr: "0".into(),
            calendar_year_chr: String::new(),
        }
    }
}

/// Selected current and tomorrow environment fields from DataEnvironment.hh.
#[derive(Clone, Debug, PartialEq)]
pub struct WeatherEnvironmentState {
    /// Selected mutable environment after SetCurrentWeather.
    pub current_weather: CurrentWeatherState,
    /// Source warning switch controlling hourly solar counts and negative sentinels.
    pub display_weather_missing_data_warnings: bool,
    /// Source override to suppress all hourly solar.
    pub ignore_solar_radiation: bool,
    /// Source override to suppress hourly beam solar.
    pub ignore_beam_radiation: bool,
    /// Source override to suppress hourly diffuse solar.
    pub ignore_diffuse_radiation: bool,
    /// Current weather ordinal.
    pub day_of_year: i32,
    /// Schedule ordinal; pure UpdateWeatherData does not write this member.
    pub day_of_year_schedule: i32,
    /// Current year.
    pub year: i32,
    /// Current month.
    pub month: i32,
    /// Current day of month.
    pub day_of_month: i32,
    /// Current weekday.
    pub day_of_week: i32,
    /// Current holiday index.
    pub holiday_index: i32,
    /// Current daylight-saving indicator.
    pub dst_indicator: i32,
    /// Tomorrow year, owned by the lifecycle caller rather than the pure copy.
    pub year_tomorrow: i32,
    /// Tomorrow month.
    pub month_tomorrow: i32,
    /// Tomorrow day of month.
    pub day_of_month_tomorrow: i32,
    /// Tomorrow weekday.
    pub day_of_week_tomorrow: i32,
    /// Tomorrow holiday index.
    pub holiday_index_tomorrow: i32,
    /// Number of design days.
    pub tot_des_days: i32,
    /// Current native environment number.
    pub cur_envir_num: i32,
    /// Run-period start weekday.
    pub run_period_start_day_of_week: i32,
    /// Current sine of solar declination.
    pub sin_solar_declin_angle: f64,
    /// Current cosine of solar declination.
    pub cos_solar_declin_angle: f64,
    /// Current equation of time.
    pub equation_of_time: f64,
    /// Selected site latitude.
    pub latitude: f64,
    /// Selected site longitude.
    pub longitude: f64,
    /// Selected site elevation.
    pub elevation: f64,
    /// Source standard barometric pressure; constructor is sea-level 101325.0.
    pub std_baro_press: f64,
    /// Source environment name.
    pub environment_name: String,
    /// Source end-month flag.
    pub end_month_flag: bool,
    /// Source end-year flag.
    pub end_year_flag: bool,
}

impl Default for WeatherEnvironmentState {
    fn default() -> Self {
        Self {
            current_weather: CurrentWeatherState::default(),
            display_weather_missing_data_warnings: false,
            ignore_solar_radiation: false,
            ignore_beam_radiation: false,
            ignore_diffuse_radiation: false,
            day_of_year: 0,
            day_of_year_schedule: 0,
            year: 0,
            month: 0,
            day_of_month: 0,
            day_of_week: 0,
            holiday_index: 0,
            dst_indicator: 0,
            year_tomorrow: 0,
            month_tomorrow: 0,
            day_of_month_tomorrow: 0,
            day_of_week_tomorrow: 0,
            holiday_index_tomorrow: 0,
            tot_des_days: 0,
            cur_envir_num: 0,
            run_period_start_day_of_week: 0,
            sin_solar_declin_angle: 0.0,
            cos_solar_declin_angle: 0.0,
            equation_of_time: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            elevation: 0.0,
            std_baro_press: 101325.0,
            environment_name: String::new(),
            end_month_flag: false,
            end_year_flag: false,
        }
    }
}

impl WeatherEnvironmentState {
    /// Source selected real fields, in snapshot order, without changing their bits.
    #[must_use]
    pub fn real_values(&self) -> [f64; 7] {
        [
            self.sin_solar_declin_angle,
            self.cos_solar_declin_angle,
            self.equation_of_time,
            self.latitude,
            self.longitude,
            self.elevation,
            self.std_baro_press,
        ]
    }
}

/// Selected WeatherManager controls and counters, separate from raw parser storage.
#[derive(Clone, Debug, PartialEq)]
pub struct WeatherOwnerState {
    /// Current native environment ordinal.
    pub envrn: i32,
    /// Number of native environments.
    pub num_of_envrn: i32,
    /// Number of weather run periods.
    pub tot_run_pers: i32,
    /// Number of design run periods.
    pub tot_run_des_pers: i32,
    /// Source number of data periods, synchronized by the actual header caller.
    pub num_data_periods: i32,
    /// Source weather intervals per hour; constructor is one.
    pub num_intervals_per_hour: i32,
    /// Source special-day count.
    pub num_special_days: i32,
    /// Source leap-year addition.
    pub leap_year_add: i32,
    /// Reported current day type.
    pub rpt_day_type: i32,
    /// Reader weekday, separate from the current transported daily weekday.
    pub cur_day_of_week: i32,
    /// Source last simulation day for the current run period.
    pub cur_sim_day_for_end_of_run_period: i32,
    /// Native one-time branch input flag.
    pub get_branch_input_one_time_flag: bool,
    /// Native first GetNextEnvironment registration flag.
    pub get_environment_first_call: bool,
    /// Native first weather-manager call flag.
    pub first_call: bool,
    /// Native water-mains reporting flag.
    pub water_mains_parameter_report: bool,
    /// Whether the first-hour cache has been seeded.
    pub last_hour_set: bool,
    /// Actual caller weather-file-existence flag.
    pub weather_file_exists: bool,
    /// Source weekday-reset flag.
    pub dates_should_be_reset: bool,
    /// Source cycle-start reset flag.
    pub start_dates_cycle_should_be_reset: bool,
    /// Source January-first reset flag.
    pub jan1_dates_should_be_reset: bool,
    /// Source eager-read request flag.
    pub rp_read_all_weather_data: bool,
    /// Source use-weather-DST setting.
    pub use_daylight_saving: bool,
    /// Source use-weather-special-day setting.
    pub use_special_days: bool,
    /// Source resolved DST activity flag.
    pub daylight_saving_is_active: bool,
    /// Source reader's current time, distinct from the caller timestep clock.
    pub read_e_plus_weather_cur_time: f64,
    /// Source timestep fraction.
    pub time_step_fraction: f64,
    /// Source rain threshold, retained without division by this carrier.
    pub is_rain_threshold: f64,
    /// Allocated weights written by SetupInterpolationValues.
    pub interpolation: Option<Vec<f64>>,
    /// Solar weights, retained separately for CLK-05.
    pub solar_interpolation: Option<Vec<f64>>,
    /// Source next hour, wrapping hour 24 to 1.
    pub next_hour: i32,
    /// Source rain report indicator.
    pub rpt_is_rain: i32,
    /// Selected environment's rain-indicator control.
    pub use_rain_values: bool,
    /// Selected environment's snow-indicator control.
    pub use_snow_values: bool,
}

impl Default for WeatherOwnerState {
    fn default() -> Self {
        Self {
            envrn: 0,
            num_of_envrn: 0,
            tot_run_pers: 0,
            tot_run_des_pers: 0,
            num_data_periods: 0,
            num_intervals_per_hour: 1,
            num_special_days: 0,
            leap_year_add: 0,
            rpt_day_type: 0,
            cur_day_of_week: 1,
            cur_sim_day_for_end_of_run_period: 0,
            get_branch_input_one_time_flag: true,
            get_environment_first_call: true,
            first_call: true,
            water_mains_parameter_report: true,
            last_hour_set: false,
            weather_file_exists: false,
            dates_should_be_reset: false,
            start_dates_cycle_should_be_reset: false,
            jan1_dates_should_be_reset: false,
            rp_read_all_weather_data: false,
            use_daylight_saving: true,
            use_special_days: true,
            daylight_saving_is_active: false,
            read_e_plus_weather_cur_time: 1.0,
            time_step_fraction: 0.0,
            is_rain_threshold: 0.8,
            interpolation: None,
            solar_interpolation: None,
            next_hour: 1,
            rpt_is_rain: 0,
            use_rain_values: true,
            use_snow_values: true,
        }
    }
}

impl WeatherOwnerState {
    /// Selected real members without numerical processing.
    #[must_use]
    pub fn real_values(&self) -> [f64; 3] {
        [
            self.read_e_plus_weather_cur_time,
            self.time_step_fraction,
            self.is_rain_threshold,
        ]
    }
}
