use crate::weather::{EpwCalendarDateRule, EpwDaylightSavingPeriod};

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

fn full_year_2017_run_period(name: &str) -> RunPeriod {
    let mut run_period = test_run_period(name, 1, 1, 12, 31);
    run_period.begin_year = Some(2017);
    run_period.end_year = Some(2017);
    run_period
}

fn daylight_saving_metadata(
    start: EpwCalendarDateRule,
    end: EpwCalendarDateRule,
) -> EpwCalendarMetadata {
    EpwCalendarMetadata {
        leap_year_observed: false,
        daylight_saving_period: Some(EpwDaylightSavingPeriod { start, end }),
        holidays: Vec::new(),
    }
}

fn assert_hourly_day_dst(axis: &crate::TimeAxis, month: u32, day_of_month: u32, expected: bool) {
    let points = axis
        .points
        .iter()
        .filter(|point| point.month == month && point.day_of_month == day_of_month)
        .collect::<Vec<_>>();
    assert_eq!(points.len(), 24);
    assert!(points.iter().all(|point| point.dst == expected));
}

fn assert_environment_day_dst(
    axis: &crate::EnvironmentTimeAxis,
    month: u32,
    day_of_month: u32,
    expected: bool,
) {
    let points = axis
        .points
        .iter()
        .filter(|point| point.month == month && point.day_of_month == day_of_month)
        .collect::<Vec<_>>();
    assert_eq!(
        points.len(),
        24 * usize::try_from(axis.zone_timestep.timesteps_per_hour).unwrap_or(0)
    );
    assert!(points.iter().all(|point| point.dst == expected));
}

#[test]
fn weather_file_fixed_date_daylight_saving_is_inclusive_on_both_time_axes()
-> Result<(), Box<dyn std::error::Error>> {
    let run_period = full_year_2017_run_period("Fixed Date DST");
    let metadata = daylight_saving_metadata(
        EpwCalendarDateRule::MonthDay {
            month: 3,
            day_of_month: 10,
        },
        EpwCalendarDateRule::MonthDay {
            month: 11,
            day_of_month: 3,
        },
    );
    let hourly_axis =
        build_hourly_time_axis_for_run_period_with_weather_metadata(&run_period, &metadata)?;

    assert!(hourly_axis.daylight_saving.weather_file_period_declared);
    assert!(
        hourly_axis
            .daylight_saving
            .run_period_uses_weather_file_period
    );
    assert!(!hourly_axis.daylight_saving.input_file_period_declared);
    assert!(hourly_axis.daylight_saving.active);
    assert_eq!(
        hourly_axis.daylight_saving.effective_source,
        crate::DaylightSavingPeriodSource::WeatherFile
    );
    let resolved = hourly_axis
        .daylight_saving
        .resolved_period
        .expect("active DST has a resolved period");
    assert_eq!(
        (
            resolved.start.month,
            resolved.start.day_of_month,
            resolved.start.day_of_year,
        ),
        (3, 10, 69)
    );
    assert_eq!(
        (
            resolved.end.month,
            resolved.end.day_of_month,
            resolved.end.day_of_year,
        ),
        (11, 3, 307)
    );
    assert!(!resolved.wraps_year);
    assert_hourly_day_dst(&hourly_axis, 3, 9, false);
    assert_hourly_day_dst(&hourly_axis, 3, 10, true);
    assert_hourly_day_dst(&hourly_axis, 11, 3, true);
    assert_hourly_day_dst(&hourly_axis, 11, 4, false);

    let model = TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 4,
        },
        run_periods: vec![run_period],
        ..TypedModel::default()
    };
    let environment_axes = build_environment_time_axes_with_weather_metadata(&model, &metadata)?;
    let environment_axis = &environment_axes[0];
    assert_eq!(
        environment_axis.daylight_saving,
        hourly_axis.daylight_saving
    );
    assert_environment_day_dst(environment_axis, 3, 9, false);
    assert_environment_day_dst(environment_axis, 3, 10, true);
    assert_environment_day_dst(environment_axis, 11, 3, true);
    assert_environment_day_dst(environment_axis, 11, 4, false);

    Ok(())
}

