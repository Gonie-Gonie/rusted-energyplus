use super::super::schedule_compact_source::decode_compact_until;
use super::super::{ALL_SCHEDULE_DAY_TYPES, Compiler, DiagnosticSeverity, compile_raw_model};
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

    // Raw Number uses the genuine JSON-kind getter projection. Strings and real existing blank
    // extensible objects use source ProcessNumber only while Until is pending.
    let scalar_result = |entry: &str,
                         with_until: bool|
     -> Result<crate::CompileResult, Box<dyn std::error::Error>> {
        let until = if with_until {
            r#",{"field":"Until:24:00"}"#
        } else {
            ""
        };
        let entry = if entry.is_empty() {
            String::new()
        } else {
            format!(",{entry}")
        };
        let source = format!(
            r#"{{
            "ScheduleTypeLimits":{{"T":{{"lower_limit_value":-1,"upper_limit_value":1,"numeric_type":"Continuous"}}}},
            "Schedule:Compact":{{"S":{{"schedule_type_limits_name":"T","data":[
                {{"field":"Through:12/31"}},{{"field":"For:AllDays"}}{until}{entry}
            ]}}}}
        }}"#
        );
        Ok(compile_raw_model(&parse_epjson_str(&source)?))
    };
    for (entry, expected) in [
        (r#"{"field":"1D0"}"#, 1.0_f64),
        (r#"{"field":"-5D-1"}"#, -0.5),
        (r#"{"field":"0E"}"#, 0.0),
        (r#"{"field":"-0"}"#, -0.0), // String alpha ProcessNumber keeps its sign.
        (r#"{"field":"1e-9999"}"#, 0.0),
        (r#"{"field":"-1e-9999"}"#, -0.0),
        (r#"{"field":"1D-9999"}"#, 0.0),
        (r#"{"field":"-1D-9999"}"#, -0.0),
        (r#"{"field":""}"#, 0.0),
        (r#"{"field":"   "}"#, 0.0),
        (r#"{}"#, 0.0),
        (r#"{"ignored":{"field":null}}"#, 0.0),
    ] {
        let result = scalar_result(entry, true)?;
        assert!(
            !result.has_errors(),
            "{entry}: {:?}",
            result.report.diagnostics
        );
        let model = result
            .model
            .ok_or_else(|| std::io::Error::other("actual Compact scalar model"))?;
        let profile = &model.compact_schedules[0].periods[0].day_profiles[0];
        assert_eq!(profile.segments.len(), 1);
        assert_eq!(profile.segments[0].value.to_bits(), expected.to_bits());
    }
    for entry in [
        r#"{"field":null}"#,
        r#"null"#,
        r#"{"field":true}"#,
        r#"{"field":"junk"}"#,
        r#"{"field":"+1"}"#,
        r#"{"field":"nan"}"#, // ErrorZero is not a valid scalar value.
        r#"{"field":"nanE"}"#,
        r#"{"field":"1e9999E"}"#,
        r#"{"field":"\u00a01"}"#,
        r#"{"field":"1\u00a0"}"#,
        r#"{"field":"Through:12/31"}"#,
        "", // No entry exists after Until; do not manufacture a blank item.
    ] {
        let result = scalar_result(entry, true)?;
        assert!(result.has_errors(), "{entry}");
        assert!(result.model.is_none());
    }
    for entry in [r#"{"field":"1D0"}"#, r#"{}"#] {
        assert!(scalar_result(entry, false)?.has_errors());
    }

    // Exercise the shared JSON-kind classifier in this existing connected body.
    // Integer overflow fallback is a stored float, never arbitrary-width wrap.
    use ep_model::{SourceJsonNumber, classify_source_json_number};
    for (text, expected) in [
        ("0", 0_i64),
        ("-0", 0),
        ("9223372036854775807", i64::MAX),
        ("-9223372036854775808", i64::MIN),
        ("9223372036854775808", i64::MIN),
        ("18446744073709551615", -1),
    ] {
        assert_eq!(
            classify_source_json_number(text),
            Ok(SourceJsonNumber::Integer(expected))
        );
    }
    for (text, expected) in [
        ("18446744073709551615.0", 18446744073709551616.0_f64),
        ("18446744073709551615e0", 18446744073709551616.0),
        ("18446744073709551616", 18446744073709551616.0),
        ("-9223372036854775809", -9223372036854775808.0),
        ("-0.0", -0.0),
        ("1e-9999", 0.0),
        ("-1e-9999", -0.0),
    ] {
        let Ok(SourceJsonNumber::Float(actual)) = classify_source_json_number(text) else {
            return Err(std::io::Error::other(format!("actual JSON float kind: {text}")).into());
        };
        assert_eq!(actual.to_bits(), expected.to_bits(), "{text}");
    }
    for text in [
        "", "+1", "01", "-01", ".5", "1.", "1e", "1D0", " 1", "1 ", "nan", "1e9999",
    ] {
        assert!(classify_source_json_number(text).is_err(), "{text}");
    }
    // Parse the original literal bytes, never a serde Value/to_string roundtrip:
    // lexical -0 is a JSON Integer, whereas the decimal/exponent forms are Float.
    for (operand, expected, integer) in [
        ("-0", 0.0_f64, true),
        ("-0.0", -0.0, false),
        ("-0e0", -0.0, false),
        ("1e-9999", 0.0, false),
        ("-1e-9999", -0.0, false),
        ("1E+00", 1.0, false),
        ("18446744073709551615", -1.0, true),
    ] {
        let hours = (1..=24)
            .map(|hour| format!("\"hour_{hour}\":{operand}"))
            .collect::<Vec<_>>()
            .join(",");
        let list = vec![format!("{{\"value\":{operand}}}"); 24].join(",");
        let raw = parse_epjson_str(&format!(
            r#"{{
            "ScheduleTypeLimits":{{"Wrapped":{{"lower_limit_value":{operand},"upper_limit_value":200,"numeric_type":"Continuous"}}}},
            "Schedule:Constant":{{"C":{{"schedule_type_limits_name":"Wrapped","hourly_value":{operand}}}}},
            "Schedule:Compact":{{"S":{{"schedule_type_limits_name":"Wrapped","data":[
                {{"field":"Through:12/31"}},{{"field":"For:AllDays"}},{{"field":"Until:24:00"}},{{"field":{operand}}}]}}}},
            "Schedule:Day:Hourly":{{"H":{{"schedule_type_limits_name":"Wrapped",{hours}}}}},
            "Schedule:Day:Interval":{{"I":{{"schedule_type_limits_name":"Wrapped","data":[{{"time":"24:00","value_until_time":{operand}}}]}}}},
            "Schedule:Day:List":{{"L":{{"schedule_type_limits_name":"Wrapped","minutes_per_item":60,"extensions":[{list}]}}}}
        }}"#
        ))?;
        let constants = raw.ordered_instances("Schedule:Constant")?;
        let field = constants
            .first()
            .and_then(|(_, object)| {
                object
                    .fields
                    .get(&ep_raw_model::FieldName("hourly_value".into()))
            })
            .ok_or_else(|| std::io::Error::other("actual literal numeric field"))?;
        let ep_raw_model::RawValue::Number(text) = field else {
            return Err(std::io::Error::other("actual numeric-kind custody").into());
        };
        assert_eq!(text, operand);
        let kind = classify_source_json_number(text)
            .map_err(|_| std::io::Error::other("actual retained JSON kind"))?;
        assert_eq!(matches!(kind, SourceJsonNumber::Integer(_)), integer);
        assert_eq!(kind.as_f64().to_bits(), expected.to_bits());
        let projected = compile_raw_model(&raw);
        assert!(
            !projected.has_errors(),
            "{operand}: {:?}",
            projected.report.diagnostics
        );
        let model = projected
            .model
            .ok_or_else(|| std::io::Error::other("actual literal schedule projection"))?;
        assert_eq!(
            model.schedule_type_limits[0]
                .lower_limit
                .ok_or_else(|| std::io::Error::other("actual lower bound"))?
                .to_bits(),
            expected.to_bits()
        );
        assert_eq!(model.schedule_type_limits[0].upper_limit, Some(200.0));
        assert_eq!(
            model.schedules[0].hourly_value.to_bits(),
            expected.to_bits()
        );
        assert_eq!(
            model.compact_schedules[0].periods[0].day_profiles[0].segments[0]
                .value
                .to_bits(),
            expected.to_bits()
        );
        assert!(
            model.day_schedules[0]
                .hourly_values
                .iter()
                .all(|value| value.to_bits() == expected.to_bits())
        );
        assert_eq!(
            model.day_interval_schedules[0].segments[0].value.to_bits(),
            expected.to_bits()
        );
        assert_eq!(model.day_list_schedules[0].values.len(), 24);
        assert!(
            model.day_list_schedules[0]
                .values
                .iter()
                .all(|value| value.to_bits() == expected.to_bits())
        );
    }
    for value in [
        "18446744073709551615.0",
        "18446744073709551615e0",
        "18446744073709551616",
    ] {
        let result = scalar_result(&format!("{{\"field\":{value}}}"), true)?;
        assert!(
            result
                .report
                .diagnostics
                .iter()
                .any(|row| row.code == "ScheduleValueOutsideTypeLimits")
        );
        assert!(result.model.is_none());
    }

    let excessive = scalar_result(r#"{"field":"2D0"}"#, true)?;
    assert!(
        excessive
            .report
            .diagnostics
            .iter()
            .any(|row| row.code == "ScheduleValueOutsideTypeLimits")
    );
    assert!(excessive.model.is_none());
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

    for (fields, endpoints, warnings) in [
        (
            r#"{"field":"Until:0:0"},{"field":2},
            {"field":"Until:3:00"},{"field":0},
            {"field":"Until:12:60"},{"field":2},
            {"field":"Until:24:00"},{"field":1}"#,
            vec![0, 180, 180, 1440],
            2,
        ),
        (
            r#"{"field":"Until:0:0"},{"field":2},
            {"field":"Until:24:60"},{"field":2},
            {"field":"Until:24:00"},{"field":1}"#,
            vec![0, 0, 1440],
            2,
        ),
        (
            r#"{"field":"Until:00:20"},{"field":0},
            {"field":"Until:12:60"},{"field":2},
            {"field":"Until:24:00"},{"field":1}"#,
            vec![20, 20, 1440],
            0,
        ),
    ] {
        let source = format!(
            r#"{{"Schedule:Compact":{{"S":{{"data":[
            {{"field":"Through:12/31"}},{{"field":"For:AllDays"}},{{"field":"Interpolate:Average"}},{fields}
        ]}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&source)?);
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert_eq!(
            result
                .report
                .diagnostics
                .iter()
                .filter(|row| row.code == "ScheduleCompactZeroTimeInterval")
                .count(),
            warnings
        );
        let model = result
            .model
            .ok_or_else(|| std::io::Error::other("real empty Until owners"))?;
        let segments = &model.compact_schedules[0].periods[0].day_profiles[0].segments;
        assert_eq!(
            segments
                .iter()
                .map(|row| row.until_minute_of_day)
                .collect::<Vec<_>>(),
            endpoints
        );
        assert!(
            segments
                .iter()
                .any(|row| row.value.to_bits() == 2.0_f64.to_bits())
        );
    }
    for fields in [
        r#"{"field":"Until:12:60"},{"field":2},{"field":"Until:24:00"},{"field":1}"#,
        r#"{"field":"Until:6:00"},{"field":0},{"field":"Until:12:60"},{"field":2},
            {"field":"Until:5:00"},{"field":1},{"field":"Until:24:00"},{"field":1}"#,
    ] {
        let source = format!(
            r#"{{"Schedule:Compact":{{"S":{{"data":[
            {{"field":"Through:12/31"}},{{"field":"For:AllDays"}},{fields}
        ]}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&source)?);
        assert!(result.has_errors());
        assert!(result.model.is_none());
    }
    Ok(())
}

#[test]
fn compact_24mm_clamps_after_original_minute_alignment_warning()
-> Result<(), Box<dyn std::error::Error>> {
    for (time, raw_minute) in [("24:01", 1), ("24:59", 59)] {
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
            decode_compact_until(&format!("Until:{time}"), None),
            Some((1440, raw_minute, true))
        );
    }
    let compile_until = |directive: &str,
                         remainder: bool|
     -> Result<crate::CompileResult, Box<dyn std::error::Error>> {
        let tail = if remainder {
            r#",{"field":"Until:24:00"},{"field":0.75}"#
        } else {
            ""
        };
        let source = format!(
            r#"{{
            "Timestep":{{"T":{{"number_of_timesteps_per_hour":4}}}},
            "Schedule:Compact":{{"S":{{"data":[
                {{"field":"Through:12/31"}},{{"field":"For:AllDays"}},
                {{"field":"{directive}"}},{{"field":0.25}}{tail}
            ]}}}}
        }}"#
        );
        Ok(compile_raw_model(&parse_epjson_str(&source)?))
    };
    for (directive, expected) in [
        ("Until:6D0:0D0", 360),
        ("Until6.0:0.0", 360),
        ("Until:6E:0D", 360),
        ("Until:6E+0:0D+0", 360),
        ("Until:24:", 1440),
        ("Until 2.4D1:0E", 1440),
        ("Until::15", 15),
        ("Until::", 0), // Real explicit initial0:0; later full-day row remains.
        ("Until :30", 30),
        ("Until::15+E0", 15),
        ("Until:-0:15", 15),
        ("Until:1e-9999:0", 0),
        ("Until:-1D-9999:15", 15),
        ("Until:6:1e-9999", 360),
        ("Until:24:-1D-9999", 1440),
    ] {
        let result = compile_until(directive, expected < 1440)?;
        assert!(
            !result.has_errors(),
            "{directive}: {:?}",
            result.report.diagnostics
        );
        let model = result
            .model
            .ok_or_else(|| std::io::Error::other("actual integral Until model"))?;
        assert_eq!(
            model.compact_schedules[0].periods[0].day_profiles[0].segments[0].until_minute_of_day,
            expected
        );
    }
    // Compiler rejection is a bounded projection; initialMM60 and the genuine
    // nonIntegral format-fatal prefix are not relabeled as Native type errors.
    for directive in [
        "Until:6.5:0",
        "Until:6:0.5",
        "Until:6E-1:0",
        "Until:+6:0",
        "Until:6:+0",
        "Until:nan:0",
        "Until:nanE:0",
        "Until:6:1e9999E",
        "Until:\u{00a0}6:0",
        "Until:6:\u{00a0}0",
        "Until:6:60",
        "Until",
        "Until:",
        "Until:24",
        " Until:6D0:0",
        "Until :24:00",
    ] {
        let result = compile_until(directive, true)?;
        assert!(result.has_errors(), "{directive}");
        assert!(
            result
                .report
                .diagnostics
                .iter()
                .any(|row| row.code == "InvalidScheduleCompactUntil"),
            "{directive}"
        );
        assert!(result.model.is_none());
    }
    Ok(())
}
