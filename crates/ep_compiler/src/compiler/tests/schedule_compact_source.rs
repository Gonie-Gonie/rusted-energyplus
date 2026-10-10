use super::super::{
    ALL_SCHEDULE_DAY_TYPES, Compiler, DiagnosticSeverity, compile_raw_model,
    parse_schedule_time_minute,
};
use ep_model::{ScheduleDayType, ScheduleId, ScheduleInterpolation, ScheduleTypeLimitId};
use ep_raw_model::parse_epjson_str;

#[test]
fn compact_registration_precedes_constants_with_distinct_actual_ids()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Schedule:Constant":{"Constant":{"hourly_value":0.25}},
            "Schedule:Compact":{"Compact":{"data":[
                {"field":"Through:12/31"},{"field":"For:AllDays"},
                {"field":"Until:24:00"},{"field":0.5}
            ]}}
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors());
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("valid selected schedule model"))?;
    assert_eq!(model.compact_schedules[0].id, ScheduleId(0));
    assert_eq!(model.schedules[0].id, ScheduleId(1));
    assert_eq!(model.schedule_names.resolve("Compact"), Some(ScheduleId(0)));
    assert_eq!(
        model.schedule_names.resolve("Constant"),
        Some(ScheduleId(1))
    );
    Ok(())
}

#[test]
fn cross_family_duplicate_is_reported_by_constant_registration()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Schedule:Constant":{"S":{"hourly_value":0.25}},
            "Schedule:Compact":{"S":{"data":[
                {"field":"Through:12/31"},{"field":"For:AllDays"},
                {"field":"Until:24:00"},{"field":0.5}
            ]}}
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(result.has_errors());
    assert!(
        result.report.diagnostics.iter().any(|item| {
            item.code == "DuplicateName" && item.object_type == "Schedule:Constant"
        })
    );
    Ok(())
}

#[test]
fn selected_type_references_warn_without_inventing_owners() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits":{"Declared":{}},
            "Schedule:Constant":{
                "Blank":{"schedule_type_limits_name":"","hourly_value":0.25},
                "Missing":{"schedule_type_limits_name":"Unknown","hourly_value":0.25},
                "Resolved":{"schedule_type_limits_name":"Declared","hourly_value":0.25}
            },
            "Schedule:Compact":{
                "Compact Blank":{"data":[
                    {"field":"Through:12/31"},{"field":"For:AllDays"},
                    {"field":"Until:24:00"},{"field":0.25}
                ]},
                "Compact Missing":{"schedule_type_limits_name":"Unknown","data":[
                    {"field":"Through:12/31"},{"field":"For:AllDays"},
                    {"field":"Until:24:00"},{"field":0.25}
                ]},
                "Compact Resolved":{"schedule_type_limits_name":"Declared","data":[
                    {"field":"Through:12/31"},{"field":"For:AllDays"},
                    {"field":"Until:24:00"},{"field":0.25}
                ]}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors());
    let warnings = result
        .report
        .diagnostics
        .iter()
        .filter(|item| {
            item.severity == DiagnosticSeverity::Warning
                && matches!(
                    item.code.as_str(),
                    "MissingScheduleTypeLimits" | "MissingReference"
                )
        })
        .count();
    assert_eq!(warnings, 4);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("warning-only selected schedules remain owned"))?;
    assert_eq!(model.schedule_type_limits.len(), 1);
    for item in &model.schedules {
        assert_eq!(
            item.schedule_type_limits,
            (item.name.0 == "RESOLVED").then_some(ScheduleTypeLimitId(0))
        );
    }
    for item in &model.compact_schedules {
        assert_eq!(
            item.schedule_type_limits,
            (item.name.0 == "COMPACT RESOLVED").then_some(ScheduleTypeLimitId(0))
        );
    }
    Ok(())
}

#[test]
fn malformed_type_reference_keeps_field_type_errors() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Schedule:Constant":{"C":{"schedule_type_limits_name":42,"hourly_value":0.25}},
            "Schedule:Compact":{"S":{"schedule_type_limits_name":42,"data":[
                {"field":"Through:12/31"},{"field":"For:AllDays"},
                {"field":"Until:24:00"},{"field":0.25}
            ]}}
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(result.has_errors());
    assert_eq!(
        result
            .report
            .diagnostics
            .iter()
            .filter(|item| {
                item.code == "InvalidFieldType"
                    && item.field.as_deref() == Some("schedule_type_limits_name")
            })
            .count(),
        2
    );
    assert!(
        !result
            .report
            .diagnostics
            .iter()
            .any(|item| item.code == "MissingScheduleTypeLimits")
    );
    Ok(())
}

