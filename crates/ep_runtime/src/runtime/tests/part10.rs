fn test_run_period(
    name: &str,
    begin_month: u32,
    begin_day_of_month: u32,
    end_month: u32,
    end_day_of_month: u32,
) -> RunPeriod {
    RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new(name),
        begin_month,
        begin_day_of_month,
        begin_year: None,
        end_month,
        end_day_of_month,
        end_year: None,
        day_of_week_for_start_day: None,
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        use_weather_file_holidays_and_special_days: true,
        use_weather_file_daylight_saving_period: true,
        apply_weekend_holiday_rule: true,
        use_weather_file_rain_indicators: true,
        use_weather_file_snow_indicators: true,
        treat_weather_as_actual: false,
    }
}

#[test]
fn hourly_timestamp_label_uses_projected_calendar_state() -> Result<(), Box<dyn std::error::Error>>
{
    let axis = build_hourly_time_axis_for_run_period(&test_run_period(
        "Timestamp Environment",
        1,
        1,
        1,
        1,
    ))?;

    assert_eq!(
        normalized_hourly_timestamp_label(&axis, &axis.points[0]),
        "env=TIMESTAMP ENVIRONMENT;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
    );

    Ok(())
}

#[test]
fn run_period_calendar_uses_energyplus_yearless_weekday_lookup_and_correction()
-> Result<(), Box<dyn std::error::Error>> {
    let mut yearless = test_run_period("Yearless Monday", 1, 1, 1, 1);
    yearless.day_of_week_for_start_day = Some(DayOfWeek::Monday);

    let resolved = resolve_run_period_calendar(&yearless)?;

    assert_eq!(resolved.start_year, 2007);
    assert_eq!(resolved.end_year, 2007);
    assert_eq!(resolved.start_day_of_week, DayOfWeek::Monday);

    yearless.begin_year = Some(2020);
    yearless.end_year = Some(2020);
    let corrected = resolve_run_period_calendar(&yearless)?;

    assert_eq!(corrected.start_year, 2020);
    assert_eq!(corrected.start_day_of_week, DayOfWeek::Wednesday);

    let default_leap =
        resolve_run_period_calendar(&test_run_period("Yearless Leap Default", 2, 29, 2, 29))?;
    assert_eq!(default_leap.start_year, 2012);
    assert_eq!(default_leap.start_day_of_week, DayOfWeek::Wednesday);

    let mut weekday_leap = test_run_period("Yearless Leap Monday", 2, 29, 2, 29);
    weekday_leap.day_of_week_for_start_day = Some(DayOfWeek::Monday);
    let resolved_leap = resolve_run_period_calendar(&weekday_leap)?;
    assert_eq!(resolved_leap.start_year, 2016);
    assert_eq!(resolved_leap.start_day_of_week, DayOfWeek::Monday);

    Ok(())
}

#[test]
fn run_period_calendar_rolls_omitted_end_year_and_finds_next_leap_day()
-> Result<(), Box<dyn std::error::Error>> {
    let cross_year = resolve_run_period_calendar(&test_run_period("Cross Year", 12, 31, 1, 1))?;

    assert_eq!(cross_year.start_year, 2017);
    assert_eq!(cross_year.end_year, 2018);
    assert_eq!(cross_year.total_days, 2);

    let next_leap = resolve_run_period_calendar(&test_run_period("Next Leap", 3, 1, 2, 29))?;

    assert_eq!(next_leap.start_year, 2017);
    assert_eq!(next_leap.end_year, 2020);
    assert!(next_leap.end_year_is_leap_year);

    let mut leap_transition = test_run_period("Leap Year Transition", 12, 31, 1, 1);
    leap_transition.begin_year = Some(2019);
    leap_transition.end_year = Some(2020);
    let transition_axis = build_hourly_time_axis_for_run_period(&leap_transition)?;
    assert!(!transition_axis.points[0].gregorian_year_is_leap_year);
    assert!(transition_axis.points[24].gregorian_year_is_leap_year);

    Ok(())
}

