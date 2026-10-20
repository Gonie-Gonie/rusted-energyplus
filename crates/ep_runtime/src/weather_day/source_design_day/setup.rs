//! Genuine SetUpDesignDay3494–4040 on the same actual input/modifier/daily owners.
mod profiles;
mod properties;
mod reports;
use super::*;
use crate::heat_balance::error_backend::SourceDiagnosticFlags;
use crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::day::SourceWeatherInitializationError as Failure;
use crate::weather::day::{WeatherDayValues, WeatherEnvironmentState};
use chrono::{Datelike, Local};
use std::io::Write;

pub(crate) struct SourceDesignDaySetupOwners<'a, 'backend> {
    pub(crate) environments: &'a [SourceEnvironmentData],
    pub(crate) design_day_weather: &'a mut [DailyWeatherVariables],
    pub(crate) tomorrow_values: &'a mut WeatherDayValues,
    pub(crate) clock: &'a mut WeatherGlobalState,
    pub(crate) environment: &'a mut WeatherEnvironmentState,
    pub(crate) time_step_fraction: &'a f64,
    pub(crate) current_environment_number: &'a i32,
    pub(crate) print_design_day_header: &'a mut bool,
    pub(crate) report_flags: &'a SourceSolarReportFlags,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) developer_flag: &'a bool,
    pub(crate) psychrometrics: &'a mut SourcePsychrometricAccess,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) eio: &'a mut dyn Write,
}
impl SourceDesignDayInputState {
    pub(crate) fn set_up_source_design_day(
        &mut self,
        number: i32,
        o: &mut SourceDesignDaySetupOwners<'_, '_>,
    ) -> Result<(), Failure> {
        // Real HourlyWeatherData constructor scratch; not retained output/fixture seeds.
        let mut hourly_beam_solar_radiation = [0.0; 24];
        let mut hourly_diffuse_solar_radiation = [0.0; 24];
        let environment_row = o
            .environments
            .get(ordinal(number)?)
            .ok_or(Error::Unavailable(
                "actual SetUpDesignDay Environment row3537",
            ))?;
        let saved_warmup = o.clock.warmup_flag;
        o.clock.warmup_flag = true;
        crate::psychrometrics::set_psychrometric_warmup(o.clock.warmup_flag);
        // Actual live local system year, not CalendarYear or a configuration year.
        let current_year = Local::now().year();
        if o.clock.begin_sim_flag {
            *o.print_design_day_header = true;
        }
        let design_day =
            o.design_day_weather
                .get_mut(ordinal(number)?)
                .ok_or(Error::Unavailable(
                    "actual SetUpDesignDay DesignDay row3550",
                ))?;
        let input = self
            .design_day_input
            .as_mut()
            .and_then(|v| v.get_mut(ordinal(number).ok()?))
            .ok_or(Error::Unavailable(
                "actual SetUpDesignDay DesDayInput row3551",
            ))?;
        design_day.year = current_year;
        design_day.month = input.month;
        design_day.day_of_month = input.day_of_month;
        design_day.day_of_year =
            crate::weather::raw::source_ordinal_day(design_day.month, design_day.day_of_month, 0)?;
        o.environment.current_month_day = format!("{:02}/{:02}", input.month, input.day_of_month);
        o.environment.run_period_environment = false;
        o.environment.environment_start_end = format!(
            "{} - {}",
            o.environment.current_month_day, o.environment.current_month_day
        );
        let flags = SourceDiagnosticFlags {
            kickoff: &o.globals.kickoff_simulation,
            developer: o.developer_flag,
            warmup: &o.clock.warmup_flag,
            doing_sizing: &o.globals.doing_sizing,
        };
        correct_source_design_day_pressure(input, o.environment, &flags, o.backend)?;
        if input.humidity_indicator_type == 1 && input.dew_point_needs_set {
            let humidity = properties::from_relative_humidity(
                o.psychrometrics,
                input.maximum_dry_bulb,
                1.0,
                input.barometric_pressure,
            )?;
            input.humidity_indicator_value =
                properties::dew_point(o.psychrometrics, humidity, input.barometric_pressure)?;
        }
        design_day.day_of_week = 2;
        if input.day_type <= 7 {
            design_day.day_of_week = input.day_type;
        }
        design_day.holiday_index = 0;
        if input.day_type > 7 {
            design_day.holiday_index = input.day_type;
        }
        design_day.daylight_saving_index = input.daylight_saving_indicator;
        // Mutating pressure/dewpoint prefix has ended. Reborrow the same whole
        // input slice for Native3650's distinct global-Envrn WBProfDef lookup.
        let inputs = self
            .design_day_input
            .as_ref()
            .ok_or(Error::Unavailable("actual SetUpDesignDay input allocation"))?;
        let input = inputs
            .get(ordinal(number)?)
            .ok_or(Error::Unavailable("actual SetUpDesignDay input ordinal"))?;
        let coefficients = super::super::design_day::solar::source_daily_coefficients(
            design_day,
            o.environment.latitude,
        );
        reports::write(
            input,
            inputs,
            *o.current_environment_number,
            design_day,
            coefficients,
            o.print_design_day_header,
            o.report_flags,
            o.eio,
        )?;
        o.clock.current_time = 25.0;
        use profiles::HumidityState;
        let humidity = match input.humidity_indicator_type {
            0 => HumidityState::Constant(properties::from_wet_bulb(
                o.psychrometrics,
                input.maximum_dry_bulb,
                input.humidity_indicator_value,
                input.barometric_pressure,
            )?),
            1 => HumidityState::Constant(properties::from_dew_point(
                o.psychrometrics,
                input.humidity_indicator_value,
                input.barometric_pressure,
            )?),
            3 => HumidityState::Constant(input.humidity_indicator_value),
            2 => HumidityState::Constant(properties::from_enthalpy(
                o.psychrometrics,
                input.maximum_dry_bulb,
                input.humidity_indicator_value,
            )?),
            4 => {
                let modifiers = self
                    .design_day_modifiers
                    .as_ref()
                    .and_then(|v| v.get(ordinal(number).ok()?))
                    .and_then(Option::as_ref)
                    .ok_or(Error::Unavailable(
                        "actual DD scheduled humidity modifier array3700",
                    ))?;
                for hour in 1..=24 {
                    for timestep in 1..=o.clock.time_steps_in_hour {
                        profiles::slot_mut(o.tomorrow_values, hour, timestep)?.out_rel_hum =
                            modifiers.cell(timestep, hour)?.outdoor_relative_humidity;
                    }
                }
                HumidityState::Variable
            }
            5..=7 => HumidityState::Variable,
            _ => {
                if o.backend.state.warmup_flag != o.clock.warmup_flag {
                    return Err(Error::Unavailable("actual SetUp invalid-humidity diagnostic backend must borrow current DataGlobal WarmupFlag").into());
                }
                o.backend
                    .show_severe_error("SetUpDesignDay: Invalid Humidity Indicator type")?;
                o.backend
                    .show_continue_error(&format!("Occurred in Design Day={}", input.title))?;
                HumidityState::Uninitialized
            }
        };
        let opaque_sky;
        if input.rain_indicator != 0 {
            opaque_sky = 10.0;
            for hour in 1..=24 {
                for timestep in 1..=o.clock.time_steps_in_hour {
                    let slot = profiles::slot_mut(o.tomorrow_values, hour, timestep)?;
                    slot.is_rain = true;
                    slot.liquid_precip = 3.0;
                }
            }
        } else {
            opaque_sky = 0.0;
            for hour in 1..=24 {
                for timestep in 1..=o.clock.time_steps_in_hour {
                    let slot = profiles::slot_mut(o.tomorrow_values, hour, timestep)?;
                    slot.is_rain = false;
                    slot.liquid_precip = 0.0;
                }
            }
        }
        let _ground_reflectivity;
        if input.snow_indicator == 0 {
            _ground_reflectivity = 0.2;
            for hour in 1..=24 {
                for timestep in 1..=o.clock.time_steps_in_hour {
                    profiles::slot_mut(o.tomorrow_values, hour, timestep)?.is_snow = false;
                }
            }
        } else {
            _ground_reflectivity = 0.7;
            for hour in 1..=24 {
                for timestep in 1..=o.clock.time_steps_in_hour {
                    profiles::slot_mut(o.tomorrow_values, hour, timestep)?.is_snow = true;
                }
            }
        }
        for hour in 1..=24 {
            for timestep in 1..=o.clock.time_steps_in_hour {
                let slot = profiles::slot_mut(o.tomorrow_values, hour, timestep)?;
                slot.out_baro_press = input.barometric_pressure;
                slot.wind_speed = input.wind_speed;
                slot.wind_dir = input.wind_direction;
                slot.albedo = 0.0;
            }
        }
        let modifiers = self
            .design_day_modifiers
            .as_ref()
            .and_then(|v| v.get(ordinal(number).ok()?))
            .and_then(Option::as_ref)
            .ok_or(Error::Unavailable(
                "actual SetUpDesignDay modifier array3795",
            ))?;
        profiles::write(
            input,
            modifiers,
            design_day,
            coefficients,
            environment_row,
            opaque_sky,
            humidity,
            o.tomorrow_values,
            o.clock.time_steps_in_hour,
            o.time_step_fraction,
            o.environment,
            o.psychrometrics,
        )?;
        // Native3960–3980 performs real local hourly scratch backfill even though not consumed.
        for hour in 1..=24 {
            let previous_hour = (hour + 22) % 24 + 1;
            let now = profiles::slot(o.tomorrow_values, hour, o.clock.time_steps_in_hour)?;
            let previous =
                profiles::slot(o.tomorrow_values, previous_hour, o.clock.time_steps_in_hour)?;
            let mut beam = (previous.beam_solar_rad + now.beam_solar_rad) / 2.0;
            let mut diffuse = (previous.dif_solar_rad + now.dif_solar_rad) / 2.0;
            if o.clock.time_steps_in_hour > 1 {
                for timestep in 1..=o.clock.time_steps_in_hour - 1 {
                    let slot = profiles::slot(o.tomorrow_values, hour, timestep)?;
                    beam += slot.beam_solar_rad;
                    diffuse += slot.dif_solar_rad;
                }
            }
            hourly_beam_solar_radiation[ordinal(hour)?] =
                beam / f64::from(o.clock.time_steps_in_hour);
            hourly_diffuse_solar_radiation[ordinal(hour)?] =
                diffuse / f64::from(o.clock.time_steps_in_hour);
        }
        if environment_row.weather_properties_type_one != 0 {
            return Err(Error::Unavailable(
                "actual SetUpDesignDay WPSkyTemperature reference3983 before schedule child",
            )
            .into());
        }
        o.clock.warmup_flag = saved_warmup;
        crate::psychrometrics::set_psychrometric_warmup(o.clock.warmup_flag);
        Ok(())
    }
}

