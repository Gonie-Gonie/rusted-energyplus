use super::{
    ScheduleSeriesKind, compiled_day_schedule_value, file_schedule_hourly_8760_value,
    precompile_day_schedule_table, precompute_schedule_value_series,
    precompute_schedule_value_series_for_environment_time_axis,
    precompute_schedule_value_series_for_time_axis, year_schedule_hourly_value,
};
use crate::time_axis::{
    build_environment_time_axis_for_run_period_with_zone_timesteps, build_hourly_time_axis,
};
use ep_model::{
    DayOfWeek, DayScheduleId, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod,
    RunPeriodId, ScheduleCompactSegment, ScheduleDayHourly, ScheduleDayInterval, ScheduleDayList,
    ScheduleDayType, ScheduleFile, ScheduleFileColumnSeparator, ScheduleId, ScheduleInterpolation,
    ScheduleWeekDaily, ScheduleYear, TimestepConfig, TypedModel, WeekScheduleId,
};

fn hourly_file_schedule() -> ScheduleFile {
    ScheduleFile {
        id: ScheduleId(17),
        name: NormalizedName::new("Selected Column"),
        schedule_type_limits: None,
        file_name: "schedule.csv".to_string(),
        column_number: 2,
        rows_to_skip_at_top: 1,
        number_of_hours_of_data: 8760,
        column_separator: ScheduleFileColumnSeparator::Comma,
        interpolate_to_timestep: false,
        minutes_per_item: 60,
        adjust_schedule_for_daylight_savings: false,
        values: (1..=8760).map(f64::from).collect(),
    }
}

fn annual_day_week_year_model() -> TypedModel {
    let regular_day = ScheduleDayHourly {
        id: DayScheduleId(0),
        name: NormalizedName::new("Regular Day"),
        schedule_type_limits: None,
        hourly_values: std::array::from_fn(|index| {
            1.0 + f64::from(u32::try_from(index).unwrap_or_default())
        }),
    };
    let march_day = ScheduleDayHourly {
        id: DayScheduleId(1),
        name: NormalizedName::new("March Day"),
        schedule_type_limits: None,
        hourly_values: std::array::from_fn(|index| {
            101.0 + f64::from(u32::try_from(index).unwrap_or_default())
        }),
    };
    let regular_week = ScheduleWeekDaily {
        id: WeekScheduleId(0),
        name: NormalizedName::new("Regular Week"),
        day_schedules: [regular_day.id; 12],
    };
    let march_week = ScheduleWeekDaily {
        id: WeekScheduleId(1),
        name: NormalizedName::new("March Week"),
        day_schedules: [march_day.id; 12],
    };
    let mut week_schedules = [regular_week.id; 366];
    // Mirrors the compiler's completed leap-shaped table: ordinal 60 falls
    // back to the ordinal-59 pointer when only February 29 is unassigned.
    week_schedules[59] = week_schedules[58];
    week_schedules[60] = march_week.id;
    let year_schedule = ScheduleYear {
        id: ScheduleId(29),
        name: NormalizedName::new("Annual"),
        schedule_type_limits: None,
        week_schedules,
    };

    TypedModel {
        day_schedules: vec![regular_day, march_day],
        week_schedules: vec![regular_week, march_week],
        year_schedules: vec![year_schedule],
        ..TypedModel::default()
    }
}

fn interval_mode_segments() -> Vec<ScheduleCompactSegment> {
    vec![
        ScheduleCompactSegment {
            until_minute_of_day: 20,
            value: 10.0,
        },
        ScheduleCompactSegment {
            until_minute_of_day: 75,
            value: 175.0,
        },
        ScheduleCompactSegment {
            until_minute_of_day: 1440,
            value: 175.0,
        },
    ]
}

