use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    AutoOrNumber, MaterialDefinition, MaterialMoisturePenetrationDepthSettingsId, NormalizedName,
    TypedModel,
};
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawModel, RawValue, parse_epjson_str};

const OBJECT_TYPE: &str = "MaterialProperty:MoisturePenetrationDepth:Settings";

fn valid_settings_fields() -> &'static str {
    r#"
        "water_vapor_diffusion_resistance_factor":27.3,
        "moisture_equation_coefficient_a":0.072549,
        "moisture_equation_coefficient_b":0.397173,
        "moisture_equation_coefficient_c":0.007774,
        "moisture_equation_coefficient_d":11.7057,
        "coating_layer_thickness":0,
        "coating_layer_water_vapor_diffusion_resistance_factor":0
    "#
}

fn one_material_model(name: &str, extra_settings_fields: &str) -> String {
    let settings = valid_settings_fields();
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
            "{OBJECT_TYPE}": {{
                "{name}": {{{settings}{extra_settings_fields}}}
            }}
        }}"#
    )
}

fn settings_object_mut<'a>(
    raw: &'a mut RawModel,
    name: &str,
) -> Result<&'a mut ep_raw_model::RawObject, Box<dyn std::error::Error>> {
    raw.objects
        .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
        .and_then(|instances| instances.get_mut(&ObjectName(name.to_string())))
        .ok_or_else(|| std::io::Error::other("missing raw EMPD settings object").into())
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
fn moisture_penetration_depth_materializes_regular_targets_and_existing_attachments()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Auto Wall": {
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                },
                "Explicit Wall": {
                    "roughness":"MediumRough",
                    "thickness":0.2,
                    "conductivity":0.5,
                    "density":800.0,
                    "specific_heat":900.0
                }
            },
            "MaterialProperty:VariableAbsorptance": {
                "Absorptance Overlay": {
                    "reference_material_name":"Auto Wall",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Constant-1.0"
                }
            },
            "MaterialProperty:PhaseChangeHysteresis": {
                "Auto Wall": {
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
            "MaterialProperty:PhaseChange": {"Auto Wall": {}},
            "MaterialProperty:VariableThermalConductivity": {"Auto Wall": {}},
            "MaterialProperty:MoisturePenetrationDepth:Settings": {
                "auto wall": {
                    "water_vapor_diffusion_resistance_factor":0,
                    "moisture_equation_coefficient_a":-1,
                    "moisture_equation_coefficient_b":0,
                    "moisture_equation_coefficient_c":1,
                    "moisture_equation_coefficient_d":2,
                    "deep_layer_penetration_depth":0,
                    "coating_layer_thickness":0,
                    "coating_layer_water_vapor_diffusion_resistance_factor":0
                },
                "explicit wall": {
                    "water_vapor_diffusion_resistance_factor":27.3,
                    "moisture_equation_coefficient_a":0.072549,
                    "moisture_equation_coefficient_b":0.397173,
                    "moisture_equation_coefficient_c":0.007774,
                    "moisture_equation_coefficient_d":11.7057,
                    "surface_layer_penetration_depth":0.004,
                    "deep_layer_penetration_depth":0.003,
                    "coating_layer_thickness":0.0001,
                    "coating_layer_water_vapor_diffusion_resistance_factor":10
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
    assert_eq!(model.material_moisture_penetration_depth_settings.len(), 2);

    let auto_id = model
        .material_names
        .resolve("AUTO WALL")
        .ok_or_else(|| std::io::Error::other("expected auto material"))?;
    let explicit_id = model
        .material_names
        .resolve("EXPLICIT WALL")
        .ok_or_else(|| std::io::Error::other("expected explicit material"))?;
    let auto = model
        .material_moisture_penetration_depth_settings
        .iter()
        .find(|settings| settings.reference_material == auto_id)
        .ok_or_else(|| std::io::Error::other("expected auto EMPD settings"))?;
    let explicit = model
        .material_moisture_penetration_depth_settings
        .iter()
        .find(|settings| settings.reference_material == explicit_id)
        .ok_or_else(|| std::io::Error::other("expected explicit EMPD settings"))?;

    assert_eq!(auto.id, MaterialMoisturePenetrationDepthSettingsId(0));
    assert_eq!(auto.name, NormalizedName::new("auto wall"));
    assert_eq!(auto.water_vapor_diffusion_resistance_factor, 0.0);
    assert_eq!(auto.moisture_equation_coefficient_a, -1.0);
    assert_eq!(auto.moisture_equation_coefficient_b, 0.0);
    assert_eq!(auto.moisture_equation_coefficient_c, 1.0);
    assert_eq!(auto.moisture_equation_coefficient_d, 2.0);
    assert_eq!(
        auto.surface_layer_penetration_depth_m,
        AutoOrNumber::AutoCalculate
    );
    assert_eq!(
        auto.deep_layer_penetration_depth_m,
        AutoOrNumber::Value(0.0)
    );
    assert_eq!(auto.coating_layer_thickness_m, 0.0);
    assert_eq!(
        auto.coating_layer_water_vapor_diffusion_resistance_factor,
        0.0
    );

    assert_eq!(explicit.id, MaterialMoisturePenetrationDepthSettingsId(1));
    assert_eq!(
        explicit.surface_layer_penetration_depth_m,
        AutoOrNumber::Value(0.004)
    );
    assert_eq!(
        explicit.deep_layer_penetration_depth_m,
        AutoOrNumber::Value(0.003)
    );
    assert_eq!(explicit.coating_layer_thickness_m, 0.0001);
    assert_eq!(
        explicit.coating_layer_water_vapor_diffusion_resistance_factor,
        10.0
    );
    assert!(matches!(
        model.materials[auto_id.0 as usize].definition,
        MaterialDefinition::Regular(_)
    ));
    assert!(matches!(
        model.materials[explicit_id.0 as usize].definition,
        MaterialDefinition::Regular(_)
    ));

    assert_eq!(model.material_variable_absorptances.len(), 1);
    assert_eq!(model.material_phase_change_hystereses.len(), 1);
    assert_eq!(model.material_phase_changes.len(), 1);
    assert_eq!(model.material_variable_thermal_conductivities.len(), 1);
    assert_eq!(model.object_count(), 9);
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
fn moisture_penetration_depth_preserves_all_autocalculate_spellings()
-> Result<(), Box<dyn std::error::Error>> {
    let fields = valid_settings_fields();
    let raw = parse_epjson_str(&format!(
        r#"{{
            "Material": {{
                "Absent": {{"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
                "Blank": {{"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
                "Explicit Auto": {{"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}}
            }},
            "{OBJECT_TYPE}": {{
                "Absent": {{{fields}}},
                "Blank": {{{fields},"surface_layer_penetration_depth":"","deep_layer_penetration_depth":""}},
                "Explicit Auto": {{{fields},"surface_layer_penetration_depth":"  autocalculate  ","deep_layer_penetration_depth":"AUTOCALCULATE"}}
            }}
        }}"#
    ))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.material_moisture_penetration_depth_settings.len(), 3);
    for settings in &model.material_moisture_penetration_depth_settings {
        assert_eq!(
            settings.surface_layer_penetration_depth_m,
            AutoOrNumber::AutoCalculate
        );
        assert_eq!(
            settings.deep_layer_penetration_depth_m,
            AutoOrNumber::AutoCalculate
        );
    }
    Ok(())
}

#[test]
fn moisture_penetration_depth_requires_complete_finite_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let required_fields = [
        "water_vapor_diffusion_resistance_factor",
        "moisture_equation_coefficient_a",
        "moisture_equation_coefficient_b",
        "moisture_equation_coefficient_c",
        "moisture_equation_coefficient_d",
        "coating_layer_thickness",
        "coating_layer_water_vapor_diffusion_resistance_factor",
    ];
    for field in required_fields {
        let mut raw = parse_epjson_str(&one_material_model("EMPD Wall", ""))?;
        settings_object_mut(&mut raw, "EMPD Wall")?
            .fields
            .remove(&FieldName(field.to_string()));
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "MissingRequiredField", "EMPD Wall", Some(field)),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for field in required_fields.into_iter().chain([
        "surface_layer_penetration_depth",
        "deep_layer_penetration_depth",
    ]) {
        let mut raw = parse_epjson_str(&one_material_model("EMPD Wall", ""))?;
        settings_object_mut(&mut raw, "EMPD Wall")?.fields.insert(
            FieldName(field.to_string()),
            RawValue::Number("NaN".to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "InvalidNumber", "EMPD Wall", Some(field)),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for field in required_fields.into_iter().chain([
        "surface_layer_penetration_depth",
        "deep_layer_penetration_depth",
    ]) {
        let mut raw = parse_epjson_str(&one_material_model("EMPD Wall", ""))?;
        settings_object_mut(&mut raw, "EMPD Wall")?
            .fields
            .insert(FieldName(field.to_string()), RawValue::Null);
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "InvalidFieldType", "EMPD Wall", Some(field)),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn moisture_penetration_depth_enforces_schema_bounds_without_cross_field_rejection()
-> Result<(), Box<dyn std::error::Error>> {
    for (field, value) in [
        ("water_vapor_diffusion_resistance_factor", -0.1),
        ("surface_layer_penetration_depth", 0.0),
        ("surface_layer_penetration_depth", -0.1),
        ("deep_layer_penetration_depth", -0.1),
        ("coating_layer_thickness", -0.1),
        (
            "coating_layer_water_vapor_diffusion_resistance_factor",
            -0.1,
        ),
    ] {
        let mut raw = parse_epjson_str(&one_material_model("EMPD Wall", ""))?;
        settings_object_mut(&mut raw, "EMPD Wall")?.fields.insert(
            FieldName(field.to_string()),
            RawValue::Number(value.to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "InvalidNumericRange", "EMPD Wall", Some(field)),
            "field={field}, value={value}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for field in [
        "surface_layer_penetration_depth",
        "deep_layer_penetration_depth",
    ] {
        let mut raw = parse_epjson_str(&one_material_model("EMPD Wall", ""))?;
        settings_object_mut(&mut raw, "EMPD Wall")?.fields.insert(
            FieldName(field.to_string()),
            RawValue::String("Autosize".to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(has_error(
            &result,
            "InvalidEnumValue",
            "EMPD Wall",
            Some(field)
        ));
    }

    let raw = parse_epjson_str(&one_material_model(
        "EMPD Wall",
        r#", "surface_layer_penetration_depth":0.02, "deep_layer_penetration_depth":0.01"#,
    ))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    Ok(())
}

#[test]
fn moisture_penetration_depth_accepts_only_public_regular_mass_targets()
-> Result<(), Box<dyn std::error::Error>> {
    for (object_type, name, base_fields) in [
        (
            "Material:NoMass",
            "NoMass",
            r#""roughness":"Rough","thermal_resistance":1"#,
        ),
        ("Material:AirGap", "Gap", r#""thermal_resistance":0.2"#),
        ("Material:InfraredTransparent", "IRT", ""),
        ("Material:RoofVegetation", "Roof", ""),
        (
            "WindowMaterial:Gas",
            "Gas",
            r#""gas_type":"Air","thickness":0.012"#,
        ),
    ] {
        let settings = valid_settings_fields();
        let raw = parse_epjson_str(&format!(
            r#"{{
                "{object_type}": {{"{name}": {{{base_fields}}}}},
                "{OBJECT_TYPE}": {{"{name}": {{{settings}}}}}
            }}"#
        ))?;
        let result = compile_raw_model(&raw);
        assert!(
            has_error(
                &result,
                "InvalidMoisturePenetrationDepthMaterialType",
                name,
                Some("name")
            ),
            "target={object_type}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    let mut missing = parse_epjson_str(&one_material_model("Missing", ""))?;
    missing.objects.remove(&ObjectType("Material".to_string()));
    let result = compile_raw_model(&missing);
    assert!(has_error(
        &result,
        "MissingReference",
        "Missing",
        Some("name")
    ));
    let blank = parse_epjson_str(&one_material_model("   ", ""))?;
    let result = compile_raw_model(&blank);
    assert!(has_error(
        &result,
        "MissingRequiredField",
        "   ",
        Some("name")
    ));
    Ok(())
}

#[test]
fn moisture_penetration_depth_duplicate_target_fails_and_invalid_first_does_not_reserve()
-> Result<(), Box<dyn std::error::Error>> {
    let settings = valid_settings_fields();
    let duplicate = parse_epjson_str(&format!(
        r#"{{
            "Material": {{"M": {{"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}}}},
            "{OBJECT_TYPE}": {{"M": {{{settings}}}, "m": {{{settings}}}}}
        }}"#
    ))?;
    let result = compile_raw_model(&duplicate);
    assert!(has_error(
        &result,
        "DuplicateMoisturePenetrationDepthMaterial",
        "m",
        Some("name")
    ));

    let raw = parse_epjson_str(&format!(
        r#"{{
            "Material": {{"M": {{"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}}}},
            "{OBJECT_TYPE}": {{
                "M": {{{settings},"surface_layer_penetration_depth":0}},
                "m": {{{settings}}}
            }}
        }}"#
    ))?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_material_moisture_penetration_depth_settings(&mut model);
    assert_eq!(model.material_moisture_penetration_depth_settings.len(), 1);
    assert_eq!(
        model.material_moisture_penetration_depth_settings[0].id,
        MaterialMoisturePenetrationDepthSettingsId(0)
    );
    assert_eq!(
        model.material_moisture_penetration_depth_settings[0].name,
        NormalizedName::new("m")
    );
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange" && diagnostic.object_name.as_deref() == Some("M")
    }));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| { diagnostic.code != "DuplicateMoisturePenetrationDepthMaterial" })
    );
    Ok(())
}
