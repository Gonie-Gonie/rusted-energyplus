use std::collections::BTreeSet;

use ep_conformance::{
    OutputFrequency, OutputRequest, SourceArtifact, TimestampContract, VariableClass,
};
use ep_model::{NormalizedName, ScheduleCompact, ScheduleCompactSegment, ScheduleId, TypedModel};
use ep_runtime::EpwRecord;

use super::{
    build_hourly_time_axis, precompute_schedule_value_series_for_time_axis, schedule_samples,
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
