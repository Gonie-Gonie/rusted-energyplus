use super::super::{DiagnosticSeverity, compile_raw_model};
use ep_raw_model::parse_epjson_str;

const DIAGNOSTIC_CODE: &str = "ScheduleValueOutsideTypeLimits";

#[derive(Clone, Copy)]
enum ScalarScheduleFamily {
    Constant,
    ExternalInterface,
    FmuImport,
    FmuExport,
}

impl ScalarScheduleFamily {
    fn object_type(self) -> &'static str {
        match self {
            Self::Constant => "Schedule:Constant",
            Self::ExternalInterface => "ExternalInterface:Schedule",
            Self::FmuImport => "ExternalInterface:FunctionalMockupUnitImport:To:Schedule",
            Self::FmuExport => "ExternalInterface:FunctionalMockupUnitExport:To:Schedule",
        }
    }

    fn value_field(self) -> &'static str {
        match self {
            Self::Constant => "hourly_value",
            Self::ExternalInterface | Self::FmuImport | Self::FmuExport => "initial_value",
        }
    }

    fn object_json(self, value: f64) -> String {
        match self {
            Self::Constant => format!(
                r#""Schedule:Constant": {{
                    "Probe": {{
                        "schedule_type_limits_name": "Bounded",
                        "hourly_value": {value}
                    }}
                }}"#
            ),
            Self::ExternalInterface => format!(
                r#""ExternalInterface:Schedule": {{
                    "Probe": {{
                        "schedule_type_limits_name": "Bounded",
                        "initial_value": {value}
                    }}
                }}"#
            ),
            Self::FmuImport => format!(
                r#""ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {{
                    "Probe": {{
                        "schedule_type_limits_names": "Bounded",
                        "fmu_file_name": "Probe.fmu",
                        "fmu_instance_name": "ProbeInstance",
                        "fmu_variable_name": "Probe.Output",
                        "initial_value": {value}
                    }}
                }}"#
            ),
            Self::FmuExport => format!(
                r#""ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {{
                    "Export Instance": {{
                        "schedule_name": "Probe",
                        "schedule_type_limits_names": "Bounded",
                        "fmu_variable_name": "Probe.Input",
                        "initial_value": {value}
                    }}
                }}"#
            ),
        }
    }
}

fn compile_family(family: ScalarScheduleFamily, value: f64) -> super::super::CompileResult {
    let source = format!(
        r#"{{
            "ScheduleTypeLimits": {{
                "Bounded": {{
                    "lower_limit_value": 0,
                    "upper_limit_value": 1,
                    "numeric_type": "Continuous"
                }}
            }},
            {}
        }}"#,
        family.object_json(value)
    );
    let raw_model = parse_epjson_str(&source).expect("test epJSON should parse");
    compile_raw_model(&raw_model)
}

fn range_errors(result: &super::super::CompileResult) -> Vec<&super::super::ModelDiagnostic> {
    result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error && diagnostic.code == DIAGNOSTIC_CODE
        })
        .collect()
}

#[test]
fn rejects_out_of_range_value_for_each_scalar_schedule_family() {
    for family in [
        ScalarScheduleFamily::Constant,
        ScalarScheduleFamily::ExternalInterface,
        ScalarScheduleFamily::FmuImport,
        ScalarScheduleFamily::FmuExport,
    ] {
        let result = compile_family(family, 1.5);
        let errors = range_errors(&result);

        assert!(result.has_errors());
        assert!(result.model.is_none());
        assert_eq!(errors.len(), 1, "{:?}", result.report.diagnostics);
        assert_eq!(errors[0].object_type, family.object_type());
        assert_eq!(errors[0].object_name.as_deref(), Some("PROBE"));
        assert_eq!(errors[0].field.as_deref(), Some(family.value_field()));
        assert!(errors[0].message.contains("inclusive range [0, 1]"));
    }
}

#[test]
fn accepts_exact_inclusive_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {
                "Bounded": {
                    "lower_limit_value": -1,
                    "upper_limit_value": 1,
                    "numeric_type": "Continuous"
                }
            },
            "Schedule:Constant": {
                "At Lower": {
                    "schedule_type_limits_name": "Bounded",
                    "hourly_value": -1
                },
                "At Upper": {
                    "schedule_type_limits_name": "Bounded",
                    "hourly_value": 1
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.model.is_some());
    assert!(range_errors(&result).is_empty());
    Ok(())
}

#[test]
fn applies_f32_epsilon_tolerance_on_both_sides() {
    let epsilon = f64::from(f32::EPSILON);

    for accepted in [-epsilon, 1.0 + epsilon] {
        let result = compile_family(ScalarScheduleFamily::Constant, accepted);
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert!(result.model.is_some());
    }

    for rejected in [-(2.0 * epsilon), 1.0 + 2.0 * epsilon] {
        let result = compile_family(ScalarScheduleFamily::Constant, rejected);
        assert!(result.has_errors());
        assert!(result.model.is_none());
        assert_eq!(range_errors(&result).len(), 1);
    }
}