#[test]
fn weather_calendar_separates_gregorian_weather_and_schedule_leap_state()
-> Result<(), Box<dyn std::error::Error>> {
    let mut run_period = test_run_period("Leap Policy Window", 2, 28, 3, 1);
    run_period.begin_year = Some(2016);
    run_period.end_year = Some(2016);
    run_period.day_of_week_for_start_day = Some(DayOfWeek::Sunday);

    let leap_metadata = EpwCalendarMetadata {
        leap_year_observed: true,
    };
    let non_leap_metadata = EpwCalendarMetadata {
        leap_year_observed: false,
    };
    let gregorian_axis = build_hourly_time_axis_for_run_period(&run_period)?;
    let leap_axis =
        build_hourly_time_axis_for_run_period_with_weather_metadata(&run_period, &leap_metadata)?;
    let non_leap_axis = build_hourly_time_axis_for_run_period_with_weather_metadata(
        &run_period,
        &non_leap_metadata,
    )?;

    assert_eq!(gregorian_axis.sample_count(), 72);
    assert!(gregorian_axis.weather_calendar.is_none());
    assert_eq!(leap_axis.sample_count(), 72);
    assert_eq!(non_leap_axis.sample_count(), 48);

    let leap_calendar = leap_axis
        .weather_calendar
        .as_ref()
        .expect("leap-aware axis carries weather calendar state");
    assert_eq!(leap_calendar.gregorian.total_days, 3);
    assert_eq!(leap_calendar.total_days, 3);
    assert_eq!(leap_calendar.leap_days_skipped, 0);
    assert!(leap_calendar.weather_file_allows_leap_years);
    assert!(leap_calendar.start_year_is_weather_effective_leap_year);

    let non_leap_calendar = non_leap_axis
        .weather_calendar
        .as_ref()
        .expect("non-leap axis carries weather calendar state");
    assert_eq!(non_leap_calendar.gregorian.total_days, 3);
    assert_eq!(non_leap_calendar.total_days, 2);
    assert_eq!(non_leap_calendar.leap_days_skipped, 1);
    assert!(!non_leap_calendar.weather_file_allows_leap_years);
    assert!(!non_leap_calendar.start_year_is_weather_effective_leap_year);

    let leap_march_first = &leap_axis.points[48];
    assert_eq!(leap_march_first.day_of_sim, 3);
    assert_eq!(leap_march_first.day_of_month, 1);
    assert_eq!(leap_march_first.gregorian_day_of_year, 61);
    assert_eq!(leap_march_first.day_of_year, 61);
    assert_eq!(leap_march_first.schedule_day_of_year, 61);
    assert_eq!(leap_march_first.gregorian_day_of_week, DayOfWeek::Tuesday);
    assert_eq!(leap_march_first.day_of_week, DayOfWeek::Tuesday);
    assert!(leap_march_first.weather_effective_year_is_leap_year);
    assert_eq!(leap_march_first.leap_year_add, 1);

    assert!(
        non_leap_axis
            .points
            .iter()
            .all(|point| !(point.month == 2 && point.day_of_month == 29))
    );
    let non_leap_march_first = &non_leap_axis.points[24];
    assert_eq!(non_leap_march_first.day_of_sim, 2);
    assert_eq!(non_leap_march_first.day_of_month, 1);
    assert_eq!(non_leap_march_first.gregorian_day_of_year, 61);
    assert_eq!(non_leap_march_first.day_of_year, 60);
    assert_eq!(non_leap_march_first.schedule_day_of_year, 61);
    assert_eq!(
        non_leap_march_first.gregorian_day_of_week,
        DayOfWeek::Tuesday
    );
    assert_eq!(non_leap_march_first.day_of_week, DayOfWeek::Monday);
    assert_eq!(non_leap_march_first.day_type.label(), "Monday");
    assert!(non_leap_march_first.gregorian_year_is_leap_year);
    assert!(!non_leap_march_first.weather_effective_year_is_leap_year);
    assert_eq!(non_leap_march_first.leap_year_add, 0);
    assert_eq!(
        normalized_hourly_timestamp_label(&non_leap_axis, non_leap_march_first),
        "env=LEAP POLICY WINDOW;day=2;month=3;date=1;dst=0;hour=1;start=0.00;end=60.00;day_type=Monday"
    );

    let model = TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 4,
        },
        run_periods: vec![run_period],
        ..TypedModel::default()
    };
    let leap_environments =
        build_environment_time_axes_with_weather_metadata(&model, &leap_metadata)?;
    let non_leap_environments =
        build_environment_time_axes_with_weather_metadata(&model, &non_leap_metadata)?;
    assert_eq!(leap_environments[0].sample_count(), 288);
    assert_eq!(non_leap_environments[0].sample_count(), 192);
    assert!(non_leap_environments[0].points[0].begin_environment);
    assert!(
        non_leap_environments[0]
            .points
            .last()
            .expect("environment has a final point")
            .end_environment
    );

    Ok(())
}

