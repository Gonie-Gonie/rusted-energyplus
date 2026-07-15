use super::super::{
    DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model, typed_coverage_status,
};
use ep_model::{ScheduleId, ScheduleTypeLimitId};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "ExternalInterface:FunctionalMockupUnitExport:To:Schedule";

#[test]
fn compiles_inner_schedule_name_and_retain_case_field_after_fmu_import()
-> Result<(), Box<dyn std::error::Error>> {
    let synthetic_outer_name = "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 1";
    let raw_model = parse_epjson_str(&format!(
        r#"{{
            "ScheduleTypeLimits": {{
                "Any Number": {{}}
            }},
            "Schedule:Constant": {{
                "Constant First": {{
                    "schedule_type_limits_name": "Any Number",
                    "hourly_value": 1
                }}
            }},
            "ExternalInterface:Schedule": {{
                "External Second": {{
                    "schedule_type_limits_name": "Any Number",
                    "initial_value": 2
                }}
            }},
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {{
                "FMU Import Third": {{
                    "schedule_type_limits_names": "Any Number",
                    "fmu_file_name": "Plant.fmu",
                    "fmu_instance_name": "PlantInstance",
                    "fmu_variable_name": "Plant.Output",
                    "initial_value": 3
                }}
            }},
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {{
                "{synthetic_outer_name}": {{
                    "schedule_name": "FMU Export Fourth",
                    "schedule_type_limits_names": "Any Number",
                    "fmu_variable_name": "Plant.MixedCase.Input",
                    "initial_value": -4.5
                }}
            }}
        }}"#,
    ))?;

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
    assert_eq!(model.object_count(), 6);
    assert_eq!(model.schedules[0].id, ScheduleId(0));
    assert_eq!(model.external_interface_schedules[0].id, ScheduleId(1));
    assert_eq!(
        model.external_interface_fmu_import_schedules[0].id,
        ScheduleId(2)
    );
    let fmu_export = &model.external_interface_fmu_export_schedules[0];
    assert_eq!(fmu_export.id, ScheduleId(3));
    assert_eq!(fmu_export.name.0, "FMU EXPORT FOURTH");
    assert_eq!(
        fmu_export.schedule_type_limits,
        Some(ScheduleTypeLimitId(0))
    );
    assert_eq!(fmu_export.fmu_variable_name, "Plant.MixedCase.Input");
    assert_eq!(fmu_export.initial_value, -4.5);
    assert_eq!(
        model.schedule_names.resolve("fmu export fourth"),
        Some(ScheduleId(3))
    );
    assert_eq!(model.schedule_names.resolve(synthetic_outer_name), None);

    Ok(())
}

#[test]
fn omitted_initial_value_defaults_to_zero_for_inner_schedule_name()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {"Any Number": {}},
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 1": {
                    "schedule_name": "Defaulted Export",
                    "schedule_type_limits_names": "Any Number",
                    "fmu_variable_name": "ExactCase.Input"
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.report.defaults_applied.iter().any(|default| {
        default.object_type == OBJECT_TYPE
            && default.object_name == "Defaulted Export"
            && default.field == "initial_value"
            && default.value == "0.0"
    }));
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    let schedule = &model.external_interface_fmu_export_schedules[0];
    assert_eq!(schedule.initial_value, 0.0);
    assert_eq!(schedule.fmu_variable_name, "ExactCase.Input");

    Ok(())
}

#[test]
fn emits_one_inactive_family_warning_for_multiple_schedules()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {"Any Number": {}},
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 1": {
                    "schedule_name": "First Export",
                    "schedule_type_limits_names": "Any Number",
                    "fmu_variable_name": "FirstInput",
                    "initial_value": 1
                },
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 2": {
                    "schedule_name": "Second Export",
                    "schedule_type_limits_names": "Any Number",
                    "fmu_variable_name": "SecondInput",
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
                && diagnostic.code == "InactiveExternalInterfaceFmuExportScheduleHeldAtInitialValue"
        })
        .collect::<Vec<_>>();
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].object_type, OBJECT_TYPE);
    assert_eq!(warnings[0].object_name, None);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.external_interface_fmu_export_schedules.len(), 2);
    assert_eq!(
        model.external_interface_fmu_export_schedules[0].id,
        ScheduleId(0)
    );
    assert_eq!(
        model.external_interface_fmu_export_schedules[1].id,
        ScheduleId(1)
    );

    Ok(())
}

