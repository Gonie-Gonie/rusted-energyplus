use super::super::{ObjectCoverageStatus, compile_raw_model, typed_coverage_status};
use ep_model::{DayScheduleId, ScheduleInterpolation, ScheduleTypeLimitId};
use ep_raw_model::parse_epjson_str;

fn extensions(count: usize, missing_value_index: Option<usize>) -> String {
    (0..count)
        .map(|index| {
            if missing_value_index == Some(index) {
                "{}".to_string()
            } else {
                format!(r#"{{"value": {}}}"#, index + 1)
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[test]
fn compiles_source_order_defaults_shared_offsets_and_week_references()
-> Result<(), Box<dyn std::error::Error>> {
    let hourly = extensions(24, Some(1));
    let half_hourly = extensions(48, None);
    let quarter_hourly = extensions(96, None);
    let epjson = format!(
        r#"{{
            "ScheduleTypeLimits": {{
                "Any Number": {{}}
            }},
            "Schedule:Day:Hourly": {{
                "Hourly A": {{}}
            }},
            "Schedule:Day:Interval": {{
                "Interval A": {{
                    "data": [{{"time": "24:00", "value_until_time": 9}}]
                }}
            }},
            "Schedule:Day:List": {{
                "List A Default": {{
                    "schedule_type_limits_name": "Any Number",
                    "interpolate_to_timestep": "",
                    "minutes_per_item": 60,
                    "extensions": [{hourly}]
                }},
                "List B Average": {{
                    "interpolate_to_timestep": "Average",
                    "minutes_per_item": 30,
                    "extensions": [{half_hourly}]
                }},
                "List C Linear": {{
                    "interpolate_to_timestep": "Linear",
                    "minutes_per_item": 15,
                    "extensions": [{quarter_hourly}]
                }}
            }},
            "Schedule:Week:Daily": {{
                "Mixed Week": {{
                    "sunday_schedule_day_name": "Hourly A",
                    "monday_schedule_day_name": "Interval A",
                    "tuesday_schedule_day_name": "List A Default",
                    "wednesday_schedule_day_name": "List B Average",
                    "thursday_schedule_day_name": "List C Linear",
                    "friday_schedule_day_name": "List A Default",
                    "saturday_schedule_day_name": "List B Average",
                    "holiday_schedule_day_name": "List C Linear",
                    "summerdesignday_schedule_day_name": "List A Default",
                    "winterdesignday_schedule_day_name": "List B Average",
                    "customday1_schedule_day_name": "List C Linear",
                    "customday2_schedule_day_name": "Hourly A"
                }}
            }}
        }}"#
    );
    let raw_model = parse_epjson_str(&epjson)?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status("Schedule:Day:List"),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.defaults_applied.iter().any(|default| {
        default.object_type == "Schedule:Day:List"
            && default.object_name == "List A Default"
            && default.field == "interpolate_to_timestep"
            && default.value == "No"
    }));
    assert!(result.report.defaults_applied.iter().any(|default| {
        default.object_type == "Schedule:Day:List"
            && default.object_name == "List A Default[1]"
            && default.field == "value"
            && default.value == "0.0"
    }));
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.day_schedules.len(), 1);
    assert_eq!(model.day_interval_schedules.len(), 1);
    assert_eq!(model.day_list_schedules.len(), 3);
    assert_eq!(
        model
            .day_list_schedules
            .iter()
            .map(|schedule| schedule.id)
            .collect::<Vec<_>>(),
        vec![DayScheduleId(2), DayScheduleId(3), DayScheduleId(4)]
    );

    let default = &model.day_list_schedules[0];
    assert_eq!(default.schedule_type_limits, Some(ScheduleTypeLimitId(0)));
    assert_eq!(default.interpolation, ScheduleInterpolation::No);
    assert_eq!(default.minutes_per_item, 60);
    assert_eq!(default.values.len(), 24);
    assert_eq!(default.values[0..3], [1.0, 0.0, 3.0]);
    assert_eq!(default.values[23], 24.0);

    let average = &model.day_list_schedules[1];
    assert_eq!(average.interpolation, ScheduleInterpolation::Average);
    assert_eq!(average.minutes_per_item, 30);
    assert_eq!(average.values.len(), 48);
    assert_eq!(average.values[0..3], [1.0, 2.0, 3.0]);
    assert_eq!(average.values[47], 48.0);

    let linear = &model.day_list_schedules[2];
    assert_eq!(linear.interpolation, ScheduleInterpolation::Linear);
    assert_eq!(linear.minutes_per_item, 15);
    assert_eq!(linear.values.len(), 96);
    assert_eq!(linear.values[0..3], [1.0, 2.0, 3.0]);
    assert_eq!(linear.values[95], 96.0);

    assert_eq!(
        model.day_schedule_names.resolve("list a default"),
        Some(default.id)
    );
    assert_eq!(
        model.week_schedules[0].day_schedules,
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
fn rejects_duplicate_name_in_shared_day_schedule_namespace()
-> Result<(), Box<dyn std::error::Error>> {
    let values = extensions(24, None);
    let epjson = format!(
        r#"{{
            "Schedule:Day:Hourly": {{
                "Shared Day": {{}}
            }},
            "Schedule:Day:List": {{
                "Shared Day": {{
                    "minutes_per_item": 60,
                    "extensions": [{values}]
                }}
            }}
        }}"#
    );
    let raw_model = parse_epjson_str(&epjson)?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == "Schedule:Day:List"
            && diagnostic.object_name.as_deref() == Some("Shared Day")
    }));
    Ok(())
}

