use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus,
    VARIABLE_ABSORPTANCE_FUNCTION_OBJECT_TYPES, compile_raw_model, typed_coverage_status,
};
use ep_model::{
    MaterialVariableAbsorptanceId, NormalizedName, ScheduleId, TypedModel,
    VariableAbsorptanceControl, VariableAbsorptanceFunctionSignal, VariableAbsorptanceSchedule,
};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "MaterialProperty:VariableAbsorptance";

fn has_error(result: &CompileResult, code: &str, object_name: &str, field: Option<&str>) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

#[test]
fn variable_absorptance_materializes_schedules_builtins_and_separate_namespace()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "A Same Name": {
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                }
            },
            "Material:NoMass": {
                "B NoMass": {
                    "roughness":"MediumRough",
                    "thermal_resistance":2.0
                }
            },
            "Schedule:Constant": {
                "Thermal Schedule": {"hourly_value":0.2}
            },
            "MaterialProperty:VariableAbsorptance": {
                "A Same Name": {
                    "reference_material_name":"a same name",
                    "control_signal":"scheduled",
                    "thermal_absorptance_schedule_name":"thermal schedule",
                    "solar_absorptance_schedule_name":"does not exist"
                },
                "B Builtins": {
                    "reference_material_name":"b nomass",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"constant-0.0",
                    "solar_absorptance_schedule_name":"CONSTANT-1.0"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.material_variable_absorptances.len(), 2);
    assert_eq!(
        model
            .material_variable_absorptance_names
            .resolve("A SAME NAME"),
        Some(MaterialVariableAbsorptanceId(0))
    );
    assert_eq!(
        model.material_variable_absorptances[0].reference_material.0,
        0
    );
    assert_eq!(
        model.material_variable_absorptances[0].control,
        VariableAbsorptanceControl::Scheduled {
            thermal: Some(VariableAbsorptanceSchedule::User(ScheduleId(0))),
            solar: None,
        }
    );
    assert_eq!(
        model.material_variable_absorptances[1].reference_material.0,
        1
    );
    assert_eq!(
        model.material_variable_absorptances[1].control,
        VariableAbsorptanceControl::Scheduled {
            thermal: Some(VariableAbsorptanceSchedule::ConstantZero),
            solar: Some(VariableAbsorptanceSchedule::ConstantOne),
        }
    );
    assert_eq!(result.report.typed_object_count, model.object_count());
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE
            && entry.object_count == 2
            && entry.status == ObjectCoverageStatus::Typed
    }));
    Ok(())
}

