use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    MaterialDefinition, MaterialHeatAndMoistureTransferSorptionIsothermId, NormalizedName,
    TypedModel,
};
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawModel, RawValue, parse_epjson_str};

const OBJECT_TYPE: &str = "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm";

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
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {{
                "Settings": {{"material_name":"{material_name}","porosity":0.5}}
            }},
            "{OBJECT_TYPE}": {{
                "{instance_name}": {{
                    "material_name":"{material_name}",
                    "number_of_isotherm_coordinates":1,
                    "relative_humidity_fraction_1":0.5,
                    "moisture_content_1":10
                }}
            }}
        }}"#
    )
}

fn isotherm_object_mut<'a>(
    raw: &'a mut RawModel,
    instance_name: &str,
) -> Result<&'a mut ep_raw_model::RawObject, Box<dyn std::error::Error>> {
    raw.objects
        .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
        .and_then(|instances| instances.get_mut(&ObjectName(instance_name.to_string())))
        .ok_or_else(|| std::io::Error::other("missing raw HAMT sorption object").into())
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
fn sorption_isotherm_materializes_source_effective_points_and_existing_attachments()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "A Wall": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "B Wall": {"roughness":"MediumRough","thickness":0.2,"conductivity":0.5,"density":800,"specific_heat":900}
            },
            "MaterialProperty:VariableAbsorptance": {
                "Absorptance": {
                    "reference_material_name":"A Wall",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Constant-1.0"
                }
            },
            "MaterialProperty:PhaseChangeHysteresis": {
                "A Wall": {
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
            "MaterialProperty:PhaseChange": {"A Wall": {}},
            "MaterialProperty:VariableThermalConductivity": {"A Wall": {}},
            "MaterialProperty:MoisturePenetrationDepth:Settings": {
                "A Wall": {
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
                "A Settings": {"material_name":"A Wall","porosity":0.5},
                "B Settings": {"material_name":"B Wall","porosity":0.8}
            },
            "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm": {
                "A Curve": {
                    "material_name":"a wall",
                    "number_of_isotherm_coordinates":3,
                    "relative_humidity_fraction_1":0.8,
                    "moisture_content_1":300,
                    "relative_humidity_fraction_2":0.2,
                    "moisture_content_2":400,
                    "relative_humidity_fraction_3":0.5,
                    "moisture_content_3":100
                },
                "B Curve": {
                    "material_name":"B WALL",
                    "number_of_isotherm_coordinates":1,
                    "relative_humidity_fraction_1":0.5,
                    "moisture_content_1":10
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
    assert_eq!(
        model
            .material_heat_and_moisture_transfer_sorption_isotherms
            .len(),
        2
    );

    let a = &model.material_heat_and_moisture_transfer_sorption_isotherms[0];
    let b = &model.material_heat_and_moisture_transfer_sorption_isotherms[1];
    assert_eq!(a.id, MaterialHeatAndMoistureTransferSorptionIsothermId(0));
    assert_eq!(a.name, NormalizedName::new("A Curve"));
    assert_eq!(a.reference_settings.0, 0);
    assert_eq!(a.number_of_isotherm_coordinates, 3);
    assert_eq!(a.input_points.len(), 3);
    assert_eq!(a.input_points[0].relative_humidity_fraction, 0.8);
    assert_eq!(a.effective_points.len(), 5);
    assert_eq!(
        a.effective_points
            .iter()
            .map(|point| (
                point.relative_humidity_fraction,
                point.moisture_content_kg_per_m3
            ))
            .collect::<Vec<_>>(),
        vec![
            (0.0, 0.0),
            (0.2, 250.0),
            (0.5, 250.0),
            (0.8, 300.0),
            (1.01, 500.0)
        ]
    );
    assert!(a.moisture_content_was_adjusted);
    assert_eq!(b.reference_settings.0, 1);
    assert_eq!(b.number_of_isotherm_coordinates, 1);
    assert_eq!(
        b.effective_points
            .iter()
            .map(|point| (
                point.relative_humidity_fraction,
                point.moisture_content_kg_per_m3
            ))
            .collect::<Vec<_>>(),
        vec![(0.0, 0.0), (0.5, 10.0), (1.01, 800.0)]
    );
    assert!(!b.moisture_content_was_adjusted);
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Warning
            && diagnostic.code == "HeatAndMoistureTransferSorptionDataAdjusted"
            && diagnostic.object_name.as_deref() == Some("A Curve")
    }));
    assert!(matches!(
        model.materials[a.reference_material.0 as usize].definition,
        MaterialDefinition::Regular(_)
    ));
    assert_eq!(model.material_variable_absorptances.len(), 1);
    assert_eq!(model.material_phase_change_hystereses.len(), 1);
    assert_eq!(model.material_phase_changes.len(), 1);
    assert_eq!(model.material_variable_thermal_conductivities.len(), 1);
    assert_eq!(model.material_moisture_penetration_depth_settings.len(), 1);
    assert_eq!(model.material_heat_and_moisture_transfer_settings.len(), 2);
    assert_eq!(model.object_count(), 12);
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
fn sorption_isotherm_preserves_nonsemantic_keys_count_zero_fill_and_inactive_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "A": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "B": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "C": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}
            },
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {
                "A": {"material_name":"A","porosity":0.5},
                "B": {"material_name":"B","porosity":0.5},
                "C": {"material_name":"C","porosity":0.5}
            },
            "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm": {
                "   ": {
                    "material_name":"A",
                    "number_of_isotherm_coordinates":3,
                    "relative_humidity_fraction_1":0.4,
                    "moisture_content_1":40,
                    "moisture_content_2":20,
                    "relative_humidity_fraction_3":0.9
                },
                "Curve": {
                    "material_name":"B",
                    "number_of_isotherm_coordinates":1,
                    "relative_humidity_fraction_1":0.25,
                    "moisture_content_1":2,
                    "relative_humidity_fraction_25":0.9,
                    "moisture_content_25":999
                },
                "curve": {
                    "material_name":"C",
                    "number_of_isotherm_coordinates":25,
                    "relative_humidity_fraction_1":0.5,
                    "moisture_content_1":5
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
    let curves = &model.material_heat_and_moisture_transfer_sorption_isotherms;
    assert_eq!(curves.len(), 3);
    assert_eq!(curves[0].name, NormalizedName::new("   "));
    assert_eq!(curves[1].name, curves[2].name);
    assert_eq!(curves[0].input_points.len(), 3);
    assert_eq!(curves[0].input_points[1].relative_humidity_fraction, 0.0);
    assert_eq!(curves[0].input_points[1].moisture_content_kg_per_m3, 20.0);
    assert_eq!(curves[0].input_points[2].relative_humidity_fraction, 0.9);
    assert_eq!(curves[0].input_points[2].moisture_content_kg_per_m3, 0.0);
    assert_eq!(curves[1].input_points.len(), 1);
    assert_eq!(curves[1].input_points[0].moisture_content_kg_per_m3, 2.0);
    assert_eq!(curves[2].input_points.len(), 25);
    assert_eq!(curves[2].input_points[24].relative_humidity_fraction, 0.0);
    assert_eq!(curves[2].input_points[24].moisture_content_kg_per_m3, 0.0);
    Ok(())
}

#[test]
fn sorption_isotherm_requires_integer_count_and_schema_bounded_finite_fields()
-> Result<(), Box<dyn std::error::Error>> {
    for field in [
        "material_name",
        "number_of_isotherm_coordinates",
        "relative_humidity_fraction_1",
        "moisture_content_1",
    ] {
        let mut raw = parse_epjson_str(&one_material_model("Curve", "HAMT Wall"))?;
        isotherm_object_mut(&mut raw, "Curve")?
            .fields
            .remove(&FieldName(field.to_string()));
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "MissingRequiredField", "Curve", Some(field)),
            "field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for (value, code) in [
        (0.0, "InvalidNumericRange"),
        (26.0, "InvalidNumericRange"),
        (1.5, "InvalidInteger"),
    ] {
        let mut raw = parse_epjson_str(&one_material_model("Curve", "HAMT Wall"))?;
        isotherm_object_mut(&mut raw, "Curve")?.fields.insert(
            FieldName("number_of_isotherm_coordinates".to_string()),
            RawValue::Number(value.to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(
            has_error(
                &result,
                code,
                "Curve",
                Some("number_of_isotherm_coordinates")
            ),
            "value={value}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for (field, value) in [
        ("relative_humidity_fraction_1", -0.1),
        ("relative_humidity_fraction_1", 1.1),
        ("moisture_content_1", -0.1),
        ("relative_humidity_fraction_25", 1.1),
        ("moisture_content_25", -0.1),
    ] {
        let mut raw = parse_epjson_str(&one_material_model("Curve", "HAMT Wall"))?;
        isotherm_object_mut(&mut raw, "Curve")?.fields.insert(
            FieldName(field.to_string()),
            RawValue::Number(value.to_string()),
        );
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, "InvalidNumericRange", "Curve", Some(field)),
            "field={field}, value={value}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    for field in [
        "number_of_isotherm_coordinates",
        "relative_humidity_fraction_1",
        "moisture_content_1",
        "relative_humidity_fraction_25",
        "moisture_content_25",
    ] {
        for value in ["NaN", "inf"] {
            let mut raw = parse_epjson_str(&one_material_model("Curve", "HAMT Wall"))?;
            isotherm_object_mut(&mut raw, "Curve")?.fields.insert(
                FieldName(field.to_string()),
                RawValue::Number(value.to_string()),
            );
            let result = compile_raw_model(&raw);
            assert!(
                has_error(&result, "InvalidNumber", "Curve", Some(field)),
                "field={field}, value={value}, diagnostics={:?}",
                result.report.diagnostics
            );
        }
    }
    Ok(())
}

#[test]
fn sorption_isotherm_requires_an_existing_hamt_settings_target()
-> Result<(), Box<dyn std::error::Error>> {
    let without_settings = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm": {
                "Curve": {"material_name":"M","number_of_isotherm_coordinates":1,"relative_humidity_fraction_1":0.5,"moisture_content_1":10}
            }
        }"#,
    )?;
    let result = compile_raw_model(&without_settings);
    assert!(has_error(
        &result,
        "MissingHeatAndMoistureTransferSettings",
        "Curve",
        Some("material_name")
    ));

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
                    "Curve": {{"material_name":"Target","number_of_isotherm_coordinates":1,"relative_humidity_fraction_1":0.5,"moisture_content_1":10}}
                }}
            }}"#
        ))?;
        let result = compile_raw_model(&raw);
        assert!(
            has_error(
                &result,
                "MissingHeatAndMoistureTransferSettings",
                "Curve",
                Some("material_name")
            ),
            "target={object_type}, diagnostics={:?}",
            result.report.diagnostics
        );
    }

    let mut missing = parse_epjson_str(&one_material_model("Curve", "Missing"))?;
    missing.objects.remove(&ObjectType("Material".to_string()));
    missing.objects.remove(&ObjectType(
        "MaterialProperty:HeatAndMoistureTransfer:Settings".to_string(),
    ));
    let result = compile_raw_model(&missing);
    assert!(has_error(
        &result,
        "MissingReference",
        "Curve",
        Some("material_name")
    ));

    let mut blank = parse_epjson_str(&one_material_model("Curve", "HAMT Wall"))?;
    isotherm_object_mut(&mut blank, "Curve")?.fields.insert(
        FieldName("material_name".to_string()),
        RawValue::String("   ".to_string()),
    );
    let result = compile_raw_model(&blank);
    assert!(has_error(
        &result,
        "MissingRequiredField",
        "Curve",
        Some("material_name")
    ));
    Ok(())
}