#[test]
fn weather_file_nth_weekday_daylight_saving_rules_resolve_like_energyplus()
-> Result<(), Box<dyn std::error::Error>> {
    let run_period = full_year_2017_run_period("Nth Weekday DST");
    let metadata = daylight_saving_metadata(
        EpwCalendarDateRule::NthWeekdayInMonth {
            nth: 2,
            weekday: DayOfWeek::Sunday,
            month: 3,
        },
        EpwCalendarDateRule::NthWeekdayInMonth {
            nth: 1,
            weekday: DayOfWeek::Sunday,
            month: 11,
        },
    );
    let axis = build_hourly_time_axis_for_run_period_with_weather_metadata(&run_period, &metadata)?;
    let resolved = axis
        .daylight_saving
        .resolved_period
        .expect("active DST has a resolved period");

    assert_eq!(
        (
            resolved.start.month,
            resolved.start.day_of_month,
            resolved.start.day_of_year,
        ),
        (3, 12, 71)
    );
    assert_eq!(
        (
            resolved.end.month,
            resolved.end.day_of_month,
            resolved.end.day_of_year,
        ),
        (11, 5, 309)
    );
    assert!(!resolved.wraps_year);
    assert_hourly_day_dst(&axis, 3, 11, false);
    assert_hourly_day_dst(&axis, 3, 12, true);
    assert_hourly_day_dst(&axis, 11, 5, true);
    assert_hourly_day_dst(&axis, 11, 6, false);

    Ok(())
}

#[test]
fn weather_file_nth_weekday_daylight_saving_preserves_run_period_month_weekdays()
-> Result<(), Box<dyn std::error::Error>> {
    let mut run_period = test_run_period("Leap Policy DST", 1, 1, 12, 31);
    run_period.begin_year = Some(2016);
    run_period.end_year = Some(2016);
    let period = EpwDaylightSavingPeriod {
        start: EpwCalendarDateRule::NthWeekdayInMonth {
            nth: 1,
            weekday: DayOfWeek::Tuesday,
            month: 3,
        },
        end: EpwCalendarDateRule::MonthDay {
            month: 11,
            day_of_month: 1,
        },
    };
    let leap_axis = build_hourly_time_axis_for_run_period_with_weather_metadata(
        &run_period,
        &EpwCalendarMetadata {
            leap_year_observed: true,
            daylight_saving_period: Some(period),
            holidays: Vec::new(),
        },
    )?;
    let non_leap_axis = build_hourly_time_axis_for_run_period_with_weather_metadata(
        &run_period,
        &EpwCalendarMetadata {
            leap_year_observed: false,
            daylight_saving_period: Some(period),
            holidays: Vec::new(),
        },
    )?;

    let leap_start = leap_axis
        .daylight_saving
        .resolved_period
        .expect("active leap-shaped DST period")
        .start;
    let non_leap_start = non_leap_axis
        .daylight_saving
        .resolved_period
        .expect("active non-leap-shaped DST period")
        .start;
    assert_eq!((leap_start.month, leap_start.day_of_month), (3, 2));
    assert_eq!(
        (non_leap_start.month, non_leap_start.day_of_month),
        (3, 2)
    );
    assert_eq!(leap_start.day_of_year, 62);
    assert_eq!(non_leap_start.day_of_year, 61);
    assert_hourly_day_dst(&leap_axis, 3, 1, false);
    assert_hourly_day_dst(&leap_axis, 3, 2, true);
    assert_hourly_day_dst(&non_leap_axis, 3, 1, false);
    assert_hourly_day_dst(&non_leap_axis, 3, 2, true);

    Ok(())
}