fn interval_mode_model() -> TypedModel {
    let hourly_day = ScheduleDayHourly {
        id: DayScheduleId(0),
        name: NormalizedName::new("Hourly Prefix"),
        schedule_type_limits: None,
        hourly_values: [7.0; 24],
    };
    let interval_schedules = [
        ("Interval No", ScheduleInterpolation::No),
        ("Interval Average", ScheduleInterpolation::Average),
        ("Interval Linear", ScheduleInterpolation::Linear),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (name, interpolation))| ScheduleDayInterval {
        id: DayScheduleId(u32::try_from(index + 1).unwrap_or_default()),
        name: NormalizedName::new(name),
        schedule_type_limits: None,
        interpolation,
        segments: interval_mode_segments(),
    })
    .collect::<Vec<_>>();
    let week_schedules = interval_schedules
        .iter()
        .enumerate()
        .map(|(index, day_schedule)| ScheduleWeekDaily {
            id: WeekScheduleId(u32::try_from(index).unwrap_or_default()),
            name: NormalizedName::new(&format!("Week {}", index + 1)),
            day_schedules: [day_schedule.id; 12],
        })
        .collect::<Vec<_>>();
    let year_schedules = week_schedules
        .iter()
        .enumerate()
        .map(|(index, week_schedule)| ScheduleYear {
            id: ScheduleId(40 + u32::try_from(index).unwrap_or_default()),
            name: NormalizedName::new(&format!("Year {}", index + 1)),
            schedule_type_limits: None,
            week_schedules: [week_schedule.id; 366],
        })
        .collect::<Vec<_>>();

    TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 4,
        },
        run_periods: vec![RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("Interval Day"),
            begin_month: 1,
            begin_day_of_month: 1,
            begin_year: Some(2032),
            end_month: 1,
            end_day_of_month: 1,
            end_year: Some(2032),
            day_of_week_for_start_day: Some(DayOfWeek::Thursday),
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
            use_weather_file_holidays_and_special_days: false,
            use_weather_file_daylight_saving_period: false,
            apply_weekend_holiday_rule: false,
            use_weather_file_rain_indicators: false,
            use_weather_file_snow_indicators: false,
            treat_weather_as_actual: false,
        }],
        day_schedules: vec![hourly_day],
        day_interval_schedules: interval_schedules,
        week_schedules,
        year_schedules,
        ..TypedModel::default()
    }
}

fn day_list_mode_values() -> Vec<f64> {
    let mut values = vec![10.0, 70.0, 160.0];
    values.extend(std::iter::repeat_n(175.0, 69));
    values
}

fn day_list_mode_model() -> TypedModel {
    let hourly_day = ScheduleDayHourly {
        id: DayScheduleId(0),
        name: NormalizedName::new("Hourly Prefix"),
        schedule_type_limits: None,
        hourly_values: [7.0; 24],
    };
    let interval_day = ScheduleDayInterval {
        id: DayScheduleId(1),
        name: NormalizedName::new("Interval Prefix"),
        schedule_type_limits: None,
        interpolation: ScheduleInterpolation::No,
        segments: vec![ScheduleCompactSegment {
            until_minute_of_day: 1440,
            value: 8.0,
        }],
    };
    let day_list_schedules = [
        ("List No", ScheduleInterpolation::No),
        ("List Average", ScheduleInterpolation::Average),
        ("List Linear", ScheduleInterpolation::Linear),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (name, interpolation))| ScheduleDayList {
        id: DayScheduleId(2 + u32::try_from(index).unwrap_or_default()),
        name: NormalizedName::new(name),
        schedule_type_limits: None,
        interpolation,
        minutes_per_item: 20,
        values: day_list_mode_values(),
    })
    .collect::<Vec<_>>();
    let week_schedules = day_list_schedules
        .iter()
        .enumerate()
        .map(|(index, day_schedule)| ScheduleWeekDaily {
            id: WeekScheduleId(u32::try_from(index).unwrap_or_default()),
            name: NormalizedName::new(&format!("List Week {}", index + 1)),
            day_schedules: [day_schedule.id; 12],
        })
        .collect::<Vec<_>>();
    let year_schedules = week_schedules
        .iter()
        .enumerate()
        .map(|(index, week_schedule)| ScheduleYear {
            id: ScheduleId(50 + u32::try_from(index).unwrap_or_default()),
            name: NormalizedName::new(&format!("List Year {}", index + 1)),
            schedule_type_limits: None,
            week_schedules: [week_schedule.id; 366],
        })
        .collect::<Vec<_>>();

    TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 4,
        },
        run_periods: vec![RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("List Day"),
            begin_month: 1,
            begin_day_of_month: 1,
            begin_year: Some(2032),
            end_month: 1,
            end_day_of_month: 1,
            end_year: Some(2032),
            day_of_week_for_start_day: Some(DayOfWeek::Thursday),
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
            use_weather_file_holidays_and_special_days: false,
            use_weather_file_daylight_saving_period: false,
            apply_weekend_holiday_rule: false,
            use_weather_file_rain_indicators: false,
            use_weather_file_snow_indicators: false,
            treat_weather_as_actual: false,
        }],
        day_schedules: vec![hourly_day],
        day_interval_schedules: vec![interval_day],
        day_list_schedules,
        week_schedules,
        year_schedules,
        ..TypedModel::default()
    }
}

