use super::{
    ScheduleSeriesKind, file_schedule_hourly_8760_value, precompute_schedule_value_series,
};
use ep_model::{NormalizedName, ScheduleFile, ScheduleFileColumnSeparator, ScheduleId, TypedModel};

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