#[test]
fn sorption_isotherm_duplicate_target_fails_and_invalid_first_does_not_reserve()
-> Result<(), Box<dyn std::error::Error>> {
    let duplicate = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {"Settings": {"material_name":"M","porosity":0.5}},
            "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm": {
                "A": {"material_name":"M","number_of_isotherm_coordinates":1,"relative_humidity_fraction_1":0.25,"moisture_content_1":10},
                "B": {"material_name":"m","number_of_isotherm_coordinates":1,"relative_humidity_fraction_1":0.75,"moisture_content_1":20}
            }
        }"#,
    )?;
    let result = compile_raw_model(&duplicate);
    assert!(has_error(
        &result,
        "DuplicateHeatAndMoistureTransferSorptionMaterial",
        "B",
        Some("material_name")
    ));

    let raw = parse_epjson_str(
        r#"{
            "Material": {"M": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}},
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {"Settings": {"material_name":"M","porosity":0.5}},
            "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm": {
                "A": {"material_name":"M","number_of_isotherm_coordinates":1,"relative_humidity_fraction_1":0.25,"moisture_content_1":-1},
                "B": {"material_name":"m","number_of_isotherm_coordinates":1,"relative_humidity_fraction_1":0.75,"moisture_content_1":20}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_material_heat_and_moisture_transfer_settings(&mut model);
    compiler.parse_material_heat_and_moisture_transfer_sorption_isotherms(&mut model);
    assert_eq!(
        model
            .material_heat_and_moisture_transfer_sorption_isotherms
            .len(),
        1
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_sorption_isotherms[0].id,
        MaterialHeatAndMoistureTransferSorptionIsothermId(0)
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_sorption_isotherms[0].name,
        NormalizedName::new("B")
    );
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange" && diagnostic.object_name.as_deref() == Some("A")
    }));
    assert!(compiler.diagnostics.iter().all(|diagnostic| {
        diagnostic.code != "DuplicateHeatAndMoistureTransferSorptionMaterial"
    }));
    Ok(())
}

