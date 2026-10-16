use super::handoff::update_weather_data;
use super::state::{
    DailyWeatherVariables, ExtendedWeatherVars, WeatherDayState, WeatherDayValues,
    WeatherDayValuesError, WeatherEnvironmentState, WeatherGlobalState, WeatherOwnerState,
    WeatherVarCounts, WeatherVars,
};

fn real(seed: u64, field: u64) -> f64 {
    f64::from_bits(0x3ff0_0000_0000_0000 + (seed << 8) + field)
}

fn weather(seed: u64) -> WeatherVars {
    WeatherVars {
        is_rain: seed.is_multiple_of(2),
        is_snow: seed.is_multiple_of(3),
        out_dry_bulb_temp: real(seed, 0),
        out_dew_point_temp: real(seed, 1),
        out_baro_press: real(seed, 2),
        out_rel_hum: real(seed, 3),
        wind_speed: real(seed, 4),
        wind_dir: real(seed, 5),
        sky_temp: real(seed, 6),
        horiz_ir_sky: real(seed, 7),
        beam_solar_rad: real(seed, 8),
        dif_solar_rad: real(seed, 9),
        albedo: real(seed, 10),
        water_precip: real(seed, 11),
        liquid_precip: real(seed, 12),
        total_sky_cover: if seed == 7 { -0.0 } else { real(seed, 13) },
        opaque_sky_cover: real(seed, 14),
    }
}

fn daily(seed: i32, holiday_index: i32) -> DailyWeatherVariables {
    DailyWeatherVariables {
        day_of_year: seed + 1,
        day_of_year_schedule: seed + 2,
        year: seed + 3,
        month: seed + 4,
        day_of_month: seed + 5,
        day_of_week: seed + 6,
        daylight_saving_index: seed + 7,
        holiday_index,
        sin_solar_declin_angle: -0.0,
        cos_solar_declin_angle: real(seed as u64, 1),
        equation_of_time: real(seed as u64, 2),
    }
}

fn counts(seed: i32) -> WeatherVarCounts {
    WeatherVarCounts {
        out_dry_bulb_temp: seed,
        out_dew_point_temp: seed + 1,
        out_rel_hum: seed + 2,
        out_baro_press: seed + 3,
        wind_dir: seed + 4,
        wind_speed: seed + 5,
        beam_solar_rad: seed + 6,
        dif_solar_rad: seed + 7,
        total_sky_cover: seed + 8,
        opaque_sky_cover: seed + 9,
        visibility: seed + 10,
        ceiling: seed + 11,
        liquid_precip: seed + 12,
        water_precip: seed + 13,
        aer_opt_depth: seed + 14,
        snow_depth: seed + 15,
        days_last_snow: seed + 16,
        weath_codes: seed + 17,
        albedo: seed + 18,
    }
}