#[test]
fn variable_absorptance_materializes_all_function_signals_and_deferred_references()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "A": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "B": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "C": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "D": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}
            },
            "Curve:Linear": {
                "Thermal Curve": {},
                "Solar Curve": {}
            },
            "Table:Lookup": {
                "Solar Lookup": {}
            },
            "MaterialProperty:VariableAbsorptance": {
                "A Default": {
                    "reference_material_name":"A",
                    "thermal_absorptance_function_name":"thermal curve",
                    "solar_absorptance_schedule_name":"unresolved and ignored"
                },
                "B Solar": {
                    "reference_material_name":"B",
                    "control_signal":"SurfaceReceivedSolarRadiation",
                    "thermal_absorptance_function_name":"missing sibling",
                    "solar_absorptance_function_name":"solar lookup"
                },
                "C Mode": {
                    "reference_material_name":"C",
                    "control_signal":"spaceheatingcoolingmode",
                    "thermal_absorptance_function_name":"Thermal Curve",
                    "solar_absorptance_function_name":"SOLAR CURVE"
                },
                "D Blank": {
                    "reference_material_name":"D",
                    "control_signal":"",
                    "thermal_absorptance_function_name":"Thermal Curve"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.material_variable_absorptances.len(), 4);

    let VariableAbsorptanceControl::Function {
        signal,
        thermal,
        solar,
    } = &model.material_variable_absorptances[0].control
    else {
        return Err(std::io::Error::other("expected default function control").into());
    };
    assert_eq!(
        *signal,
        VariableAbsorptanceFunctionSignal::SurfaceTemperature
    );
    assert_eq!(
        thermal.as_ref().map(|value| value.object_type.as_str()),
        Some("Curve:Linear")
    );
    assert_eq!(
        thermal.as_ref().map(|value| &value.name),
        Some(&NormalizedName::new("Thermal Curve"))
    );
    assert!(solar.is_none());

    let VariableAbsorptanceControl::Function {
        signal,
        thermal,
        solar,
    } = &model.material_variable_absorptances[1].control
    else {
        return Err(std::io::Error::other("expected solar function control").into());
    };
    assert_eq!(
        *signal,
        VariableAbsorptanceFunctionSignal::SurfaceReceivedSolarRadiation
    );
    assert!(thermal.is_none());
    assert_eq!(
        solar.as_ref().map(|value| value.object_type.as_str()),
        Some("Table:Lookup")
    );

    let VariableAbsorptanceControl::Function { signal, .. } =
        &model.material_variable_absorptances[2].control
    else {
        return Err(std::io::Error::other("expected mode function control").into());
    };
    assert_eq!(
        *signal,
        VariableAbsorptanceFunctionSignal::SpaceHeatingCoolingMode
    );
    let VariableAbsorptanceControl::Function { signal, .. } =
        &model.material_variable_absorptances[3].control
    else {
        return Err(std::io::Error::other("expected blank-default function control").into());
    };
    assert_eq!(
        *signal,
        VariableAbsorptanceFunctionSignal::SurfaceTemperature
    );
    assert!(result.report.defaults_applied.iter().any(|default| {
        default.object_type == OBJECT_TYPE
            && default.object_name == "A Default"
            && default.field == "control_signal"
            && default.value == "SurfaceTemperature"
    }));
    assert!(result.report.defaults_applied.iter().any(|default| {
        default.object_type == OBJECT_TYPE
            && default.object_name == "D Blank"
            && default.field == "control_signal"
            && default.value == "SurfaceTemperature"
    }));
    Ok(())
}