#[test]
fn weather_calendar_uses_non_leap_ordinals_for_february_29_endpoints()
-> Result<(), Box<dyn std::error::Error>> {
    let metadata = EpwCalendarMetadata {
        leap_year_observed: false,
    };
    let mut february_29_only = test_run_period("February 29 Only", 2, 29, 2, 29);
    february_29_only.begin_year = Some(2016);
    february_29_only.end_year = Some(2016);

    let one_day =
        build_hourly_time_axis_for_run_period_with_weather_metadata(&february_29_only, &metadata)?;

    assert_eq!(one_day.sample_count(), 24);
    assert_eq!(
        one_day
            .weather_calendar
            .as_ref()
            .expect("weather calendar")
            .total_days,
        1
    );
    assert_eq!(
        one_day
            .weather_calendar
            .as_ref()
            .expect("weather calendar")
            .leap_days_skipped,
        1
    );
    let march_first = &one_day.points[0];
    assert_eq!((march_first.month, march_first.day_of_month), (3, 1));
    assert_eq!(march_first.day_of_sim, 1);
    assert_eq!(march_first.gregorian_day_of_year, 61);
    assert_eq!(march_first.day_of_year, 60);
    assert_eq!(march_first.schedule_day_of_year, 61);
    assert_eq!(march_first.gregorian_day_of_week, DayOfWeek::Tuesday);
    assert_eq!(march_first.day_of_week, DayOfWeek::Monday);
    assert_eq!(march_first.leap_year_add, 0);

    let mut february_29_to_march_1 = test_run_period("February 29 To March 1", 2, 29, 3, 1);
    february_29_to_march_1.begin_year = Some(2016);
    february_29_to_march_1.end_year = Some(2016);
    let aliased_range = build_hourly_time_axis_for_run_period_with_weather_metadata(
        &february_29_to_march_1,
        &metadata,
    )?;

    assert_eq!(aliased_range.sample_count(), 24);
    assert_eq!(
        (
            aliased_range.points[0].month,
            aliased_range.points[0].day_of_month
        ),
        (3, 1)
    );
    assert_eq!(aliased_range.points[0].day_of_sim, 1);
    assert_eq!(aliased_range.points[0].day_of_week, DayOfWeek::Monday);

    let mut through_february_29 = test_run_period("Through February 29", 2, 28, 2, 29);
    through_february_29.begin_year = Some(2016);
    through_february_29.end_year = Some(2016);
    let two_days = build_hourly_time_axis_for_run_period_with_weather_metadata(
        &through_february_29,
        &metadata,
    )?;

    assert_eq!(two_days.sample_count(), 48);
    assert!(
        two_days
            .points
            .iter()
            .all(|point| !(point.month == 2 && point.day_of_month == 29))
    );
    assert_eq!(
        (two_days.points[0].month, two_days.points[0].day_of_month),
        (2, 28)
    );
    assert_eq!(two_days.points[0].day_of_week, DayOfWeek::Sunday);
    assert_eq!(
        (two_days.points[24].month, two_days.points[24].day_of_month),
        (3, 1)
    );
    assert_eq!(two_days.points[24].day_of_sim, 2);
    assert_eq!(two_days.points[24].day_of_week, DayOfWeek::Monday);
    assert_eq!(two_days.points[24].gregorian_day_of_year, 61);
    assert_eq!(two_days.points[24].day_of_year, 60);
    assert_eq!(two_days.points[24].schedule_day_of_year, 61);

    Ok(())
}

#[test]
fn metadata_aware_calendar_rejects_actual_weather_for_both_leap_policies() {
    let mut run_period = test_run_period("Actual Leap Weather", 2, 28, 3, 1);
    run_period.begin_year = Some(2016);
    run_period.end_year = Some(2016);
    run_period.treat_weather_as_actual = true;

    for leap_year_observed in [false, true] {
        assert!(matches!(
            resolve_weather_environment_calendar(
                &run_period,
                &EpwCalendarMetadata { leap_year_observed }
            ),
            Err(TimeAxisError::ActualWeatherUnsupported { .. })
        ));
    }
}

