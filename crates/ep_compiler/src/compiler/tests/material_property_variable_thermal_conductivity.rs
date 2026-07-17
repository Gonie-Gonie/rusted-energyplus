use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    MaterialDefinition, MaterialVariableThermalConductivityId, NormalizedName, TypedModel,
};
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue, parse_epjson_str};

const OBJECT_TYPE: &str = "MaterialProperty:VariableThermalConductivity";

fn temperature_conductivity_values(count: usize) -> String {
    (0..count)
        .map(|index| {
            let temperature = index as i64 - 50;
            let conductivity = index as i64 - 25;
            format!(r#"{{"temperature":{temperature},"thermal_conductivity":{conductivity}}}"#)
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn one_material_model(name: &str, attachment_fields: &str) -> String {
    format!(
        r#"{{
            "Material": {{
                "{name}": {{
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                }}
            }},
            "{OBJECT_TYPE}": {{"{name}": {{{attachment_fields}}}}}
        }}"#
    )
}

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
fn variable_thermal_conductivity_materializes_regular_nomass_and_existing_attachments()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Variable K Mass": {
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                }
            },
            "Material:NoMass": {
                "Variable K NoMass": {
                    "roughness":"MediumRough",
                    "thermal_resistance":2.0
                }
            },
            "MaterialProperty:VariableAbsorptance": {
                "Absorptance Overlay": {
                    "reference_material_name":"Variable K Mass",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Constant-1.0"
                }
            },
            "MaterialProperty:PhaseChangeHysteresis": {
                "Variable K Mass": {
                    "latent_heat_during_the_entire_phase_change_process":10000,
                    "liquid_state_thermal_conductivity":1.5,
                    "liquid_state_density":2200,
                    "liquid_state_specific_heat":2000,
                    "high_temperature_difference_of_melting_curve":1,
                    "peak_melting_temperature":20,
                    "low_temperature_difference_of_melting_curve":1,
                    "solid_state_thermal_conductivity":1.8,
                    "solid_state_density":2300,
                    "solid_state_specific_heat":1800,
                    "high_temperature_difference_of_freezing_curve":1,
                    "peak_freezing_temperature":23,
                    "low_temperature_difference_of_freezing_curve":1
                }
            },
            "MaterialProperty:PhaseChange": {
                "Variable K Mass": {
                    "temperature_coefficient_for_thermal_conductivity":-0.1,
                    "values":[
                        {"temperature":-20,"enthalpy":-100},
                        {"temperature":0,"enthalpy":-100},
                        {"temperature":20,"enthalpy":50000}
                    ]
                }
            },
            "MaterialProperty:VariableThermalConductivity": {
                "variable k mass": {
                    "values":[
                        {"temperature":-20,"thermal_conductivity":-1},
                        {"temperature":0,"thermal_conductivity":-1},
                        {"temperature":20,"thermal_conductivity":2.5}
                    ]
                },
                "variable k nomass": {}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.material_variable_thermal_conductivities.len(), 2);

    let mass_id = model
        .material_names
        .resolve("VARIABLE K MASS")
        .ok_or_else(|| std::io::Error::other("expected mass material"))?;
    let no_mass_id = model
        .material_names
        .resolve("VARIABLE K NOMASS")
        .ok_or_else(|| std::io::Error::other("expected no-mass material"))?;
    let mass = model
        .material_variable_thermal_conductivities
        .iter()
        .find(|attachment| attachment.reference_material == mass_id)
        .ok_or_else(|| std::io::Error::other("expected mass attachment"))?;
    let no_mass = model
        .material_variable_thermal_conductivities
        .iter()
        .find(|attachment| attachment.reference_material == no_mass_id)
        .ok_or_else(|| std::io::Error::other("expected no-mass attachment"))?;

    assert_eq!(mass.id, MaterialVariableThermalConductivityId(0));
    assert_eq!(mass.name, NormalizedName::new("variable k mass"));
    assert_eq!(mass.temperature_conductivity_points.len(), 3);
    assert_eq!(mass.temperature_conductivity_points[0].temperature_c, -20.0);
    assert_eq!(
        mass.temperature_conductivity_points[0].thermal_conductivity_w_per_m_k,
        -1.0
    );
    assert_eq!(mass.temperature_conductivity_points[1].temperature_c, 0.0);
    assert_eq!(
        mass.temperature_conductivity_points[1].thermal_conductivity_w_per_m_k,
        -1.0
    );
    assert_eq!(mass.temperature_conductivity_points[2].temperature_c, 20.0);
    assert_eq!(
        mass.temperature_conductivity_points[2].thermal_conductivity_w_per_m_k,
        2.5
    );
    assert!(no_mass.temperature_conductivity_points.is_empty());
    assert!(matches!(
        model.materials[mass_id.0 as usize].definition,
        MaterialDefinition::Regular(_)
    ));
    assert!(matches!(
        model.materials[no_mass_id.0 as usize].definition,
        MaterialDefinition::NoMass(_)
    ));
    assert_eq!(model.material_variable_absorptances.len(), 1);
    assert_eq!(model.material_phase_change_hystereses.len(), 1);
    assert_eq!(model.material_phase_changes.len(), 1);
    assert_eq!(model.object_count(), 8);
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
fn variable_thermal_conductivity_preserves_zero_one_two_three_and_large_tables()
-> Result<(), Box<dyn std::error::Error>> {
    for count in [0, 1, 2, 3, 100, 101, 102] {
        let values = temperature_conductivity_values(count);
        let fields = if count == 0 {
            String::new()
        } else {
            format!(r#""values":[{values}]"#)
        };
        let raw = parse_epjson_str(&one_material_model("Variable K", &fields))?;
        let result = compile_raw_model(&raw);
        assert!(
            !result.has_errors(),
            "count={count}, diagnostics={:?}",
            result.report.diagnostics
        );
        let model = result
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected typed model"))?;
        assert_eq!(
            model.material_variable_thermal_conductivities[0]
                .temperature_conductivity_points
                .len(),
            count
        );
    }

    let raw = parse_epjson_str(&one_material_model("Variable K", r#""values":[]"#))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    Ok(())
}

#[test]
fn variable_thermal_conductivity_rejects_malformed_and_nonfinite_values()
-> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        (r#""values":null"#, "InvalidFieldType", Some("values")),
        (r#""values":3"#, "InvalidFieldType", Some("values")),
        (r#""values":[null]"#, "InvalidFieldType", Some("values")),
        (r#""values":["bad"]"#, "InvalidFieldType", Some("values")),
        (
            r#""values":[{"thermal_conductivity":0}]"#,
            "MissingRequiredField",
            Some("values[0].temperature"),
        ),
        (
            r#""values":[{"temperature":0}]"#,
            "MissingRequiredField",
            Some("values[0].thermal_conductivity"),
        ),
        (
            r#""values":[{"temperature":"bad","thermal_conductivity":0}]"#,
            "InvalidFieldType",
            Some("values[0].temperature"),
        ),
        (
            r#""values":[{"temperature":0,"thermal_conductivity":"bad"}]"#,
            "InvalidFieldType",
            Some("values[0].thermal_conductivity"),
        ),
    ];

    for (fields, code, field) in cases {
        let raw = parse_epjson_str(&one_material_model("Variable K", fields))?;
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, code, "Variable K", field),
            "fields={fields}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for field in ["temperature", "thermal_conductivity"] {
        let mut raw = parse_epjson_str(&one_material_model(
            "Variable K",
            r#""values":[{"temperature":0,"thermal_conductivity":0}]"#,
        ))?;
        let object = raw
            .objects
            .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
            .and_then(|instances| instances.get_mut(&ObjectName("Variable K".to_string())))
            .ok_or_else(|| std::io::Error::other("missing raw variable-k test object"))?;
        let points = match object.fields.get_mut(&FieldName("values".to_string())) {
            Some(RawValue::Array(points)) => points,
            _ => return Err(std::io::Error::other("missing raw values array").into()),
        };
        let point = match points.first_mut() {
            Some(RawValue::Object(point)) => point,
            _ => return Err(std::io::Error::other("missing raw values point").into()),
        };
        point.insert(
            FieldName(field.to_string()),
            RawValue::Number("NaN".to_string()),
        );

        let result = compile_raw_model(&raw);
        let diagnostic_field = format!("values[0].{field}");
        assert!(has_error(
            &result,
            "InvalidNumber",
            "Variable K",
            Some(&diagnostic_field)
        ));
    }
    Ok(())
}

#[test]
fn variable_thermal_conductivity_requires_strict_temperatures_only()
-> Result<(), Box<dyn std::error::Error>> {
    for fields in [
        r#""values":[{"temperature":0,"thermal_conductivity":1},{"temperature":0,"thermal_conductivity":2}]"#,
        r#""values":[{"temperature":1,"thermal_conductivity":1},{"temperature":0,"thermal_conductivity":2}]"#,
    ] {
        let raw = parse_epjson_str(&one_material_model("Variable K", fields))?;
        let result = compile_raw_model(&raw);
        assert!(has_error(
            &result,
            "NonIncreasingVariableThermalConductivityTemperature",
            "Variable K",
            Some("values[1].temperature")
        ));
    }

    let raw = parse_epjson_str(&one_material_model(
        "Variable K",
        r#""values":[{"temperature":-3,"thermal_conductivity":2},{"temperature":-2,"thermal_conductivity":2},{"temperature":-1,"thermal_conductivity":-4}]"#,
    ))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    Ok(())
}

#[test]
fn variable_thermal_conductivity_accepts_only_public_regular_group_targets()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:AirGap": {"Gap": {"thermal_resistance":0.2}},
            "Material:InfraredTransparent": {"IRT": {}},
            "Material:RoofVegetation": {"Roof": {}},
            "WindowMaterial:Gas": {"Gas": {"gas_type":"Air","thickness":0.012}},
            "MaterialProperty:VariableThermalConductivity": {
                "Gap": {},
                "IRT": {},
                "Roof": {},
                "Gas": {},
                "Missing": {},
                "   ": {}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    for target in ["Gap", "IRT", "Roof", "Gas"] {
        assert!(has_error(
            &result,
            "InvalidVariableThermalConductivityMaterialType",
            target,
            Some("name")
        ));
    }
    assert!(has_error(
        &result,
        "MissingReference",
        "Missing",
        Some("name")
    ));
    assert!(has_error(
        &result,
        "MissingRequiredField",
        "   ",
        Some("name")
    ));
    Ok(())
}

#[test]
fn variable_thermal_conductivity_duplicate_target_fails_and_invalid_first_does_not_reserve()
-> Result<(), Box<dyn std::error::Error>> {
    let duplicate = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:VariableThermalConductivity": {"M": {}, "m": {}}
        }"#,
    )?;
    let result = compile_raw_model(&duplicate);
    assert!(has_error(
        &result,
        "DuplicateVariableThermalConductivityMaterial",
        "m",
        Some("name")
    ));

    let raw = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:VariableThermalConductivity": {
                "M": {"values":[{"temperature":1,"thermal_conductivity":0},{"temperature":0,"thermal_conductivity":1}]},
                "m": {}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_material_variable_thermal_conductivities(&mut model);
    assert_eq!(model.material_variable_thermal_conductivities.len(), 1);
    assert_eq!(
        model.material_variable_thermal_conductivities[0].id,
        MaterialVariableThermalConductivityId(0)
    );
    assert_eq!(
        model.material_variable_thermal_conductivities[0].name,
        NormalizedName::new("m")
    );
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "NonIncreasingVariableThermalConductivityTemperature"
            && diagnostic.object_name.as_deref() == Some("M")
    }));
    assert!(
        compiler.diagnostics.iter().all(|diagnostic| {
            diagnostic.code != "DuplicateVariableThermalConductivityMaterial"
        })
    );
    Ok(())
}
