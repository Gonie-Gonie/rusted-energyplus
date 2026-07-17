use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{MaterialDefinition, MaterialPhaseChangeId, NormalizedName, TypedModel};
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue, parse_epjson_str};

const OBJECT_TYPE: &str = "MaterialProperty:PhaseChange";

fn temperature_enthalpy_values(count: usize) -> String {
    (0..count)
        .map(|index| {
            let temperature = index as i64 - 50;
            let enthalpy = index as i64 * 100;
            format!(r#"{{"temperature":{temperature},"enthalpy":{enthalpy}}}"#)
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn one_material_model(name: &str, phase_change_fields: &str) -> String {
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
            "{OBJECT_TYPE}": {{"{name}": {{{phase_change_fields}}}}}
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
fn phase_change_materializes_regular_nomass_and_existing_attachment_coexistence()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "PCM Mass": {
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                }
            },
            "Material:NoMass": {
                "PCM NoMass": {
                    "roughness":"MediumRough",
                    "thermal_resistance":2.0
                }
            },
            "MaterialProperty:VariableAbsorptance": {
                "Absorptance Overlay": {
                    "reference_material_name":"PCM Mass",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Constant-1.0"
                }
            },
            "MaterialProperty:PhaseChangeHysteresis": {
                "PCM Mass": {
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
                "pcm mass": {
                    "values":[
                        {"temperature":-20,"enthalpy":-100},
                        {"temperature":0,"enthalpy":-100},
                        {"temperature":20,"enthalpy":50000}
                    ]
                },
                "pcm nomass": {
                    "temperature_coefficient_for_thermal_conductivity":-0.25
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
    assert_eq!(model.material_phase_changes.len(), 2);

    let mass_id = model
        .material_names
        .resolve("PCM MASS")
        .ok_or_else(|| std::io::Error::other("expected mass material"))?;
    let no_mass_id = model
        .material_names
        .resolve("PCM NOMASS")
        .ok_or_else(|| std::io::Error::other("expected no-mass material"))?;
    let mass = model
        .material_phase_changes
        .iter()
        .find(|attachment| attachment.reference_material == mass_id)
        .ok_or_else(|| std::io::Error::other("expected mass attachment"))?;
    let no_mass = model
        .material_phase_changes
        .iter()
        .find(|attachment| attachment.reference_material == no_mass_id)
        .ok_or_else(|| std::io::Error::other("expected no-mass attachment"))?;

    assert_eq!(mass.id, MaterialPhaseChangeId(0));
    assert_eq!(mass.name, NormalizedName::new("pcm mass"));
    assert_eq!(
        mass.temperature_coefficient_for_thermal_conductivity_w_per_m_k2,
        0.0
    );
    assert_eq!(mass.temperature_enthalpy_points.len(), 3);
    assert_eq!(mass.temperature_enthalpy_points[0].temperature_c, -20.0);
    assert_eq!(
        mass.temperature_enthalpy_points[0].enthalpy_j_per_kg,
        -100.0
    );
    assert_eq!(mass.temperature_enthalpy_points[1].temperature_c, 0.0);
    assert_eq!(
        mass.temperature_enthalpy_points[1].enthalpy_j_per_kg,
        -100.0
    );
    assert_eq!(mass.temperature_enthalpy_points[2].temperature_c, 20.0);
    assert_eq!(
        mass.temperature_enthalpy_points[2].enthalpy_j_per_kg,
        50000.0
    );
    assert_eq!(
        no_mass.temperature_coefficient_for_thermal_conductivity_w_per_m_k2,
        -0.25
    );
    assert!(no_mass.temperature_enthalpy_points.is_empty());
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
fn phase_change_preserves_zero_one_two_three_and_more_than_one_hundred_complete_pairs()
-> Result<(), Box<dyn std::error::Error>> {
    for count in [0, 1, 2, 3, 101] {
        let values = temperature_enthalpy_values(count);
        let fields = if count == 0 {
            String::new()
        } else {
            format!(r#""values":[{values}]"#)
        };
        let raw = parse_epjson_str(&one_material_model("PCM", &fields))?;
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
        assert_eq!(model.material_phase_changes.len(), 1);
        assert_eq!(
            model.material_phase_changes[0]
                .temperature_enthalpy_points
                .len(),
            count
        );
        assert_eq!(
            model.material_phase_changes[0]
                .temperature_coefficient_for_thermal_conductivity_w_per_m_k2,
            0.0
        );
    }

    let raw = parse_epjson_str(&one_material_model(
        "PCM",
        r#""temperature_coefficient_for_thermal_conductivity":"","values":[]"#,
    ))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        result
            .model
            .as_ref()
            .map(|model| model.material_phase_changes[0]
                .temperature_enthalpy_points
                .len()),
        Some(0)
    );
    Ok(())
}

#[test]
fn phase_change_rejects_malformed_coefficient_and_values() -> Result<(), Box<dyn std::error::Error>>
{
    let cases = [
        (
            r#""temperature_coefficient_for_thermal_conductivity":"bad","values":[]"#,
            "InvalidFieldType",
            Some("temperature_coefficient_for_thermal_conductivity"),
        ),
        (r#""values":3"#, "InvalidFieldType", Some("values")),
        (r#""values":["bad"]"#, "InvalidFieldType", Some("values")),
        (
            r#""values":[{"enthalpy":0}]"#,
            "MissingRequiredField",
            Some("values[0].temperature"),
        ),
        (
            r#""values":[{"temperature":0}]"#,
            "MissingRequiredField",
            Some("values[0].enthalpy"),
        ),
        (
            r#""values":[{"temperature":"bad","enthalpy":0}]"#,
            "InvalidFieldType",
            Some("values[0].temperature"),
        ),
        (
            r#""values":[{"temperature":0,"enthalpy":"bad"}]"#,
            "InvalidFieldType",
            Some("values[0].enthalpy"),
        ),
    ];

    for (fields, code, field) in cases {
        let raw = parse_epjson_str(&one_material_model("PCM", fields))?;
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, code, "PCM", field),
            "fields={fields}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert_eq!(
            result
                .model
                .as_ref()
                .map(|model| model.material_phase_changes.len()),
            None
        );
    }
    Ok(())
}

#[test]
fn phase_change_rejects_nonfinite_coefficient_temperature_and_enthalpy()
-> Result<(), Box<dyn std::error::Error>> {
    for field in [
        "temperature_coefficient_for_thermal_conductivity",
        "temperature",
        "enthalpy",
    ] {
        let mut raw = parse_epjson_str(&one_material_model(
            "PCM",
            r#""temperature_coefficient_for_thermal_conductivity":0,"values":[{"temperature":0,"enthalpy":0}]"#,
        ))?;
        let object = raw
            .objects
            .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
            .and_then(|instances| instances.get_mut(&ObjectName("PCM".to_string())))
            .ok_or_else(|| std::io::Error::other("missing raw phase-change test object"))?;
        if field == "temperature_coefficient_for_thermal_conductivity" {
            object.fields.insert(
                FieldName(field.to_string()),
                RawValue::Number("NaN".to_string()),
            );
        } else {
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
        }

        let result = compile_raw_model(&raw);
        let diagnostic_field = if field == "temperature_coefficient_for_thermal_conductivity" {
            field.to_string()
        } else {
            format!("values[0].{field}")
        };
        assert!(
            has_error(&result, "InvalidNumber", "PCM", Some(&diagnostic_field)),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn phase_change_requires_strict_temperatures_and_nondecreasing_enthalpy()
-> Result<(), Box<dyn std::error::Error>> {
    for fields in [
        r#""values":[{"temperature":0,"enthalpy":0},{"temperature":0,"enthalpy":1}]"#,
        r#""values":[{"temperature":1,"enthalpy":0},{"temperature":0,"enthalpy":1}]"#,
    ] {
        let raw = parse_epjson_str(&one_material_model("PCM", fields))?;
        let result = compile_raw_model(&raw);
        assert!(has_error(
            &result,
            "NonIncreasingPhaseChangeTemperature",
            "PCM",
            Some("values[1].temperature")
        ));
    }

    let raw = parse_epjson_str(&one_material_model(
        "PCM",
        r#""values":[{"temperature":0,"enthalpy":2},{"temperature":1,"enthalpy":1}]"#,
    ))?;
    let result = compile_raw_model(&raw);
    assert!(has_error(
        &result,
        "DecreasingPhaseChangeEnthalpy",
        "PCM",
        Some("values[1].enthalpy")
    ));

    let raw = parse_epjson_str(&one_material_model(
        "PCM",
        r#""temperature_coefficient_for_thermal_conductivity":-9,"values":[{"temperature":-3,"enthalpy":-4},{"temperature":-2,"enthalpy":-4}]"#,
    ))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    Ok(())
}

#[test]
fn phase_change_accepts_only_public_regular_group_targets_and_rejects_blank_key()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:AirGap": {"Gap": {"thermal_resistance":0.2}},
            "Material:InfraredTransparent": {"IRT": {}},
            "WindowMaterial:Gas": {"Gas": {"gas_type":"Air","thickness":0.012}},
            "MaterialProperty:PhaseChange": {
                "Gap": {"values":[]},
                "IRT": {"values":[]},
                "Gas": {"values":[]},
                "Missing": {"values":[]},
                "   ": {"values":[]}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    for target in ["Gap", "IRT", "Gas"] {
        assert!(has_error(
            &result,
            "InvalidPhaseChangeMaterialType",
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
fn phase_change_duplicate_target_fails_and_invalid_first_does_not_reserve()
-> Result<(), Box<dyn std::error::Error>> {
    let duplicate = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:PhaseChange": {"M": {"values":[]}, "m": {"values":[]}}
        }"#,
    )?;
    let result = compile_raw_model(&duplicate);
    assert!(has_error(
        &result,
        "DuplicatePhaseChangeMaterial",
        "m",
        Some("name")
    ));

    let raw = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:PhaseChange": {
                "M": {"values":[{"temperature":1,"enthalpy":0},{"temperature":0,"enthalpy":1}]},
                "m": {"values":[]}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_material_phase_changes(&mut model);
    assert_eq!(model.material_phase_changes.len(), 1);
    assert_eq!(model.material_phase_changes[0].id, MaterialPhaseChangeId(0));
    assert_eq!(
        model.material_phase_changes[0].name,
        NormalizedName::new("m")
    );
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "NonIncreasingPhaseChangeTemperature"
            && diagnostic.object_name.as_deref() == Some("M")
    }));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "DuplicatePhaseChangeMaterial")
    );
    Ok(())
}
