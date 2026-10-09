//! Source-order selected weather-environment, initialization and reader calls.

pub use super::configuration::WeatherEnvironmentConfiguration;
use super::configuration::{between, ordinal, weekday_index};
use super::hourly::process_hour;
use super::producer::{ProducedWeatherDay, produce_day};
use super::{
    DailyWeatherVariables, WeatherDayError, WeatherDayPhase, WeatherDayState, WeatherDayValues,
    WeatherVars, update_weather_data,
};
use crate::weather::raw::{
    HeaderCallerPreparation, RawDayReadCallbacks, RawDayReadMode, RawDayReadSelection,
    RawDayReadState, RawEpwCursorOwner, RawEpwHeaderState, RawWeatherDay, RawWeatherSlot,
    project_record,
};

/// One live input stream and its separate raw and processed weather owners.
#[derive(Debug)]
pub struct WeatherSession {
    /// Complete selected daily and auxiliary state.
    pub state: WeatherDayState,
    /// Calendar and input controls; never a physical preview series.
    pub configuration: WeatherEnvironmentConfiguration,
    /// Authoritative raw cursor/header/parser owner.
    pub cursor: RawEpwCursorOwner,
    /// Whether the selected environment is available after admission.
    pub available: bool,
    /// Actual nonfatal header/caller error flag, separate from source-fatal errors.
    pub errors_found: bool,
    /// Whether environment initialization requested its reporting stamp.
    pub print_environment_stamp: bool,
    /// Actual first-day reader cycle count.
    pub current_cycle: i32,
    /// Actual source first-cycle weekday-registration switch.
    pub set_week_days: bool,
    pub(super) today_raw: Option<RawWeatherDay>,
    pub(super) tomorrow_raw: Option<RawWeatherDay>,
    pub(super) today_produced: Option<ProducedWeatherDay>,
    pub(super) tomorrow_produced: Option<ProducedWeatherDay>,
}

impl WeatherSession {
    /// Opens the supplied bytes through the admitted raw header owner.
    #[allow(clippy::field_reassign_with_default)] // Preserve constructor, allocation, then caller preparation.
    pub fn new(
        bytes: Vec<u8>,
        configuration: WeatherEnvironmentConfiguration,
    ) -> Result<Self, WeatherDayError> {
        let mut cursor = RawEpwCursorOwner::open_from_bytes(
            bytes,
            RawEpwHeaderState::default(),
            HeaderCallerPreparation {
                run_period_control_special_days: configuration.input_special_day_count,
            },
        )?;
        let mut state = WeatherDayState::default();
        configuration
            .solar_controls
            .apply_to(&mut state.environment);
        state.today_values = WeatherDayValues::allocated(configuration.steps() as usize)?;
        state.tomorrow_values = WeatherDayValues::allocated(configuration.steps() as usize)?;
        state.global.time_steps_in_hour = configuration.steps() as i32;
        state.global.time_step_zone = 1.0 / f64::from(configuration.steps());
        state.global.time_step_zone_sec = state.global.time_step_zone * 3600.0;
        state.global.minutes_in_time_step = 60 / configuration.steps() as i32;
        // Genuine prepared caller order: SetupInterpolationValues, then fraction.
        state.setup_interpolation_values()?;
        state.global.do_weath_sim = true;
        state.weather.time_step_fraction = state.global.time_step_zone;
        state.weather.weather_file_exists = true;
        state.weather.get_branch_input_one_time_flag = false;
        state.weather.get_environment_first_call = false;
        state.weather.water_mains_parameter_report = false;
        state.weather.envrn = configuration.design_day_count;
        state.weather.num_of_envrn = configuration.design_day_count + 1;
        state.weather.tot_run_pers = 1;
        state.weather.num_data_periods = cursor.header().number_data_periods;
        state.weather.num_intervals_per_hour = cursor.header().intervals_per_hour;
        state.weather.num_special_days = cursor.header().number_special_days;
        state.environment.tot_des_days = configuration.design_day_count;
        state.environment.latitude = configuration.site.latitude_deg;
        state.environment.longitude = configuration.site.longitude_deg;
        state.environment.elevation = configuration.site.elevation_m;
        state.environment.day_of_week = weekday_index(
            configuration
                .day_point(1)
                .ok_or_else(|| WeatherDayError::admission("missing prepared environment weekday"))?
                .day_of_week,
        );
        // ResolveLocationInformation, WeatherManager.cc:4476. This preparatory
        // formula is not certified by the current daily-transport comparison.
        state.environment.std_baro_press =
            101325.0 * (1.0 - 2.25577e-5 * configuration.site.elevation_m).powf(5.2559);
        cursor.close();
        let errors_found = cursor.header_errors();
        Ok(Self {
            state,
            configuration,
            cursor,
            available: false,
            errors_found,
            print_environment_stamp: false,
            current_cycle: 0,
            set_week_days: false,
            today_raw: None,
            tomorrow_raw: None,
            today_produced: None,
            tomorrow_produced: None,
        })
    }