#[test]
fn hourly_8760_lookup_duplicates_february_28_on_leap_day() {
    let schedule = hourly_file_schedule();

    assert_eq!(
        file_schedule_hourly_8760_value(&schedule, 59, 1),
        Some(1393.0)
    );
    assert_eq!(
        file_schedule_hourly_8760_value(&schedule, 59, 24),
        Some(1416.0)
    );
    assert_eq!(
        file_schedule_hourly_8760_value(&schedule, 60, 1),
        Some(1393.0)
    );
    assert_eq!(
        file_schedule_hourly_8760_value(&schedule, 60, 24),
        Some(1416.0)
    );
    assert_eq!(
        file_schedule_hourly_8760_value(&schedule, 61, 1),
        Some(1417.0)
    );
    assert_eq!(
        file_schedule_hourly_8760_value(&schedule, 366, 24),
        Some(8760.0)
    );
}

#[test]
fn hour_only_api_rejects_file_schedule_without_calendar_axis() {
    let model = TypedModel {
        file_schedules: vec![hourly_file_schedule()],
        ..TypedModel::default()
    };

    let error = precompute_schedule_value_series(&model, 24)
        .expect_err("file schedule must require a calendar-aware axis");

    assert!(error.contains("calendar-aware TimeAxis"));
}

#[test]
fn file_series_kind_records_immutable_source_count() {
    let kind = ScheduleSeriesKind::FileHourly8760 {
        source_value_count: 8760,
    };
    assert_eq!(
        kind,
        ScheduleSeriesKind::FileHourly8760 {
            source_value_count: 8760
        }
    );
}

#[test]
fn annual_schedule_precomputes_direct_hourly_values_on_time_axis() {
    let model = annual_day_week_year_model();
    let time_axis = build_hourly_time_axis(&model);
    assert!(time_axis.is_ok(), "default one-day axis should build");
    let Ok(time_axis) = time_axis else {
        return;
    };

    let series = precompute_schedule_value_series_for_time_axis(&model, &time_axis);
    let trace = series
        .iter()
        .find(|trace| trace.schedule_id == ScheduleId(29));
    assert!(trace.is_some(), "annual schedule trace should be present");
    let Some(trace) = trace else {
        return;
    };

    assert_eq!(
        trace.values,
        (1_u32..=24).map(f64::from).collect::<Vec<_>>()
    );
    assert_eq!(
        trace.kind,
        ScheduleSeriesKind::YearWeekDayHourlyDirect {
            schedule_day_count: 366
        }
    );
}

#[test]
fn annual_schedule_uses_materialized_day_60_fallback_pointer() {
    let model = annual_day_week_year_model();
    let schedule = &model.year_schedules[0];

    assert_eq!(
        year_schedule_hourly_value(&model, schedule, 59, ScheduleDayType::Monday, 60),
        Some(1.0)
    );
    assert_eq!(
        year_schedule_hourly_value(&model, schedule, 60, ScheduleDayType::Monday, 60),
        Some(1.0)
    );
    assert_eq!(
        year_schedule_hourly_value(&model, schedule, 61, ScheduleDayType::Monday, 60),
        Some(101.0)
    );
}

#[test]
fn hour_only_api_rejects_annual_schedule_without_calendar_axis() {
    let model = annual_day_week_year_model();

    let error = precompute_schedule_value_series(&model, 24)
        .expect_err("annual schedule must require a calendar-aware axis");

    assert!(error.contains("Schedule:Year"));
    assert!(error.contains("calendar-aware TimeAxis"));
}

#[test]
fn annual_series_kind_records_immutable_direct_pointer_count() {
    let kind = ScheduleSeriesKind::YearWeekDayHourlyDirect {
        schedule_day_count: 366,
    };
    assert_eq!(
        kind,
        ScheduleSeriesKind::YearWeekDayHourlyDirect {
            schedule_day_count: 366
        }
    );
}