#[test]
fn rejects_invalid_minutes_counts_types_and_missing_inputs()
-> Result<(), Box<dyn std::error::Error>> {
    let valid = extensions(24, None);
    let short = extensions(23, None);
    let mut mistyped_entry_values = (0..24)
        .map(|index| format!(r#"{{"value": {}}}"#, index + 1))
        .collect::<Vec<_>>();
    mistyped_entry_values[0] = r#""not-an-object""#.to_string();
    let mistyped_entry = mistyped_entry_values.join(",");
    let mut mistyped_value_values = (0..24)
        .map(|index| format!(r#"{{"value": {}}}"#, index + 1))
        .collect::<Vec<_>>();
    mistyped_value_values[0] = r#"{"value": "one"}"#.to_string();
    let mistyped_value = mistyped_value_values.join(",");
    let epjson = format!(
        r#"{{
            "Schedule:Day:List": {{
                "Fractional Minutes": {{
                    "minutes_per_item": 7.5,
                    "extensions": [{valid}]
                }},
                "Out Of Range Minutes": {{
                    "minutes_per_item": 61,
                    "extensions": [{valid}]
                }},
                "Nondivisor Minutes": {{
                    "minutes_per_item": 7,
                    "extensions": [{valid}]
                }},
                "Wrong Value Count": {{
                    "minutes_per_item": 60,
                    "extensions": [{short}]
                }},
                "Mistyped Minutes": {{
                    "minutes_per_item": "sixty",
                    "extensions": [{valid}]
                }},
                "Missing Minutes": {{
                    "extensions": [{valid}]
                }},
                "Missing Extensions": {{
                    "minutes_per_item": 60
                }},
                "Mistyped Extensions": {{
                    "minutes_per_item": 60,
                    "extensions": "not-an-array"
                }},
                "Mistyped Entry": {{
                    "minutes_per_item": 60,
                    "extensions": [{mistyped_entry}]
                }},
                "Mistyped Value": {{
                    "minutes_per_item": 60,
                    "extensions": [{mistyped_value}]
                }},
                "Invalid Interpolation": {{
                    "interpolate_to_timestep": "Spline",
                    "minutes_per_item": 60,
                    "extensions": [{valid}]
                }},
                "Missing Type Limit": {{
                    "schedule_type_limits_name": "Does Not Exist",
                    "minutes_per_item": 60,
                    "extensions": [{valid}]
                }}
            }}
        }}"#
    );
    let raw_model = parse_epjson_str(&epjson)?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.model.is_none());
    for (code, object_name, field) in [
        ("InvalidInteger", "Fractional Minutes", "minutes_per_item"),
        (
            "InvalidNumericRange",
            "Out Of Range Minutes",
            "minutes_per_item",
        ),
        (
            "InvalidScheduleDayListMinutesPerItem",
            "Nondivisor Minutes",
            "minutes_per_item",
        ),
        (
            "InvalidScheduleDayListValueCount",
            "Wrong Value Count",
            "extensions",
        ),
        ("InvalidFieldType", "Mistyped Minutes", "minutes_per_item"),
        (
            "MissingRequiredField",
            "Missing Minutes",
            "minutes_per_item",
        ),
        ("MissingRequiredField", "Missing Extensions", "extensions"),
        ("InvalidFieldType", "Mistyped Extensions", "extensions"),
        ("InvalidFieldType", "Mistyped Entry", "extensions"),
        ("InvalidFieldType", "Mistyped Value[0]", "value"),
        (
            "InvalidEnumValue",
            "Invalid Interpolation",
            "interpolate_to_timestep",
        ),
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