    /// Admits the one selected nonactual weather run period, without reading data rows.
    pub fn get_next_environment(&mut self) -> Result<bool, WeatherDayError> {
        let before = super::production_trace::snapshot(self);
        let result = self.admit_environment();
        super::production_trace::record_operation(
            "GetNextEnvironment",
            before,
            self,
            result.as_ref().err(),
        );
        result
    }

    fn admit_environment(&mut self) -> Result<bool, WeatherDayError> {
        self.cursor.close();
        if self.available {
            self.available = false;
            self.state.weather.envrn = 0;
            self.state.environment.cur_envir_num = 0;
            return Ok(false);
        }
        let first = self
            .configuration
            .day_point(1)
            .ok_or_else(|| WeatherDayError::admission("missing first environment date"))?;
        let last = self
            .configuration
            .day_point(self.configuration.total_days())
            .ok_or_else(|| WeatherDayError::admission("missing last environment date"))?;
        self.state.weather.envrn = self.configuration.design_day_count + 1;
        self.state.weather.dates_should_be_reset = false;
        self.state.global.kind_of_sim = 3; // Constant::KindOfSim::RunPeriodWeather.
        self.state.environment.day_of_year = first.day_of_year as i32;
        self.state.environment.day_of_month =
            self.configuration.run_period.begin_day_of_month as i32;
        self.state.environment.month = self.configuration.run_period.begin_month as i32;
        self.state.global.calendar_year = first.year as i32;
        self.state.global.calendar_year_chr = first.year.to_string();
        self.state.global.num_of_day_in_envrn = self.configuration.total_days() as i32;
        self.cursor.reopen_without_processing_header()?;
        self.state.environment.environment_name =
            self.configuration.run_period.name.0.to_ascii_uppercase();
        self.state.environment.cur_envir_num = self.state.weather.envrn;
        self.state.environment.run_period_start_day_of_week = 0;
        self.state.weather.leap_year_add = first.leap_year_add as i32;
        let header = self.cursor.header();
        self.state.weather.num_data_periods = header.number_data_periods;
        self.state.weather.num_intervals_per_hour = header.intervals_per_hour;
        self.state.weather.num_special_days = header.number_special_days;
        self.state.weather.use_daylight_saving = self
            .configuration
            .run_period
            .use_weather_file_daylight_saving_period;
        self.state.weather.use_special_days = self
            .configuration
            .run_period
            .use_weather_file_holidays_and_special_days;
        self.state.weather.use_rain_values = self
            .configuration
            .run_period
            .use_weather_file_rain_indicators;
        self.state.weather.use_snow_values = self
            .configuration
            .run_period
            .use_weather_file_snow_indicators;
        if header.intervals_per_hour != 1 {
            return Err(WeatherDayError::admission(
                "live weather currently admits one record per hour",
            ));
        }
        self.available = true;
        let leap = self.state.weather.leap_year_add;
        let ok = header.data_periods.values.iter().any(|period| {
            let start = ordinal(period.start_month, period.start_day, leap);
            let end = ordinal(period.end_month, period.end_day, leap);
            match (start, end) {
                (Ok(start), Ok(end)) => {
                    (start == 1 && (end == 365 || end == 366))
                        || (between(first.day_of_year as i32, start, end)
                            && between(last.day_of_year as i32, start, end))
                }
                _ => false,
            }
        });
        if !ok {
            return Err(WeatherDayError::source_fatal(
                "run period outside weather DATA PERIODS",
            ));
        }
        self.state.environment.run_period_start_day_of_week = weekday_index(first.day_of_week);
        self.state.weather.cur_sim_day_for_end_of_run_period =
            self.configuration.total_days() as i32;
        self.state.weather.daylight_saving_is_active =
            self.configuration.time_axis.daylight_saving.active;
        self.state.weather.start_dates_cycle_should_be_reset =
            self.configuration.run_period.begin_month != 1
                || self.configuration.run_period.begin_day_of_month != 1;
        self.state.weather.jan1_dates_should_be_reset = true;
        self.set_week_days = false;
        if self.state.global.begin_sim_flag && self.state.weather.get_environment_first_call {
            self.state.weather.get_environment_first_call = false;
        }
        Ok(true)
    }