#[test]
fn sorption_isotherm_preserves_source_hundred_pass_correction_cap()
-> Result<(), Box<dyn std::error::Error>> {
    let mut raw = parse_epjson_str(&one_material_model("Curve", "HAMT Wall"))?;
    let object = isotherm_object_mut(&mut raw, "Curve")?;
    object.fields.insert(
        FieldName("number_of_isotherm_coordinates".to_string()),
        RawValue::Number("25".to_string()),
    );
    for coordinate in 1..=25 {
        object.fields.insert(
            FieldName(format!("relative_humidity_fraction_{coordinate}")),
            RawValue::Number((f64::from(coordinate) / 26.0).to_string()),
        );
        object.fields.insert(
            FieldName(format!("moisture_content_{coordinate}")),
            RawValue::Number(((26 - coordinate) * 100).to_string()),
        );
    }

    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let curve = &result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?
        .material_heat_and_moisture_transfer_sorption_isotherms[0];
    assert_eq!(curve.effective_points.len(), 27);
    assert!(curve.moisture_content_was_adjusted);
    assert_eq!(
        curve
            .effective_points
            .windows(2)
            .filter(|points| {
                points[0].moisture_content_kg_per_m3 > points[1].moisture_content_kg_per_m3
            })
            .count(),
        24
    );
    assert_eq!(
        curve.effective_points[1]
            .moisture_content_kg_per_m3
            .to_bits(),
        1524.7461503893173_f64.to_bits()
    );
    assert_eq!(
        curve.effective_points[2]
            .moisture_content_kg_per_m3
            .to_bits(),
        1517.3329546022942_f64.to_bits()
    );
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Warning
            && diagnostic.code == "HeatAndMoistureTransferSorptionDataAdjusted"
            && diagnostic.message
                == "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm/Curve source-equivalent adjacent moisture averaging was applied"
    }));
    Ok(())
}