#[test]
fn variable_absorptance_resolves_every_bounded_curve_and_table_family()
-> Result<(), Box<dyn std::error::Error>> {
    for function_type in VARIABLE_ABSORPTANCE_FUNCTION_OBJECT_TYPES {
        let raw = parse_epjson_str(&format!(
            r#"{{
                "Material": {{"M": {{"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}}}},
                "{function_type}": {{"Function": {{}}}},
                "MaterialProperty:VariableAbsorptance": {{
                    "Overlay": {{
                        "reference_material_name":"M",
                        "thermal_absorptance_function_name":"function"
                    }}
                }}
            }}"#
        ))?;
        let result = compile_raw_model(&raw);
        assert!(
            !result.has_errors(),
            "{function_type}: {:?}",
            result.report.diagnostics
        );
        let model = result
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected typed model"))?;
        let VariableAbsorptanceControl::Function { thermal, .. } =
            &model.material_variable_absorptances[0].control
        else {
            return Err(std::io::Error::other("expected function control").into());
        };
        assert_eq!(
            thermal
                .as_ref()
                .map(|reference| reference.object_type.as_str()),
            Some(*function_type)
        );
    }

    let hidden_alias = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "AirflowNetwork:MultiZone:WindPressureCoefficientValues": {"Function": {}},
            "MaterialProperty:VariableAbsorptance": {
                "Overlay": {
                    "reference_material_name":"M",
                    "thermal_absorptance_function_name":"Function"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&hidden_alias);
    assert!(has_error(
        &result,
        "MissingVariableAbsorptanceFunction",
        "Overlay",
        Some("control_signal")
    ));
    Ok(())
}

#[test]
fn variable_absorptance_accepts_only_regular_and_nomass_targets()
-> Result<(), Box<dyn std::error::Error>> {
    for (target_type, target_name, target_body) in [
        ("Material:AirGap", "Gap", r#"{"thermal_resistance":0.2}"#),
        ("Material:InfraredTransparent", "IRT", "{}"),
        (
            "WindowMaterial:Gas",
            "Gas",
            r#"{"gas_type":"Air","thickness":0.012}"#,
        ),
    ] {
        let raw = parse_epjson_str(&format!(
            r#"{{
                "{target_type}": {{"{target_name}": {target_body}}},
                "MaterialProperty:VariableAbsorptance": {{
                    "Overlay": {{
                        "reference_material_name":"{target_name}",
                        "control_signal":"Scheduled",
                        "thermal_absorptance_schedule_name":"Constant-1.0"
                    }}
                }}
            }}"#
        ))?;
        let result = compile_raw_model(&raw);
        assert!(has_error(
            &result,
            "InvalidVariableAbsorptanceMaterialType",
            "Overlay",
            Some("reference_material_name")
        ));
    }
    Ok(())
}

#[test]
fn variable_absorptance_enforces_selected_dependency_before_cross_family_conflict()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "A": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "B": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "C": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "D": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}
            },
            "Schedule:Constant": {"Schedule": {"hourly_value":0.5}},
            "Curve:Linear": {"Curve": {}},
            "MaterialProperty:VariableAbsorptance": {
                "A Missing Selected": {
                    "reference_material_name":"A",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_function_name":"Curve"
                },
                "B Opposite Function": {
                    "reference_material_name":"B",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Schedule",
                    "thermal_absorptance_function_name":"Curve"
                },
                "C Missing Selected": {
                    "reference_material_name":"C",
                    "control_signal":"SurfaceTemperature",
                    "thermal_absorptance_schedule_name":"Schedule"
                },
                "D Opposite Schedule": {
                    "reference_material_name":"D",
                    "control_signal":"SurfaceTemperature",
                    "thermal_absorptance_function_name":"Curve",
                    "thermal_absorptance_schedule_name":"Schedule"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(has_error(
        &result,
        "MissingVariableAbsorptanceSchedule",
        "A Missing Selected",
        Some("control_signal")
    ));
    assert!(!has_error(
        &result,
        "UnexpectedVariableAbsorptanceFunction",
        "A Missing Selected",
        Some("control_signal")
    ));
    assert!(has_error(
        &result,
        "UnexpectedVariableAbsorptanceFunction",
        "B Opposite Function",
        Some("control_signal")
    ));
    assert!(has_error(
        &result,
        "MissingVariableAbsorptanceFunction",
        "C Missing Selected",
        Some("control_signal")
    ));
    assert!(!has_error(
        &result,
        "UnexpectedVariableAbsorptanceSchedule",
        "C Missing Selected",
        Some("control_signal")
    ));
    assert!(has_error(
        &result,
        "UnexpectedVariableAbsorptanceSchedule",
        "D Opposite Schedule",
        Some("control_signal")
    ));
    Ok(())
}

#[test]
fn variable_absorptance_fails_closed_on_missing_ambiguous_and_duplicate_references()
-> Result<(), Box<dyn std::error::Error>> {
    let missing_target = parse_epjson_str(
        r#"{
            "MaterialProperty:VariableAbsorptance": {
                "Missing": {
                    "reference_material_name":"Absent",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Constant-1.0"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&missing_target);
    assert!(has_error(
        &result,
        "MissingReference",
        "Missing",
        Some("reference_material_name")
    ));

    let ambiguous_curve = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "Curve:Linear": {"Shared": {}},
            "Table:Lookup": {"shared": {}},
            "MaterialProperty:VariableAbsorptance": {
                "Overlay": {
                    "reference_material_name":"M",
                    "thermal_absorptance_function_name":"SHARED"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&ambiguous_curve);
    assert!(has_error(
        &result,
        "AmbiguousVariableAbsorptanceFunction",
        "Overlay",
        Some("thermal_absorptance_function_name")
    ));

    let built_in_collision = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "Schedule:Constant": {"Constant-0.0": {"hourly_value":0}},
            "MaterialProperty:VariableAbsorptance": {
                "Overlay": {
                    "reference_material_name":"M",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"constant-0.0"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&built_in_collision);
    assert!(has_error(
        &result,
        "AmbiguousVariableAbsorptanceSchedule",
        "Overlay",
        Some("thermal_absorptance_schedule_name")
    ));

    let duplicate_target = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:VariableAbsorptance": {
                "A": {"reference_material_name":"M","control_signal":"Scheduled","thermal_absorptance_schedule_name":"Constant-0.0"},
                "B": {"reference_material_name":"M","control_signal":"Scheduled","solar_absorptance_schedule_name":"Constant-1.0"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&duplicate_target);
    assert!(has_error(
        &result,
        "DuplicateVariableAbsorptanceMaterial",
        "B",
        Some("reference_material_name")
    ));

    let duplicate_name = parse_epjson_str(
        r#"{
            "Material": {
                "A": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "B": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}
            },
            "MaterialProperty:VariableAbsorptance": {
                "Same": {"reference_material_name":"A","control_signal":"Scheduled","thermal_absorptance_schedule_name":"Constant-0.0"},
                "same": {"reference_material_name":"B","control_signal":"Scheduled","solar_absorptance_schedule_name":"Constant-1.0"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&duplicate_name);
    assert!(has_error(&result, "DuplicateName", "same", None));
    Ok(())
}

#[test]
fn invalid_variable_absorptance_does_not_reserve_name_or_target()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:VariableAbsorptance": {
                "A": {"reference_material_name":"M","control_signal":"Scheduled"},
                "a": {"reference_material_name":"M","control_signal":"Scheduled","thermal_absorptance_schedule_name":"Constant-1.0"}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_material_variable_absorptances(&mut model);
    assert_eq!(model.material_variable_absorptances.len(), 1);
    assert_eq!(
        model.material_variable_absorptances[0].id,
        MaterialVariableAbsorptanceId(0)
    );
    assert_eq!(
        model.material_variable_absorptance_names.resolve("A"),
        Some(MaterialVariableAbsorptanceId(0))
    );
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingVariableAbsorptanceSchedule"
            && diagnostic.object_name.as_deref() == Some("A")
    }));
    assert!(compiler.diagnostics.iter().all(|diagnostic| {
        diagnostic.code != "DuplicateName"
            && diagnostic.code != "DuplicateVariableAbsorptanceMaterial"
    }));
    Ok(())
}

#[test]
fn variable_absorptance_rejects_invalid_enum_and_malformed_string_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "A": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "B": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}
            },
            "MaterialProperty:VariableAbsorptance": {
                "Bad Enum": {
                    "reference_material_name":"A",
                    "control_signal":"NotAControl",
                    "thermal_absorptance_function_name":"Missing"
                },
                "Bad Types": {
                    "reference_material_name":"B",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_function_name":1,
                    "thermal_absorptance_schedule_name":true,
                    "solar_absorptance_function_name":{},
                    "solar_absorptance_schedule_name":[]
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(has_error(
        &result,
        "InvalidEnumValue",
        "Bad Enum",
        Some("control_signal")
    ));
    for field in [
        "thermal_absorptance_function_name",
        "thermal_absorptance_schedule_name",
        "solar_absorptance_function_name",
        "solar_absorptance_schedule_name",
    ] {
        assert!(has_error(
            &result,
            "InvalidFieldType",
            "Bad Types",
            Some(field)
        ));
    }
    assert!(result.model.is_none());

    let blank_name = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:VariableAbsorptance": {
                "   ": {
                    "reference_material_name":"M",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Constant-1.0"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&blank_name);
    assert!(has_error(
        &result,
        "MissingRequiredField",
        "   ",
        Some("name")
    ));
    Ok(())
}