fn canary_state(begin_envrn: bool, holiday: i32) -> WeatherDayState {
    let mut today_values = WeatherDayValues::allocated(4).unwrap();
    let mut tomorrow_values = WeatherDayValues::allocated(4).unwrap();
    for (index, slot) in today_values.slots_mut().iter_mut().enumerate() {
        *slot = weather(200 + index as u64);
    }
    for (index, slot) in tomorrow_values.slots_mut().iter_mut().enumerate() {
        *slot = weather(1 + index as u64);
    }
    WeatherDayState {
        today_variables: daily(400, 7),
        tomorrow_variables: daily(500, holiday),
        today_values,
        tomorrow_values,
        last_hour: weather(700),
        next_hour: weather(701),
        missing_values: ExtendedWeatherVars {
            base: weather(702),
            visibility: real(703, 0),
            ceiling: -0.0,
            aer_opt_depth: real(703, 2),
            snow_depth: real(703, 3),
            days_last_snow: 704,
        },
        missed_counts: counts(800),
        out_of_range_counts: counts(900),
        ground_temperature_models: Default::default(),
        global: WeatherGlobalState {
            begin_sim_flag: true,
            begin_envrn_flag: begin_envrn,
            begin_day_flag: true,
            begin_hour_flag: true,
            begin_time_step_flag: true,
            end_day_flag: true,
            end_hour_flag: true,
            end_envrn_flag: true,
            end_design_day_envrns_flag: true,
            warmup_flag: true,
            day_of_sim: 11,
            calendar_year: 12,
            previous_hour: 13,
            hour_of_day: 14,
            num_of_day_in_envrn: 15,
            time_steps_in_hour: 4,
            time_step: 3,
            kind_of_sim: 3,
            do_weath_sim: true,
            weather_simulation_requested: true,
            do_des_day_sim: true,
            do_output_reporting: true,
            time_step_zone: -0.0,
            time_step_zone_sec: real(301, 0),
            minutes_in_time_step: 302,
            weight_now: real(301, 1),
            weight_previous_hour: real(301, 2),
            current_time: -0.0,
            sim_time_steps: 303,
            day_of_sim_chr: "daily-copy-canary".into(),
            calendar_year_chr: "calendar-canary".into(),
        },
        environment: WeatherEnvironmentState {
            outdoor_environment_parameters: crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentParameters {
                site_temperature_gradient_k_per_m: real(311, 0),
                weather_file_temperature_modifier_c: -0.0,
                site_wind_exponent: real(311, 1),
                site_wind_boundary_layer_height_m: real(311, 2),
                weather_file_wind_modifier: real(311, 3),
            },
            source_weather_station_initialized: true,
            source_site_atmosphere_initialized: true,
            force_begin_env_reset_suppress: true,
            previous_sol_rad_positive: true,
            cloud_fraction: real(315, 0),
            exterior_horizontal_sky_illuminance: real(315, 1),
            exterior_horizontal_beam_illuminance: real(315, 2),
            exterior_beam_normal_illuminance: real(315, 3),
            direct_solar_luminous_efficacy: real(315, 4),
            diffuse_solar_luminous_efficacy: real(315, 5),
            daylighting_sky_clearness: real(315, 6),
            daylighting_sky_brightness: real(315, 7),

            ground_temp: [0.0;4],
            ground_temp_kelvin: 0.0,
            ground_temp_inputs: [false;4],
            water_mains_temperature: real(311, 4),
            ems_out_dry_bulb_override_on: true,
            ems_out_dry_bulb_override_value: real(316, 0),
            ems_out_dew_point_temp_override_on: true,
            ems_out_dew_point_temp_override_value: real(316, 1),
            ems_out_rel_hum_override_on: true,
            ems_out_rel_hum_override_value: real(316, 2),
            ems_wind_speed_override_on: true,
            ems_wind_speed_override_value: real(316, 3),
            ems_wind_dir_override_on: true,
            ems_wind_dir_override_value: real(316, 4),
            ems_dif_solar_rad_override_on: true,
            ems_dif_solar_rad_override_value: real(316, 5),
            ems_beam_solar_rad_override_on: true,
            ems_beam_solar_rad_override_value: real(316, 6),
            current_weather: super::CurrentWeatherState {
                out_dry_bulb_temp: -0.0,
                out_dew_point_temp: real(304, 0),
                out_baro_press: real(304, 1),
                out_rel_hum: real(304, 2),
                out_rel_hum_value: real(304, 3),
                out_hum_rat: real(304, 4),
                out_wet_bulb_temp: real(304, 5),
                wind_speed: real(304, 6),
                wind_dir: real(304, 7),
                liquid_precipitation: real(304, 8),
                is_rain: true,
                sun_is_up: true,
                sun_is_up_previous_timestep: false,
                solar_direction_cosines: [-0.0, real(308, 0), real(308, 1)],
                beam_solar_rad: real(308, 2),
                dif_solar_rad: real(308, 3),
                gnd_solar_rad: real(308, 4),
                gnd_reflectance: real(308, 5),
                gnd_reflectance_for_daylighting: real(308, 6),
                sky_temp: real(308, 7),
                sky_temp_kelvin: real(308, 8),
                is_snow: true,
                total_cloud_cover: real(308, 9),
                opaque_cloud_cover: real(308, 10),
                out_enthalpy: real(304, 9),
                out_air_density: real(304, 10),
            },
            display_weather_missing_data_warnings: true,
            ignore_solar_radiation: false,
            ignore_beam_radiation: true,
            ignore_diffuse_radiation: false,
            print_environment_stamp_warmup: true,
            print_environment_stamp_warmup_printed: true,
            day_of_year: 101,
            day_of_year_schedule: 102,
            year: 103,
            month: 104,
            day_of_month: 105,
            day_of_week: 106,
            holiday_index: 107,
            dst_indicator: 108,
            year_tomorrow: 109,
            month_tomorrow: 110,
            day_of_month_tomorrow: 111,
            day_of_week_tomorrow: 112,
            holiday_index_tomorrow: 113,
            tot_des_days: 114,
            current_overall_sim_day: 330,
            total_overall_sim_days: 331,
            max_number_sim_years: 332,
            start_year: 333,
            end_year: 334,
            current_year_is_leap_year: true,
            cur_envir_num: 115,
            run_period_start_day_of_week: 116,
            sin_solar_declin_angle: real(100, 0),
            cos_solar_declin_angle: real(100, 1),
            equation_of_time: real(100, 2),
            latitude: -0.0,
            longitude: real(100, 4),
            time_zone_number: real(313, 0),
            rho_air_stp: real(313, 1),
            varying_location_latitude_schedule: Some(crate::heat_balance::selected_schedule_manager::SourceScheduleIndex(7)),
            varying_location_longitude_schedule: Some(crate::heat_balance::selected_schedule_manager::SourceScheduleIndex(8)),
            varying_orientation_schedule: Some(crate::heat_balance::selected_schedule_manager::SourceScheduleIndex(9)),
            time_zone_meridian: real(309, 0),
            sin_latitude: -0.0,
            cos_latitude: real(309, 1),
            one_step_solar_time_offset: real(309, 2),
            elevation: real(100, 5),
            std_baro_press: real(100, 6),
            std_rho_air: real(300, 6),
            environment_name: "environment-canary".into(),
            current_month_day: "current-month-day-canary".into(),
            current_month_day_hour: "current-month-day-hour-canary".into(),
            current_month_day_year: "current-month-day-year-canary".into(),
            environment_start_end: "environment-start-end-canary".into(),
            run_period_environment: true,
            end_month_flag: true,
            end_year_flag: true,
        },
        weather: WeatherOwnerState {
            location_title: "location-title-canary".into(),
            location_gathered: true,
            keep_user_site_location_definition: true,
            environment_report_number: 314,
            environment_report_character: "report-number-canary".into(),
            source_environments: Some(vec![super::SourceEnvironmentData {title:"environment-row-canary".into(),..super::SourceEnvironmentData::default()}]),
            source_design_day_weather: Some(vec![DailyWeatherVariables {year:905,..DailyWeatherVariables::default()}]),
            source_run_period_input: Some(vec![super::source_run_period::SourceRunPeriodData {title:"run-period-canary".into(),..Default::default()}]),
            underwater_boundaries: vec![super::source_primary::SourceUnderwaterBoundary { name: "underwater-row-canary".into(), distance_from_leading_edge: real(310, 0), oscm_index: 311, water_temperature_schedule: None, velocity_schedule: None }],
            source_weather_properties: Some(vec![super::source_user_input::SourceWeatherProperty {name:"weather-property-canary".into(),..Default::default()}]),
            source_design_day_input: super::source_design_day::SourceDesignDayInputState { sizing_period_site_schedule_numbers: vec![906], ..Default::default() },
            number_weather_properties: 304,
            envrn: 201,
            num_of_envrn: 202,
            tot_run_pers: 203,
            tot_run_des_pers: 204,
            tot_report_periods: 34,
            tot_thermal_report_periods: 31,
            tot_co2_report_periods: 32,
            tot_visual_report_periods: 33,
            num_data_periods: 205,
            num_intervals_per_hour: 1,
            num_special_days: 207,
            leap_year_add: 208,
            rpt_day_type: 209,
            cur_day_of_week: 210,
            cur_sim_day_for_end_of_run_period: 211,
            get_branch_input_one_time_flag: false,
            get_environment_first_call: false,
            first_call: false,
            year_of_sim: 1903,
            number_days_in_year: 1307,
            print_design_day_header: true,
            print_environment_headers: false,
            weekday_types: [335; 366],
            special_day_types: [336; 366],
            dst_index: [337; 366],
            end_day_of_month_with_leap_day: [338; 12],
            water_mains_parameter_report: false,
            last_hour_set: true,
            weather_file_exists: true,
            dates_should_be_reset: true,
            start_dates_cycle_should_be_reset: true,
            jan1_dates_should_be_reset: true,
            rp_read_all_weather_data: true,
            use_daylight_saving: false,
            use_special_days: false,
            daylight_saving_is_active: true,
            read_e_plus_weather_cur_time: -0.0,
            time_step_fraction: real(200, 1),
            is_rain_threshold: real(200, 2),
            interpolation: Some(vec![real(305, 0), -0.0]),
            solar_interpolation: Some(vec![real(305, 1)]),
            next_hour: 306,
            rpt_is_rain: 307,
            rpt_is_snow: 310,
            hour_angle: real(309, 3),
            solar_altitude_angle: real(309, 4),
            solar_azimuth_angle: real(309, 5),
            horizontal_infrared_sky: -0.0,
            ground_reflectances: [real(309, 6); 12],
            snow_ground_reflectance_modifier: real(309, 7),
            snow_ground_reflectance_for_daylighting_modifier: real(309, 8),
            water_mains_temperature_method: 309,
            water_mains_annual_average_air_temperature: real(312, 0),
            water_mains_maximum_air_temperature_difference: real(312, 1),
            water_mains_temperature_multiplier: real(312, 2),
            water_mains_temperature_offset: real(312, 3),
            use_rain_values: false,
            use_snow_values: false,
        },
    }
}

