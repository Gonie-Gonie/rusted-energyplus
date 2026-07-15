use super::super::{ObjectCoverageStatus, compile_raw_model, typed_coverage_status};
use ep_model::{DayScheduleId, WeekScheduleId};
use ep_raw_model::parse_epjson_str;

#[test]
fn materializes_source_ordered_selectors_and_shared_week_ids()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Schedule:Day:Hourly": {
                "Day A": {"hour_1": 1},
                "Day B": {"hour_1": 2},
                "Day C": {"hour_1": 3}
            },
            "Schedule:Week:Daily": {
                "Daily First": {
                    "sunday_schedule_day_name": "Day A",
                    "monday_schedule_day_name": "Day A",
                    "tuesday_schedule_day_name": "Day A",
                    "wednesday_schedule_day_name": "Day A",
                    "thursday_schedule_day_name": "Day A",
                    "friday_schedule_day_name": "Day A",
                    "saturday_schedule_day_name": "Day A",
                    "holiday_schedule_day_name": "Day A",
                    "summerdesignday_schedule_day_name": "Day A",
                    "winterdesignday_schedule_day_name": "Day A",
                    "customday1_schedule_day_name": "Day A",
                    "customday2_schedule_day_name": "Day A"
                }
            },
            "Schedule:Week:Compact": {
                "Noisy Week": {
                    "data": [
                        {
                            "daytype_list": "For: Monday Monday Funday AllOtherDays",
                            "schedule_day_name": "Day C"
                        }
                    ]
                },
                "Residual Week": {
                    "data": [
                        {"daytype_list": "For: Weekdays", "schedule_day_name": "Day A"},
                        {
                            "daytype_list": "For Weekends WinterDesignDay",
                            "schedule_day_name": "Day B"
                        },
                        {"daytype_list": "AllOtherDays", "schedule_day_name": "Day C"}
                    ]
                }
            },
            "Schedule:Year": {
                "Compact Year": {
                    "schedule_weeks": [
                        {
                            "schedule_week_name": "Residual Week",
                            "start_month": 1,
                            "start_day": 1,
                            "end_month": 12,
                            "end_day": 31
                        }
                    ]
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status("Schedule:Week:Compact"),
        ObjectCoverageStatus::Typed
    );
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.week_schedules[0].id, WeekScheduleId(0));
    assert_eq!(model.week_compact_schedules.len(), 2);

    let noisy_id = model
        .week_schedule_names
        .resolve("Noisy Week")
        .ok_or_else(|| std::io::Error::other("missing noisy week"))?;
    let residual_id = model
        .week_schedule_names
        .resolve("Residual Week")
        .ok_or_else(|| std::io::Error::other("missing residual week"))?;
    assert_eq!(noisy_id, WeekScheduleId(1));
    assert_eq!(residual_id, WeekScheduleId(2));
    assert_eq!(
        model.week_compact_schedules[0].day_schedules,
        [DayScheduleId(2); 12]
    );
    assert_eq!(
        model.week_compact_schedules[1].day_schedules,
        [
            DayScheduleId(1),
            DayScheduleId(0),
            DayScheduleId(0),
            DayScheduleId(0),
            DayScheduleId(0),
            DayScheduleId(0),
            DayScheduleId(1),
            DayScheduleId(2),
            DayScheduleId(2),
            DayScheduleId(1),
            DayScheduleId(2),
            DayScheduleId(2),
        ]
    );
    assert_eq!(model.year_schedules[0].week_schedules, [residual_id; 366]);
    Ok(())
}

#[test]
fn rejects_overlaps_missing_days_and_unrecognized_selectors()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Schedule:Day:Hourly": {"Day": {}},
            "Schedule:Week:Compact": {
                "Duplicate Days": {
                    "data": [
                        {"daytype_list": "Weekdays", "schedule_day_name": "Day"},
                        {"daytype_list": "Monday", "schedule_day_name": "Day"},
                        {"daytype_list": "AllOtherDays", "schedule_day_name": "Day"}
                    ]
                },
                "Missing Days": {
                    "data": [
                        {"daytype_list": "Monday", "schedule_day_name": "Day"}
                    ]
                },
                "Unknown Days": {
                    "data": [
                        {"daytype_list": "Funday", "schedule_day_name": "Day"},
                        {"daytype_list": "AllOtherDays", "schedule_day_name": "Day"}
                    ]
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for (code, object_name) in [
        ("DuplicateScheduleWeekCompactDayType", "Duplicate Days[1]"),
        ("MissingScheduleWeekCompactDayAssignments", "Missing Days"),
        ("InvalidScheduleWeekCompactDayTypeList", "Unknown Days[0]"),
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == code && diagnostic.object_name.as_deref() == Some(object_name)
        }));
    }
    Ok(())
}

#[test]
fn missing_reference_does_not_consume_day_and_week_names_are_shared()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Schedule:Day:Hourly": {"Day": {}},
            "Schedule:Week:Daily": {
                "Shared Week": {
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
            "Schedule:Week:Compact": {
                "Reference Recovery": {
                    "data": [
                        {"daytype_list": "Monday", "schedule_day_name": "Missing Day"},
                        {"daytype_list": "Monday", "schedule_day_name": "Day"},
                        {"daytype_list": "AllOtherDays", "schedule_day_name": "Day"}
                    ]
                },
                "Shared Week": {
                    "data": [
                        {"daytype_list": "AllDays", "schedule_day_name": "Day"}
                    ]
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_name.as_deref() == Some("Reference Recovery[0]")
    }));
    assert!(!result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateScheduleWeekCompactDayType"
            && diagnostic.object_name.as_deref() == Some("Reference Recovery[1]")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_name.as_deref() == Some("Shared Week")
            && diagnostic.object_type == "Schedule:Week:Compact"
    }));
    Ok(())
}