    /// Advances source BeginEnvrn/BeginDay/EndEnvrn branches using current caller flags.
    pub fn initialize_weather(&mut self) -> Result<(), WeatherDayError> {
        let before = super::production_trace::snapshot(self);
        let result = self.initialize_weather_body();
        super::production_trace::record_operation(
            "InitializeWeather",
            before,
            self,
            result.as_ref().err(),
        );
        result
    }

    fn initialize_weather_body(&mut self) -> Result<(), WeatherDayError> {
        if !self.available {
            return Err(WeatherDayError::admission(
                "weather environment has not been admitted",
            ));
        }
        if self.state.global.begin_sim_flag && self.state.weather.first_call {
            self.state.weather.first_call = false;
            self.state.environment.end_month_flag = false;
        }
        if self.state.global.begin_envrn_flag {
            self.initialize_missing_values();
            self.cursor.set_weather_code_missed_count(0);
            self.read_weather_day(RawDayReadMode::FirstDay, 1, false)?;
        }
        if self.state.global.begin_day_flag {
            update_weather_data(&mut self.state);
            self.today_raw = self.tomorrow_raw.clone();
            self.today_produced = self.tomorrow_produced.clone();
            if !self.state.global.warmup_flag
                && self.state.global.day_of_sim < self.state.global.num_of_day_in_envrn
            {
                self.read_weather_day(
                    RawDayReadMode::NextDay,
                    self.state.global.day_of_sim as usize + 1,
                    false,
                )?;
            }
            self.state.environment.end_year_flag = false;
            let month = self.state.environment.month as usize;
            let ends = [
                31,
                28 + self.state.weather.leap_year_add,
                31,
                30,
                31,
                30,
                31,
                31,
                30,
                31,
                30,
                31,
            ];
            if self.state.environment.day_of_month == ends[month - 1] {
                self.state.environment.end_month_flag = true;
                self.state.environment.end_year_flag = month == 12;
            }
            let next = self.state.tomorrow_variables;
            self.state.environment.year_tomorrow = next.year;
            self.state.environment.month_tomorrow = next.month;
            self.state.environment.day_of_month_tomorrow = next.day_of_month;
            self.state.environment.day_of_week_tomorrow = next.day_of_week;
            self.state.environment.holiday_index_tomorrow = next.holiday_index;
        }
        if !self.state.global.begin_day_flag
            && !self.state.global.warmup_flag
            && (self.state.environment.month != self.configuration.run_period.begin_month as i32
                || self.state.environment.day_of_month
                    != self.configuration.run_period.begin_day_of_month as i32)
            && !self.state.weather.dates_should_be_reset
        {
            self.state.weather.dates_should_be_reset = true;
        }
        if self.state.global.end_envrn_flag {
            self.cursor.rewind_and_skip_header()?;
        }
        self.state.global.end_design_day_envrns_flag = self.state.global.end_envrn_flag;
        self.state.weather.water_mains_parameter_report = false;
        Ok(())
    }

