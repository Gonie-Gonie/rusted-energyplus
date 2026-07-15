use super::super::{
    DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model, typed_coverage_status,
};
use ep_model::{DayScheduleId, ScheduleId, ScheduleInterpolation, ScheduleTypeLimitId};
use ep_raw_model::parse_epjson_str;

#[test]
fn compiles_modes_with_shared_hourly_offset_and_week_references()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Timestep": {
                "Timestep 1": {"number_of_timesteps_per_hour": 4}
            },
            "ScheduleTypeLimits": {
                "Any Number": {}
            },
            "Schedule:Constant": {
                "Top Level Stable": {"hourly_value": 9}
            },
            "Schedule:Day:Hourly": {
                "Hourly A": {"hour_1": 1},
                "Hourly B": {"hour_1": 2}
            },
            "Schedule:Day:Interval": {
                "Interval A Default": {
                    "schedule_type_limits_name": "Any Number",
                    "interpolate_to_timestep": "",
                    "data": [
                        {"time": "08:00", "value_until_time": 1},
                        {"time": "24:00", "value_until_time": 2}
                    ]
                },
                "Interval B Average": {
                    "interpolate_to_timestep": "Average",
                    "data": [
                        {"time": "00:20", "value_until_time": 3},
                        {"time": "24:00", "value_until_time": 4}
                    ]
                },
                "Interval C Linear": {
                    "interpolate_to_timestep": "Linear",
                    "data": [
                        {"time": "Until: 01:00", "value_until_time": 5},
                        {"time": "24:00", "value_until_time": 6}
                    ]
                }
            },
            "Schedule:Week:Daily": {
                "Mixed Week": {
                    "sunday_schedule_day_name": "Hourly A",
                    "monday_schedule_day_name": "Hourly B",
                    "tuesday_schedule_day_name": "Interval A Default",
                    "wednesday_schedule_day_name": "Interval B Average",
                    "thursday_schedule_day_name": "Interval C Linear",
                    "friday_schedule_day_name": "Interval A Default",
                    "saturday_schedule_day_name": "Interval B Average",
                    "holiday_schedule_day_name": "Interval C Linear",
                    "summerdesignday_schedule_day_name": "Interval A Default",
                    "winterdesignday_schedule_day_name": "Interval B Average",
                    "customday1_schedule_day_name": "Interval C Linear",
                    "customday2_schedule_day_name": "Hourly A"
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status("Schedule:Day:Interval"),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.defaults_applied.iter().any(|default| {
        default.object_type == "Schedule:Day:Interval"
            && default.object_name == "Interval A Default"
            && default.field == "interpolate_to_timestep"
            && default.value == "No"
    }));
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.day_schedules.len(), 2);
    assert_eq!(model.day_interval_schedules.len(), 3);
    assert_eq!(model.schedules[0].id, ScheduleId(0));
    assert_eq!(
        model
            .day_interval_schedules
            .iter()
            .map(|schedule| schedule.id)
            .collect::<Vec<_>>(),
        vec![DayScheduleId(2), DayScheduleId(3), DayScheduleId(4)]
    );

    let default = model
        .day_interval_schedules
        .iter()
        .find(|schedule| schedule.name.0 == "INTERVAL A DEFAULT")
        .ok_or_else(|| std::io::Error::other("missing default interval"))?;
    assert_eq!(default.interpolation, ScheduleInterpolation::No);
    assert_eq!(default.schedule_type_limits, Some(ScheduleTypeLimitId(0)));
    assert_eq!(default.segments.len(), 2);
    assert_eq!(default.segments[0].until_minute_of_day, 8 * 60);
    assert_eq!(default.segments[1].until_minute_of_day, 24 * 60);

    let average = model
        .day_interval_schedules
        .iter()
        .find(|schedule| schedule.name.0 == "INTERVAL B AVERAGE")
        .ok_or_else(|| std::io::Error::other("missing average interval"))?;
    assert_eq!(average.interpolation, ScheduleInterpolation::Average);
    assert_eq!(average.segments[0].until_minute_of_day, 20);

    let linear = model
        .day_interval_schedules
        .iter()
        .find(|schedule| schedule.name.0 == "INTERVAL C LINEAR")
        .ok_or_else(|| std::io::Error::other("missing linear interval"))?;
    assert_eq!(linear.interpolation, ScheduleInterpolation::Linear);
    assert_eq!(linear.segments[0].until_minute_of_day, 60);

    assert_eq!(
        model.day_schedule_names.resolve("hourly a"),
        Some(DayScheduleId(0))
    );
    assert_eq!(
        model.day_schedule_names.resolve("interval a default"),
        Some(default.id)
    );
    let week = &model.week_schedules[0];
    assert_eq!(
        week.day_schedules,
        [
            DayScheduleId(0),
            DayScheduleId(1),
            default.id,
            average.id,
            linear.id,
            default.id,
            average.id,
            linear.id,
            default.id,
            average.id,
            linear.id,
            DayScheduleId(0),
        ]
    );
    Ok(())
}

