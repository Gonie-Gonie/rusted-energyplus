use super::{
    ScheduleSeriesKind, file_schedule_hourly_8760_value, precompute_schedule_value_series,
    precompute_schedule_value_series_for_time_axis, year_schedule_hourly_value,
};
use crate::time_axis::build_hourly_time_axis;
use ep_model::{
    DayScheduleId, NormalizedName, ScheduleDayHourly, ScheduleDayType, ScheduleFile,
    ScheduleFileColumnSeparator, ScheduleId, ScheduleWeekDaily, ScheduleYear, TypedModel,
    WeekScheduleId,
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
