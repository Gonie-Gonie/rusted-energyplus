use ep_model::{
    CalendarDateRule as SpecialDayDateRule, RunPeriodSpecialDay, RunPeriodSpecialDayId,
    SpecialDayType,
};

fn special_day_test_run_period(
    name: &str,
    begin: (u32, u32, u32),
    end: (u32, u32, u32),
    apply_weekend_holiday_rule: bool,
) -> RunPeriod {
    RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new(name),
        begin_month: begin.1,
        begin_day_of_month: begin.2,
        begin_year: Some(begin.0),
        end_month: end.1,
        end_day_of_month: end.2,
        end_year: Some(end.0),
        day_of_week_for_start_day: None,
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        use_weather_file_holidays_and_special_days: false,
        use_weather_file_daylight_saving_period: false,
        apply_weekend_holiday_rule,
        use_weather_file_rain_indicators: true,
        use_weather_file_snow_indicators: true,
        treat_weather_as_actual: false,
    }
}

fn special_day_input(
    id: u32,
    name: &str,
    start_date: SpecialDayDateRule,
    duration_days: u32,
    special_day_type: SpecialDayType,
) -> RunPeriodSpecialDay {
    RunPeriodSpecialDay {
        id: RunPeriodSpecialDayId(id),
        name: NormalizedName::new(name),
        start_date,
        duration_days,
        special_day_type,
    }
}

fn special_day_test_model(
    run_period: RunPeriod,
    run_period_special_days: Vec<RunPeriodSpecialDay>,
) -> TypedModel {
    TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 1,
        },
        run_periods: vec![run_period],
        run_period_special_days,
        ..TypedModel::default()
    }
}

fn assert_hourly_special_day(
    axis: &crate::TimeAxis,
    date: (u32, u32, u32),
    expected: (crate::DayType, Option<crate::DayType>, u32),
) {
    let points = axis
        .points
        .iter()
        .filter(|point| (point.year, point.month, point.day_of_month) == (date.0, date.1, date.2))
        .collect::<Vec<_>>();

    assert_eq!(points.len(), 24, "hourly point count for {date:?}");
    assert_eq!(expected.0.energyplus_index(), expected.2);
    assert!(points.iter().all(|point| point.day_type == expected.0));
    assert!(
        points
            .iter()
            .all(|point| point.special_day_type == expected.1)
    );
    assert!(
        points
            .iter()
            .all(|point| point.day_type.energyplus_index() == expected.2)
    );
}

fn assert_environment_special_day(
    axis: &crate::EnvironmentTimeAxis,
    date: (u32, u32, u32),
    expected: (crate::DayType, Option<crate::DayType>, u32),
) {
    let points = axis
        .points
        .iter()
        .filter(|point| (point.year, point.month, point.day_of_month) == (date.0, date.1, date.2))
        .collect::<Vec<_>>();

    assert_eq!(axis.zone_timestep.timesteps_per_hour, 1);
    assert_eq!(points.len(), 24, "environment point count for {date:?}");
    assert_eq!(expected.0.energyplus_index(), expected.2);
    assert!(points.iter().all(|point| point.day_type == expected.0));
    assert!(
        points
            .iter()
            .all(|point| point.special_day_type == expected.1)
    );
    assert!(
        points
            .iter()
            .all(|point| point.day_type.energyplus_index() == expected.2)
    );
}

#[test]
fn model_special_day_overrides_both_axes_for_every_hour_of_leap_day()
-> Result<(), Box<dyn std::error::Error>> {
    let model = special_day_test_model(
        special_day_test_run_period("Leap Day Special", (2016, 2, 28), (2016, 3, 1), false),
        vec![special_day_input(
            0,
            "Leap Day Holiday",
            SpecialDayDateRule::MonthDay {
                month: 2,
                day_of_month: 29,
            },
            1,
            SpecialDayType::Holiday,
        )],
    );

    let hourly_axis = build_hourly_time_axis(&model)?;
    let environment_axes = build_environment_time_axes(&model)?;
    let environment_axis = &environment_axes[0];

    assert_eq!(hourly_axis.sample_count(), 72);
    assert_eq!(environment_axes.len(), 1);
    assert_eq!(environment_axis.sample_count(), 72);
    assert_eq!(hourly_axis.special_days.input_file_special_days_declared, 1);
    assert_eq!(
        environment_axis
            .special_days
            .input_file_special_days_declared,
        1
    );

    assert_hourly_special_day(
        &hourly_axis,
        (2016, 2, 28),
        (crate::DayType::Sunday, None, 1),
    );
    assert_hourly_special_day(
        &hourly_axis,
        (2016, 2, 29),
        (crate::DayType::Holiday, Some(crate::DayType::Holiday), 8),
    );
    assert_hourly_special_day(
        &hourly_axis,
        (2016, 3, 1),
        (crate::DayType::Tuesday, None, 3),
    );
    assert_environment_special_day(
        environment_axis,
        (2016, 2, 28),
        (crate::DayType::Sunday, None, 1),
    );
    assert_environment_special_day(
        environment_axis,
        (2016, 2, 29),
        (crate::DayType::Holiday, Some(crate::DayType::Holiday), 8),
    );
    assert_environment_special_day(
        environment_axis,
        (2016, 3, 1),
        (crate::DayType::Tuesday, None, 3),
    );

    Ok(())
}

