use super::super::{
    DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model, typed_coverage_status,
};
use ep_model::{ScheduleId, ScheduleTypeLimitId};
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue, parse_epjson_str};

const OBJECT_TYPE: &str = "ExternalInterface:FunctionalMockupUnitImport:To:Schedule";

#[test]
fn compiles_retain_case_fields_after_external_interface_schedule()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {
                "Any Number": {}
            },
            "Schedule:Constant": {
                "Constant First": {
                    "schedule_type_limits_name": "Any Number",
                    "hourly_value": 1
                }
            },
            "ExternalInterface:Schedule": {
                "External Second": {
                    "schedule_type_limits_name": "Any Number",
                    "initial_value": 2
                }
            },
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {
                "FMU Third": {
                    "schedule_type_limits_names": "Any Number",
                    "fmu_file_name": "MixedCase/PlantModel.fmu",
                    "fmu_instance_name": "Plant_Instance_A",
                    "fmu_variable_name": "Room.Temperature",
                    "initial_value": -3.5
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|coverage| coverage.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing coverage entry"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    assert_eq!(coverage.object_count, 1);

    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.object_count(), 5);
    assert_eq!(model.schedules[0].id, ScheduleId(0));
    assert_eq!(model.external_interface_schedules[0].id, ScheduleId(1));
    let fmu_import = &model.external_interface_fmu_import_schedules[0];
    assert_eq!(fmu_import.id, ScheduleId(2));
    assert_eq!(fmu_import.name.0, "FMU THIRD");
    assert_eq!(
        fmu_import.schedule_type_limits,
        Some(ScheduleTypeLimitId(0))
    );
    assert_eq!(fmu_import.fmu_file_name, "MixedCase/PlantModel.fmu");
    assert_eq!(fmu_import.fmu_instance_name, "Plant_Instance_A");
    assert_eq!(fmu_import.fmu_variable_name, "Room.Temperature");
    assert_eq!(fmu_import.initial_value, -3.5);
    assert_eq!(
        model.schedule_names.resolve("fmu third"),
        Some(ScheduleId(2))
    );

    Ok(())
}

#[test]
fn emits_one_inactive_family_warning_for_multiple_schedules()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {
                "First": {
                    "fmu_file_name": "first.fmu",
                    "fmu_instance_name": "FirstInstance",
                    "fmu_variable_name": "FirstOutput",
                    "initial_value": 1
                },
                "Second": {
                    "fmu_file_name": "second.fmu",
                    "fmu_instance_name": "SecondInstance",
                    "fmu_variable_name": "SecondOutput",
                    "initial_value": 2
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let warnings = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Warning
                && diagnostic.code == "InactiveExternalInterfaceFmuImportScheduleHeldAtInitialValue"
        })
        .collect::<Vec<_>>();
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].object_type, OBJECT_TYPE);
    assert_eq!(warnings[0].object_name, None);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.external_interface_fmu_import_schedules.len(), 2);
    assert_eq!(
        model.external_interface_fmu_import_schedules[0].id,
        ScheduleId(0)
    );
    assert_eq!(
        model.external_interface_fmu_import_schedules[1].id,
        ScheduleId(1)
    );

    Ok(())
}

#[test]
fn missing_and_blank_type_limits_warn_per_schedule_and_compile_unvalidated()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {
                "Missing Type Limits": {
                    "fmu_file_name": "missing.fmu",
                    "fmu_instance_name": "MissingInstance",
                    "fmu_variable_name": "MissingOutput",
                    "initial_value": 1
                },
                "Blank Type Limits": {
                    "schedule_type_limits_names": "   ",
                    "fmu_file_name": "blank.fmu",
                    "fmu_instance_name": "BlankInstance",
                    "fmu_variable_name": "BlankOutput",
                    "initial_value": 2
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let warnings = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "MissingExternalInterfaceFmuImportScheduleTypeLimits"
        })
        .collect::<Vec<_>>();
    assert_eq!(warnings.len(), 2);
    assert!(warnings.iter().all(|warning| {
        warning.severity == DiagnosticSeverity::Warning
            && warning.object_type == OBJECT_TYPE
            && warning.field.as_deref() == Some("schedule_type_limits_names")
            && warning.message.contains("Schedule will not be validated.")
    }));
    assert!(
        warnings
            .iter()
            .any(|warning| { warning.object_name.as_deref() == Some("Missing Type Limits") })
    );
    assert!(
        warnings
            .iter()
            .any(|warning| warning.object_name.as_deref() == Some("Blank Type Limits"))
    );
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.external_interface_fmu_import_schedules.len(), 2);
    assert!(
        model
            .external_interface_fmu_import_schedules
            .iter()
            .all(|schedule| schedule.schedule_type_limits.is_none())
    );

    Ok(())
}

