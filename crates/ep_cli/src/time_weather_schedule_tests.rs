use std::collections::BTreeSet;

use ep_conformance::{
    ConformanceCase, OutputFrequency, OutputRequest, SourceArtifact, TimestampContract,
    VariableClass, parse_case_str,
};
use ep_model::{
    CalendarDateRule, DayOfWeek, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod,
    RunPeriodId, RunPeriodSpecialDay, RunPeriodSpecialDayId, ScheduleCompact,
    ScheduleCompactDayProfile, ScheduleCompactPeriod, ScheduleCompactSegment, ScheduleConstant,
    ScheduleDayType, ScheduleId, ScheduleInterpolation, SpecialDayType, TimestepConfig, TypedModel,
};
use ep_runtime::{
    DayType, DaylightSavingPeriodSource, EpwCalendarDateRule, EpwCalendarMetadata,
    EpwDaylightSavingPeriod, EpwRecord, ResolvedDaylightSavingDate, ResolvedDaylightSavingPeriod,
    ScheduleSeriesIndexKind, build_environment_time_axes,
    build_hourly_time_axis_with_weather_metadata, precompute_constant_schedule_cache,
};

use super::{
    append_daylight_saving_markdown, build_hourly_time_axis,
    precompute_schedule_cache_for_environment_time_axis, precompute_schedule_cache_for_time_axis,
    schedule_samples, schedule_timestep_samples, validate_manifest, weather_calendar_json,
    weather_samples,
};

fn report_manifest(outputs: &str) -> Result<ConformanceCase, Box<dyn std::error::Error>> {
    let manifest = format!(
        r#"
id = "time_weather_schedule_test"
title = "Time weather schedule test"
milestone = "test"
purpose = "Exercise report validation"
comparison_class = "conformance"
conformance_claim = true
oracle_version = "26.1.0"

[input]
idf = "test.idf"

{outputs}

[[tolerances]]
variable_class = "schedule"
max_abs = 0.0

[[tolerances]]
variable_class = "weather"
max_abs = 0.0

[report]
format = "markdown"
path = "test-report.md"

[gate]
script = "scripts/dev.cmd test"
blocking = true
"#
    );
    Ok(parse_case_str(&manifest)?)
}

fn all_schedule_day_types() -> Vec<ScheduleDayType> {
    vec![
        ScheduleDayType::Sunday,
        ScheduleDayType::Monday,
        ScheduleDayType::Tuesday,
        ScheduleDayType::Wednesday,
        ScheduleDayType::Thursday,
        ScheduleDayType::Friday,
        ScheduleDayType::Saturday,
        ScheduleDayType::Holiday,
        ScheduleDayType::SummerDesignDay,
        ScheduleDayType::WinterDesignDay,
        ScheduleDayType::CustomDay1,
        ScheduleDayType::CustomDay2,
    ]
}

fn compact_day_profile(day_types: Vec<ScheduleDayType>, value: f64) -> ScheduleCompactDayProfile {
    ScheduleCompactDayProfile {
        day_types,
        interpolation: ScheduleInterpolation::No,
        segments: vec![ScheduleCompactSegment {
            until_minute_of_day: 24 * 60,
            value,
        }],
    }
}

fn cross_year_day_type_compact_schedule(id: ScheduleId) -> ScheduleCompact {
    let period_one_other_days = all_schedule_day_types()
        .into_iter()
        .filter(|day_type| *day_type != ScheduleDayType::Thursday)
        .collect();
    let period_two_other_days = all_schedule_day_types()
        .into_iter()
        .filter(|day_type| {
            !matches!(
                day_type,
                ScheduleDayType::Tuesday | ScheduleDayType::Wednesday | ScheduleDayType::Holiday
            )
        })
        .collect();
    ScheduleCompact {
        id,
        name: NormalizedName::new("Cross Year Day Type"),
        schedule_type_limits: None,
        periods: vec![
            ScheduleCompactPeriod {
                through_schedule_day_of_year: 1,
                day_profiles: vec![
                    compact_day_profile(vec![ScheduleDayType::Thursday], 105.0),
                    compact_day_profile(period_one_other_days, 199.0),
                ],
            },
            ScheduleCompactPeriod {
                through_schedule_day_of_year: 366,
                day_profiles: vec![
                    compact_day_profile(vec![ScheduleDayType::Tuesday], 103.0),
                    compact_day_profile(vec![ScheduleDayType::Wednesday], 104.0),
                    compact_day_profile(vec![ScheduleDayType::Holiday], 108.0),
                    compact_day_profile(period_two_other_days, 199.0),
                ],
            },
        ],
    }
}