#[test]
fn sorption_isotherm_fails_closed_when_source_averaging_overflows()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "A": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "B": {"roughness":"Rough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000}
            },
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {
                "A": {"material_name":"A","porosity":1},
                "B": {"material_name":"B","porosity":1}
            },
            "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm": {
                "A": {
                    "material_name":"A",
                    "number_of_isotherm_coordinates":2,
                    "relative_humidity_fraction_1":0.1,
                    "moisture_content_1":1.7e308,
                    "relative_humidity_fraction_2":0.2,
                    "moisture_content_2":1.6e308
                },
                "B": {"material_name":"B","number_of_isotherm_coordinates":1,"relative_humidity_fraction_1":0.5,"moisture_content_1":10}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_material_heat_and_moisture_transfer_settings(&mut model);
    compiler.parse_material_heat_and_moisture_transfer_sorption_isotherms(&mut model);
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidHeatAndMoistureTransferSorptionDerivedState"
            && diagnostic.object_name.as_deref() == Some("A")
    }));
    assert_eq!(
        model
            .material_heat_and_moisture_transfer_sorption_isotherms
            .len(),
        1
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_sorption_isotherms[0].id,
        MaterialHeatAndMoistureTransferSorptionIsothermId(0)
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_sorption_isotherms[0].name,
        NormalizedName::new("B")
    );
    Ok(())
}