#[test]
fn only_fmu_import_activation_fails_closed_case_insensitively()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface": {
                "External Interface 1": {
                    "name_of_external_interface": "fUnCtIoNaLmOcKuPuNiTiMpOrT"
                }
            },
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {
                "Live Schedule": {
                    "fmu_file_name": "live.fmu",
                    "fmu_instance_name": "LiveInstance",
                    "fmu_variable_name": "LiveOutput",
                    "initial_value": 0.625
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    let errors = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "UnsupportedExternalInterfaceLiveExchange")
        .collect::<Vec<_>>();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].severity, DiagnosticSeverity::Error);
    assert_eq!(errors[0].object_type, OBJECT_TYPE);
    assert!(errors[0].message.contains("FunctionalMockupUnitImport"));
    assert!(!result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InactiveExternalInterfaceFmuImportScheduleHeldAtInitialValue"
    }));
    assert!(result.model.is_none());

    Ok(())
}

#[test]
fn unrelated_interface_keys_keep_fmu_import_schedule_at_initial_value()
-> Result<(), Box<dyn std::error::Error>> {
    for key in [
        "PtolemyServer",
        "FunctionalMockupUnitExport",
        "",
        "NotAnInterface",
    ] {
        let raw_model = parse_epjson_str(&format!(
            r#"{{
                "ExternalInterface": {{
                    "External Interface 1": {{
                        "name_of_external_interface": "{key}"
                    }}
                }},
                "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {{
                    "Inactive Schedule": {{
                        "fmu_file_name": "unused.fmu",
                        "fmu_instance_name": "UnusedInstance",
                        "fmu_variable_name": "UnusedOutput",
                        "initial_value": 0.625
                    }}
                }}
            }}"#,
        ))?;

        let result = compile_raw_model(&raw_model);

        assert!(
            !result.has_errors(),
            "key={key:?}: {:?}",
            result.report.diagnostics
        );
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InactiveExternalInterfaceFmuImportScheduleHeldAtInitialValue"
        }));
        assert!(
            !result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "UnsupportedExternalInterfaceLiveExchange"
            })
        );
        let model = result
            .model
            .ok_or_else(|| std::io::Error::other("expected typed model"))?;
        assert_eq!(
            model.external_interface_fmu_import_schedules[0].initial_value,
            0.625
        );
    }

    Ok(())
}

#[test]
fn rejects_each_missing_required_payload_field() -> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {
                "Missing File": {
                    "fmu_instance_name": "Instance",
                    "fmu_variable_name": "Output",
                    "initial_value": 1
                },
                "Missing Instance": {
                    "fmu_file_name": "model.fmu",
                    "fmu_variable_name": "Output",
                    "initial_value": 1
                },
                "Missing Variable": {
                    "fmu_file_name": "model.fmu",
                    "fmu_instance_name": "Instance",
                    "initial_value": 1
                },
                "Missing Initial": {
                    "fmu_file_name": "model.fmu",
                    "fmu_instance_name": "Instance",
                    "fmu_variable_name": "Output"
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for (object_name, field) in [
        ("Missing File", "fmu_file_name"),
        ("Missing Instance", "fmu_instance_name"),
        ("Missing Variable", "fmu_variable_name"),
        ("Missing Initial", "initial_value"),
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "MissingRequiredField"
                && diagnostic.object_type == OBJECT_TYPE
                && diagnostic.object_name.as_deref() == Some(object_name)
                && diagnostic.field.as_deref() == Some(field)
        }));
    }

    Ok(())
}

#[test]
fn rejects_non_finite_initial_value() -> Result<(), Box<dyn std::error::Error>> {
    let mut raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {
                "Nonfinite": {
                    "fmu_file_name": "model.fmu",
                    "fmu_instance_name": "Instance",
                    "fmu_variable_name": "Output",
                    "initial_value": 0
                }
            }
        }"#,
    )?;
    let object = raw_model
        .objects
        .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
        .and_then(|instances| instances.get_mut(&ObjectName("Nonfinite".to_string())))
        .ok_or_else(|| std::io::Error::other("missing raw test object"))?;
    object.fields.insert(
        FieldName("initial_value".to_string()),
        RawValue::Number("NaN".to_string()),
    );

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumber"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Nonfinite")
            && diagnostic.field.as_deref() == Some("initial_value")
    }));

    Ok(())
}

#[test]
fn earlier_external_schedule_wins_global_name_collision() -> Result<(), Box<dyn std::error::Error>>
{
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {"Any Number": {}},
            "ExternalInterface:Schedule": {
                "Shared Name": {
                    "schedule_type_limits_name": "Any Number",
                    "initial_value": 1
                }
            },
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {
                " shared name ": {
                    "schedule_type_limits_names": "Any Number",
                    "fmu_file_name": "model.fmu",
                    "fmu_instance_name": "Instance",
                    "fmu_variable_name": "Output",
                    "initial_value": 2
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(" shared name ")
    }));
    assert_eq!(result.report.typed_object_count, 3);

    Ok(())
}