// The genuine WeatherManager3564–3579 pressure child retains its mutable
// DesDayInput cell. Diagnostic failures return before the Source assignment.
fn correct_source_design_day_pressure(
    input: &mut SourceDesignDayData,
    environment: &WeatherEnvironmentState,
    flags: &SourceDiagnosticFlags<'_>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    if input.pressure_entered {
        if ((input.barometric_pressure - environment.std_baro_press) / environment.std_baro_press)
            .abs()
            > 0.1
        {
            backend.show_source_warning_error(&format!("SetUpDesignDay: Entered DesignDay Barometric Pressure={} differs by more than 10% from Standard Barometric Pressure={}.", rounded(input.barometric_pressure, 0)?, rounded(environment.std_baro_press, 0)?), flags)?;
            backend.show_continue_error(&format!(
                "...occurs in DesignDay={}, Standard Pressure (based on elevation) will be used.",
                environment.environment_name
            ))?;
            input.barometric_pressure = environment.std_baro_press;
        }
    } else {
        input.barometric_pressure = environment.std_baro_press;
    }
    Ok(())
}

#[cfg(test)]
mod pressure_tests {
    use super::*;
    use crate::heat_balance::error_backend::{
        SourceDiagnosticFlags, SourceErrorBackend, SourceErrorState, SourceErrorTracking,
    };
    use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
    use crate::heat_balance::selected_project_data::SelectedProjectData;
    use std::io::{self, Write};

