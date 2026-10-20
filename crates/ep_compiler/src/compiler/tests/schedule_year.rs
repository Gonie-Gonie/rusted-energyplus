use super::super::{
    ObjectCoverageStatus, compile_raw_model, compile_raw_model_with_auxiliary_root,
    typed_coverage_status,
};
use ep_model::ScheduleId;
use ep_raw_model::parse_epjson_str;
use std::path::PathBuf;

fn test_root(label: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root = std::env::temp_dir().join(format!(
        "rusted-energyplus-schedule-year-{}-{label}",
        std::process::id()
    ));
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }
    std::fs::create_dir_all(&root)?;
    Ok(root)
}

#[test]
fn resolves_day_week_year_values_wrap_and_february_29_fallback()
-> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("resolved-chain")?;
    std::fs::write(root.join("hourly.csv"), "3\n".repeat(8760))?;
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {"Any Number": {}},
            "Schedule:Constant": {
                "Constant First": {"hourly_value": 1}
            },
            "Schedule:Compact": {
                "Compact Second": {
                    "data": [
                        {"field": "Through: 12/31"},
                        {"field": "For: AllDays"},
                        {"field": "Until: 24:00"},
                        {"field": 2}
                    ]
                }
            },
            "Schedule:File": {
                "File Third": {
                    "file_name": "hourly.csv",
                    "column_number": 1,
                    "rows_to_skip_at_top": 0,
                    "number_of_hours_of_data": 8760,
                    "column_separator": "Comma",
                    "interpolate_to_timestep": "No",
                    "minutes_per_item": 60,
                    "adjust_schedule_for_daylight_savings": "No"
                }
            },
            "Schedule:Day:Hourly": {
                "Primary Day": {
                    "schedule_type_limits_name": "Any Number",
                    "hour_1": 1,
                    "hour_24": 24
                },
                "Wrap Day": {"hour_1": 10}
            },
            "Schedule:Week:Daily": {
                "Main Week": {
                    "sunday_schedule_day_name": "Primary Day",
                    "monday_schedule_day_name": "Primary Day",
                    "tuesday_schedule_day_name": "Primary Day",
                    "wednesday_schedule_day_name": "Primary Day",
                    "thursday_schedule_day_name": "Primary Day",
                    "friday_schedule_day_name": "Primary Day",
                    "saturday_schedule_day_name": "Primary Day",
                    "holiday_schedule_day_name": "Primary Day",
                    "summerdesignday_schedule_day_name": "Primary Day",
                    "winterdesignday_schedule_day_name": "Primary Day",
                    "customday1_schedule_day_name": "Primary Day",
                    "customday2_schedule_day_name": "Primary Day"
                },
                "Wrap Week": {
                    "sunday_schedule_day_name": "Wrap Day",
                    "monday_schedule_day_name": "Wrap Day",
                    "tuesday_schedule_day_name": "Wrap Day",
                    "wednesday_schedule_day_name": "Wrap Day",
                    "thursday_schedule_day_name": "Wrap Day",
                    "friday_schedule_day_name": "Wrap Day",
                    "saturday_schedule_day_name": "Wrap Day",
                    "holiday_schedule_day_name": "Wrap Day",
                    "summerdesignday_schedule_day_name": "Wrap Day",
                    "winterdesignday_schedule_day_name": "Wrap Day",
                    "customday1_schedule_day_name": "Wrap Day",
                    "customday2_schedule_day_name": "Wrap Day"
                }
            },
            "Schedule:Year": {
                "Year Fourth": {
                    "schedule_type_limits_name": "Any Number",
                    "schedule_weeks": [
                        {
                            "schedule_week_name": "Wrap Week",
                            "start_month": 12,
                            "start_day": 31,
                            "end_month": 1,
                            "end_day": 1
                        },
                        {
                            "schedule_week_name": "Main Week",
                            "start_month": 1,
                            "start_day": 2,
                            "end_month": 2,
                            "end_day": 28
                        },
                        {
                            "schedule_week_name": "Main Week",
                            "start_month": 3,
                            "start_day": 1,
                            "end_month": 12,
                            "end_day": 30
                        }
                    ]
                }
            }
        }"#,
    )?;

    let result = compile_raw_model_with_auxiliary_root(&raw_model, &root);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    for object_type in [
        "Schedule:Day:Hourly",
        "Schedule:Week:Daily",
        "Schedule:Year",
    ] {
        assert_eq!(
            typed_coverage_status(object_type),
            ObjectCoverageStatus::Typed
        );
    }
    assert!(result.defaults_applied_for("Schedule:Day:Hourly", "Primary Day", "hour_2"));
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    let primary_day_id = model
        .day_schedule_names
        .resolve("Primary Day")
        .ok_or_else(|| std::io::Error::other("missing primary day"))?;
    let primary_day = model
        .day_schedules
        .iter()
        .find(|schedule| schedule.id == primary_day_id)
        .ok_or_else(|| std::io::Error::other("missing primary day payload"))?;
    assert_eq!(primary_day.hourly_values[0], 1.0);
    assert_eq!(primary_day.hourly_values[1], 0.0);
    assert_eq!(primary_day.hourly_values[23], 24.0);

    let main_week_id = model
        .week_schedule_names
        .resolve("Main Week")
        .ok_or_else(|| std::io::Error::other("missing main week"))?;
    let wrap_week_id = model
        .week_schedule_names
        .resolve("Wrap Week")
        .ok_or_else(|| std::io::Error::other("missing wrap week"))?;
    let main_week = model
        .week_schedules
        .iter()
        .find(|schedule| schedule.id == main_week_id)
        .ok_or_else(|| std::io::Error::other("missing main week payload"))?;
    assert_eq!(main_week.day_schedules, [primary_day_id; 12]);

    assert_eq!(model.schedules[0].id, ScheduleId(1));
    assert_eq!(model.compact_schedules[0].id, ScheduleId(0));
    assert_eq!(model.file_schedules[0].id, ScheduleId(2));
    let year = &model.year_schedules[0];
    assert_eq!(year.id, ScheduleId(3));
    assert_eq!(model.schedule_names.resolve("Year Fourth"), Some(year.id));
    assert_eq!(year.week_schedules[0], wrap_week_id);
    assert_eq!(year.week_schedules[58], main_week_id);
    assert_eq!(year.week_schedules[59], main_week_id);
    assert_eq!(year.week_schedules[60], main_week_id);
    assert_eq!(year.week_schedules[365], wrap_week_id);

    // Schema-bounded Year month/day fields reach the actual leap OrdinalDay
    // projection, not the canonical shared calendar-date validator. Reuse the
    // existing complete Day/Week/File graph and its same staged auxiliary root.
    let with_ranges = |ranges: &str| -> Result<ep_raw_model::RawModel, Box<dyn std::error::Error>> {
        use ep_raw_model::{FieldName, ObjectName, ObjectType};
        let json =
            format!(r#"{{"Schedule:Year":{{"Year Fourth":{{"schedule_weeks":{ranges}}}}}}}"#);
        let supplied = parse_epjson_str(&json)?;
        let kind = ObjectType("Schedule:Year".into());
        let name = ObjectName("Year Fourth".into());
        let field = FieldName("schedule_weeks".into());
        let ranges = supplied
            .objects
            .get(&kind)
            .and_then(|objects| objects.get(&name))
            .and_then(|object| object.fields.get(&field))
            .ok_or("actual authored Year ranges")?
            .clone();
        let mut raw = raw_model.clone();
        raw.objects
            .get_mut(&kind)
            .and_then(|objects| objects.get_mut(&name))
            .ok_or("actual retained Year input")?
            .fields
            .insert(field, ranges);
        Ok(raw)
    };
    for (month, day, ordinal, next_month, next_day) in [
        (2, 30, 61_usize, 3, 2),
        (2, 31, 62, 3, 3),
        (4, 31, 122, 5, 2),
        (6, 31, 183, 7, 2),
        (9, 31, 275, 10, 2),
        (11, 31, 336, 12, 2),
    ] {
        // Integral JSON decimal/exponent forms are already valid Number
        // operands; don't add the Compact alpha ProcessNumber grammar here.
        let ranges = format!(
            r#"[
            {{"schedule_week_name":"Main Week","start_month":1.0,"start_day":1e0,"end_month":{month},"end_day":{day}}},
            {{"schedule_week_name":"Wrap Week","start_month":{next_month},"start_day":{next_day},"end_month":1.2e1,"end_day":3.1e1}}
        ]"#
        );
        let result = compile_raw_model_with_auxiliary_root(&with_ranges(&ranges)?, &root);
        assert!(
            !result.has_errors(),
            "{month}/{day}: {:?}",
            result.report.diagnostics
        );
        let model = result.model.ok_or("folded Year typed graph")?;
        let pointers = &model.year_schedules[0].week_schedules;
        assert!(pointers[..ordinal].iter().all(|week| *week == main_week_id));
        assert!(pointers[ordinal..].iter().all(|week| *week == wrap_week_id));
        assert_eq!(
            (pointers[ordinal - 1], pointers[ordinal]),
            (main_week_id, wrap_week_id)
        );
    }
    // A one-range wrap from ordinal62 through folded61 covers every day once.
    let ranges = r#"[{"schedule_week_name":"Main Week","start_month":3,"start_day":2,"end_month":2,"end_day":30}]"#;
    let result = compile_raw_model_with_auxiliary_root(&with_ranges(ranges)?, &root);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(
        result.model.ok_or("folded wrapped Year")?.year_schedules[0]
            .week_schedules
            .iter()
            .all(|week| *week == main_week_id)
    );
    // Native's missingday60 copies59 even beside a later folded range boundary.
    let ranges = r#"[
        {"schedule_week_name":"Main Week","start_month":1,"start_day":1,"end_month":2,"end_day":28},
        {"schedule_week_name":"Wrap Week","start_month":3,"start_day":1,"end_month":4,"end_day":31},
        {"schedule_week_name":"Main Week","start_month":5,"start_day":2,"end_month":12,"end_day":31}
    ]"#;
    let result = compile_raw_model_with_auxiliary_root(&with_ranges(ranges)?, &root);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result.model.ok_or("folded Year leap fallback")?;
    let pointers = &model.year_schedules[0].week_schedules;
    assert_eq!(
        (
            pointers[58],
            pointers[59],
            pointers[60],
            pointers[121],
            pointers[122]
        ),
        (
            main_week_id,
            main_week_id,
            wrap_week_id,
            wrap_week_id,
            main_week_id
        )
    );
    // The Year-only relaxation does not admit schema-invalid fields or alter
    // Compact's strict two-field civil-date validation.
    for (month, day) in [(0, 1), (13, 1), (2, 0), (2, 32)] {
        let ranges = format!(
            r#"[{{"schedule_week_name":"Main Week","start_month":{month},"start_day":{day},"end_month":12,"end_day":31}}]"#
        );
        let result = compile_raw_model_with_auxiliary_root(&with_ranges(&ranges)?, &root);
        assert!(result.has_errors(), "{month}/{day}");
        assert!(
            result
                .report
                .diagnostics
                .iter()
                .any(|row| row.code == "InvalidScheduleYearDate")
        );
    }
    let invalid_compact = parse_epjson_str(
        r#"{"Schedule:Compact":{"S":{"data":[{"field":"Through:2/30"},{"field":"For:AllDays"},{"field":"Until:24:00"},{"field":0}]}}}"#,
    )?;
    assert!(compile_raw_model(&invalid_compact).has_errors());

    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn rejects_missing_and_invalid_week_day_references() -> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Schedule:Day:Hourly": {"Only Day": {}},
            "Schedule:Week:Daily": {
                "Incomplete Week": {
                    "sunday_schedule_day_name": "Only Day",
                    "monday_schedule_day_name": "Only Day",
                    "tuesday_schedule_day_name": "Only Day",
                    "wednesday_schedule_day_name": "Only Day",
                    "thursday_schedule_day_name": "Only Day",
                    "friday_schedule_day_name": "Only Day",
                    "saturday_schedule_day_name": "Only Day",
                    "holiday_schedule_day_name": "Only Day",
                    "summerdesignday_schedule_day_name": "Only Day",
                    "winterdesignday_schedule_day_name": "Only Day",
                    "customday1_schedule_day_name": "Only Day"
                },
                "Broken Week": {
                    "sunday_schedule_day_name": "Only Day",
                    "monday_schedule_day_name": "Only Day",
                    "tuesday_schedule_day_name": "Only Day",
                    "wednesday_schedule_day_name": "Only Day",
                    "thursday_schedule_day_name": "Only Day",
                    "friday_schedule_day_name": "Only Day",
                    "saturday_schedule_day_name": "Only Day",
                    "holiday_schedule_day_name": "Only Day",
                    "summerdesignday_schedule_day_name": "Only Day",
                    "winterdesignday_schedule_day_name": "Only Day",
                    "customday1_schedule_day_name": "Only Day",
                    "customday2_schedule_day_name": "Missing Day"
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_name.as_deref() == Some("Incomplete Week")
            && diagnostic.field.as_deref() == Some("customday2_schedule_day_name")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_name.as_deref() == Some("Broken Week")
            && diagnostic.field.as_deref() == Some("customday2_schedule_day_name")
    }));
    Ok(())
}