#[test]
fn special_day_duration_is_inclusive_and_wraps_the_same_year_annual_table()
-> Result<(), Box<dyn std::error::Error>> {
    let model = special_day_test_model(
        special_day_test_run_period("Wrapped Duration", (2016, 1, 1), (2016, 1, 3), false),
        vec![special_day_input(
            0,
            "Three Day Holiday",
            SpecialDayDateRule::MonthDay {
                month: 12,
                day_of_month: 31,
            },
            3,
            SpecialDayType::Holiday,
        )],
    );
    let axis = build_hourly_time_axis(&model)?;

    assert_eq!(axis.sample_count(), 3 * 24);
    assert_eq!(axis.special_days.resolved_days[0].start.day_of_year, 366);
    assert_eq!(axis.special_days.resolved_days[0].duration_days, 3);
    for date in [(2016, 1, 1), (2016, 1, 2)] {
        assert_hourly_special_day(
            &axis,
            date,
            (crate::DayType::Holiday, Some(crate::DayType::Holiday), 8),
        );
    }
    assert_hourly_special_day(&axis, (2016, 1, 3), (crate::DayType::Sunday, None, 1));

    Ok(())
}

#[test]
fn cross_year_special_days_reuse_the_source_start_year_annual_table()
-> Result<(), Box<dyn std::error::Error>> {
    let model = special_day_test_model(
        special_day_test_run_period("Cross Year Special", (2031, 12, 30), (2032, 1, 2), false),
        vec![special_day_input(
            0,
            "First Thursday In January",
            SpecialDayDateRule::NthWeekdayInMonth {
                nth: 1,
                weekday: DayOfWeek::Thursday,
                month: 1,
            },
            1,
            SpecialDayType::Holiday,
        )],
    );
    let hourly_axis = build_hourly_time_axis(&model)?;
    let environment_axes = build_environment_time_axes(&model)?;
    let environment_axis = &environment_axes[0];

    assert_eq!(
        (
            hourly_axis.special_days.resolved_days[0].start.month,
            hourly_axis.special_days.resolved_days[0].start.day_of_month,
            hourly_axis.special_days.resolved_days[0].start.day_of_year,
        ),
        (1, 2, 2)
    );
    assert_eq!(hourly_axis.sample_count(), 4 * 24);
    assert_eq!(environment_axis.sample_count(), 4 * 24);

    for (date, expected) in [
        ((2031, 12, 30), (crate::DayType::Tuesday, None, 3)),
        ((2031, 12, 31), (crate::DayType::Wednesday, None, 4)),
        ((2032, 1, 1), (crate::DayType::Thursday, None, 5)),
        (
            (2032, 1, 2),
            (crate::DayType::Holiday, Some(crate::DayType::Holiday), 8),
        ),
    ] {
        assert_hourly_special_day(&hourly_axis, date, expected);
        assert_environment_special_day(environment_axis, date, expected);
    }

    Ok(())
}

#[test]
fn later_typed_special_day_definition_overwrites_an_earlier_definition()
-> Result<(), Box<dyn std::error::Error>> {
    let start_date = SpecialDayDateRule::MonthDay {
        month: 6,
        day_of_month: 15,
    };
    let model = special_day_test_model(
        special_day_test_run_period("Ordered Special Days", (2017, 6, 15), (2017, 6, 15), false),
        vec![
            special_day_input(0, "First Holiday", start_date, 1, SpecialDayType::Holiday),
            special_day_input(
                1,
                "Later Custom Day",
                start_date,
                1,
                SpecialDayType::CustomDay2,
            ),
        ],
    );
    let axis = build_hourly_time_axis(&model)?;

    assert_eq!(axis.special_days.resolved_days.len(), 2);
    assert_eq!(axis.special_days.resolved_days[0].name, "FIRST HOLIDAY");
    assert_eq!(axis.special_days.resolved_days[1].name, "LATER CUSTOM DAY");
    assert_hourly_special_day(
        &axis,
        (2017, 6, 15),
        (
            crate::DayType::CustomDay2,
            Some(crate::DayType::CustomDay2),
            12,
        ),
    );

    Ok(())
}