    fn initialize_missing_values(&mut self) {
        let missing = &mut self.state.missing_values;
        missing.base.out_baro_press = self.state.environment.std_baro_press;
        missing.base.out_dry_bulb_temp = 6.0;
        missing.base.out_dew_point_temp = 3.0;
        missing.base.out_rel_hum = 50.0;
        missing.base.wind_speed = 2.5;
        missing.base.wind_dir = 180.0;
        missing.base.total_sky_cover = 5.0;
        missing.base.opaque_sky_cover = 5.0;
        missing.visibility = 777.7;
        missing.ceiling = 77777.0;
        missing.aer_opt_depth = 0.0;
        missing.snow_depth = 0.0;
        missing.days_last_snow = 88;
        missing.base.albedo = 0.0;
        missing.base.liquid_precip = 0.0;
        self.state.missed_counts = Default::default();
        self.state.out_of_range_counts = Default::default();
        self.state.weather.is_rain_threshold = 0.8 / f64::from(self.configuration.steps());
        if !self.state.weather.rp_read_all_weather_data {
            self.print_environment_stamp = true;
        }
    }

    /// Performs a genuine first-day search or next-day read at the current cursor.
    pub fn read_weather_day(
        &mut self,
        mode: RawDayReadMode,
        calendar_day: usize,
        backspace_after_read: bool,
    ) -> Result<(), WeatherDayError> {
        let before = super::production_trace::snapshot(self);
        let prior_hour = self.tomorrow_produced.as_ref().map(|day| day.records[23]);
        let mut read_state = RawDayReadState {
            current_day_of_week: self.state.weather.cur_day_of_week,
            read_time: self.state.weather.read_e_plus_weather_cur_time,
            last_hour_set: self.state.weather.last_hour_set,
        };
        let selection = RawDayReadSelection {
            start_month: self.configuration.run_period.begin_month as i32,
            start_day: self.configuration.run_period.begin_day_of_month as i32,
            start_year: self
                .configuration
                .day_point(1)
                .map_or(0, |point| point.year as i32),
            match_year: false,
            days_in_year: 365 + self.state.weather.leap_year_add,
        };
        let mut sink = DayReadSink {
            state: &mut self.state,
            configuration: &self.configuration,
            calendar_day,
            current_cycle: &mut self.current_cycle,
            set_week_days: &mut self.set_week_days,
        };
        let raw_result = self.cursor.read_day(
            selection,
            &mut read_state,
            mode,
            backspace_after_read,
            &mut sink,
        );
        self.state.weather.cur_day_of_week = read_state.current_day_of_week;
        self.state.weather.read_e_plus_weather_cur_time = read_state.read_time;
        self.state.weather.last_hour_set = read_state.last_hour_set;
        self.state.missed_counts.weath_codes = self.cursor.parser_state().weather_code_missed_count;
        let result = match raw_result {
            Ok(raw) => produce_day(
                &raw,
                if mode == RawDayReadMode::FirstDay {
                    None
                } else {
                    prior_hour
                },
                self.configuration.steps(),
                self.configuration.first_policy(),
                &mut self.state,
            )
            .map(|produced| {
                self.tomorrow_raw = Some(raw);
                self.tomorrow_produced = Some(produced);
            }),
            Err(error) => Err(error.into()),
        };
        super::production_trace::record_operation(
            "ReadEPlusWeatherForDay",
            before,
            self,
            result.as_ref().err(),
        );
        result
    }

