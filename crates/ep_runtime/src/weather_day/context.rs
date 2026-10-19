//! Selected native global/environment/weather context carried by daily weather.
//!
//! Defaults follow DataGlobals.hh, DataEnvironment.hh and WeatherManager.hh;
//! this is selected storage, not a complete EnergyPlus state constructor.

use super::super::CurrentWeatherState;
use super::super::source_environment_data::SourceEnvironmentData;
use super::super::source_primary::SourceUnderwaterBoundary;
use super::super::{
    DailyWeatherVariables, source_run_period::SourceRunPeriodData,
    source_user_input::SourceWeatherProperty,
};
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;

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
    /// Actual DataGlobals.hh127 WeathSimReq constructorfalse, written by ReadUser.
    pub weather_simulation_requested: bool,
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
            weather_simulation_requested: false,
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
    /// Sole DataEnvironment constructor/site/station operands, retained in this session.
    pub outdoor_environment_parameters:
        crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentParameters,
    /// Rust witness assigned only after the real selected station/EIO return.
    pub(crate) source_weather_station_initialized: bool,
    /// Rust witness assigned only after the real selected site/EIO return.
    pub(crate) source_site_atmosphere_initialized: bool,
    /// Native DataEnvironment.hh216 ctortrue; real BaseSizer reporting clears it.
    pub one_time_component_report_header_flag: bool,
    /// Actual DataEnvironment constructor false; GetProjectData may override it.
    pub force_begin_env_reset_suppress: bool,
    /// Actual DataEnvironment.hh130 EMS control, constructorfalse.
    pub ems_out_dry_bulb_override_on: bool,
    /// Actual same EMS override value, constructor0.
    pub ems_out_dry_bulb_override_value: f64,
    /// Actual DataEnvironment.hh142 EMS control, constructorfalse.
    pub ems_out_dew_point_temp_override_on: bool,
    /// Actual same EMS override value, constructor0.
    pub ems_out_dew_point_temp_override_value: f64,
    /// Actual DataEnvironment.hh135 EMS control, constructorfalse.
    pub ems_out_rel_hum_override_on: bool,
    /// Actual same EMS override value, constructor0.
    pub ems_out_rel_hum_override_value: f64,
    /// Actual DataEnvironment.hh153 EMS control, constructorfalse.
    pub ems_wind_speed_override_on: bool,
    /// Actual same EMS override value, constructor0.
    pub ems_wind_speed_override_value: f64,
    /// Actual DataEnvironment.hh150 EMS control, constructorfalse.
    pub ems_wind_dir_override_on: bool,
    /// Actual same EMS override value, constructor0.
    pub ems_wind_dir_override_value: f64,
    /// Actual DataEnvironment.hh108 EMS control, constructorfalse.
    pub ems_dif_solar_rad_override_on: bool,
    /// Actual same EMS override value, constructor0.
    pub ems_dif_solar_rad_override_value: f64,
    /// Actual DataEnvironment.hh99 EMS control, constructorfalse.
    pub ems_beam_solar_rad_override_on: bool,
    /// Actual same EMS override value, constructor0.
    pub ems_beam_solar_rad_override_value: f64,
    /// Selected mutable environment after SetCurrentWeather.
    pub current_weather: CurrentWeatherState,
    /// Actual DataEnvironment159?166 constructor0; DayltgCurrent writes the same field.
    pub cloud_fraction: f64,
    /// Actual DataEnvironment159?166 constructor0; DayltgCurrent writes the same field.
    pub exterior_horizontal_sky_illuminance: f64,
    /// Actual DataEnvironment159?166 constructor0; DayltgCurrent writes the same field.
    pub exterior_horizontal_beam_illuminance: f64,
    /// Actual DataEnvironment159?166 constructor0; DayltgCurrent writes the same field.
    pub exterior_beam_normal_illuminance: f64,
    /// Actual DataEnvironment159?166 constructor0; DayltgCurrent writes the same field.
    pub direct_solar_luminous_efficacy: f64,
    /// Actual DataEnvironment159?166 constructor0; DayltgCurrent writes the same field.
    pub diffuse_solar_luminous_efficacy: f64,
    /// Actual DataEnvironment159?166 constructor0; DayltgCurrent writes the same field.
    pub daylighting_sky_clearness: f64,
    /// Actual DataEnvironment159?166 constructor0; DayltgCurrent writes the same field.
    pub daylighting_sky_brightness: f64,

    /// Actual InitSolarHeatGains environment owner (DataEnvironment.hh148).
    pub previous_sol_rad_positive: bool,
    /// DataEnvironment.hh118 GroundTemp in BuildingSurface/Shallow/Deep/FC order.
    pub ground_temp: [f64; 4],
    /// Actual GroundTempKelvin constructor0, written immediately after BuildingSurface.
    pub ground_temp_kelvin: f64,
    /// Actual GroundTempInputs constructor4false; factories write the same flags.
    pub ground_temp_inputs: [bool; 4],
    /// Actual DataEnvironment.hh155 constructor0; CalcWaterMainsTemp writes it.
    pub water_mains_temperature: f64,
    /// Source warning switch controlling hourly solar counts and negative sentinels.
    pub display_weather_missing_data_warnings: bool,
    /// Source override to suppress all hourly solar.
    pub ignore_solar_radiation: bool,
    /// Source override to suppress hourly beam solar.
    pub ignore_beam_radiation: bool,
    /// Source override to suppress hourly diffuse solar.
    pub ignore_diffuse_radiation: bool,
    /// Actual DataEnvironment.hh204 constructor false, shared with HVAC warmup reports.
    pub print_environment_stamp_warmup: bool,
    /// Actual DataEnvironment.hh205 constructor false, set/cleared by genuine callers.
    pub print_environment_stamp_warmup_printed: bool,
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
    /// Actual DataEnvironment.hh181 ctor0; distinct from number of design periods.
    pub tot_run_des_pers_days: i32,
    /// Actual DataEnvironment.hh182-184 constructor0; first GetNext762-764 writes.
    pub current_overall_sim_day: i32,
    /// Actual total across the real Environment array, constructor0.
    pub total_overall_sim_days: i32,
    /// Actual maximum requested run years, constructor0 (first caller writes1).
    pub max_number_sim_years: i32,
    /// Actual DataEnvironment.hh207 constructor0.
    pub start_year: i32,
    /// Actual DataEnvironment.hh208 constructor0.
    pub end_year: i32,
    /// Actual DataEnvironment.hh210-211 constructorfalse.
    pub current_year_is_leap_year: bool,
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
    /// Actual DataEnvironment TimeZoneNumber constructor0, written by Location.
    pub time_zone_number: f64,
    /// Actual DataEnvironment rhoAirSTP constructor0, written at GetNext708.
    pub rho_air_stp: f64,
    /// Actual Site:VariableLocation schedule pointer constructors nullptr.
    pub(crate) varying_location_latitude_schedule: Option<SourceScheduleIndex>,
    /// Actual longitude schedule pointer, independent of latitude pointer.
    pub(crate) varying_location_longitude_schedule: Option<SourceScheduleIndex>,
    /// Actual DataEnvironment.hh214 varyingOrientationSched constructor nullptr.
    pub(crate) varying_orientation_schedule: Option<SourceScheduleIndex>,
    /// Actual standard time-zone meridian from ResolveLocationInformation.
    pub time_zone_meridian: f64,
    /// Actual stored sine and cosine of site latitude.
    pub sin_latitude: f64,
    /// Actual stored cosine of site latitude.
    pub cos_latitude: f64,
    /// Actual TS1TimeOffset constructor default for hourly solar.
    pub one_step_solar_time_offset: f64,
    /// Selected site elevation.
    pub elevation: f64,
    /// Source standard barometric pressure; constructor is sea-level 101325.0.
    pub std_baro_press: f64,
    /// Actual DataEnvironment.hh170 ctor0; ResolveLocationInformation4477 writer.
    pub std_rho_air: f64,
    /// Source environment name.
    pub environment_name: String,
    /// Actual CurMnDy formatting owner, written by DD setup and current weather.
    pub current_month_day: String,
    /// Actual CurMnDyHr string, constructor empty; SetCurrentWeather2082 writes it.
    pub current_month_day_hour: String,
    /// Actual CurMnDyYr owner, assigned immediately after CurMnDy.
    pub current_month_day_year: String,
    /// Actual EnvironmentStartEnd adaptive-report formatting carrier.
    pub environment_start_end: String,
    /// Actual retained RunPeriodEnvironment, assigned by the environment getter.
    pub run_period_environment: bool,
    /// Source end-month flag.
    pub end_month_flag: bool,
    /// Source end-year flag.
    pub end_year_flag: bool,
}

