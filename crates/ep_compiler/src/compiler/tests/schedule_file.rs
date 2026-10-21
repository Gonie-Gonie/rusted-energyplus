use super::super::{
    DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model, compile_raw_model_for_source,
    compile_raw_model_with_auxiliary_root, typed_coverage_status,
};
use ep_model::{ScheduleFileColumnSeparator, ScheduleId};
use ep_raw_model::parse_epjson_str;
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
            ep_model::NormalizedName::new(&declaration.source_object_name)
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
        "MIXED FILE"
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
    // Existing normalized-name collisions still fail; this route does not
    // claim Native whitespace-distinct schedule namespace support.
    for raw in [
        r#"{"Schedule:File":{"File":{}," File ":{}}}"#,
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
    std::fs::remove_dir_all(root)?;
    Ok(())
}