#[test]
fn manifest_accepts_homogeneous_timestep_schedule_outputs() -> Result<(), Box<dyn std::error::Error>>
{
    let manifest = report_manifest(
        r#"
[[outputs]]
key = "TIMESTEP SCHEDULE"
variable = "Schedule Value"
frequency = "timestep"
class = "schedule"
source = "eso"
level = "conformance"
"#,
    )?;

    validate_manifest(&manifest).map_err(std::io::Error::other)?;
    Ok(())
}

#[test]
fn manifest_rejects_mixed_hourly_and_timestep_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = report_manifest(
        r#"
[[outputs]]
key = "HOURLY SCHEDULE"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"
level = "conformance"

[[outputs]]
key = "TIMESTEP SCHEDULE"
variable = "Schedule Value"
frequency = "timestep"
class = "schedule"
source = "eso"
level = "conformance"
"#,
    )?;

    let error = validate_manifest(&manifest).expect_err("mixed frequencies must fail");
    assert!(error.contains("mixed output frequencies are unsupported"));
    Ok(())
}

#[test]
fn manifest_rejects_timestep_weather_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = report_manifest(
        r#"
[[outputs]]
key = "ENVIRONMENT"
variable = "Site Outdoor Air Drybulb Temperature"
frequency = "timestep"
class = "weather"
source = "eso"
level = "conformance"
"#,
    )?;

    let error = validate_manifest(&manifest).expect_err("timestep weather must fail");
    assert!(error.contains("does not support timestep weather outputs"));
    Ok(())
}