impl Default for WeatherEnvironmentState {
    fn default() -> Self {
        Self {
            outdoor_environment_parameters: crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentParameters {
                site_temperature_gradient_k_per_m: 0.0065,
                weather_file_temperature_modifier_c: 0.0,
                site_wind_exponent: 0.22,
                site_wind_boundary_layer_height_m: 370.0,
                weather_file_wind_modifier: 1.5863,
            },
            source_weather_station_initialized: false,
            source_site_atmosphere_initialized: false,
            one_time_component_report_header_flag: true,
            force_begin_env_reset_suppress: false,
            ems_out_dry_bulb_override_on: false,
            ems_out_dry_bulb_override_value: 0.0,
            ems_out_dew_point_temp_override_on: false,
            ems_out_dew_point_temp_override_value: 0.0,
            ems_out_rel_hum_override_on: false,
            ems_out_rel_hum_override_value: 0.0,
            ems_wind_speed_override_on: false,
            ems_wind_speed_override_value: 0.0,
            ems_wind_dir_override_on: false,
            ems_wind_dir_override_value: 0.0,
            ems_dif_solar_rad_override_on: false,
            ems_dif_solar_rad_override_value: 0.0,
            ems_beam_solar_rad_override_on: false,
            ems_beam_solar_rad_override_value: 0.0,
            current_weather: CurrentWeatherState::default(),
            cloud_fraction: 0.0,
            exterior_horizontal_sky_illuminance: 0.0,
            exterior_horizontal_beam_illuminance: 0.0,
            exterior_beam_normal_illuminance: 0.0,
            direct_solar_luminous_efficacy: 0.0,
            diffuse_solar_luminous_efficacy: 0.0,
            daylighting_sky_clearness: 0.0,
            daylighting_sky_brightness: 0.0,

            previous_sol_rad_positive: false,
            ground_temp: [0.0; 4],
            ground_temp_kelvin: 0.0,
            ground_temp_inputs: [false; 4],
            water_mains_temperature: 0.0,
            display_weather_missing_data_warnings: false,
            ignore_solar_radiation: false,
            ignore_beam_radiation: false,
            ignore_diffuse_radiation: false,
            print_environment_stamp_warmup: false,
            print_environment_stamp_warmup_printed: false,
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
            tot_run_des_pers_days: 0,
            current_overall_sim_day: 0,
            total_overall_sim_days: 0,
            max_number_sim_years: 0,
            start_year: 0,
            end_year: 0,
            current_year_is_leap_year: false,
            cur_envir_num: 0,
            run_period_start_day_of_week: 0,
            sin_solar_declin_angle: 0.0,
            cos_solar_declin_angle: 0.0,
            equation_of_time: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            time_zone_number: 0.0,
            rho_air_stp: 0.0,
            varying_location_latitude_schedule: None,
            varying_location_longitude_schedule: None,
            varying_orientation_schedule: None,
            time_zone_meridian: 0.0,
            sin_latitude: 0.0,
            cos_latitude: 0.0,
            one_step_solar_time_offset: -0.5,
            elevation: 0.0,
            std_baro_press: 101325.0,
            std_rho_air: 0.0,
            environment_name: String::new(),
            current_month_day: String::new(),
            current_month_day_hour: String::new(),
            current_month_day_year: String::new(),
            environment_start_end: String::new(),
            run_period_environment: false,
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
    /// Actual WeatherManager LocationTitle string constructor.
    pub location_title: String,
    /// Actual LocationGathered constructorfalse.
    pub location_gathered: bool,
    /// Actual Keep Site Location Information constructorfalse.
    pub keep_user_site_location_definition: bool,
    /// Actual EnvironmentReportNbr constructor0, assigned by ReportOutputFileHeaders.
    pub environment_report_number: i32,
    /// Actual EnvironmentReportChr string constructor.
    pub environment_report_character: String,
    /// Sole native Environment array, unallocated until ReadUserWeatherInput4767.
    pub source_environments: Option<Vec<SourceEnvironmentData>>,
    /// Native DesignDay EPVector, allocated at ReadUser4766 only.
    pub source_design_day_weather: Option<Vec<DailyWeatherVariables>>,
    /// Sole genuine DesDayInput/desDayMods/SPSiteSchedules compartment; ctor-unallocated.
    pub(crate) source_design_day_input: super::super::source_design_day::SourceDesignDayInputState,
    /// Native RunPeriodInput Array1D, allocated by actual GetRunPeriodData5031.
    pub source_run_period_input: Option<Vec<SourceRunPeriodData>>,
    /// Actual WeatherManager.hh913 persistent vector, native constructor-empty.
    pub(crate) underwater_boundaries: Vec<SourceUnderwaterBoundary>,
    /// Native WPSkyTemperature EPVector, allocated by actual weather-property getter.
    pub source_weather_properties: Option<Vec<SourceWeatherProperty>>,
    /// Native WeatherManager.hh838 constructor0; actual getter writes this count.
    pub number_weather_properties: i32,
    /// Current native environment ordinal.
    pub envrn: i32,
    /// Number of native environments.
    pub num_of_envrn: i32,
    /// Number of weather run periods.
    pub tot_run_pers: i32,
    /// Number of design run periods.
    pub tot_run_des_pers: i32,
    /// Actual WeatherManager812 total; writer remains ReadUserWeatherInput4756.
    pub tot_report_periods: i32,
    /// Actual WeatherManager813–815 report-period constructor counts.
    pub tot_thermal_report_periods: i32,
    /// Actual WeatherManager814 CO2 report-period constructor count.
    pub tot_co2_report_periods: i32,
    /// Actual WeatherManager815 visual report-period constructor count.
    pub tot_visual_report_periods: i32,
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
    /// Actual WeatherManager.hh782 constructor1, incremented only at Source1831.
    pub year_of_sim: i32,
    /// Actual WeatherManager.hh783 constructor365, read by Source EOF2663.
    pub number_days_in_year: i32,
    /// Actual PrintDDHeader constructor and BeginSim SetUpDesignDay owner.
    pub print_design_day_header: bool,
    /// Actual WeatherManager.hh776 constructortrue; report850-859 consumes it.
    pub print_environment_headers: bool,
    /// Actual WeatherManager.hh819 constructor366 zeros; GetNext1085 resets.
    pub weekday_types: [i32; 366],
    /// Actual WeatherManager.hh818 constructor366 zeros; SetSpecialDayDates1645 resets.
    pub special_day_types: [i32; 366],
    /// Actual WeatherManager.hh820 constructor366 zeros; genuine DST child writes.
    pub dst_index: [i32; 366],
    /// Actual WeatherManager.hh862-863 distinct twelve-element native calendar.
    pub end_day_of_month_with_leap_day: [i32; 12],
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
    /// Actual RptIsSnow report indicator.
    pub rpt_is_snow: i32,
    /// Actual DetermineSunUpDown HrAngle in degrees.
    pub hour_angle: f64,
    /// Actual SolarAltitudeAngle in degrees.
    pub solar_altitude_angle: f64,
    /// Actual SolarAzimuthAngle in degrees.
    pub solar_azimuth_angle: f64,
    /// Actual HorizIRSky copied from the selected Today slot.
    pub horizontal_infrared_sky: f64,
    /// Actual default GroundReflectances allocation, independently retained.
    pub ground_reflectances: [f64; 12],
    /// Actual SnowGndRefModifier constructor default.
    pub snow_ground_reflectance_modifier: f64,
    /// Actual SnowGndRefModifierForDayltg constructor default.
    pub snow_ground_reflectance_for_daylighting_modifier: f64,
    /// Actual Weather.hh98-106 enum representation; FixedDefault is3.
    pub water_mains_temperature_method: i32,
    /// Actual Weather.hh804 constructor0; positive correlation getter is separate.
    pub water_mains_annual_average_air_temperature: f64,
    /// Actual Weather.hh805 constructor0.
    pub water_mains_maximum_air_temperature_difference: f64,
    /// Actual Weather.hh806 constructor1.
    pub water_mains_temperature_multiplier: f64,
    /// Actual Weather.hh807 constructor0.
    pub water_mains_temperature_offset: f64,
    /// Selected environment's rain-indicator control.
    pub use_rain_values: bool,
    /// Selected environment's snow-indicator control.
    pub use_snow_values: bool,
}

impl Default for WeatherOwnerState {
    fn default() -> Self {
        Self {
            location_title: String::new(),
            location_gathered: false,
            keep_user_site_location_definition: false,
            environment_report_number: 0,
            environment_report_character: String::new(),
            source_environments: None,
            source_design_day_weather: None,
            source_design_day_input: Default::default(),
            source_run_period_input: None,
            underwater_boundaries: Vec::new(),
            source_weather_properties: None,
            number_weather_properties: 0,
            envrn: 0,
            num_of_envrn: 0,
            tot_run_pers: 0,
            tot_run_des_pers: 0,
            tot_report_periods: 0,
            tot_thermal_report_periods: 0,
            tot_co2_report_periods: 0,
            tot_visual_report_periods: 0,
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
            year_of_sim: 1,
            number_days_in_year: 365,
            print_design_day_header: true,
            print_environment_headers: true,
            weekday_types: [0; 366],
            special_day_types: [0; 366],
            dst_index: [0; 366],
            end_day_of_month_with_leap_day: [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
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
            rpt_is_snow: 0,
            hour_angle: 0.0,
            solar_altitude_angle: 0.0,
            solar_azimuth_angle: 0.0,
            horizontal_infrared_sky: 0.0,
            ground_reflectances: [0.2; 12],
            snow_ground_reflectance_modifier: 1.0,
            snow_ground_reflectance_for_daylighting_modifier: 1.0,
            water_mains_temperature_method: 3,
            water_mains_annual_average_air_temperature: 0.0,
            water_mains_maximum_air_temperature_difference: 0.0,
            water_mains_temperature_multiplier: 1.0,
            water_mains_temperature_offset: 0.0,
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
