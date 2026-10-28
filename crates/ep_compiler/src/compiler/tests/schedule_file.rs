use super::super::{
    DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model, compile_raw_model_for_source,
    compile_raw_model_with_auxiliary_root, typed_coverage_status,
};
use ep_model::{ScheduleFileColumnSeparator, ScheduleId};
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue, parse_epjson_str};
use std::path::PathBuf;

fn test_root(label: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root = std::env::temp_dir().join(format!(
        "rusted-energyplus-schedule-file-{}-{label}",
        std::process::id()
    ));
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }
    std::fs::create_dir_all(&root)?;
    Ok(root)
}

fn schedule_file_raw_model(
    extra_fields: &str,
) -> Result<ep_raw_model::RawModel, Box<dyn std::error::Error>> {
    Ok(parse_epjson_str(&format!(
        r#"{{
            "ScheduleTypeLimits": {{
                "Any Number": {{}}
            }},
            "Schedule:File": {{
                "Selected Column": {{
                    "schedule_type_limits_name": "Any Number",
                    "file_name": "schedule.csv",
                    "column_number": 2,
                    "rows_to_skip_at_top": 1,
                    "number_of_hours_of_data": 8760,
                    "column_separator": "Comma",
                    "interpolate_to_timestep": "No",
                    "minutes_per_item": 60,
                    "adjust_schedule_for_daylight_savings": "No"
                    {extra_fields}
                }}
            }}
        }}"#
    ))?)
}

