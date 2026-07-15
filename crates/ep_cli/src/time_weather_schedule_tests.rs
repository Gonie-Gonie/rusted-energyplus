use std::collections::BTreeSet;

use ep_conformance::{
    OutputFrequency, OutputRequest, SourceArtifact, TimestampContract, VariableClass,
};
use ep_model::{NormalizedName, ScheduleCompact, ScheduleCompactSegment, ScheduleId, TypedModel};
use ep_runtime::{
    DayType, DaylightSavingPeriodSource, EpwCalendarMetadata, EpwRecord,
    ResolvedDaylightSavingDate, ResolvedDaylightSavingPeriod,
    build_hourly_time_axis_with_weather_metadata,
};

use super::{
    append_daylight_saving_markdown, build_hourly_time_axis,
    precompute_schedule_value_series_for_time_axis, schedule_samples, weather_calendar_json,
    weather_samples,
};

#[test]
fn schedule_samples_resolves_compact_trace_from_shared_name_registry()
-> Result<(), Box<dyn std::error::Error>> {
    let mut model = TypedModel::default();
    let schedule_id = ScheduleId(0);
    assert!(
        model
            .schedule_names
            .insert("Calendar Hourly", schedule_id)
            .is_none()
    );
    model.compact_schedules.push(ScheduleCompact {
        id: schedule_id,
        name: NormalizedName::new("Calendar Hourly"),
        schedule_type_limits: None,
        segments: (1..=24)
            .map(|hour| ScheduleCompactSegment {
                until_minute_of_day: hour * 60,
                value: f64::from(hour),
            })
            .collect(),
    });
    let time_axis = build_hourly_time_axis(&model).map_err(std::io::Error::other)?;
    let schedule_series = precompute_schedule_value_series_for_time_axis(&model, &time_axis);
    let output = OutputRequest {
        key: "CALENDAR HOURLY".to_string(),
        variable: "Schedule Value".to_string(),
        frequency: OutputFrequency::Hourly,
        class: VariableClass::Schedule,
        source: SourceArtifact::Eso,
        timestamp_contract: Some(TimestampContract::OrderedExactUnique),
        domain: None,
        level: None,
        abs_tol: None,
        rmse_tol: None,
        rel_tol: None,
    };

    let samples = schedule_samples(&output, &model, &time_axis, &schedule_series)
        .map_err(std::io::Error::other)?;
    let timestamps = samples
        .iter()
        .filter_map(|sample| sample.timestamp.as_deref())
        .collect::<BTreeSet<_>>();

    assert_eq!(samples.len(), 24);
    assert_eq!(timestamps.len(), 24);
    assert_eq!(samples.first().map(|sample| sample.value), Some(1.0));
    assert_eq!(samples.last().map(|sample| sample.value), Some(24.0));
    Ok(())
}

#[test]
fn daylight_saving_status_samples_come_from_the_shared_time_axis()
-> Result<(), Box<dyn std::error::Error>> {
    let mut time_axis =
        build_hourly_time_axis(&TypedModel::default()).map_err(std::io::Error::other)?;
    time_axis.points[0].dst = true;
    time_axis.points[23].dst = true;
    let weather_record = EpwRecord {
        year: 2017,
        month: 1,
        day: 1,
        hour: 1,
        minute: 60,
        dry_bulb_c: 10.0,
        dew_point_c: 5.0,
        relative_humidity_percent: 50.0,
        atmospheric_pressure_pa: 101_325.0,
        horizontal_infrared_radiation_wh_per_m2: 0.0,
        global_horizontal_radiation_wh_per_m2: 0.0,
        direct_normal_radiation_wh_per_m2: 0.0,
        diffuse_horizontal_radiation_wh_per_m2: 0.0,
        wind_direction_deg: 0.0,
        wind_speed_m_per_s: 0.0,
        liquid_precipitation_depth_mm: 0.0,
    };
    let weather_records = vec![weather_record; time_axis.sample_count()];
    let output = OutputRequest {
        key: "ENVIRONMENT".to_string(),
        variable: "Site Daylight Saving Time Status".to_string(),
        frequency: OutputFrequency::Hourly,
        class: VariableClass::Weather,
        source: SourceArtifact::Eso,
        timestamp_contract: Some(TimestampContract::OrderedExactUnique),
        domain: None,
        level: None,
        abs_tol: None,
        rmse_tol: None,
        rel_tol: None,
    };

    let samples = weather_samples(&output, &time_axis, Some(&weather_records))
        .map_err(std::io::Error::other)?;

    assert_eq!(samples.len(), 24);
    assert_eq!(samples.first().map(|sample| sample.value), Some(1.0));
    assert_eq!(samples.get(1).map(|sample| sample.value), Some(0.0));
    assert_eq!(samples.last().map(|sample| sample.value), Some(1.0));
    Ok(())
}