    struct StopAtContinue {
        bytes: Vec<u8>,
        fail: bool,
    }
    impl Write for StopAtContinue {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let mut candidate = self.bytes.clone();
            candidate.extend_from_slice(bytes);
            if self.fail
                && candidate
                    .windows(b" **   ~~~   ** ".len())
                    .any(|part| part == b" **   ~~~   ** ")
            {
                return Err(io::Error::other("authored Continue sink stop"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn source_error_state() -> SourceErrorState {
        SourceErrorState {
            output_error_header: true,
            version_string: "authored caller".into(),
            idd_version_string: "authored input".into(),
            doing_input_processing: false,
            print_console_output: true,
            kickoff_simulation: false,
            developer_flag: false,
            warmup_flag: false,
            doing_sizing: false,
            tracking: SourceErrorTracking {
                ask_for_plant_check_on_abort: false,
                exit_during_simulations: false,
                ask_for_connections_report: false,
                ask_for_surfaces_report: false,
                total_multiplied_windows: 0,
                total_coincident_vertices: 0,
                total_degenerate_surfaces: 0,
                total_receiving_non_convex_surfaces: 0,
                total_casting_non_convex_surfaces: 0,
                match_counts: [0; 20],
                abort_processing: false,
                total_severe_errors: 0,
                total_severe_errors_during_warmup: 0,
                total_severe_errors_during_sizing: 0,
                last_severe_error: String::new(),
                total_warning_errors: 0,
                total_warning_errors_during_warmup: 0,
                total_warning_errors_during_sizing: 0,
            },
        }
    }

    #[test]
    fn source_design_day_pressure_uses_live_phase_and_assignment_follows_continue()
    -> Result<(), Box<dyn std::error::Error>> {
        for (kickoff, sizing, fail_continue, expected_warmup, expected_sizing) in [
            (true, false, false, 0, 0),
            (false, false, false, 1, 0),
            (false, true, false, 0, 1),
            (true, false, true, 0, 0),
        ] {
            let mut input = SourceDesignDayData {
                pressure_entered: true,
                barometric_pressure: 81198.0,
                ..SourceDesignDayData::default()
            };
            let environment = WeatherEnvironmentState {
                std_baro_press: 100129.0,
                environment_name: "LIVE DD".into(),
                ..WeatherEnvironmentState::default()
            };
            let clock = WeatherGlobalState {
                warmup_flag: true,
                ..WeatherGlobalState::default()
            };
            let globals = SelectedHeatBalanceGlobals {
                kickoff_simulation: kickoff,
                doing_sizing: sizing,
                ..SelectedHeatBalanceGlobals::default()
            };
            let project = SelectedProjectData::default();
            let flags = SourceDiagnosticFlags {
                kickoff: &globals.kickoff_simulation,
                developer: &project.system.developer_flag,
                warmup: &clock.warmup_flag,
                doing_sizing: &globals.doing_sizing,
            };
            let mut state = source_error_state();
            let (mut sink, mut console) = (
                StopAtContinue {
                    bytes: Vec::new(),
                    fail: fail_continue,
                },
                Vec::new(),
            );
            let result = {
                let mut backend = SourceErrorBackend {
                    state: &mut state,
                    error_stream: Some(&mut sink),
                    console: &mut console,
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                };
                correct_source_design_day_pressure(&mut input, &environment, &flags, &mut backend)
            };
            let text = std::str::from_utf8(&sink.bytes)?;
            assert!(text.contains("Entered DesignDay Barometric Pressure=81198 differs by more than 10% from Standard Barometric Pressure=100129."));
            if fail_continue {
                assert!(matches!(result, Err(Error::Backend(_))));
                assert_eq!(input.barometric_pressure, 81198.0);
            } else {
                result?;
                assert!(text.contains("...occurs in DesignDay=LIVE DD, Standard Pressure (based on elevation) will be used."));
                assert_eq!(input.barometric_pressure, environment.std_baro_press);
            }
            assert_eq!(
                (
                    state.tracking.total_warning_errors,
                    state.tracking.total_warning_errors_during_warmup,
                    state.tracking.total_warning_errors_during_sizing
                ),
                (1, expected_warmup, expected_sizing)
            );
            assert!(
                !state.warmup_flag
                    && !state.doing_sizing
                    && !state.kickoff_simulation
                    && clock.warmup_flag
            );
        }
        Ok(())
    }
}