fn assert_weather_bits(actual: &WeatherVars, original: &WeatherVars) {
    assert_eq!(actual.boolean_values(), original.boolean_values());
    assert_eq!(
        actual.real_values().map(f64::to_bits),
        original.real_values().map(f64::to_bits)
    );
}

fn assert_daily_bits(actual: &DailyWeatherVariables, original: &DailyWeatherVariables) {
    assert_eq!(actual.integer_values(), original.integer_values());
    assert_eq!(
        actual.real_values().map(f64::to_bits),
        original.real_values().map(f64::to_bits)
    );
}

fn assert_grid_bits(actual: &WeatherDayValues, original: &WeatherDayValues) {
    assert_eq!(actual.is_allocated(), original.is_allocated());
    assert_eq!(actual.time_steps(), original.time_steps());
    assert_eq!(actual.hours(), original.hours());
    assert_eq!(actual.slots().len(), original.slots().len());
    for (a, b) in actual.slots().iter().zip(original.slots()) {
        assert_weather_bits(a, b);
    }
}

#[test]
fn whole_day_transport_preserves_every_slot_and_unowned_canary() {
    for (begin_envrn, holiday, expected_report_type, expected_previous_hour) in
        [(true, 5, 5, 24), (false, 0, 506, 13), (false, -7, 506, 13)]
    {
        let mut state = canary_state(begin_envrn, holiday);
        let before = state.clone();
        update_weather_data(&mut state);

        assert_daily_bits(&state.today_variables, &before.tomorrow_variables);
        assert_grid_bits(&state.today_values, &before.tomorrow_values);
        assert_daily_bits(&state.tomorrow_variables, &before.tomorrow_variables);
        assert_grid_bits(&state.tomorrow_values, &before.tomorrow_values);
        assert_eq!(state.today_values.slots().len(), 96);
        assert_eq!(
            state.today_values.slots()[6].total_sky_cover.to_bits(),
            (-0.0_f64).to_bits()
        );
        assert_weather_bits(&state.last_hour, &before.last_hour);
        assert_weather_bits(&state.next_hour, &before.next_hour);
        assert_weather_bits(&state.missing_values.base, &before.missing_values.base);
        assert_eq!(
            state.missing_values.extra_real_values().map(f64::to_bits),
            before.missing_values.extra_real_values().map(f64::to_bits)
        );
        assert_eq!(
            state.missing_values.days_last_snow,
            before.missing_values.days_last_snow
        );
        assert_eq!(state.missed_counts, before.missed_counts);
        assert_eq!(state.out_of_range_counts, before.out_of_range_counts);

        let mut globals = before.global.clone();
        globals.previous_hour = expected_previous_hour;
        assert_eq!(state.global, globals);
        assert_eq!(
            state.global.time_step_zone.to_bits(),
            before.global.time_step_zone.to_bits()
        );
        let mut owner = before.weather.clone();
        owner.rpt_day_type = expected_report_type;
        assert_eq!(state.weather, owner);
        assert_eq!(
            state.weather.real_values().map(f64::to_bits),
            before.weather.real_values().map(f64::to_bits)
        );

        let current = &state.environment;
        assert_eq!(
            [
                current.day_of_year,
                current.year,
                current.month,
                current.day_of_month,
                current.day_of_week,
                current.holiday_index,
                current.dst_indicator
            ],
            [501, 503, 504, 505, 506, holiday, 507]
        );
        assert_eq!(
            current.real_values()[..3]
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            before.tomorrow_variables.real_values().map(f64::to_bits)
        );
        let mut unchanged = current.clone();
        unchanged.day_of_year = before.environment.day_of_year;
        unchanged.year = before.environment.year;
        unchanged.month = before.environment.month;
        unchanged.day_of_month = before.environment.day_of_month;
        unchanged.day_of_week = before.environment.day_of_week;
        unchanged.holiday_index = before.environment.holiday_index;
        unchanged.dst_indicator = before.environment.dst_indicator;
        unchanged.sin_solar_declin_angle = before.environment.sin_solar_declin_angle;
        unchanged.cos_solar_declin_angle = before.environment.cos_solar_declin_angle;
        unchanged.equation_of_time = before.environment.equation_of_time;
        assert_eq!(unchanged, before.environment);
        assert_eq!(
            current.real_values()[3..]
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            before.environment.real_values()[3..]
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn hour_access_preserves_other_intervals_and_has_native_slot_order() {
    let mut values = WeatherDayValues::allocated(4).unwrap();
    for (index, slot) in values.slots_mut().iter_mut().enumerate() {
        *slot = weather(index as u64 + 1);
    }
    let before = values.clone();
    // Native interval-one pre-clear: caller changes exactly one slot, not the whole hour.
    values.hour_mut(2).unwrap()[0] = WeatherVars::default();
    for index in 0..96 {
        let expected = if index == 4 {
            WeatherVars::default()
        } else {
            before.slots()[index]
        };
        assert_weather_bits(&values.slots()[index], &expected);
    }
    assert_weather_bits(&values.hour(24).unwrap()[3], &before.slots()[95]);
    assert_eq!(
        values.hour(0),
        Err(WeatherDayValuesError::HourOutsideDay(0))
    );
    assert_eq!(
        values.hour_mut(25),
        Err(WeatherDayValuesError::HourOutsideDay(25))
    );
}

#[test]
fn source_constructor_defaults_and_allocation_are_distinct() {
    let state = WeatherDayState::default();
    assert_eq!(state.today_variables.integer_values(), [0; 8]);
    assert_eq!(
        state.today_variables.real_values().map(f64::to_bits),
        [0; 3]
    );
    assert_eq!(state.last_hour.boolean_values(), [false; 2]);
    assert_eq!(state.last_hour.real_values().map(f64::to_bits), [0; 15]);
    assert_eq!(
        state.missing_values.extra_real_values().map(f64::to_bits),
        [0; 4]
    );
    assert_eq!(state.missed_counts.integer_values(), [0; 19]);
    assert_eq!(state.out_of_range_counts.integer_values(), [0; 19]);
    assert!(!state.today_values.is_allocated());
    assert_eq!(
        (state.today_values.time_steps(), state.today_values.hours()),
        (0, 0)
    );
    assert!(state.today_values.slots().is_empty());
    assert_eq!(
        state.today_values.hour(1),
        Err(WeatherDayValuesError::Unallocated)
    );
    assert!(!state.environment.source_weather_station_initialized);
    assert!(!state.environment.source_site_atmosphere_initialized);
    let parameters = state.environment.outdoor_environment_parameters;
    assert_eq!(
        [
            parameters.site_temperature_gradient_k_per_m,
            parameters.weather_file_temperature_modifier_c,
            parameters.site_wind_exponent,
            parameters.site_wind_boundary_layer_height_m,
            parameters.weather_file_wind_modifier,
        ]
        .map(f64::to_bits),
        [0.0065_f64, 0.0, 0.22, 370.0, 1.5863].map(f64::to_bits)
    );
    assert_eq!(state.global.kind_of_sim, -1);
    assert_eq!(state.global.day_of_sim_chr, "0");
    assert!(state.global.calendar_year_chr.is_empty());
    assert_eq!(
        state.environment.std_baro_press.to_bits(),
        101325.0_f64.to_bits()
    );
    assert!(
        state.weather.get_environment_first_call && state.weather.get_branch_input_one_time_flag
    );
    assert!(state.weather.first_call && state.weather.water_mains_parameter_report);
    assert!(state.weather.use_daylight_saving && state.weather.use_special_days);
    assert_eq!(state.weather.num_intervals_per_hour, 1);
    assert_eq!(state.weather.cur_day_of_week, 1);
    assert_eq!(
        state.weather.read_e_plus_weather_cur_time.to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(state.weather.is_rain_threshold.to_bits(), 0.8_f64.to_bits());
    assert_eq!(
        WeatherDayValues::allocated(0),
        Err(WeatherDayValuesError::ZeroTimeSteps)
    );
    assert_eq!(
        WeatherDayValues::allocated(usize::MAX),
        Err(WeatherDayValuesError::SizeOverflow)
    );
    let allocated = WeatherDayValues::allocated(4).unwrap();
    assert_eq!(
        (
            allocated.time_steps(),
            allocated.hours(),
            allocated.slots().len()
        ),
        (4, 24, 96)
    );
    for value in allocated.slots() {
        assert_weather_bits(value, &WeatherVars::default());
    }
}

#[test]
fn whole_owner_copy_retains_unallocated_and_reallocated_shapes() {
    let mut state = WeatherDayState {
        today_values: WeatherDayValues::allocated(4).unwrap(),
        ..WeatherDayState::default()
    };
    update_weather_data(&mut state);
    assert!(!state.today_values.is_allocated());
    state.tomorrow_values = WeatherDayValues::allocated(1).unwrap();
    state.tomorrow_values.hour_mut(24).unwrap()[0] = weather(7);
    update_weather_data(&mut state);
    assert_grid_bits(&state.today_values, &state.tomorrow_values);
    assert_eq!(
        (state.today_values.time_steps(), state.today_values.hours()),
        (1, 24)
    );
    state.today_values.hour_mut(24).unwrap()[0].is_rain = true;
    assert!(!state.tomorrow_values.hour(24).unwrap()[0].is_rain);
}