#[test]
fn weekend_rule_shifts_only_fixed_single_day_special_days_to_monday()
-> Result<(), Box<dyn std::error::Error>> {
    let model = special_day_test_model(
        special_day_test_run_period("Weekend Rule", (2017, 1, 1), (2017, 1, 23), true),
        vec![
            special_day_input(
                0,
                "Sunday Fixed",
                SpecialDayDateRule::MonthDay {
                    month: 1,
                    day_of_month: 1,
                },
                1,
                SpecialDayType::Holiday,
            ),
            special_day_input(
                1,
                "Saturday Fixed",
                SpecialDayDateRule::MonthDay {
                    month: 1,
                    day_of_month: 7,
                },
                1,
                SpecialDayType::CustomDay1,
            ),
            special_day_input(
                2,
                "Multi Day Fixed",
                SpecialDayDateRule::MonthDay {
                    month: 1,
                    day_of_month: 15,
                },
                2,
                SpecialDayType::SummerDesignDay,
            ),
            special_day_input(
                3,
                "Nth Sunday",
                SpecialDayDateRule::NthWeekdayInMonth {
                    nth: 4,
                    weekday: DayOfWeek::Sunday,
                    month: 1,
                },
                1,
                SpecialDayType::WinterDesignDay,
            ),
        ],
    );
    let axis = build_hourly_time_axis(&model)?;

    assert_eq!(
        axis.special_days
            .resolved_days
            .iter()
            .map(|day| day.weekend_shift_days)
            .collect::<Vec<_>>(),
        vec![1, 2, 0, 0]
    );
    assert_eq!(
        axis.special_days
            .resolved_days
            .iter()
            .map(|day| (day.start.month, day.start.day_of_month))
            .collect::<Vec<_>>(),
        vec![(1, 2), (1, 9), (1, 15), (1, 22)]
    );
    assert_hourly_special_day(&axis, (2017, 1, 1), (crate::DayType::Sunday, None, 1));
    assert_hourly_special_day(
        &axis,
        (2017, 1, 2),
        (crate::DayType::Holiday, Some(crate::DayType::Holiday), 8),
    );
    assert_hourly_special_day(&axis, (2017, 1, 7), (crate::DayType::Saturday, None, 7));
    assert_hourly_special_day(
        &axis,
        (2017, 1, 9),
        (
            crate::DayType::CustomDay1,
            Some(crate::DayType::CustomDay1),
            11,
        ),
    );
    for date in [(2017, 1, 15), (2017, 1, 16)] {
        assert_hourly_special_day(
            &axis,
            date,
            (
                crate::DayType::SummerDesignDay,
                Some(crate::DayType::SummerDesignDay),
                9,
            ),
        );
    }
    assert_hourly_special_day(
        &axis,
        (2017, 1, 22),
        (
            crate::DayType::WinterDesignDay,
            Some(crate::DayType::WinterDesignDay),
            10,
        ),
    );
    assert_hourly_special_day(&axis, (2017, 1, 23), (crate::DayType::Monday, None, 2));

    Ok(())
}

#[test]
fn nth_and_last_special_day_rules_resolve_and_nonexistent_nth_is_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let model = special_day_test_model(
        special_day_test_run_period("Nth And Last", (2017, 3, 1), (2017, 5, 31), true),
        vec![
            special_day_input(
                0,
                "Second Sunday In March",
                SpecialDayDateRule::NthWeekdayInMonth {
                    nth: 2,
                    weekday: DayOfWeek::Sunday,
                    month: 3,
                },
                1,
                SpecialDayType::Holiday,
            ),
            special_day_input(
                1,
                "Last Monday In May",
                SpecialDayDateRule::LastWeekdayInMonth {
                    weekday: DayOfWeek::Monday,
                    month: 5,
                },
                1,
                SpecialDayType::CustomDay1,
            ),
        ],
    );
    let axis = build_hourly_time_axis(&model)?;

    assert_eq!(
        axis.special_days
            .resolved_days
            .iter()
            .map(|day| {
                (
                    day.start.month,
                    day.start.day_of_month,
                    day.start.day_of_year,
                    day.weekend_shift_days,
                )
            })
            .collect::<Vec<_>>(),
        vec![(3, 12, 71, 0), (5, 29, 149, 0)]
    );
    assert_hourly_special_day(
        &axis,
        (2017, 3, 12),
        (crate::DayType::Holiday, Some(crate::DayType::Holiday), 8),
    );
    assert_hourly_special_day(
        &axis,
        (2017, 5, 29),
        (
            crate::DayType::CustomDay1,
            Some(crate::DayType::CustomDay1),
            11,
        ),
    );

    let nonexistent_model = special_day_test_model(
        special_day_test_run_period("Nonexistent Nth", (2017, 2, 1), (2017, 2, 28), false),
        vec![special_day_input(
            0,
            "Fifth Monday In February",
            SpecialDayDateRule::NthWeekdayInMonth {
                nth: 5,
                weekday: DayOfWeek::Monday,
                month: 2,
            },
            1,
            SpecialDayType::Holiday,
        )],
    );
    assert_eq!(
        build_hourly_time_axis(&nonexistent_model).expect_err("2017-02 has four Mondays"),
        TimeAxisError::SpecialDayDateRuleDoesNotExist {
            run_period_name: "NONEXISTENT NTH".to_string(),
            special_day_name: "FIFTH MONDAY IN FEBRUARY".to_string(),
            nth: 5,
            weekday: DayOfWeek::Monday,
            month: 2,
        }
    );

    Ok(())
}