#[test]
fn rejects_duplicate_name_across_hourly_and_interval_namespaces()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Schedule:Day:Hourly": {
                "Shared Day": {}
            },
            "Schedule:Day:Interval": {
                "Shared Day": {
                    "data": [{"time": "24:00", "value_until_time": 1}]
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == "Schedule:Day:Interval"
            && diagnostic.object_name.as_deref() == Some("Shared Day")
    }));
    Ok(())
}

#[test]
fn warns_when_no_interpolation_time_is_not_timestep_aligned()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Timestep": {
                "Timestep 1": {"number_of_timesteps_per_hour": 4}
            },
            "Schedule:Day:Interval": {
                "Unaligned No": {
                    "interpolate_to_timestep": "No",
                    "data": [
                        {"time": "00:20", "value_until_time": 1},
                        {"time": "24:00", "value_until_time": 2}
                    ]
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Warning
            && diagnostic.code == "ScheduleDayIntervalTimeNotAlignedToTimestep"
            && diagnostic.object_name.as_deref() == Some("Unaligned No[0]")
            && diagnostic.field.as_deref() == Some("time")
    }));
    assert_eq!(
        result
            .model
            .as_ref()
            .map(|model| model.day_interval_schedules.len()),
        Some(1)
    );
    Ok(())
}

#[test]
fn rejects_invalid_missing_mistyped_unordered_and_incomplete_data()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Schedule:Day:Interval": {
                "Missing Data": {},
                "Mistyped Data": {"data": "not-an-array"},
                "Empty Data": {"data": []},
                "Invalid Interpolation": {
                    "interpolate_to_timestep": "Spline",
                    "data": [{"time": "24:00", "value_until_time": 1}]
                },
                "Missing Time": {
                    "data": [{"value_until_time": 1}]
                },
                "Missing Value": {
                    "data": [{"time": "24:00"}]
                },
                "Mistyped Entry": {
                    "data": ["not-an-object"]
                },
                "Mistyped Time": {
                    "data": [{"time": 24, "value_until_time": 1}]
                },
                "Mistyped Value": {
                    "data": [{"time": "24:00", "value_until_time": "one"}]
                },
                "Invalid Time": {
                    "data": [{"time": "24:15", "value_until_time": 1}]
                },
                "Unordered Time": {
                    "data": [
                        {"time": "12:00", "value_until_time": 1},
                        {"time": "11:00", "value_until_time": 2},
                        {"time": "24:00", "value_until_time": 3}
                    ]
                },
                "Incomplete Day": {
                    "data": [{"time": "23:00", "value_until_time": 1}]
                },
                "Missing Type Limit": {
                    "schedule_type_limits_name": "Does Not Exist",
                    "data": [{"time": "24:00", "value_until_time": 1}]
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for (code, object_name, field) in [
        ("MissingRequiredField", "Missing Data", "data"),
        ("InvalidFieldType", "Mistyped Data", "data"),
        ("MissingScheduleDayIntervalData", "Empty Data", "data"),
        (
            "InvalidEnumValue",
            "Invalid Interpolation",
            "interpolate_to_timestep",
        ),
        ("MissingRequiredField", "Missing Time[0]", "time"),
        (
            "MissingRequiredField",
            "Missing Value[0]",
            "value_until_time",
        ),
        ("InvalidFieldType", "Mistyped Entry", "data"),
        ("InvalidFieldType", "Mistyped Time[0]", "time"),
        ("InvalidFieldType", "Mistyped Value[0]", "value_until_time"),
        ("InvalidScheduleDayIntervalTime", "Invalid Time[0]", "time"),
        (
            "InvalidScheduleDayIntervalTimeOrder",
            "Unordered Time[1]",
            "time",
        ),
        ("IncompleteScheduleDayInterval", "Incomplete Day", "data"),
        (
            "MissingReference",
            "Missing Type Limit",
            "schedule_type_limits_name",
        ),
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == code
                    && diagnostic.object_name.as_deref() == Some(object_name)
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing diagnostic {code} for {object_name}/{field}: {:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}