#[test]
fn weather_file_last_weekday_daylight_saving_rules_resolve_like_energyplus()
-> Result<(), Box<dyn std::error::Error>> {
    let run_period = full_year_2017_run_period("Last Weekday DST");
    let metadata = daylight_saving_metadata(
        EpwCalendarDateRule::LastWeekdayInMonth {
            weekday: DayOfWeek::Sunday,
            month: 3,
        },
        EpwCalendarDateRule::LastWeekdayInMonth {
            weekday: DayOfWeek::Sunday,
            month: 10,
        },
    );
    let axis = build_hourly_time_axis_for_run_period_with_weather_metadata(&run_period, &metadata)?;
    let resolved = axis
        .daylight_saving
        .resolved_period
        .expect("active DST has a resolved period");

    assert_eq!(
        (
            resolved.start.month,
            resolved.start.day_of_month,
            resolved.start.day_of_year,
        ),
        (3, 26, 85)
    );
    assert_eq!(
        (
            resolved.end.month,
            resolved.end.day_of_month,
            resolved.end.day_of_year,
        ),
        (10, 29, 302)
    );
    assert!(!resolved.wraps_year);
    assert_hourly_day_dst(&axis, 3, 25, false);
    assert_hourly_day_dst(&axis, 3, 26, true);
    assert_hourly_day_dst(&axis, 10, 29, true);
    assert_hourly_day_dst(&axis, 10, 30, false);

    Ok(())
}

#[test]
fn weather_file_daylight_saving_range_wraps_across_the_weather_year()
-> Result<(), Box<dyn std::error::Error>> {
    let run_period = full_year_2017_run_period("Southern DST");
    let metadata = daylight_saving_metadata(
        EpwCalendarDateRule::MonthDay {
            month: 10,
            day_of_month: 1,
        },
        EpwCalendarDateRule::MonthDay {
            month: 3,
            day_of_month: 31,
        },
    );
    let axis = build_hourly_time_axis_for_run_period_with_weather_metadata(&run_period, &metadata)?;
    let resolved = axis
        .daylight_saving
        .resolved_period
        .expect("active DST has a resolved period");

    assert_eq!(resolved.start.day_of_year, 274);
    assert_eq!(resolved.end.day_of_year, 90);
    assert!(resolved.wraps_year);
    assert_hourly_day_dst(&axis, 1, 1, true);
    assert_hourly_day_dst(&axis, 3, 31, true);
    assert_hourly_day_dst(&axis, 4, 1, false);
    assert_hourly_day_dst(&axis, 9, 30, false);
    assert_hourly_day_dst(&axis, 10, 1, true);
    assert_hourly_day_dst(&axis, 12, 31, true);

    Ok(())
}

#[test]
fn run_period_can_disable_the_declared_weather_file_daylight_saving_period()
-> Result<(), Box<dyn std::error::Error>> {
    let mut run_period = full_year_2017_run_period("Disabled DST");
    run_period.use_weather_file_daylight_saving_period = false;
    let metadata = daylight_saving_metadata(
        EpwCalendarDateRule::MonthDay {
            month: 3,
            day_of_month: 10,
        },
        EpwCalendarDateRule::MonthDay {
            month: 11,
            day_of_month: 3,
        },
    );
    let axis = build_hourly_time_axis_for_run_period_with_weather_metadata(&run_period, &metadata)?;

    assert!(axis.daylight_saving.weather_file_period_declared);
    assert!(!axis.daylight_saving.run_period_uses_weather_file_period);
    assert!(!axis.daylight_saving.input_file_period_declared);
    assert!(!axis.daylight_saving.active);
    assert_eq!(
        axis.daylight_saving.effective_source,
        crate::DaylightSavingPeriodSource::None
    );
    assert_eq!(axis.daylight_saving.resolved_period, None);
    assert!(axis.points.iter().all(|point| !point.dst));

    Ok(())
}