#[test]
fn metadata_aware_calendar_rejects_cross_year_run_periods() {
    let mut run_period = test_run_period("Cross Year Weather", 3, 1, 3, 1);
    run_period.begin_year = Some(2016);
    run_period.end_year = Some(2017);

    for leap_year_observed in [false, true] {
        assert!(matches!(
            resolve_weather_environment_calendar(
                &run_period,
                &EpwCalendarMetadata { leap_year_observed }
            ),
            Err(TimeAxisError::WeatherMetadataCrossYearUnsupported {
                start_year: 2016,
                end_year: 2017,
                ..
            })
        ));
    }
}

#[test]
fn run_period_calendar_rejects_pre_gregorian_start_and_end_year_without_start() {
    let mut pre_gregorian = test_run_period("Too Early", 1, 1, 1, 1);
    pre_gregorian.begin_year = Some(1582);
    assert!(matches!(
        resolve_run_period_calendar(&pre_gregorian),
        Err(TimeAxisError::StartYearBeforeGregorianCalendar { year: 1582, .. })
    ));

    let mut missing_start_year = test_run_period("Missing Start Year", 1, 1, 1, 1);
    missing_start_year.end_year = Some(2020);
    assert!(matches!(
        resolve_run_period_calendar(&missing_start_year),
        Err(TimeAxisError::EndYearWithoutStartYear { .. })
    ));
}

#[test]
fn environment_time_axis_uses_zone_timestep_source_order_and_flags()
-> Result<(), Box<dyn std::error::Error>> {
    let mut model = TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 4,
        },
        ..TypedModel::default()
    };
    let mut run_period = test_run_period("Quarter Hour", 1, 1, 1, 1);
    run_period.begin_year = Some(2021);
    run_period.end_year = Some(2021);
    model.run_periods.push(run_period);

    let axes = build_environment_time_axes(&model)?;
    let axis = &axes[0];

    assert_eq!(axis.sample_count(), 96);
    assert_eq!(axis.zone_timestep.timesteps_per_hour, 4);
    assert_eq!(axis.points[0].zone_timestep, 1);
    assert_eq!(axis.points[0].start_minute, 0.0);
    assert_eq!(axis.points[0].end_minute, 15.0);
    assert_eq!(axis.points[0].current_time_hours, 0.25);
    assert_eq!(axis.points[0].simulation_timestep, 1);
    assert!(axis.points[0].begin_environment);
    assert!(axis.points[0].begin_day);
    assert!(axis.points[0].begin_hour);
    assert!(!axis.points[0].end_hour);
    assert_eq!(axis.points[3].current_time_hours, 1.0);
    assert!(axis.points[3].end_hour);
    assert!(!axis.points[3].end_day);
    assert!(!axis.points[4].begin_day);
    assert!(axis.points[4].begin_hour);
    let final_point = axis.points.last().expect("one-day axis has a final point");
    assert_eq!(final_point.current_time_hours, 24.0);
    assert_eq!(final_point.simulation_timestep, 96);
    assert!(final_point.end_hour);
    assert!(final_point.end_day);
    assert!(final_point.end_environment);
    assert!(!final_point.dst);
    assert_eq!(final_point.special_day_type, None);
    assert!(!final_point.gregorian_year_is_leap_year);

    Ok(())
}

#[test]
fn environment_time_axes_include_every_run_period_and_fallback()
-> Result<(), Box<dyn std::error::Error>> {
    let fallback = build_environment_time_axes(&TypedModel::default())?;
    assert_eq!(fallback.len(), 1);
    assert_eq!(fallback[0].environment_index, 1);
    assert_eq!(fallback[0].sample_count(), 24 * 6);

    let mut model = TypedModel::default();
    model
        .run_periods
        .push(test_run_period("First Environment", 1, 1, 1, 1));
    model
        .run_periods
        .push(test_run_period("Second Environment", 2, 1, 2, 1));

    let axes = build_environment_time_axes(&model)?;

    assert_eq!(axes.len(), 2);
    assert_eq!(axes[0].environment_index, 1);
    assert_eq!(axes[1].environment_index, 2);
    assert_eq!(axes[0].environment_name, "FIRST ENVIRONMENT");
    assert_eq!(axes[1].environment_name, "SECOND ENVIRONMENT");
    assert_eq!(axes[1].points[0].environment_index, 2);

    Ok(())
}