#[test]
fn missing_and_blank_type_limits_warn_per_inner_schedule_name()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 1": {
                    "schedule_name": "Missing Type Limits",
                    "fmu_variable_name": "MissingInput",
                    "initial_value": 1
                },
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 2": {
                    "schedule_name": "Blank Type Limits",
                    "schedule_type_limits_names": "   ",
                    "fmu_variable_name": "BlankInput",
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
            diagnostic.code == "MissingExternalInterfaceFmuExportScheduleTypeLimits"
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
            .any(|warning| { warning.object_name.as_deref() == Some("Blank Type Limits") })
    );

    Ok(())
}

#[test]
fn only_fmu_export_activation_fails_closed_case_insensitively()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface": {
                "External Interface 1": {
                    "name_of_external_interface": "fUnCtIoNaLmOcKuPuNiTeXpOrT"
                }
            },
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 1": {
                    "schedule_name": "Live Export",
                    "fmu_variable_name": "LiveInput",
                    "initial_value": 0.875
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
    assert!(errors[0].message.contains("FunctionalMockupUnitExport"));
    assert!(!result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InactiveExternalInterfaceFmuExportScheduleHeldAtInitialValue"
    }));
    assert!(result.model.is_none());

    Ok(())
}

#[test]
fn unrelated_interface_keys_keep_fmu_export_schedule_at_initial_value()
-> Result<(), Box<dyn std::error::Error>> {
    for key in [
        "PtolemyServer",
        "FunctionalMockupUnitImport",
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
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {{
                    "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 1": {{
                        "schedule_name": "Inactive Export",
                        "fmu_variable_name": "UnusedInput",
                        "initial_value": 0.875
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
            diagnostic.code == "InactiveExternalInterfaceFmuExportScheduleHeldAtInitialValue"
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
            model.external_interface_fmu_export_schedules[0].initial_value,
            0.875
        );
    }

    Ok(())
}

#[test]
fn rejects_missing_inner_schedule_name_and_fmu_variable() -> Result<(), Box<dyn std::error::Error>>
{
    let raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 1": {
                    "fmu_variable_name": "Input",
                    "initial_value": 1
                },
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 2": {
                    "schedule_name": "Missing Variable",
                    "initial_value": 2
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.field.as_deref() == Some("schedule_name")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Missing Variable")
            && diagnostic.field.as_deref() == Some("fmu_variable_name")
    }));
    assert_eq!(result.report.typed_object_count, 1);

    Ok(())
}

#[test]
fn earlier_schedule_family_wins_global_inner_name_collision()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {"Any Number": {}},
            "Schedule:Constant": {
                "Shared Name": {
                    "schedule_type_limits_name": "Any Number",
                    "hourly_value": 1
                }
            },
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {
                "Unrelated Synthetic Outer Key": {
                    "schedule_name": " shared name ",
                    "schedule_type_limits_names": "Any Number",
                    "fmu_variable_name": "Input",
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
    assert!(!result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_name.as_deref() == Some("Unrelated Synthetic Outer Key")
    }));
    assert_eq!(result.report.typed_object_count, 3);

    Ok(())
}

#[test]
fn unknown_type_limits_remain_a_strict_missing_reference_error()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule 1": {
                    "schedule_name": "Unknown Limits Export",
                    "schedule_type_limits_names": "Not Declared",
                    "fmu_variable_name": "Input",
                    "initial_value": 0.875
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Unknown Limits Export")
            && diagnostic.field.as_deref() == Some("schedule_type_limits_names")
    }));

    Ok(())
}