#[test]
fn rejects_invalid_missing_and_overlapping_year_ranges() -> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Schedule:Day:Hourly": {"Day": {}},
            "Schedule:Week:Daily": {
                "Week": {
                    "sunday_schedule_day_name": "Day",
                    "monday_schedule_day_name": "Day",
                    "tuesday_schedule_day_name": "Day",
                    "wednesday_schedule_day_name": "Day",
                    "thursday_schedule_day_name": "Day",
                    "friday_schedule_day_name": "Day",
                    "saturday_schedule_day_name": "Day",
                    "holiday_schedule_day_name": "Day",
                    "summerdesignday_schedule_day_name": "Day",
                    "winterdesignday_schedule_day_name": "Day",
                    "customday1_schedule_day_name": "Day",
                    "customday2_schedule_day_name": "Day"
                }
            },
            "Schedule:Year": {
                "Missing Days": {
                    "schedule_weeks": [
                        {"schedule_week_name": "Week", "start_month": 1, "start_day": 1, "end_month": 2, "end_day": 27},
                        {"schedule_week_name": "Week", "start_month": 3, "start_day": 1, "end_month": 12, "end_day": 31}
                    ]
                },
                "Overlap Days": {
                    "schedule_weeks": [
                        {"schedule_week_name": "Week", "start_month": 1, "start_day": 1, "end_month": 12, "end_day": 31},
                        {"schedule_week_name": "Week", "start_month": 12, "start_day": 31, "end_month": 1, "end_day": 1}
                    ]
                },
                "Invalid Date": {
                    "schedule_weeks": [
                        {"schedule_week_name": "Week", "start_month": 1, "start_day": 1, "end_month": 13, "end_day": 30}
                    ]
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for (code, object_name) in [
        ("MissingScheduleYearDays", "Missing Days"),
        ("OverlappingScheduleYearDays", "Overlap Days"),
        ("InvalidScheduleYearDate", "Invalid Date[0]"),
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == code && diagnostic.object_name.as_deref() == Some(object_name)
        }));
    }
    Ok(())
}

trait CompileResultAssertions {
    fn defaults_applied_for(&self, object_type: &str, object_name: &str, field: &str) -> bool;
}

impl CompileResultAssertions for super::super::CompileResult {
    fn defaults_applied_for(&self, object_type: &str, object_name: &str, field: &str) -> bool {
        self.report.defaults_applied.iter().any(|default| {
            default.object_type == object_type
                && default.object_name == object_name
                && default.field == field
        })
    }
}