#[test]
fn timestep_schedule_samples_use_environment_axis_values_and_unique_labels()
-> Result<(), Box<dyn std::error::Error>> {
    let schedule_id = ScheduleId(52);
    let mut model = TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 4,
        },
        compact_schedules: vec![ScheduleCompact {
            id: schedule_id,
            name: NormalizedName::new("Timestep Schedule"),
            schedule_type_limits: None,
            periods: vec![ScheduleCompactPeriod {
                through_schedule_day_of_year: 366,
                day_profiles: vec![ScheduleCompactDayProfile {
                    day_types: all_schedule_day_types(),
                    interpolation: ScheduleInterpolation::No,
                    segments: vec![
                        ScheduleCompactSegment {
                            until_minute_of_day: 15,
                            value: 1.0,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 30,
                            value: 2.0,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 45,
                            value: 3.0,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 60,
                            value: 4.0,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 24 * 60,
                            value: 9.0,
                        },
                    ],
                }],
            }],
        }],
        ..TypedModel::default()
    };
    assert!(
        model
            .schedule_names
            .insert("Timestep Schedule", schedule_id)
            .is_none()
    );
    let time_axis = build_environment_time_axes(&model)
        .map_err(std::io::Error::other)?
        .into_iter()
        .next()
        .ok_or_else(|| std::io::Error::other("missing environment time axis"))?;
    let schedule_cache = precompute_schedule_cache_for_environment_time_axis(&model, &time_axis);
    let output = OutputRequest {
        key: "TIMESTEP SCHEDULE".to_string(),
        variable: "Schedule Value".to_string(),
        frequency: OutputFrequency::Timestep,
        class: VariableClass::Schedule,
        source: SourceArtifact::Eso,
        timestamp_contract: None,
        domain: None,
        level: None,
        abs_tol: None,
        rmse_tol: None,
        rel_tol: None,
    };

    let samples = schedule_timestep_samples(&output, &model, &time_axis, &schedule_cache)
        .map_err(std::io::Error::other)?;
    let timestamps = samples
        .iter()
        .filter_map(|sample| sample.timestamp.as_deref())
        .collect::<BTreeSet<_>>();

    assert_eq!(samples.len(), 96);
    assert_eq!(timestamps.len(), 96);
    assert_eq!(
        samples
            .iter()
            .take(5)
            .map(|sample| sample.value)
            .collect::<Vec<_>>(),
        vec![1.0, 2.0, 3.0, 4.0, 9.0]
    );
    assert!(
        samples[0]
            .timestamp
            .as_deref()
            .is_some_and(|timestamp| timestamp.contains("hour=1;start=0.00;end=15.00"))
    );
    assert!(
        samples[3]
            .timestamp
            .as_deref()
            .is_some_and(|timestamp| timestamp.contains("hour=1;start=45.00;end=60.00"))
    );
    Ok(())
}

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
        periods: vec![ScheduleCompactPeriod {
            through_schedule_day_of_year: 366,
            day_profiles: vec![ScheduleCompactDayProfile {
                day_types: all_schedule_day_types(),
                interpolation: ScheduleInterpolation::No,
                segments: (1..=24)
                    .map(|hour| ScheduleCompactSegment {
                        until_minute_of_day: hour * 60,
                        value: f64::from(hour),
                    })
                    .collect(),
            }],
        }],
    });
    let time_axis = build_hourly_time_axis(&model).map_err(std::io::Error::other)?;
    let schedule_cache = precompute_schedule_cache_for_time_axis(&model, &time_axis);
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

    let samples = schedule_samples(&output, &model, &time_axis, &schedule_cache)
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
fn schedule_samples_consume_cross_year_through_and_for_profiles()
-> Result<(), Box<dyn std::error::Error>> {
    let schedule_id = ScheduleId(41);
    let mut model = TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 1,
        },
        run_periods: vec![RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("Cross Year Schedule"),
            begin_month: 12,
            begin_day_of_month: 30,
            begin_year: Some(2031),
            end_month: 1,
            end_day_of_month: 3,
            end_year: Some(2032),
            day_of_week_for_start_day: Some(DayOfWeek::Tuesday),
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
            use_weather_file_holidays_and_special_days: false,
            use_weather_file_daylight_saving_period: false,
            apply_weekend_holiday_rule: false,
            use_weather_file_rain_indicators: false,
            use_weather_file_snow_indicators: false,
            treat_weather_as_actual: false,
        }],
        run_period_special_days: vec![RunPeriodSpecialDay {
            id: RunPeriodSpecialDayId(0),
            name: NormalizedName::new("January Second Holiday"),
            start_date: CalendarDateRule::MonthDay {
                month: 1,
                day_of_month: 2,
            },
            duration_days: 1,
            special_day_type: SpecialDayType::Holiday,
        }],
        compact_schedules: vec![cross_year_day_type_compact_schedule(schedule_id)],
        ..TypedModel::default()
    };
    assert!(
        model
            .schedule_names
            .insert("Cross Year Day Type", schedule_id)
            .is_none()
    );

    let time_axis = build_hourly_time_axis(&model).map_err(std::io::Error::other)?;
    let schedule_cache = precompute_schedule_cache_for_time_axis(&model, &time_axis);
    let output = OutputRequest {
        key: "CROSS YEAR DAY TYPE".to_string(),
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

    let samples = schedule_samples(&output, &model, &time_axis, &schedule_cache)
        .map_err(std::io::Error::other)?;
    assert_eq!(samples.len(), 120);
    assert_eq!(
        time_axis
            .points
            .chunks_exact(24)
            .map(|day| day[0].schedule_day_of_year)
            .collect::<Vec<_>>(),
        vec![365, 366, 1, 2, 3]
    );
    assert_eq!(
        samples
            .chunks_exact(24)
            .map(|day| day[0].value)
            .collect::<Vec<_>>(),
        vec![103.0, 104.0, 105.0, 108.0, 199.0]
    );
    assert!(
        samples
            .chunks_exact(24)
            .all(|day| day.iter().all(|sample| sample.value == day[0].value))
    );
    for (index, label) in [
        (0, "day_type=Tuesday"),
        (24, "day_type=Wednesday"),
        (48, "day_type=Thursday"),
        (72, "day_type=Holiday"),
        (96, "day_type=Saturday"),
    ] {
        assert!(
            samples[index]
                .timestamp
                .as_deref()
                .is_some_and(|timestamp| timestamp.contains(label))
        );
    }

    Ok(())
}