    pub(super) fn set_phase(&mut self, phase: WeatherDayPhase, begin_environment: bool) {
        let (warmup, day) = match phase {
            WeatherDayPhase::Warmup { day } => (true, day as i32),
            WeatherDayPhase::Run { day } => (false, day as i32),
        };
        self.state.global.begin_sim_flag = begin_environment;
        self.state.global.begin_envrn_flag = begin_environment;
        self.state.global.begin_day_flag = true;
        self.state.global.warmup_flag = warmup;
        self.state.global.day_of_sim = day;
        self.state.global.day_of_sim_chr = if warmup { "0".into() } else { day.to_string() };
        self.state.global.end_day_flag = false;
        self.state.global.end_envrn_flag = false;
        self.state.global.hour_of_day = 1;
        self.state.global.time_step = 1;
    }
}

struct DayReadSink<'a> {
    state: &'a mut WeatherDayState,
    configuration: &'a WeatherEnvironmentConfiguration,
    calendar_day: usize,
    current_cycle: &'a mut i32,
    set_week_days: &'a mut bool,
}

impl RawDayReadCallbacks for DayReadSink<'_> {
    fn first_day_selected(
        &mut self,
        state: &mut RawDayReadState,
        _slot: &RawWeatherSlot,
    ) -> Result<(), String> {
        *self.current_cycle += 1;
        if *self.current_cycle == 1 {
            let point = self
                .configuration
                .day_point(1)
                .ok_or("missing first calendar day")?;
            state.current_day_of_week = weekday_index(point.day_of_week) - 1;
            *self.set_week_days = true;
        } else {
            state.current_day_of_week = self.state.environment.day_of_week_tomorrow;
        }
        Ok(())
    }
    fn clear_source_interval(&mut self, hour: usize, interval: usize) -> Result<(), String> {
        let slots = self
            .state
            .tomorrow_values
            .hour_mut(hour)
            .map_err(|error| error.to_string())?;
        *slots
            .get_mut(interval - 1)
            .ok_or("weather input interval outside allocation")? = WeatherVars::default();
        Ok(())
    }
    fn parsed_hour(
        &mut self,
        state: &mut RawDayReadState,
        hour: usize,
        slot: &RawWeatherSlot,
    ) -> Result<(), String> {
        let raw = &slot.raw;
        let record = project_record(raw, hour).map_err(|error| error.to_string())?;
        if hour == 1 {
            if state.current_day_of_week <= 7 {
                state.current_day_of_week = state.current_day_of_week % 7 + 1;
            }
            let point = self
                .configuration
                .day_point(self.calendar_day)
                .ok_or("read beyond calendar environment")?;
            let day_of_year =
                ordinal(raw.dates[1], raw.dates[2], self.state.weather.leap_year_add)?;
            let (sin, cos, equation) =
                crate::heat_balance::solar::energyplus_daily_solar_coefficients(day_of_year as u32);
            self.state.tomorrow_variables = DailyWeatherVariables {
                day_of_year,
                day_of_year_schedule: ordinal(raw.dates[1], raw.dates[2], 1)?,
                year: raw.dates[0],
                month: raw.dates[1],
                day_of_month: raw.dates[2],
                day_of_week: state.current_day_of_week,
                daylight_saving_index: i32::from(point.dst),
                holiday_index: point
                    .special_day_type
                    .map_or(0, |day_type| day_type.energyplus_index() as i32),
                sin_solar_declin_angle: sin,
                cos_solar_declin_angle: cos,
                equation_of_time: equation,
            };
        }
        let hourly = process_hour(raw, record, self.state);
        self.state
            .tomorrow_values
            .hour_mut(hour)
            .map_err(|error| error.to_string())?[0] = hourly;
        Ok(())
    }
    fn complete_day(
        &mut self,
        _state: &mut RawDayReadState,
        _day: &RawWeatherDay,
    ) -> Result<(), String> {
        Ok(())
    }
}
