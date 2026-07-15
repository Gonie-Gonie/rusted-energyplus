use super::super::{
    DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    compile_raw_model_with_auxiliary_root, typed_coverage_status,
};
use ep_model::ScheduleId;
use ep_raw_model::parse_epjson_str;
use std::path::PathBuf;

fn test_root(label: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root = std::env::temp_dir().join(format!(
        "rusted-energyplus-schedule-file-shading-{}-{label}",
        std::process::id()
    ));
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }
    std::fs::create_dir_all(&root)?;
    Ok(root)
}

fn shading_raw_model(
    file_name: &str,
    extra_objects: &str,
) -> Result<ep_raw_model::RawModel, Box<dyn std::error::Error>> {
    Ok(parse_epjson_str(&format!(
        r#"{{
            "Timestep": {{
                "Zone Timestep": {{"number_of_timesteps_per_hour": 1}}
            }},
            "Schedule:File:Shading": {{
                "Imported Shading": {{"file_name": "{file_name}"}}
            }}
            {extra_objects}
        }}"#
    ))?)
}

#[test]
fn loads_sorted_unique_columns_and_legacy_trailing_parenthesis()
-> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("sorted-columns")?;
    let mut csv = String::from("Timestamp,West Wall,East Wall,West Wall,()\n");
    for value in 1..=8760 {
        csv.push_str(&format!(
            "row-{value},{value},{},{} ,\n",
            -f64::from(value),
            10_000 + value
        ));
    }
    std::fs::write(root.join("shading.csv"), csv)?;
    let raw_model = shading_raw_model("shading.csv", "")?;

    let result = compile_raw_model_with_auxiliary_root(&raw_model, &root);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status("Schedule:File:Shading"),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Warning
            && diagnostic.code == "ScheduleFileShadingLegacyEmptySurfaceColumnRemoved"
    }));
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.object_count(), 2);
    let shading = model
        .file_shading_schedule
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing shading schedule"))?;
    assert_eq!(shading.file_name, "shading.csv");
    assert_eq!(shading.timesteps_per_hour, 1);
    assert_eq!(shading.source_day_count, 365);
    assert_eq!(shading.columns.len(), 2);

    let east = &shading.columns[0];
    assert_eq!(east.id, ScheduleId(0));
    assert_eq!(east.surface_header, "East Wall");
    assert_eq!(east.schedule_name.0, "EAST WALL_SHADING");
    assert_eq!(east.values.len(), 8760);
    assert_eq!(east.values[0], -1.0);
    assert_eq!(east.values[8759], -8760.0);

    let west = &shading.columns[1];
    assert_eq!(west.id, ScheduleId(1));
    assert_eq!(west.surface_header, "West Wall");
    assert_eq!(west.schedule_name.0, "WEST WALL_SHADING");
    assert_eq!(west.values[0], 1.0);
    assert_eq!(west.values[8759], 8760.0);
    assert_eq!(
        model.schedule_names.resolve("east wall_shading"),
        Some(east.id)
    );
    assert_eq!(
        model.schedule_names.resolve("WEST WALL_SHADING"),
        Some(west.id)
    );

    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn accepts_exact_leap_year_row_count() -> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("leap-rows")?;
    let mut csv = String::from("Timestamp,Leap Surface\n");
    for value in 1..=8784 {
        csv.push_str(&format!("row-{value},{value}\n"));
    }
    std::fs::write(root.join("leap.csv"), csv)?;
    let raw_model = shading_raw_model("leap.csv", "")?;

    let result = compile_raw_model_with_auxiliary_root(&raw_model, &root);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    let shading = model
        .file_shading_schedule
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing shading schedule"))?;
    assert_eq!(shading.source_day_count, 366);
    assert_eq!(shading.columns[0].values.len(), 8784);
    assert_eq!(shading.columns[0].values[8783], 8784.0);

    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn generated_columns_precede_all_ordinary_schedule_families()
-> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("shared-ids")?;
    let mut shading_csv = String::from("Timestamp,Only Surface\n");
    for value in 1..=8760 {
        shading_csv.push_str(&format!("row-{value},0.5\n"));
    }
    std::fs::write(root.join("shading.csv"), shading_csv)?;
    std::fs::write(root.join("ordinary.csv"), "3\n".repeat(8760))?;
    let raw_model = shading_raw_model(
        "shading.csv",
        r#",
            "Schedule:Constant": {
                "Constant Second": {"hourly_value": 1}
            },
            "Schedule:Compact": {
                "Compact Third": {
                    "data": [
                        {"field": "Through: 12/31"},
                        {"field": "For: AllDays"},
                        {"field": "Until: 24:00"},
                        {"field": 2}
                    ]
                }
            },
            "Schedule:File": {
                "File Fourth": {
                    "file_name": "ordinary.csv",
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
                "Only Day": {}
            },
            "Schedule:Week:Daily": {
                "Only Week": {
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
                    "customday2_schedule_day_name": "Only Day"
                }
            },
            "Schedule:Year": {
                "Year Fifth": {
                    "schedule_weeks": [{
                        "schedule_week_name": "Only Week",
                        "start_month": 1,
                        "start_day": 1,
                        "end_month": 12,
                        "end_day": 31
                    }]
                }
            }"#,
    )?;

    let result = compile_raw_model_with_auxiliary_root(&raw_model, &root);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    let shading = model
        .file_shading_schedule
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing shading schedule"))?;
    assert_eq!(shading.columns[0].id, ScheduleId(0));
    assert_eq!(model.schedules[0].id, ScheduleId(1));
    assert_eq!(model.compact_schedules[0].id, ScheduleId(2));
    assert_eq!(model.file_schedules[0].id, ScheduleId(3));
    assert_eq!(model.year_schedules[0].id, ScheduleId(4));

    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn rejects_missing_root_bad_rows_and_non_finite_values() -> Result<(), Box<dyn std::error::Error>> {
    let raw_model = shading_raw_model("shading.csv", "")?;
    let missing_root = compile_raw_model(&raw_model);
    assert!(missing_root.has_errors());
    assert!(missing_root.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingScheduleFileShadingAuxiliaryRoot"
            && diagnostic.object_type == "Schedule:File:Shading"
    }));

    let root = test_root("invalid-data")?;
    std::fs::write(root.join("shading.csv"), "Timestamp,Surface\n")?;
    let zero_timestep = parse_epjson_str(
        r#"{
            "Timestep": {
                "Zero": {"number_of_timesteps_per_hour": 0}
            },
            "Schedule:File:Shading": {
                "Imported Shading": {"file_name": "shading.csv"}
            }
        }"#,
    )?;
    let invalid_timestep = compile_raw_model_with_auxiliary_root(&zero_timestep, &root);
    assert!(invalid_timestep.has_errors());
    assert!(
        invalid_timestep
            .report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "InvalidScheduleFileShadingTimestep")
    );

    std::fs::write(root.join("shading.csv"), "Timestamp,Surface\nrow-1,0.5\n")?;
    let short = compile_raw_model_with_auxiliary_root(&raw_model, &root);
    assert!(short.has_errors());
    assert!(
        short
            .report
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "InvalidScheduleFileShadingRowCount" })
    );

    let mut inconsistent = String::from("Timestamp,Surface\n");
    for value in 1..=8760 {
        if value == 2 {
            inconsistent.push_str("row-2\n");
        } else {
            inconsistent.push_str(&format!("row-{value},0.5\n"));
        }
    }
    std::fs::write(root.join("shading.csv"), inconsistent)?;
    let bad_columns = compile_raw_model_with_auxiliary_root(&raw_model, &root);
    assert!(bad_columns.has_errors());
    assert!(
        bad_columns
            .report
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "InvalidScheduleFileShadingColumnCount" })
    );

    let mut non_finite = String::from("Timestamp,Surface\n");
    for value in 1..=8760 {
        if value == 2 {
            non_finite.push_str("row-2,NaN\n");
        } else {
            non_finite.push_str(&format!("row-{value},0.5\n"));
        }
    }
    std::fs::write(root.join("shading.csv"), non_finite)?;
    let bad_value = compile_raw_model_with_auxiliary_root(&raw_model, &root);
    assert!(bad_value.has_errors());
    assert!(
        bad_value
            .report
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "ScheduleFileShadingColumnNonNumeric" })
    );

    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn reports_generated_schedule_name_collisions() -> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("name-collision")?;
    let mut csv = String::from("Timestamp,Surface\n");
    for value in 1..=8760 {
        csv.push_str(&format!("row-{value},0.5\n"));
    }
    std::fs::write(root.join("shading.csv"), csv)?;
    let raw_model = shading_raw_model(
        "shading.csv",
        r#",
            "Schedule:Constant": {
                "surface_SHADING": {"hourly_value": 1}
            }"#,
    )?;

    let result = compile_raw_model_with_auxiliary_root(&raw_model, &root);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == "Schedule:Constant"
            && diagnostic.object_name.as_deref() == Some("surface_SHADING")
    }));

    std::fs::remove_dir_all(root)?;
    Ok(())
}