#[test]
fn preserves_energyplus_difference_first_rounding_at_epsilon_boundary() {
    let upper_limit = 6.099_981_700_054_9e-5;
    let value = 6.111_902_629_009_979e-5;
    let source = format!(
        r#"{{
            "ScheduleTypeLimits": {{
                "Bounded": {{
                    "lower_limit_value": 0,
                    "upper_limit_value": {upper_limit},
                    "numeric_type": "Continuous"
                }}
            }},
            "Schedule:Constant": {{
                "Probe": {{
                    "schedule_type_limits_name": "Bounded",
                    "hourly_value": {value}
                }}
            }}
        }}"#
    );
    let raw_model = parse_epjson_str(&source).expect("test epJSON should parse");

    let result = compile_raw_model(&raw_model);

    assert!(value <= upper_limit + f64::from(f32::EPSILON));
    assert!(value - upper_limit > f64::from(f32::EPSILON));
    assert!(result.has_errors());
    assert!(result.model.is_none());
    assert_eq!(range_errors(&result).len(), 1);

    let lower_limit = 1.275_224_529_692_495_7e-8;
    let value = -1.064_570_442_538_563e-7;
    let source = format!(
        r#"{{
            "ScheduleTypeLimits": {{
                "Bounded": {{
                    "lower_limit_value": {lower_limit},
                    "upper_limit_value": 1,
                    "numeric_type": "Continuous"
                }}
            }},
            "Schedule:Constant": {{
                "Probe": {{
                    "schedule_type_limits_name": "Bounded",
                    "hourly_value": {value}
                }}
            }}
        }}"#
    );
    let raw_model = parse_epjson_str(&source).expect("test epJSON should parse");

    let result = compile_raw_model(&raw_model);

    assert!(value < lower_limit - f64::from(f32::EPSILON));
    assert_eq!(lower_limit - value, f64::from(f32::EPSILON));
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.model.is_some());
    assert!(range_errors(&result).is_empty());
}

#[test]
fn skips_unbounded_and_one_sided_type_limits() -> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {
                "Unbounded": {},
                "Lower Only": {"lower_limit_value": 0},
                "Upper Only": {"upper_limit_value": 1}
            },
            "Schedule:Constant": {
                "Unbounded Probe": {
                    "schedule_type_limits_name": "Unbounded",
                    "hourly_value": 1000
                },
                "Lower Probe": {
                    "schedule_type_limits_name": "Lower Only",
                    "hourly_value": -1000
                },
                "Upper Probe": {
                    "schedule_type_limits_name": "Upper Only",
                    "hourly_value": 1000
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.model.is_some());
    assert!(range_errors(&result).is_empty());
    Ok(())
}

#[test]
fn post_parse_validation_accumulates_all_scalar_family_violations()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {
                "Bounded": {
                    "lower_limit_value": 0,
                    "upper_limit_value": 1,
                    "numeric_type": "Continuous"
                }
            },
            "Schedule:Constant": {
                "Constant Probe": {
                    "schedule_type_limits_name": "Bounded",
                    "hourly_value": -2
                }
            },
            "ExternalInterface:Schedule": {
                "External Probe": {
                    "schedule_type_limits_name": "Bounded",
                    "initial_value": 2
                }
            },
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {
                "Import Probe": {
                    "schedule_type_limits_names": "Bounded",
                    "fmu_file_name": "Probe.fmu",
                    "fmu_instance_name": "ProbeInstance",
                    "fmu_variable_name": "Probe.Output",
                    "initial_value": -3
                }
            },
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule": {
                "Export Instance": {
                    "schedule_name": "Export Probe",
                    "schedule_type_limits_names": "Bounded",
                    "fmu_variable_name": "Probe.Input",
                    "initial_value": 3
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);
    let errors = range_errors(&result);

    assert!(result.has_errors());
    assert!(result.model.is_none());
    assert_eq!(errors.len(), 4, "{:?}", result.report.diagnostics);
    assert_eq!(
        errors
            .iter()
            .map(|diagnostic| diagnostic.object_type.as_str())
            .collect::<Vec<_>>(),
        vec![
            "Schedule:Constant",
            "ExternalInterface:Schedule",
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule",
            "ExternalInterface:FunctionalMockupUnitExport:To:Schedule",
        ]
    );
    Ok(())
}

#[test]
fn post_parse_validation_only_checks_successfully_typed_objects()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "ScheduleTypeLimits": {
                "Bounded": {
                    "lower_limit_value": 0,
                    "upper_limit_value": 1,
                    "numeric_type": "Continuous"
                }
            },
            "ExternalInterface:FunctionalMockupUnitImport:To:Schedule": {
                "Incomplete Import": {
                    "schedule_type_limits_names": "Bounded",
                    "fmu_file_name": "Probe.fmu",
                    "fmu_instance_name": "ProbeInstance",
                    "initial_value": 2
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.model.is_none());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == "MissingRequiredField"
            && diagnostic.field.as_deref() == Some("fmu_variable_name")
    }));
    assert!(range_errors(&result).is_empty());
    Ok(())
}