#[test]
fn interval_modes_precompute_exact_zone_timestep_vectors() {
    let model = interval_mode_model();
    let axis =
        build_environment_time_axis_for_run_period_with_zone_timesteps(&model.run_periods[0], 1, 4);
    assert!(axis.is_ok(), "one-day interval axis should build");
    let Ok(axis) = axis else {
        return;
    };

    let traces = precompute_schedule_value_series_for_environment_time_axis(&model, &axis);
    assert_eq!(
        traces.iter().map(|trace| trace.values.len()).sum::<usize>(),
        288
    );
    for (schedule_id, first_values) in [
        (ScheduleId(40), vec![10.0, 175.0, 175.0, 175.0, 175.0]),
        (ScheduleId(41), vec![10.0, 120.0, 175.0, 175.0, 175.0]),
        (ScheduleId(42), vec![10.0, 40.0, 85.0, 130.0, 175.0]),
    ] {
        let trace = traces.iter().find(|trace| trace.schedule_id == schedule_id);
        assert!(trace.is_some(), "interval-backed annual trace should exist");
        let Some(trace) = trace else {
            return;
        };
        assert_eq!(&trace.values[..5], first_values.as_slice());
        assert!(trace.values[5..].iter().all(|value| *value == 175.0));
        assert_eq!(
            trace.kind,
            ScheduleSeriesKind::YearWeekDayCompiledProfiles {
                schedule_day_count: 366,
                compiled_day_schedule_count: 4,
                minutes_per_timestep: 15,
            }
        );
    }
}

#[test]
fn compiled_day_table_uses_shared_id_offsets_and_is_immutable() {
    let mut model = interval_mode_model();
    assert_eq!(
        model
            .day_interval_schedules
            .iter()
            .map(|schedule| schedule.id)
            .collect::<Vec<_>>(),
        vec![DayScheduleId(1), DayScheduleId(2), DayScheduleId(3)]
    );

    let table = precompile_day_schedule_table(&model, 4);
    assert_eq!(table.resolved_schedule_count(), 4);
    assert_eq!(
        compiled_day_schedule_value(&table, DayScheduleId(0), 30),
        Some(7.0)
    );
    assert_eq!(
        compiled_day_schedule_value(&table, DayScheduleId(1), 30),
        Some(175.0)
    );

    model.day_interval_schedules[0].segments[1].value = 999.0;
    assert_eq!(
        compiled_day_schedule_value(&table, DayScheduleId(1), 30),
        Some(175.0),
        "compiled day cache must not observe later model mutation"
    );
}

#[test]
fn hourly_axis_fails_closed_for_interval_profiles_needing_aggregation() {
    let model = interval_mode_model();
    let axis = build_hourly_time_axis(&model);
    assert!(axis.is_ok(), "one-day hourly axis should build");
    let Ok(axis) = axis else {
        return;
    };

    let traces = precompute_schedule_value_series_for_time_axis(&model, &axis);
    for schedule_id in [ScheduleId(40), ScheduleId(41), ScheduleId(42)] {
        let trace = traces.iter().find(|trace| trace.schedule_id == schedule_id);
        assert!(trace.is_some(), "interval-backed annual trace should exist");
        let Some(trace) = trace else {
            return;
        };
        assert!(trace.values.iter().all(|value| value.is_nan()));
    }
}

#[test]
fn day_list_modes_precompute_exact_zone_timestep_vectors() {
    let model = day_list_mode_model();
    let axis =
        build_environment_time_axis_for_run_period_with_zone_timesteps(&model.run_periods[0], 1, 4);
    assert!(axis.is_ok(), "one-day list axis should build");
    let Ok(axis) = axis else {
        return;
    };

    let traces = precompute_schedule_value_series_for_environment_time_axis(&model, &axis);
    assert_eq!(
        traces.iter().map(|trace| trace.values.len()).sum::<usize>(),
        288
    );
    for (schedule_id, first_values) in [
        (ScheduleId(50), vec![10.0, 70.0, 160.0, 160.0]),
        (ScheduleId(51), vec![10.0, 50.0, 100.0, 160.0]),
        (ScheduleId(52), vec![10.0, 70.0, 160.0, 160.0]),
    ] {
        let trace = traces.iter().find(|trace| trace.schedule_id == schedule_id);
        assert!(trace.is_some(), "list-backed annual trace should exist");
        let Some(trace) = trace else {
            return;
        };
        assert_eq!(&trace.values[..4], first_values.as_slice());
        assert!(trace.values[4..].iter().all(|value| *value == 175.0));
        assert_eq!(
            trace.kind,
            ScheduleSeriesKind::YearWeekDayCompiledProfiles {
                schedule_day_count: 366,
                compiled_day_schedule_count: 5,
                minutes_per_timestep: 15,
            }
        );
    }
}