#[test]
fn selector_preserves_substrings_duplicate_mutations_and_real_day_order()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str("{}")?;
    let mut compiler = Compiler::new(&raw, None);
    let mut assigned = [false; 12];
    assert!(
        compiler
            .compact_schedule_day_types("S", "For:Weekdays Monday", &mut assigned)
            .is_empty()
    );
    assert!(assigned[1..=5].iter().all(|value| *value));
    assert_eq!(
        compiler.compact_schedule_day_types("S", "For:AllOtherDays", &mut assigned),
        vec![
            ScheduleDayType::Sunday,
            ScheduleDayType::Saturday,
            ScheduleDayType::Holiday,
            ScheduleDayType::SummerDesignDay,
            ScheduleDayType::WinterDesignDay,
            ScheduleDayType::CustomDay1,
            ScheduleDayType::CustomDay2,
        ]
    );
    let mut fresh = [false; 12];
    assert_eq!(
        compiler.compact_schedule_day_types("S", "For:Monday Monday", &mut fresh),
        vec![ScheduleDayType::Monday]
    );
    let mut fresh = [false; 12];
    assert_eq!(
        compiler.compact_schedule_day_types("S", "For:Funday AllOtherDays", &mut fresh),
        ALL_SCHEDULE_DAY_TYPES
    );
    let mut fresh = [false; 12];
    assert!(
        compiler
            .compact_schedule_day_types("S", "For:Funday", &mut fresh)
            .is_empty()
    );
    assert!(fresh.iter().all(|value| !*value));
    assert_eq!(
        compiler
            .diagnostics
            .iter()
            .filter(|item| item.severity == DiagnosticSeverity::Error)
            .count(),
        2
    );
    Ok(())
}

#[test]
fn omitted_real_day_types_receive_an_owned_zero_profile() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(
        r#"{"Schedule:Compact":{"S":{"data":[
            {"field":"Through:12/31"},{"field":"For:Monday"},
            {"field":"Until:24:00"},{"field":0.25}
        ]}}}"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors());
    assert!(result.report.diagnostics.iter().any(|item| {
        item.code == "IncompleteScheduleCompactDayTypes"
            && item.severity == DiagnosticSeverity::Warning
    }));
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("missing types use the real MissingDay fallback"))?;
    let profiles = &model.compact_schedules[0].periods[0].day_profiles;
    assert_eq!(profiles.len(), 2);
    assert_eq!(profiles[0].day_types, vec![ScheduleDayType::Monday]);
    assert_eq!(profiles[1].day_types.len(), 11);
    assert_eq!(profiles[1].interpolation, ScheduleInterpolation::No);
    assert_eq!(profiles[1].segments.len(), 1);
    assert_eq!(profiles[1].segments[0].until_minute_of_day, 1440);
    assert_eq!(profiles[1].segments[0].value.to_bits(), 0.0_f64.to_bits());
    for day in ALL_SCHEDULE_DAY_TYPES {
        assert_eq!(
            profiles
                .iter()
                .filter(|profile| profile.day_types.contains(&day))
                .count(),
            1
        );
    }
    Ok(())
}

#[test]
fn equal_until_retains_the_zero_duration_segment_and_warning()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{"Schedule:Compact":{"S":{"data":[
            {"field":"Through:12/31"},{"field":"For:AllDays"},
            {"field":"Until:06:00"},{"field":0.25},
            {"field":"Until:06:00"},{"field":0.5},
            {"field":"Until:24:00"},{"field":0.75}
        ]}}}"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors());
    assert_eq!(
        result
            .report
            .diagnostics
            .iter()
            .filter(|item| item.code == "ScheduleCompactZeroTimeInterval")
            .count(),
        1
    );
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("equal Until boundaries are not descending"))?;
    let segments = &model.compact_schedules[0].periods[0].day_profiles[0].segments;
    assert_eq!(
        segments
            .iter()
            .map(|item| item.until_minute_of_day)
            .collect::<Vec<_>>(),
        vec![360, 360, 1440]
    );
    assert_eq!(segments[1].value.to_bits(), 0.5_f64.to_bits());
    Ok(())
}

#[test]
fn compact_24mm_clamps_after_original_minute_alignment_warning()
-> Result<(), Box<dyn std::error::Error>> {
    for time in ["24:01", "24:59"] {
        let source = format!(
            r#"{{
            "Timestep":{{"T":{{"number_of_timesteps_per_hour":4}}}},
            "Schedule:Compact":{{"S":{{"data":[
                {{"field":"Through:12/31"}},{{"field":"For:AllDays"}},
                {{"field":"Until:{time}"}},{{"field":0.25}}
            ]}}}}
        }}"#
        );
        let raw = parse_epjson_str(&source)?;
        let result = compile_raw_model(&raw);
        assert!(!result.has_errors());
        let warnings = result
            .report
            .diagnostics
            .iter()
            .filter(|item| {
                matches!(
                    item.code.as_str(),
                    "ScheduleCompactUntilNotAlignedToTimestep"
                        | "ScheduleCompactUntilClampedToEndOfDay"
                )
            })
            .map(|item| item.code.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            warnings,
            vec![
                "ScheduleCompactUntilNotAlignedToTimestep",
                "ScheduleCompactUntilClampedToEndOfDay"
            ]
        );
        let model = result
            .model
            .ok_or_else(|| std::io::Error::other("24:mm terminates at 24:00"))?;
        assert_eq!(
            model.compact_schedules[0].periods[0].day_profiles[0].segments[0].until_minute_of_day,
            1440
        );
        assert_eq!(
            parse_schedule_time_minute(time),
            None,
            "other day-family parser stays strict"
        );
    }
    Ok(())
}