#[test]
fn input_file_daylight_saving_overrides_weather_file_period_and_run_period_policy()
-> Result<(), Box<dyn std::error::Error>> {
    let mut run_period = test_run_period("IDF DST PRECEDENCE", 2, 28, 3, 1);
    run_period.begin_year = Some(2016);
    run_period.end_year = Some(2016);
    run_period.use_weather_file_daylight_saving_period = false;
    let metadata = EpwCalendarMetadata {
        leap_year_observed: true,
        daylight_saving_period: Some(EpwDaylightSavingPeriod {
            start: EpwCalendarDateRule::MonthDay {
                month: 2,
                day_of_month: 29,
            },
            end: EpwCalendarDateRule::MonthDay {
                month: 3,
                day_of_month: 1,
            },
        }),
        holidays: Vec::new(),
    };
    let model = TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 1,
        },
        run_periods: vec![run_period],
        run_period_daylight_saving_time: Some(ep_model::RunPeriodDaylightSavingTime {
            start_date: ep_model::CalendarDateRule::MonthDay {
                month: 2,
                day_of_month: 28,
            },
            end_date: ep_model::CalendarDateRule::MonthDay {
                month: 2,
                day_of_month: 29,
            },
        }),
        ..TypedModel::default()
    };

    let hourly_axis = crate::build_hourly_time_axis_with_weather_metadata(&model, &metadata)?;
    assert!(hourly_axis.daylight_saving.weather_file_period_declared);
    assert!(
        !hourly_axis
            .daylight_saving
            .run_period_uses_weather_file_period
    );
    assert!(hourly_axis.daylight_saving.input_file_period_declared);
    assert!(hourly_axis.daylight_saving.active);
    assert_eq!(
        hourly_axis.daylight_saving.effective_source,
        crate::DaylightSavingPeriodSource::InputFile
    );
    let resolved = hourly_axis
        .daylight_saving
        .resolved_period
        .expect("active input-file DST has a resolved period");
    assert_eq!(
        (
            resolved.start.month,
            resolved.start.day_of_month,
            resolved.start.day_of_year,
        ),
        (2, 28, 59)
    );
    assert_eq!(
        (
            resolved.end.month,
            resolved.end.day_of_month,
            resolved.end.day_of_year,
        ),
        (2, 29, 60)
    );
    assert!(!resolved.wraps_year);
    assert_eq!(hourly_axis.points.len(), 72);
    assert_eq!(
        hourly_axis.points.iter().filter(|point| point.dst).count(),
        48
    );
    assert_hourly_day_dst(&hourly_axis, 2, 28, true);
    assert_hourly_day_dst(&hourly_axis, 2, 29, true);
    assert_hourly_day_dst(&hourly_axis, 3, 1, false);

    let environment_axes =
        build_environment_time_axes_with_weather_metadata(&model, &metadata)?;
    assert_eq!(environment_axes.len(), 1);
    assert_eq!(
        environment_axes[0].daylight_saving,
        hourly_axis.daylight_saving
    );
    assert_environment_day_dst(&environment_axes[0], 2, 28, true);
    assert_environment_day_dst(&environment_axes[0], 2, 29, true);
    assert_environment_day_dst(&environment_axes[0], 3, 1, false);

    Ok(())
}

#[test]
fn nonexistent_nth_weekday_daylight_saving_rule_is_rejected() {
    let run_period = full_year_2017_run_period("Missing Nth Weekday DST");
    let metadata = daylight_saving_metadata(
        EpwCalendarDateRule::NthWeekdayInMonth {
            nth: 5,
            weekday: DayOfWeek::Monday,
            month: 2,
        },
        EpwCalendarDateRule::MonthDay {
            month: 11,
            day_of_month: 1,
        },
    );

    assert!(matches!(
        build_hourly_time_axis_for_run_period_with_weather_metadata(&run_period, &metadata),
        Err(TimeAxisError::DaylightSavingDateRuleDoesNotExist {
            boundary: "start",
            nth: 5,
            weekday: DayOfWeek::Monday,
            month: 2,
            ..
        })
    ));
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
        daylight_saving_period: None,
        holidays: Vec::new(),
    };
    let non_leap_metadata = EpwCalendarMetadata {
        leap_year_observed: false,
        daylight_saving_period: None,
        holidays: Vec::new(),
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
        daylight_saving_period: None,
        holidays: Vec::new(),
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
                &EpwCalendarMetadata {
                    leap_year_observed,
                    daylight_saving_period: None,
                    holidays: Vec::new(),
                }
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
                &EpwCalendarMetadata {
                    leap_year_observed,
                    daylight_saving_period: None,
                    holidays: Vec::new(),
                }
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