#[test]
fn compiled_day_table_extends_shared_offsets_and_keeps_list_values_immutable() {
    let mut model = day_list_mode_model();
    assert_eq!(model.day_interval_schedules[0].id, DayScheduleId(1));
    assert_eq!(
        model
            .day_list_schedules
            .iter()
            .map(|schedule| schedule.id)
            .collect::<Vec<_>>(),
        vec![DayScheduleId(2), DayScheduleId(3), DayScheduleId(4)]
    );

    let table = precompile_day_schedule_table(&model, 4);
    assert_eq!(table.resolved_schedule_count(), 5);
    assert_eq!(
        compiled_day_schedule_value(&table, DayScheduleId(0), 30),
        Some(7.0)
    );
    assert_eq!(
        compiled_day_schedule_value(&table, DayScheduleId(1), 30),
        Some(8.0)
    );
    assert_eq!(
        compiled_day_schedule_value(&table, DayScheduleId(2), 30),
        Some(70.0)
    );

    model.day_list_schedules[0].values[1] = 999.0;
    assert_eq!(
        compiled_day_schedule_value(&table, DayScheduleId(2), 30),
        Some(70.0),
        "compiled day cache must not observe later list mutation"
    );

    let mut invalid_model = day_list_mode_model();
    invalid_model.day_list_schedules[0].id = DayScheduleId(1);
    let invalid_table = precompile_day_schedule_table(&invalid_model, 4);
    assert_eq!(invalid_table.resolved_schedule_count(), 3);
    assert_eq!(
        compiled_day_schedule_value(&invalid_table, DayScheduleId(1), 30),
        None,
        "duplicate shared IDs must fail closed"
    );
    assert_eq!(
        compiled_day_schedule_value(&invalid_table, DayScheduleId(2), 30),
        None,
        "missing shared IDs must fail closed"
    );
}

#[test]
fn hourly_axis_fails_closed_for_subhourly_day_list_profiles() {
    let model = day_list_mode_model();
    let axis = build_hourly_time_axis(&model);
    assert!(axis.is_ok(), "one-day hourly axis should build");
    let Ok(axis) = axis else {
        return;
    };

    let traces = precompute_schedule_value_series_for_time_axis(&model, &axis);
    for schedule_id in [ScheduleId(50), ScheduleId(51), ScheduleId(52)] {
        let trace = traces.iter().find(|trace| trace.schedule_id == schedule_id);
        assert!(trace.is_some(), "list-backed annual trace should exist");
        let Some(trace) = trace else {
            return;
        };
        assert!(trace.values.iter().all(|value| value.is_nan()));
    }
}

#[test]
fn hourly_axis_treats_day_list_linear_as_no_but_rejects_average() {
    let mut model = day_list_mode_model();
    for (schedule, value) in model.day_list_schedules.iter_mut().zip([11.0, 22.0, 33.0]) {
        schedule.minutes_per_item = 60;
        schedule.values = vec![value; 24];
    }
    let axis = build_hourly_time_axis(&model);
    assert!(axis.is_ok(), "one-day hourly axis should build");
    let Ok(axis) = axis else {
        return;
    };

    let traces = precompute_schedule_value_series_for_time_axis(&model, &axis);
    for (schedule_id, value) in [(ScheduleId(50), 11.0), (ScheduleId(52), 33.0)] {
        let trace = traces.iter().find(|trace| trace.schedule_id == schedule_id);
        assert!(trace.is_some(), "hour-aligned list trace should exist");
        let Some(trace) = trace else {
            return;
        };
        assert_eq!(trace.values, vec![value; 24]);
    }

    let average_trace = traces
        .iter()
        .find(|trace| trace.schedule_id == ScheduleId(51));
    assert!(average_trace.is_some(), "average list trace should exist");
    let Some(average_trace) = average_trace else {
        return;
    };
    assert!(average_trace.values.iter().all(|value| value.is_nan()));
}
