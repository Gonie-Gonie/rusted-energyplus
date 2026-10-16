//! Sole WeatherManager EnvironmentData; allocation belongs to actual4767.
//! Kind=-1/ClarkAllen=0 follow pinned enum identities, not a prepared phase.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceEnvironmentData {
    /// Actual retained native EnvironmentData member.
    pub title: String,
    /// Actual retained native EnvironmentData member.
    pub kind_of_environment_character: String,
    /// Actual retained native EnvironmentData member.
    pub kind_of_environment: i32,
    /// Actual retained native EnvironmentData member.
    pub design_day_number: i32,
    /// Actual retained native EnvironmentData member.
    pub run_period_design_number: i32,
    /// Actual retained native EnvironmentData member.
    pub seed_environment_number: i32,
    /// Actual retained native EnvironmentData member.
    pub hvac_sizing_iteration_number: i32,
    /// Actual retained native EnvironmentData member.
    pub total_days: i32,
    /// Actual retained native EnvironmentData member.
    pub start_julian_day: i32,
    /// Actual retained native EnvironmentData member.
    pub start_month: i32,
    /// Actual retained native EnvironmentData member.
    pub start_day: i32,
    /// Actual retained native EnvironmentData member.
    pub start_year: i32,
    /// Actual retained native EnvironmentData member.
    pub start_date: i32,
    /// Actual retained native EnvironmentData member.
    pub end_month: i32,
    /// Actual retained native EnvironmentData member.
    pub end_day: i32,
    /// Actual retained native EnvironmentData member.
    pub end_julian_day: i32,
    /// Actual retained native EnvironmentData member.
    pub end_year: i32,
    /// Actual retained native EnvironmentData member.
    pub end_date: i32,
    /// Actual retained native EnvironmentData member.
    pub day_of_week: i32,
    /// Actual retained native EnvironmentData member.
    pub use_daylight_saving: bool,
    /// Actual retained native EnvironmentData member.
    pub use_holidays: bool,
    /// Actual retained native EnvironmentData member.
    pub apply_weekend_rule: bool,
    /// Actual retained native EnvironmentData member.
    pub use_rain: bool,
    /// Actual retained native EnvironmentData member.
    pub use_snow: bool,
    /// Actual retained native EnvironmentData member.
    pub month_week_day: [i32; 12],
    /// Actual retained native EnvironmentData member.
    pub set_week_days: bool,
    /// Actual retained native EnvironmentData member.
    pub number_simulation_years: i32,
    /// Actual retained native EnvironmentData member.
    pub current_cycle: i32,
    /// Actual retained native EnvironmentData member.
    pub weather_properties_type_one: i32,
    /// Actual retained native EnvironmentData member.
    pub sky_temperature_model: i32,
    /// Actual retained native EnvironmentData member.
    pub use_weather_file_horizontal_infrared: bool,
    /// Actual retained native EnvironmentData member.
    pub current_year: i32,
    /// Actual retained native EnvironmentData member.
    pub is_leap_year: bool,
    /// Actual retained native EnvironmentData member.
    pub roll_day_type_on_repeat: bool,
    /// Actual retained native EnvironmentData member.
    pub treat_years_as_consecutive: bool,
    /// Actual retained native EnvironmentData member.
    pub match_year: bool,
    /// Actual retained native EnvironmentData member.
    pub actual_weather: bool,
    /// Actual retained native EnvironmentData member.
    pub raw_simulation_days: i32,
    /// Actual retained native EnvironmentData member.
    pub first_hour_interpolation_uses_hour_one: bool,
    /// Actual retained native EnvironmentData member.
    pub maximum_cooling_outdoor_air_temperature_sizing: f64,
    /// Actual retained native EnvironmentData member.
    pub maximum_cooling_outdoor_air_dew_point_sizing: f64,
    /// Actual retained native EnvironmentData member.
    pub minimum_heating_outdoor_air_temperature_sizing: f64,
    /// Actual retained native EnvironmentData member.
    pub minimum_heating_outdoor_air_dew_point_sizing: f64,
}
impl Default for SourceEnvironmentData {
    fn default() -> Self {
        Self {
            title: String::new(),
            kind_of_environment_character: String::new(),
            kind_of_environment: -1,
            design_day_number: 0,
            run_period_design_number: 0,
            seed_environment_number: 0,
            hvac_sizing_iteration_number: 0,
            total_days: 0,
            start_julian_day: 0,
            start_month: 0,
            start_day: 0,
            start_year: 0,
            start_date: 0,
            end_month: 0,
            end_day: 0,
            end_julian_day: 0,
            end_year: 0,
            end_date: 0,
            day_of_week: 0,
            use_daylight_saving: false,
            use_holidays: false,
            apply_weekend_rule: false,
            use_rain: true,
            use_snow: true,
            month_week_day: [0; 12],
            set_week_days: false,
            number_simulation_years: 1,
            current_cycle: 0,
            weather_properties_type_one: 0,
            sky_temperature_model: 0,
            use_weather_file_horizontal_infrared: true,
            current_year: 0,
            is_leap_year: false,
            roll_day_type_on_repeat: true,
            treat_years_as_consecutive: true,
            match_year: false,
            actual_weather: false,
            raw_simulation_days: 0,
            first_hour_interpolation_uses_hour_one: false,
            maximum_cooling_outdoor_air_temperature_sizing: -1000.0,
            maximum_cooling_outdoor_air_dew_point_sizing: -1000.0,
            minimum_heating_outdoor_air_temperature_sizing: 1000.0,
            minimum_heating_outdoor_air_dew_point_sizing: 1000.0,
        }
    }
}