#[test]
fn schedule_samples_consume_constant_scalar_cache_without_dense_allocation()
-> Result<(), Box<dyn std::error::Error>> {
    let schedule_id = ScheduleId(0);
    let mut model = TypedModel {
        schedules: vec![ScheduleConstant {
            id: schedule_id,
            name: NormalizedName::new("Scalar Schedule"),
            schedule_type_limits: None,
            hourly_value: 0.625,
        }],
        compact_schedules: vec![cross_year_day_type_compact_schedule(ScheduleId(9))],
        ..TypedModel::default()
    };
    assert!(
        model
            .schedule_names
            .insert("Scalar Schedule", schedule_id)
            .is_none()
    );
    let time_axis = build_hourly_time_axis(&model).map_err(std::io::Error::other)?;
    let schedule_cache = precompute_constant_schedule_cache(&model, time_axis.sample_count());
    let profile = schedule_cache.profile();

    assert_eq!(schedule_cache.len(), 1);
    assert_eq!(profile.scalar_series_count, 1);
    assert_eq!(profile.dense_series_count, 0);
    assert_eq!(profile.logical_sample_count, time_axis.sample_count());
    assert_eq!(profile.allocated_dense_sample_count, 0);
    assert_eq!(profile.index_kind, ScheduleSeriesIndexKind::DenseIdentity);
    assert_eq!(profile.ambiguous_id_count, 0);

    let output = OutputRequest {
        key: "SCALAR SCHEDULE".to_string(),
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
    let samples = schedule_samples(&output, &model, &time_axis, &schedule_cache)
        .map_err(std::io::Error::other)?;

    assert_eq!(samples.len(), time_axis.sample_count());
    assert!(samples.iter().all(|sample| sample.value == 0.625));
    assert!(samples.iter().all(|sample| sample.timestamp.is_some()));
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
fn daylight_saving_reports_cross_year_start_year_projection()
-> Result<(), Box<dyn std::error::Error>> {
    let run_period = RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new("Cross Year DST"),
        begin_month: 12,
        begin_day_of_month: 30,
        begin_year: Some(2031),
        end_month: 1,
        end_day_of_month: 2,
        end_year: Some(2032),
        day_of_week_for_start_day: Some(DayOfWeek::Tuesday),
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        use_weather_file_holidays_and_special_days: false,
        use_weather_file_daylight_saving_period: true,
        apply_weekend_holiday_rule: false,
        use_weather_file_rain_indicators: false,
        use_weather_file_snow_indicators: false,
        treat_weather_as_actual: false,
    };
    let model = TypedModel {
        timestep: TimestepConfig {
            number_of_timesteps_per_hour: 1,
        },
        run_periods: vec![run_period],
        ..TypedModel::default()
    };
    let metadata = EpwCalendarMetadata {
        leap_year_observed: true,
        daylight_saving_period: Some(EpwDaylightSavingPeriod {
            start: EpwCalendarDateRule::NthWeekdayInMonth {
                nth: 1,
                weekday: DayOfWeek::Thursday,
                month: 1,
            },
            end: EpwCalendarDateRule::NthWeekdayInMonth {
                nth: 1,
                weekday: DayOfWeek::Friday,
                month: 1,
            },
        }),
        holidays: Vec::new(),
    };
    let time_axis = build_hourly_time_axis_with_weather_metadata(&model, &metadata)
        .map_err(std::io::Error::other)?;

    let mut markdown = String::new();
    append_daylight_saving_markdown(&mut markdown, &time_axis);
    assert!(markdown.contains("weather_file_daylight_saving_period_declared: true\n"));
    assert!(markdown.contains("run_period_uses_weather_file_daylight_saving_period: true\n"));
    assert!(markdown.contains("input_file_daylight_saving_period_declared: false\n"));
    assert!(markdown.contains("daylight_saving_active: true\n"));
    assert!(markdown.contains("daylight_saving_effective_source: weather-file\n"));
    assert!(
        markdown.contains("daylight_saving_resolved_period: 1/2 through 1/3 (wraps_year=false)\n")
    );
    assert!(markdown.contains("daylight_saving_hourly_samples: 24\n"));

    let json = weather_calendar_json(&time_axis);
    assert!(json.contains("\"start_year\": 2031"));
    assert!(json.contains("\"end_year\": 2032"));
    assert!(json.contains("\"start_year_gregorian_leap\": false"));
    assert!(json.contains("\"start_year_weather_effective_leap\": false"));
    assert!(json.contains("\"end_year_gregorian_leap\": true"));
    assert!(json.contains("\"end_year_weather_effective_leap\": true"));
    assert!(json.contains("\"weather_file_period_declared\": true"));
    assert!(json.contains("\"run_period_uses_weather_file_period\": true"));
    assert!(json.contains("\"input_file_period_declared\": false"));
    assert!(json.contains("\"active\": true"));
    assert!(json.contains("\"effective_source\": \"weather-file\""));
    assert!(json.contains(
        "\"resolved_period\": {\"start_month\": 1, \"start_day\": 2, \"start_day_of_year\": 2, \"end_month\": 1, \"end_day\": 3, \"end_day_of_year\": 3, \"wraps_year\": false}"
    ));
    assert!(json.contains("\"daylight_saving_hourly_samples\": 24"));

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
