use super::super::{
    DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model, typed_coverage_status,
};
use ep_model::{ScheduleId, ScheduleTypeLimitId};
use ep_raw_model::parse_epjson_str;

#[test]
fn compiles_initial_value_and_type_limits_after_prior_schedule_families()
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
                "Year Second": {
                    "schedule_type_limits_name": "Any Number",
                    "schedule_weeks": [{
                        "schedule_week_name": "Only Week",
                        "start_month": 1,
                        "start_day": 1,
                        "end_month": 12,
                        "end_day": 31
                    }]
                }
            },
            "ExternalInterface:Schedule": {
                "External Third": {
                    "schedule_type_limits_name": "Any Number",
                    "initial_value": -3.5
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status("ExternalInterface:Schedule"),
        ObjectCoverageStatus::Typed
    );
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|coverage| coverage.object_type == "ExternalInterface:Schedule")
        .ok_or_else(|| std::io::Error::other("missing coverage entry"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    assert_eq!(coverage.object_count, 1);

    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.object_count(), 7);
    assert_eq!(model.schedules[0].id, ScheduleId(0));
    assert_eq!(model.year_schedules[0].id, ScheduleId(1));
    let external = &model.external_interface_schedules[0];
    assert_eq!(external.id, ScheduleId(2));
    assert_eq!(external.name.0, "EXTERNAL THIRD");
    assert_eq!(external.schedule_type_limits, Some(ScheduleTypeLimitId(0)));
    assert_eq!(external.initial_value, -3.5);
    assert_eq!(
        model.schedule_names.resolve("external third"),
        Some(ScheduleId(2))
    );

    Ok(())
}

#[test]
fn emits_one_inactive_family_warning_for_multiple_schedules()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {"Any Number": {}},
            "ExternalInterface:Schedule": {
                "First": {
                    "schedule_type_limits_name": "Any Number",
                    "initial_value": 1
                },
                "Second": {
                    "schedule_type_limits_name": "Any Number",
                    "initial_value": 2
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let inactive_warnings = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Warning
                && diagnostic.code == "InactiveExternalInterfaceScheduleHeldAtInitialValue"
        })
        .collect::<Vec<_>>();
    assert_eq!(inactive_warnings.len(), 1);
    assert_eq!(inactive_warnings[0].object_name, None);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.external_interface_schedules.len(), 2);
    assert_eq!(model.external_interface_schedules[0].id, ScheduleId(0));
    assert_eq!(model.external_interface_schedules[1].id, ScheduleId(1));

    Ok(())
}

#[test]
fn rejects_missing_and_wrong_typed_initial_values() -> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {"Any Number": {}},
            "ExternalInterface:Schedule": {
                "Missing Initial": {
                    "schedule_type_limits_name": "Any Number"
                },
                "Wrong Type": {
                    "schedule_type_limits_name": "Any Number",
                    "initial_value": "not-a-number"
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == "ExternalInterface:Schedule"
            && diagnostic.object_name.as_deref() == Some("Missing Initial")
            && diagnostic.field.as_deref() == Some("initial_value")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidFieldType"
            && diagnostic.object_type == "ExternalInterface:Schedule"
            && diagnostic.object_name.as_deref() == Some("Wrong Type")
            && diagnostic.field.as_deref() == Some("initial_value")
    }));

    Ok(())
}

#[test]
fn rejects_name_collision_with_an_earlier_schedule_family() -> Result<(), Box<dyn std::error::Error>>
{
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {"Any Number": {}},
            "Schedule:Constant": {
                "Shared Name": {
                    "schedule_type_limits_name": "Any Number",
                    "hourly_value": 1
                }
            },
            "ExternalInterface:Schedule": {
                " shared name ": {
                    "schedule_type_limits_name": "Any Number",
                    "initial_value": 2
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == "ExternalInterface:Schedule"
            && diagnostic.object_name.as_deref() == Some(" shared name ")
    }));
    assert_eq!(result.report.typed_object_count, 3);

    Ok(())
}

#[test]
fn rejects_live_exchange_when_external_interface_is_activated()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {"Any Number": {}},
            "ExternalInterface": {
                "External Interface 1": {
                    "name_of_external_interface": "PtolemyServer"
                }
            },
            "ExternalInterface:Schedule": {
                "Live Schedule": {
                    "schedule_type_limits_name": "Any Number",
                    "initial_value": 2
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    let live_exchange_errors = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "UnsupportedExternalInterfaceLiveExchange")
        .collect::<Vec<_>>();
    assert_eq!(live_exchange_errors.len(), 1);
    assert_eq!(live_exchange_errors[0].severity, DiagnosticSeverity::Error);
    assert!(live_exchange_errors[0].message.contains("PtolemyServer"));
    assert!(!result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InactiveExternalInterfaceScheduleHeldAtInitialValue"
    }));
    assert!(result.model.is_none());

    Ok(())
}

#[test]
fn fmu_and_nonactivating_external_interface_keys_keep_schedule_at_initial_value()
-> Result<(), Box<dyn std::error::Error>> {
    for key in [
        "FunctionalMockupUnitImport",
        "FunctionalMockupUnitExport",
        "",
        "NotAnInterface",
    ] {
        let raw_model = parse_epjson_str(&format!(
            r#"{{
                "ScheduleTypeLimits": {{"Any Number": {{}}}},
                "ExternalInterface": {{
                    "External Interface 1": {{
                        "name_of_external_interface": "{key}"
                    }}
                }},
                "ExternalInterface:Schedule": {{
                    "Inactive Schedule": {{
                        "schedule_type_limits_name": "Any Number",
                        "initial_value": 0.375
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
            diagnostic.code == "InactiveExternalInterfaceScheduleHeldAtInitialValue"
        }));
        assert!(
            !result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "UnsupportedExternalInterfaceLiveExchange"
            })
        );
        let model = result
            .model
            .ok_or_else(|| std::io::Error::other("expected typed model"))?;
        assert_eq!(model.external_interface_schedules[0].initial_value, 0.375);
    }

    Ok(())
}

#[test]
fn blank_type_limits_warns_once_and_leaves_schedule_unvalidated()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ExternalInterface:Schedule": {
                "Blank Type Limits": {
                    "schedule_type_limits_name": "   ",
                    "initial_value": 4.25
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
        .filter(|diagnostic| diagnostic.code == "MissingExternalInterfaceScheduleTypeLimits")
        .collect::<Vec<_>>();
    assert_eq!(warnings.len(), 1);
    assert_eq!(
        warnings[0].object_name.as_deref(),
        Some("Blank Type Limits")
    );
    assert!(
        warnings[0]
            .message
            .contains("Schedule will not be validated.")
    );
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(
        model.external_interface_schedules[0].schedule_type_limits,
        None
    );

    Ok(())
}
