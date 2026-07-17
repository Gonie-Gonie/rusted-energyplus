use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    MaterialDefinition, MaterialHeatAndMoistureTransferSettingsId, NormalizedName, TypedModel,
};
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawModel, RawValue, parse_epjson_str};

const OBJECT_TYPE: &str = "MaterialProperty:HeatAndMoistureTransfer:Settings";

fn one_material_model(instance_name: &str, material_name: &str) -> String {
    format!(
        r#"{{
            "Material": {{
                "{material_name}": {{
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                }}
            }},
            "{OBJECT_TYPE}": {{
                "{instance_name}": {{
                    "material_name":"{material_name}",
                    "porosity":0.75
                }}
            }}
        }}"#
    )
}

fn settings_object_mut<'a>(
    raw: &'a mut RawModel,
    instance_name: &str,
) -> Result<&'a mut ep_raw_model::RawObject, Box<dyn std::error::Error>> {
    raw.objects
        .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
        .and_then(|instances| instances.get_mut(&ObjectName(instance_name.to_string())))
        .ok_or_else(|| std::io::Error::other("missing raw HAMT settings object").into())
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
fn heat_and_moisture_transfer_settings_materialize_targets_defaults_and_existing_attachments()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Default Wall": {
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
                    "reference_material_name":"Default Wall",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Constant-1.0"
                }
            },
            "MaterialProperty:PhaseChangeHysteresis": {
                "Default Wall": {
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
            "MaterialProperty:PhaseChange": {"Default Wall": {}},
            "MaterialProperty:VariableThermalConductivity": {"Default Wall": {}},
            "MaterialProperty:MoisturePenetrationDepth:Settings": {
                "Default Wall": {
                    "water_vapor_diffusion_resistance_factor":27.3,
                    "moisture_equation_coefficient_a":0.072549,
                    "moisture_equation_coefficient_b":0.397173,
                    "moisture_equation_coefficient_c":0.007774,
                    "moisture_equation_coefficient_d":11.7057,
                    "coating_layer_thickness":0,
                    "coating_layer_water_vapor_diffusion_resistance_factor":0
                }
            },
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {
                "A Synthetic Default Key": {
                    "material_name":"default wall",
                    "porosity":0
                },
                "B Synthetic Explicit Key": {
                    "material_name":"EXPLICIT WALL",
                    "porosity":1,
                    "initial_water_content_ratio":5
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
    assert_eq!(model.material_heat_and_moisture_transfer_settings.len(), 2);

    let default_id = model
        .material_names
        .resolve("DEFAULT WALL")
        .ok_or_else(|| std::io::Error::other("expected default material"))?;
    let explicit_id = model
        .material_names
        .resolve("explicit wall")
        .ok_or_else(|| std::io::Error::other("expected explicit material"))?;
    let default = model
        .material_heat_and_moisture_transfer_settings
        .iter()
        .find(|settings| settings.reference_material == default_id)
        .ok_or_else(|| std::io::Error::other("expected default HAMT settings"))?;
    let explicit = model
        .material_heat_and_moisture_transfer_settings
        .iter()
        .find(|settings| settings.reference_material == explicit_id)
        .ok_or_else(|| std::io::Error::other("expected explicit HAMT settings"))?;

    assert_eq!(default.id, MaterialHeatAndMoistureTransferSettingsId(0));
    assert_eq!(default.name, NormalizedName::new("A Synthetic Default Key"));
    assert_eq!(default.porosity, 0.0);
    assert_eq!(default.initial_water_content_ratio, 0.2);
    assert_eq!(explicit.id, MaterialHeatAndMoistureTransferSettingsId(1));
    assert_eq!(
        explicit.name,
        NormalizedName::new("B Synthetic Explicit Key")
    );
    assert_eq!(explicit.porosity, 1.0);
    assert_eq!(explicit.initial_water_content_ratio, 5.0);
    assert!(matches!(
        model.materials[default_id.0 as usize].definition,
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
    assert_eq!(model.material_moisture_penetration_depth_settings.len(), 1);
    assert_eq!(model.object_count(), 10);
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
fn heat_and_moisture_transfer_settings_default_blank_and_instance_key_are_preserved()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Absent": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "Blank": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "Explicit": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}
            },
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {
                "   ": {"material_name":"Absent","porosity":0},
                "Settings": {"material_name":"Blank","porosity":0.5,"initial_water_content_ratio":""},
                "settings": {"material_name":"Explicit","porosity":1,"initial_water_content_ratio":0}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.material_heat_and_moisture_transfer_settings.len(), 3);
    assert_eq!(
        model.material_heat_and_moisture_transfer_settings[0].name,
        NormalizedName::new("   ")
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_settings[0].initial_water_content_ratio,
        0.2
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_settings[1].initial_water_content_ratio,
        0.2
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_settings[1].name,
        model.material_heat_and_moisture_transfer_settings[2].name
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_settings[2].initial_water_content_ratio,
        0.0
    );
    Ok(())
}

#[test]
fn heat_and_moisture_transfer_settings_require_complete_finite_fields()
-> Result<(), Box<dyn std::error::Error>> {
    for field in ["material_name", "porosity"] {
        let mut raw = parse_epjson_str(&one_material_model("Settings", "HAMT Wall"))?;
        settings_object_mut(&mut raw, "Settings")?
            .fields
            .remove(&FieldName(field.to_string()));
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "MissingRequiredField", "Settings", Some(field)),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for field in ["porosity", "initial_water_content_ratio"] {
        for value in ["NaN", "inf"] {
            let mut raw = parse_epjson_str(&one_material_model("Settings", "HAMT Wall"))?;
            settings_object_mut(&mut raw, "Settings")?.fields.insert(
                FieldName(field.to_string()),
                RawValue::Number(value.to_string()),
            );
            let result = compile_raw_model(&raw);
            assert!(
                has_error(&result, "InvalidNumber", "Settings", Some(field)),
                "field={field}, value={value}, diagnostics={:?}",
                result.report.diagnostics
            );
        }
    }

    for field in ["material_name", "porosity", "initial_water_content_ratio"] {
        let mut raw = parse_epjson_str(&one_material_model("Settings", "HAMT Wall"))?;
        settings_object_mut(&mut raw, "Settings")?
            .fields
            .insert(FieldName(field.to_string()), RawValue::Null);
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "InvalidFieldType", "Settings", Some(field)),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn heat_and_moisture_transfer_settings_enforce_bounds_without_cross_field_rejection()
-> Result<(), Box<dyn std::error::Error>> {
    for (field, value) in [
        ("porosity", -0.1),
        ("porosity", 1.1),
        ("initial_water_content_ratio", -0.1),
    ] {
        let mut raw = parse_epjson_str(&one_material_model("Settings", "HAMT Wall"))?;
        settings_object_mut(&mut raw, "Settings")?.fields.insert(
            FieldName(field.to_string()),
            RawValue::Number(value.to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "InvalidNumericRange", "Settings", Some(field)),
            "field={field}, value={value}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for (porosity, initial_water_content_ratio) in [(0.0, 0.0), (1.0, 100.0)] {
        let mut raw = parse_epjson_str(&one_material_model("Settings", "HAMT Wall"))?;
        let settings = settings_object_mut(&mut raw, "Settings")?;
        settings.fields.insert(
            FieldName("porosity".to_string()),
            RawValue::Number(porosity.to_string()),
        );
        settings.fields.insert(
            FieldName("initial_water_content_ratio".to_string()),
            RawValue::Number(initial_water_content_ratio.to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    }
    Ok(())
}

#[test]
fn heat_and_moisture_transfer_settings_accept_only_public_regular_mass_targets()
-> Result<(), Box<dyn std::error::Error>> {
    for (object_type, base_fields) in [
        (
            "Material:NoMass",
            r#""roughness":"Rough","thermal_resistance":1"#,
        ),
        ("Material:AirGap", r#""thermal_resistance":0.2"#),
        ("Material:InfraredTransparent", ""),
        ("Material:RoofVegetation", ""),
        (
            "WindowMaterial:Gas",
            r#""gas_type":"Air","thickness":0.012"#,
        ),
    ] {
        let raw = parse_epjson_str(&format!(
            r#"{{
                "{object_type}": {{"Target": {{{base_fields}}}}},
                "{OBJECT_TYPE}": {{
                    "Settings": {{"material_name":"Target","porosity":0.75}}
                }}
            }}"#
        ))?;
        let result = compile_raw_model(&raw);
        assert!(
            has_error(
                &result,
                "InvalidHeatAndMoistureTransferMaterialType",
                "Settings",
                Some("material_name")
            ),
            "target={object_type}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    let mut missing = parse_epjson_str(&one_material_model("Settings", "Missing"))?;
    missing.objects.remove(&ObjectType("Material".to_string()));
    let result = compile_raw_model(&missing);
    assert!(has_error(
        &result,
        "MissingReference",
        "Settings",
        Some("material_name")
    ));

    let mut blank = parse_epjson_str(&one_material_model("Settings", "HAMT Wall"))?;
    settings_object_mut(&mut blank, "Settings")?.fields.insert(
        FieldName("material_name".to_string()),
        RawValue::String("   ".to_string()),
    );
    let result = compile_raw_model(&blank);
    assert!(has_error(
        &result,
        "MissingRequiredField",
        "Settings",
        Some("material_name")
    ));
    Ok(())
}

#[test]
fn heat_and_moisture_transfer_settings_duplicate_target_fails_and_invalid_first_does_not_reserve()
-> Result<(), Box<dyn std::error::Error>> {
    let duplicate = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {
                "A": {"material_name":"M","porosity":0.25},
                "B": {"material_name":"m","porosity":0.75}
            }
        }"#,
    )?;
    let result = compile_raw_model(&duplicate);
    assert!(has_error(
        &result,
        "DuplicateHeatAndMoistureTransferSettingsMaterial",
        "B",
        Some("material_name")
    ));

    let raw = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {
                "A": {"material_name":"M","porosity":-1},
                "B": {"material_name":"m","porosity":0.75}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_material_heat_and_moisture_transfer_settings(&mut model);
    assert_eq!(model.material_heat_and_moisture_transfer_settings.len(), 1);
    assert_eq!(
        model.material_heat_and_moisture_transfer_settings[0].id,
        MaterialHeatAndMoistureTransferSettingsId(0)
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_settings[0].name,
        NormalizedName::new("B")
    );
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange" && diagnostic.object_name.as_deref() == Some("A")
    }));
    assert!(compiler.diagnostics.iter().all(|diagnostic| {
        diagnostic.code != "DuplicateHeatAndMoistureTransferSettingsMaterial"
    }));
    Ok(())
}