#[test]
fn loads_selected_hourly_column_below_auxiliary_root() -> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("selected-column")?;
    let mut csv = String::from("decoy,selected,extra\n");
    for value in 1..=8760 {
        csv.push_str(&format!("{},{},{}\n", -value, value, 10_000 + value));
    }
    std::fs::write(root.join("schedule.csv"), csv)?;
    let raw_model = schedule_file_raw_model("")?;

    let result = compile_raw_model_with_auxiliary_root(&raw_model, &root);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status("Schedule:File"),
        ObjectCoverageStatus::Typed
    );
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.file_schedules.len(), 1);
    let schedule = &model.file_schedules[0];
    assert_eq!(schedule.name.0, "SELECTED COLUMN");
    assert_eq!(schedule.column_number, 2);
    assert_eq!(schedule.rows_to_skip_at_top, 1);
    assert_eq!(schedule.number_of_hours_of_data, 8760);
    assert_eq!(
        schedule.column_separator,
        ScheduleFileColumnSeparator::Comma
    );
    assert!(!schedule.interpolate_to_timestep);
    assert_eq!(schedule.minutes_per_item, 60);
    assert!(!schedule.adjust_schedule_for_daylight_savings);
    assert_eq!(schedule.values.len(), 8760);
    assert_eq!(schedule.values[0], 1.0);
    assert_eq!(schedule.values[1392], 1393.0);
    assert_eq!(schedule.values[8759], 8760.0);
    // The explicit Source route declares the same identity without reading or
    // constructing legacy values, even when this auxiliary file is available.
    let source = compile_raw_model_for_source(&raw_model);
    assert!(!source.has_errors(), "{:?}", source.report.diagnostics);
    assert!(source.report.defaults_applied.is_empty());
    assert_eq!(source.report.typed_object_count, 2); // implicit Timestep + ScheduleTypeLimits
    assert_eq!(
        source
            .report
            .coverage
            .iter()
            .find(|row| row.object_type == "Schedule:File")
            .map(|row| row.status),
        Some(ObjectCoverageStatus::RawOnly)
    );
    let source_model = source
        .model
        .ok_or_else(|| std::io::Error::other("expected Source declaration"))?;
    assert!(source_model.file_schedules.is_empty());
    assert_eq!(source_model.source_file_schedule_declarations.len(), 1);
    let declaration = &source_model.source_file_schedule_declarations[0];
    assert_eq!(declaration.id, schedule.id);
    assert_eq!(declaration.name, schedule.name);
    assert_eq!(declaration.source_object_name, "Selected Column");
    assert_eq!(
        source_model.schedule_names.resolve("selected column"),
        Some(declaration.id)
    );
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn requires_auxiliary_root_and_rejects_unported_branches() -> Result<(), Box<dyn std::error::Error>>
{
    let raw_model = schedule_file_raw_model("")?;
    let missing_root = compile_raw_model(&raw_model);
    assert!(missing_root.has_errors());
    assert!(missing_root.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingScheduleFileAuxiliaryRoot"
            && diagnostic.object_type == "Schedule:File"
    }));

    let unsupported = parse_epjson_str(
        r#"{
            "Schedule:File": {
                "Unsupported File": {
                    "file_name": "schedule.csv",
                    "column_number": 1,
                    "rows_to_skip_at_top": 0,
                    "number_of_hours_of_data": 8784,
                    "column_separator": "Comma",
                    "interpolate_to_timestep": "Yes",
                    "minutes_per_item": 20,
                    "adjust_schedule_for_daylight_savings": "Yes"
                }
            }
        }"#,
    )?;
    let root = test_root("unsupported")?;
    let result = compile_raw_model_with_auxiliary_root(&unsupported, &root);
    assert!(result.has_errors());
    for code in [
        "InvalidScheduleFileHoursOfData",
        "UnsupportedScheduleFileMinutesPerItem",
        "UnsupportedScheduleFileInterpolation",
        "UnsupportedScheduleFileDaylightSavingAdjustment",
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error && diagnostic.code == code
        }));
    }
    // These are request declarations, not a second field/default/file parser.
    // Malformed fields are retained for actual Source admission/getter handling.
    for raw in [
        unsupported,
        parse_epjson_str(
            r#"{"Schedule:File":{"  MiXeD File  ":{"schedule_type_limits_name":"Missing Type","file_name":false,"column_number":"bad"}}}"#,
        )?,
    ] {
        let source = compile_raw_model_for_source(&raw);
        assert!(!source.has_errors(), "{:?}", source.report.diagnostics);
        assert!(source.report.diagnostics.is_empty());
        assert!(source.report.defaults_applied.is_empty());
        assert_eq!(source.report.typed_object_count, 1); // existing implicit Timestep
        let model = source
            .model
            .ok_or_else(|| std::io::Error::other("expected request-only model"))?;
        assert!(model.file_schedules.is_empty());
        assert_eq!(model.source_file_schedule_declarations.len(), 1);
        let declaration = &model.source_file_schedule_declarations[0];
        assert_eq!(declaration.id, ScheduleId(0));
        assert_eq!(
            declaration.name,
            ep_model::NormalizedName(declaration.source_object_name.to_ascii_uppercase())
        );
        assert_eq!(
            model
                .schedule_names
                .resolve(&declaration.source_object_name),
            Some(declaration.id)
        );
    }
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn rejects_short_and_non_numeric_selected_columns() -> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("invalid-column")?;
    std::fs::write(root.join("schedule.csv"), "header,value\nunused,1\n")?;
    let raw_model = schedule_file_raw_model("")?;
    let short = compile_raw_model_with_auxiliary_root(&raw_model, &root);
    assert!(
        short
            .report
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "InvalidScheduleFileRowCount" })
    );

    let mut csv = String::from("header,value\n");
    for value in 1..=8760 {
        if value == 1393 {
            csv.push_str("unused,not-a-number\n");
        } else {
            csv.push_str(&format!("unused,{value}\n"));
        }
    }
    std::fs::write(root.join("schedule.csv"), csv)?;
    let non_numeric = compile_raw_model_with_auxiliary_root(&raw_model, &root);
    assert!(
        non_numeric
            .report
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "ScheduleFileSelectedColumnNonNumeric" })
    );
    // Source compilation must not turn the short/non-numeric sidecar into
    // values. Later inactive schedule families retain distinct reference IDs.
    let raw = parse_epjson_str(
        r#"{
        "Schedule:Constant":{"Before":{"hourly_value":1}},
        "Schedule:File":{"  MiXeD File  ":{}},
        "ExternalInterface:Schedule":{"After":{"initial_value":2}},
        "ExternalInterface:FunctionalMockupUnitImport:To:Schedule":{"Imported":{"fmu_file_name":"x.fmu","fmu_instance_name":"x","fmu_variable_name":"v","initial_value":3}},
        "ExternalInterface:FunctionalMockupUnitExport:To:Schedule":{"Exported Object":{"schedule_name":"Exported","fmu_variable_name":"v","initial_value":4}}
    }"#,
    )?;
    let source = compile_raw_model_for_source(&raw);
    assert!(!source.has_errors(), "{:?}", source.report.diagnostics);
    let model = source
        .model
        .ok_or_else(|| std::io::Error::other("expected identity inventory"))?;
    assert!(model.file_schedules.is_empty());
    assert_eq!(
        model.source_file_schedule_declarations[0].source_object_name,
        "  MiXeD File  "
    );
    assert_eq!(
        model.source_file_schedule_declarations[0].name.0,
        "  MIXED FILE  "
    );
    for (name, id) in [
        ("Before", 0),
        ("  MiXeD File  ", 1),
        ("After", 2),
        ("Imported", 3),
        ("Exported", 4),
    ] {
        assert_eq!(model.schedule_names.resolve(name), Some(ScheduleId(id)));
    }
    assert_eq!(model.object_count(), 5); // implicit Timestep + four value schedules
    assert!(compile_raw_model(&raw).has_errors());
    // Legacy compilation still rejects its trimmed-name collision. The
    // explicit Source policy instead preserves both authored identities.
    let padded = parse_epjson_str(r#"{"Schedule:File":{"File":{}," File ":{}}}"#)?;
    let legacy_padded = parse_epjson_str(
        r#"{"Schedule:Constant":{"File":{"hourly_value":1}," File ":{"hourly_value":2}}}"#,
    )?;
    assert!(
        compile_raw_model(&legacy_padded)
            .report
            .diagnostics
            .iter()
            .any(|row| row.code == "DuplicateName")
    );
    let source = compile_raw_model_for_source(&padded);
    assert!(!source.has_errors(), "{:?}", source.report.diagnostics);
    let padded_model = source.model.ok_or("Source padded declarations")?;
    let exact = padded_model
        .schedule_names
        .resolve("File")
        .ok_or("exact File")?;
    let spaced = padded_model
        .schedule_names
        .resolve(" File ")
        .ok_or("spaced File")?;
    assert_ne!(exact, spaced);
    assert_eq!(padded_model.schedule_names.resolve("file"), Some(exact));
    assert_eq!(padded_model.schedule_names.resolve(" File"), None);
    assert_eq!(padded_model.source_file_schedule_declarations.len(), 2);
    assert!(padded_model.file_schedules.is_empty());
    for raw in [
        r#"{"Schedule:File":{"File":{},"fIlE":{}}}"#,
        r#"{"Schedule:Constant":{"File":{"hourly_value":1}},"Schedule:File":{"File":{}}}"#,
    ] {
        let collision = compile_raw_model_for_source(&parse_epjson_str(raw)?);
        assert!(collision.has_errors());
        assert!(
            collision
                .report
                .diagnostics
                .iter()
                .any(|row| row.code == "DuplicateName")
        );
        assert!(collision.model.is_none());
    }
    // One Source declaration/reference graph exercises all four namespaces.
    // The file declarations remain requests: no sidecar data or runtime result
    // is supplied by this compiler-only child.
    let mut names_raw = parse_epjson_str(
        r#"{
        "Zone": {"Z": {}},
        "ScheduleTypeLimits": {
            "Range": {"lower_limit_value": 0, "upper_limit_value": 10},
            " Range ": {"lower_limit_value": 10, "upper_limit_value": 20}
        },
        "Schedule:Constant": {
            "Value": {"schedule_type_limits_name": "Range", "hourly_value": 1},
            " Value ": {"schedule_type_limits_name": " Range ", "hourly_value": 11}
        },
        "Schedule:Compact": {
            "Compact": {"schedule_type_limits_name": "Range", "data": [
                {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                {"field": "Until: 24:00"}, {"field": 2}
            ]},
            " Compact ": {"schedule_type_limits_name": " Range ", "data": [
                {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                {"field": "Until: 24:00"}, {"field": 12}
            ]}
        },
        "Schedule:File": {
            "File": {"file_name": "unused.csv", "column_number": 1, "rows_to_skip_at_top": 0},
            " File ": {"file_name": "unused.csv", "column_number": 2, "rows_to_skip_at_top": 0}
        },
        "Schedule:Day:Hourly": {"Day": {"hour_1": 1}, " Day ": {"hour_1": 2}},
        "Schedule:Day:Interval": {
            "Interval": {"data": [{"time": "24:00", "value_until_time": 3}]},
            " Interval ": {"data": [{"time": "24:00", "value_until_time": 4}]}
        },
        "Schedule:Day:List": {
            "List": {"minutes_per_item": 60, "extensions": []},
            " List ": {"minutes_per_item": 60, "extensions": []}
        },
        "Schedule:Week:Daily": {"Week": {}, " Week ": {}},
        "Schedule:Week:Compact": {
            "Compact Week": {"data": [{"daytype_list": "AllDays", "schedule_day_name": "Interval"}]},
            " Compact Week ": {"data": [{"daytype_list": "AllDays", "schedule_day_name": " List "}]}
        },
        "Schedule:Year": {
            "Year": {"schedule_weeks": [{"schedule_week_name": "Week", "start_month": 1, "start_day": 1, "end_month": 12, "end_day": 31}]},
            " Year ": {"schedule_weeks": [{"schedule_week_name": " Week ", "start_month": 1, "start_day": 1, "end_month": 12, "end_day": 31}]}
        },
        "ThermostatSetpoint:DualSetpoint": {"T": {
            "heating_setpoint_temperature_schedule_name": "Value",
            "cooling_setpoint_temperature_schedule_name": " Value "
        }},
        "People": {"P": {"zone_or_zonelist_or_space_or_spacelist_name": "Z", "number_of_people": 1, "number_of_people_schedule_name": " File "}},
        "OtherEquipment": {"E": {"zone_or_zonelist_or_space_or_spacelist_name": "Z", "design_level": 5, "schedule_name": "File"}}
    }"#,
    )?;
    for (week, day) in [("Week", "Day"), (" Week ", " Day ")] {
        let object = names_raw
            .objects
            .get_mut(&ObjectType("Schedule:Week:Daily".into()))
            .and_then(|objects| objects.get_mut(&ObjectName(week.into())))
            .ok_or("Week fixture")?;
        for field in [
            "sunday_schedule_day_name",
            "monday_schedule_day_name",
            "tuesday_schedule_day_name",
            "wednesday_schedule_day_name",
            "thursday_schedule_day_name",
            "friday_schedule_day_name",
            "saturday_schedule_day_name",
            "holiday_schedule_day_name",
            "summerdesignday_schedule_day_name",
            "winterdesignday_schedule_day_name",
            "customday1_schedule_day_name",
            "customday2_schedule_day_name",
        ] {
            object
                .fields
                .insert(FieldName(field.into()), RawValue::String(day.into()));
        }
    }
    for (name, value) in [("List", "5"), (" List ", "6")] {
        let object = names_raw
            .objects
            .get_mut(&ObjectType("Schedule:Day:List".into()))
            .and_then(|objects| objects.get_mut(&ObjectName(name.into())))
            .ok_or("List fixture")?;
        object.fields.insert(
            FieldName("extensions".into()),
            RawValue::Array(vec![
                RawValue::Object(std::collections::BTreeMap::from([(
                    FieldName("value".into()),
                    RawValue::Number(value.into())
                )]));
                24
            ]),
        );
    }
    let result = compile_raw_model_for_source(&names_raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let names = result
        .model
        .ok_or("Source exact schedule/reference graph")?;
    assert_ne!(
        names.schedule_type_limit_names.resolve("Range"),
        names.schedule_type_limit_names.resolve(" Range ")
    );
    assert_ne!(
        names.schedule_names.resolve("Value"),
        names.schedule_names.resolve(" Value ")
    );
    assert_eq!(
        names.schedule_names.resolve("vAlUe"),
        names.schedule_names.resolve("Value")
    );
    assert_eq!(names.schedule_names.resolve("Value "), None);
    let value = names
        .schedules
        .iter()
        .find(|row| row.name.0 == " VALUE ")
        .ok_or("padded Constant arena name")?;
    assert_eq!(value.hourly_value, 11.0);
    assert_eq!(
        value.schedule_type_limits,
        names.schedule_type_limit_names.resolve(" Range ")
    );
    let compact = names
        .compact_schedules
        .iter()
        .find(|row| row.name.0 == " COMPACT ")
        .ok_or("padded Compact arena name")?;
    assert_eq!(compact.schedule_type_limits, value.schedule_type_limits);
    let day = names
        .day_schedules
        .iter()
        .find(|row| row.name.0 == " DAY ")
        .ok_or("padded Day arena name")?;
    assert_eq!(day.hourly_values[0], 2.0);
    assert_eq!(names.day_schedule_names.resolve(" dAy "), Some(day.id));
    assert_ne!(names.day_schedule_names.resolve("Day"), Some(day.id));
    let week = names
        .week_schedules
        .iter()
        .find(|row| row.name.0 == " WEEK ")
        .ok_or("padded Week arena name")?;
    assert_eq!(week.day_schedules, [day.id; 12]);
    assert_eq!(names.week_schedule_names.resolve(" wEeK "), Some(week.id));
    assert_ne!(names.week_schedule_names.resolve("Week"), Some(week.id));
    let list = names
        .day_list_schedules
        .iter()
        .find(|row| row.name.0 == " LIST ")
        .ok_or("padded List arena name")?;
    assert_eq!(list.values, vec![6.0; 24]);
    let compact_week = names
        .week_compact_schedules
        .iter()
        .find(|row| row.name.0 == " COMPACT WEEK ")
        .ok_or("padded compact Week arena name")?;
    assert_eq!(compact_week.day_schedules, [list.id; 12]);
    let year = names
        .year_schedules
        .iter()
        .find(|row| row.name.0 == " YEAR ")
        .ok_or("padded Year arena name")?;
    assert_eq!(year.week_schedules, [week.id; 366]);
    let file = names
        .source_file_schedule_declarations
        .iter()
        .find(|row| row.source_object_name == " File ")
        .ok_or("raw File name retained")?;
    assert_eq!(file.name.0, " FILE ");
    assert_eq!(
        names.people[0].number_of_people_schedule,
        Some(ep_model::PeopleScheduleReference::User(file.id))
    );
    assert_eq!(
        names.other_equipment[0].schedule,
        names
            .schedule_names
            .resolve("File")
            .map(ep_model::InternalGainScheduleReference::User)
    );
    assert_ne!(
        names.other_equipment[0].schedule,
        Some(ep_model::InternalGainScheduleReference::User(file.id))
    );
    assert_eq!(
        names.thermostat_dual_setpoints[0].heating_setpoint_schedule,
        names
            .schedule_names
            .resolve("Value")
            .ok_or("bare Constant")?
    );
    assert_eq!(
        names.thermostat_dual_setpoints[0].cooling_setpoint_schedule,
        value.id
    );
    assert!(names.file_schedules.is_empty());
    // The explicit Source route preserves every nonempty schedule-reference
    // string, including RE2-nonwhitespace VT and Unicode whitespace identities.
    for (exact, escaped) in [
        ("\u{b}", "\\u000b"),
        ("\u{a0}", "\\u00a0"),
        ("\u{85}", "\\u0085"),
        ("\u{2003}", "\\u2003"),
        ("\u{2028}", "\\u2028"),
    ] {
        let mut raw = parse_epjson_str(&format!(
            r#"{{
            "Zone":{{"Z":{{}}}},
            "ScheduleTypeLimits":{{"{escaped}":{{"lower_limit_value":0,"upper_limit_value":20}}}},
            "Schedule:Constant":{{"C":{{"schedule_type_limits_name":"{escaped}","hourly_value":1}}}},
            "Schedule:File":{{"{escaped}":{{}}}},
            "Schedule:Day:Hourly":{{"{escaped}":{{"schedule_type_limits_name":"{escaped}","hour_1":1}}}},
            "Schedule:Week:Daily":{{"{escaped}":{{}}}},
            "Schedule:Week:Compact":{{"W":{{"data":[{{"daytype_list":"AllDays","schedule_day_name":"{escaped}"}}]}}}},
            "Schedule:Year":{{"Y":{{"schedule_type_limits_name":"{escaped}","schedule_weeks":[{{"schedule_week_name":"{escaped}","start_month":1,"start_day":1,"end_month":12,"end_day":31}}]}}}},
            "ThermostatSetpoint:DualSetpoint":{{"T":{{"heating_setpoint_temperature_schedule_name":"{escaped}","cooling_setpoint_temperature_schedule_name":"C"}}}},
            "People":{{"P":{{"zone_or_zonelist_or_space_or_spacelist_name":"Z","number_of_people":1,"number_of_people_schedule_name":"{escaped}"}}}},
            "OtherEquipment":{{"E":{{"zone_or_zonelist_or_space_or_spacelist_name":"Z","design_level":1,"schedule_name":"{escaped}"}}}}
        }}"#
        ))?;
        let week = raw
            .objects
            .get_mut(&ObjectType("Schedule:Week:Daily".into()))
            .and_then(|rows| rows.get_mut(&ObjectName(exact.into())))
            .ok_or("exact Week")?;
        for field in [
            "sunday_schedule_day_name",
            "monday_schedule_day_name",
            "tuesday_schedule_day_name",
            "wednesday_schedule_day_name",
            "thursday_schedule_day_name",
            "friday_schedule_day_name",
            "saturday_schedule_day_name",
            "holiday_schedule_day_name",
            "summerdesignday_schedule_day_name",
            "winterdesignday_schedule_day_name",
            "customday1_schedule_day_name",
            "customday2_schedule_day_name",
        ] {
            week.fields
                .insert(FieldName(field.into()), RawValue::String(exact.into()));
        }
        let compiled = compile_raw_model_for_source(&raw);
        assert!(
            !compiled.has_errors(),
            "{exact:?}: {:?}",
            compiled.report.diagnostics
        );
        let model = compiled.model.ok_or("Source nonempty schedule graph")?;
        let file = &model.source_file_schedule_declarations[0];
        assert_eq!(file.source_object_name, exact);
        assert_eq!(file.name.0, exact);
        assert_eq!(model.schedule_names.resolve(exact), Some(file.id));
        assert_eq!(
            model.schedules[0].schedule_type_limits,
            model.schedule_type_limit_names.resolve(exact)
        );
        assert_eq!(model.day_schedules[0].name.0, exact);
        assert_eq!(
            model.day_schedules[0].schedule_type_limits,
            model.schedule_type_limit_names.resolve(exact)
        );
        assert_eq!(model.week_schedules[0].name.0, exact);
        assert_eq!(
            model.week_schedules[0].day_schedules,
            [model.day_schedules[0].id; 12]
        );
        assert_eq!(
            model.week_compact_schedules[0].day_schedules,
            [model.day_schedules[0].id; 12]
        );
        assert_eq!(
            model.year_schedules[0].week_schedules,
            [model.week_schedules[0].id; 366]
        );
        assert_eq!(
            model.people[0].number_of_people_schedule,
            Some(ep_model::PeopleScheduleReference::User(file.id))
        );
        assert_eq!(
            model.other_equipment[0].schedule,
            Some(ep_model::InternalGainScheduleReference::User(file.id))
        );
        assert_eq!(
            model.thermostat_dual_setpoints[0].heating_setpoint_schedule,
            file.id
        );
    }
    for (text, code) in [
        ("", "MissingRequiredField"),
        (" \t\r\u{c}", "MissingReference"),
    ] {
        let mut raw = parse_epjson_str(
            r#"{"Schedule:Constant":{"C":{"hourly_value":1}},"ThermostatSetpoint:DualSetpoint":{"T":{"heating_setpoint_temperature_schedule_name":"C","cooling_setpoint_temperature_schedule_name":"C"}}}"#,
        )?;
        raw.objects
            .get_mut(&ObjectType("ThermostatSetpoint:DualSetpoint".into()))
            .and_then(|rows| rows.get_mut(&ObjectName("T".into())))
            .ok_or("required schedule fixture")?
            .fields
            .insert(
                FieldName("heating_setpoint_temperature_schedule_name".into()),
                RawValue::String(text.into()),
            );
        let result = compile_raw_model_for_source(&raw);
        assert!(result.has_errors());
        assert!(result.report.diagnostics.iter().any(|row| row.code == code
            && row.field.as_deref() == Some("heating_setpoint_temperature_schedule_name")));
        let legacy = compile_raw_model(&raw);
        assert!(
            legacy
                .report
                .diagnostics
                .iter()
                .any(|row| row.code == "MissingRequiredField")
        );
    }
    // A missing exact name must not fall back to the trimmed existing name.
    names_raw
        .objects
        .get_mut(&ObjectType("ThermostatSetpoint:DualSetpoint".into()))
        .and_then(|objects| objects.get_mut(&ObjectName("T".into())))
        .ok_or("Thermostat fixture")?
        .fields
        .insert(
            FieldName("heating_setpoint_temperature_schedule_name".into()),
            RawValue::String("Value ".into()),
        );
    let missing = compile_raw_model_for_source(&names_raw);
    assert!(missing.has_errors());
    assert!(
        missing
            .report
            .diagnostics
            .iter()
            .any(|row| row.code == "MissingReference"
                && row.field.as_deref() == Some("heating_setpoint_temperature_schedule_name"))
    );
    std::fs::remove_dir_all(root)?;
    Ok(())
}