#[test]
fn daylight_saving_reports_distinguish_input_file_precedence()
-> Result<(), Box<dyn std::error::Error>> {
    let mut time_axis = build_hourly_time_axis_with_weather_metadata(
        &TypedModel::default(),
        &EpwCalendarMetadata::default(),
    )
    .map_err(std::io::Error::other)?;
    time_axis.daylight_saving.weather_file_period_declared = true;
    time_axis
        .daylight_saving
        .run_period_uses_weather_file_period = false;
    time_axis.daylight_saving.input_file_period_declared = true;
    time_axis.daylight_saving.active = true;
    time_axis.daylight_saving.effective_source = DaylightSavingPeriodSource::InputFile;
    time_axis.daylight_saving.resolved_period = Some(ResolvedDaylightSavingPeriod {
        start: ResolvedDaylightSavingDate {
            month: 2,
            day_of_month: 28,
            day_of_year: 59,
        },
        end: ResolvedDaylightSavingDate {
            month: 2,
            day_of_month: 29,
            day_of_year: 60,
        },
        wraps_year: false,
    });
    time_axis.points[0].dst = true;

    let mut markdown = String::new();
    append_daylight_saving_markdown(&mut markdown, &time_axis);
    assert!(markdown.contains("weather_file_daylight_saving_period_declared: true\n"));
    assert!(markdown.contains("run_period_uses_weather_file_daylight_saving_period: false\n"));
    assert!(markdown.contains("input_file_daylight_saving_period_declared: true\n"));
    assert!(markdown.contains("daylight_saving_active: true\n"));
    assert!(markdown.contains("daylight_saving_effective_source: input-file\n"));
    assert!(
        markdown
            .contains("daylight_saving_resolved_period: 2/28 through 2/29 (wraps_year=false)\n")
    );
    assert!(markdown.contains("daylight_saving_hourly_samples: 1\n"));

    let json = weather_calendar_json(&time_axis);
    assert!(json.contains("\"start_year\": 2017"));
    assert!(json.contains("\"end_year\": 2017"));
    assert!(json.contains("\"end_year_gregorian_leap\": false"));
    assert!(json.contains("\"end_year_weather_effective_leap\": false"));
    assert!(json.contains("\"weather_file_period_declared\": true"));
    assert!(json.contains("\"run_period_uses_weather_file_period\": false"));
    assert!(json.contains("\"input_file_period_declared\": true"));
    assert!(json.contains("\"active\": true"));
    assert!(json.contains("\"effective_source\": \"input-file\""));
    assert!(json.contains(
        "\"resolved_period\": {\"start_month\": 2, \"start_day\": 28, \"start_day_of_year\": 59, \"end_month\": 2, \"end_day\": 29, \"end_day_of_year\": 60, \"wraps_year\": false}"
    ));
    assert!(json.contains("\"daylight_saving_hourly_samples\": 1"));
    Ok(())
}

#[test]
fn site_day_type_index_samples_come_from_the_shared_time_axis()
-> Result<(), Box<dyn std::error::Error>> {
    let mut time_axis =
        build_hourly_time_axis(&TypedModel::default()).map_err(std::io::Error::other)?;
    time_axis.points[0].day_type = DayType::Holiday;
    time_axis.points[0].special_day_type = Some(DayType::Holiday);
    time_axis.points[23].day_type = DayType::CustomDay2;
    time_axis.points[23].special_day_type = Some(DayType::CustomDay2);
    let weather_record = EpwRecord {
        year: 2017,
        month: 1,
        day: 1,
        hour: 1,
        minute: 60,
        dry_bulb_c: 10.0,
        dew_point_c: 5.0,
        relative_humidity_percent: 50.0,
        atmospheric_pressure_pa: 101_325.0,
        horizontal_infrared_radiation_wh_per_m2: 0.0,
        global_horizontal_radiation_wh_per_m2: 0.0,
        direct_normal_radiation_wh_per_m2: 0.0,
        diffuse_horizontal_radiation_wh_per_m2: 0.0,
        wind_direction_deg: 0.0,
        wind_speed_m_per_s: 0.0,
        liquid_precipitation_depth_mm: 0.0,
    };
    let weather_records = vec![weather_record; time_axis.sample_count()];
    let output = OutputRequest {
        key: "ENVIRONMENT".to_string(),
        variable: "Site Day Type Index".to_string(),
        frequency: OutputFrequency::Hourly,
        class: VariableClass::Weather,
        source: SourceArtifact::Eso,
        timestamp_contract: Some(TimestampContract::OrderedExactUnique),
        domain: None,
        level: None,
        abs_tol: None,
        rmse_tol: None,
        rel_tol: None,
    };

    let samples = weather_samples(&output, &time_axis, Some(&weather_records))
        .map_err(std::io::Error::other)?;

    assert_eq!(samples.len(), 24);
    assert_eq!(samples.first().map(|sample| sample.value), Some(8.0));
    assert_eq!(samples.get(1).map(|sample| sample.value), Some(1.0));
    assert_eq!(samples.last().map(|sample| sample.value), Some(12.0));
    assert!(
        samples
            .first()
            .and_then(|sample| sample.timestamp.as_deref())
            .is_some_and(|timestamp| timestamp.contains("day_type=Holiday"))
    );
    Ok(())
}
