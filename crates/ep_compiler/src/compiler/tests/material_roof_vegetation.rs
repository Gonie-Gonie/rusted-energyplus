use std::collections::BTreeSet;

use super::super::{
    CompileResult, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    ConstructionKind, MaterialFamily, MaterialKind, MaterialSurfaceRoughness,
    RoofVegetationMoistureDiffusionMethod,
};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "Material:RoofVegetation";

fn compile_roof_vegetation(
    object_name: &str,
    overrides: &[(&str, &str)],
) -> Result<CompileResult, Box<dyn std::error::Error>> {
    let fields = overrides
        .iter()
        .map(|(field, value)| format!(r#""{field}":{value}"#))
        .collect::<Vec<_>>()
        .join(",");
    let epjson = format!(r#"{{"{OBJECT_TYPE}":{{"{object_name}":{{{fields}}}}}}}"#);
    Ok(compile_raw_model(&parse_epjson_str(&epjson)?))
}

fn has_diagnostic(
    result: &CompileResult,
    severity: DiagnosticSeverity,
    code: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == severity
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && field.is_none_or(|field| diagnostic.field.as_deref() == Some(field))
    })
}

#[test]
fn roof_vegetation_materializes_defaults_source_state_and_opaque_projections()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Earlier Regular": {
                    "roughness": "Rough",
                    "thickness": 0.01,
                    "conductivity": 0.1,
                    "density": 500.0,
                    "specific_heat": 1000.0
                }
            },
            "WindowMaterial:Blind:EquivalentLayer": {
                "ZZZ Earlier Equivalent Blind": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.2
                }
            },
            "Material:RoofVegetation": {
                "Z Explicit Vegetation": {
                    "height_of_plants": 0.45,
                    "leaf_area_index": 4.5,
                    "leaf_reflectivity": 0.23,
                    "leaf_emissivity": 0.92,
                    "minimum_stomatal_resistance": 235.0,
                    "soil_layer_name": "Ignored Explicit Soil Label",
                    "roughness": "VerySmooth",
                    "thickness": 0.23,
                    "conductivity_of_dry_soil": 0.45,
                    "density_of_dry_soil": 988.0,
                    "specific_heat_of_dry_soil": 1346.0,
                    "thermal_absorptance": 0.93,
                    "solar_absorptance": 0.81,
                    "visible_absorptance": 0.83,
                    "saturation_volumetric_moisture_content_of_the_soil_layer": 0.45,
                    "residual_volumetric_moisture_content_of_the_soil_layer": 0.08,
                    "initial_volumetric_moisture_content_of_the_soil_layer": 0.34,
                    "moisture_diffusion_calculation_method": "Simple"
                },
                "A Default Vegetation": {}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed roof vegetation materials"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "EARLIER REGULAR",
            "ZZZ EARLIER EQUIVALENT BLIND",
            "A DEFAULT VEGETATION",
            "Z EXPLICIT VEGETATION"
        ]
    );

    assert_eq!(
        model.materials[1].kind(),
        MaterialKind::WindowBlindEquivalentLayer
    );
    let defaulted = &model.materials[2];
    assert_eq!(defaulted.kind(), MaterialKind::RoofVegetation);
    assert_eq!(defaulted.family(), MaterialFamily::Opaque);
    assert_eq!(
        defaulted.roughness(),
        Some(MaterialSurfaceRoughness::MediumRough)
    );
    assert_eq!(defaulted.thickness_m(), Some(0.1));
    assert_eq!(defaulted.conductivity_w_per_m_k(), Some(0.35));
    assert_eq!(defaulted.density_kg_per_m3(), Some(1100.0));
    assert_eq!(defaulted.specific_heat_j_per_kg_k(), Some(1200.0));
    assert_eq!(defaulted.is_resistance_only(), Some(false));
    assert_eq!(defaulted.thermal_resistance(), Some(0.1 / 0.35));
    assert_eq!(
        defaulted.heat_capacity_per_area(),
        Some(0.1 * 1100.0 * 1200.0)
    );
    assert_eq!(defaulted.thermal_absorptance(), Some(0.9));
    assert_eq!(defaulted.solar_absorptance(), Some(0.7));
    assert_eq!(defaulted.visible_absorptance(), Some(0.75));
    let defaulted_fields = defaulted
        .as_roof_vegetation()
        .ok_or_else(|| std::io::Error::other("missing defaulted roof vegetation payload"))?;
    assert_eq!(defaulted_fields.height_of_plants_m, 0.2);
    assert_eq!(defaulted_fields.leaf_area_index, 1.0);
    assert_eq!(defaulted_fields.leaf_reflectivity, 0.22);
    assert_eq!(defaulted_fields.leaf_emissivity, 0.95);
    assert_eq!(defaulted_fields.minimum_stomatal_resistance_s_per_m, 180.0);
    assert_eq!(defaulted_fields.saturation_volumetric_moisture_content, 0.3);
    assert_eq!(defaulted_fields.residual_volumetric_moisture_content, 0.01);
    assert_eq!(defaulted_fields.initial_volumetric_moisture_content, 0.1);
    assert_eq!(
        defaulted_fields.moisture_diffusion_method,
        RoofVegetationMoistureDiffusionMethod::Advanced
    );

    let explicit = &model.materials[3];
    let fields = explicit
        .as_roof_vegetation()
        .ok_or_else(|| std::io::Error::other("missing explicit roof vegetation payload"))?;
    assert_eq!(fields.height_of_plants_m, 0.45);
    assert_eq!(fields.leaf_area_index, 4.5);
    assert_eq!(fields.leaf_reflectivity, 0.23);
    assert_eq!(fields.leaf_emissivity, 0.92);
    assert_eq!(fields.minimum_stomatal_resistance_s_per_m, 235.0);
    assert_eq!(fields.roughness, MaterialSurfaceRoughness::VerySmooth);
    assert_eq!(fields.thickness_m, 0.23);
    assert_eq!(fields.dry_soil_conductivity_w_per_m_k, 0.45);
    assert_eq!(fields.dry_soil_density_kg_per_m3, 988.0);
    assert_eq!(fields.dry_soil_specific_heat_j_per_kg_k, 1346.0);
    assert_eq!(fields.surface.thermal_absorptance, 0.93);
    assert_eq!(fields.surface.solar_absorptance, 0.81);
    assert_eq!(fields.surface.visible_absorptance, 0.83);
    assert_eq!(fields.saturation_volumetric_moisture_content, 0.45);
    assert_eq!(fields.residual_volumetric_moisture_content, 0.08);
    assert_eq!(fields.initial_volumetric_moisture_content, 0.34);
    assert_eq!(
        fields.moisture_diffusion_method,
        RoofVegetationMoistureDiffusionMethod::Simple
    );
    assert_eq!(explicit.thermal_resistance(), Some(0.23 / 0.45));
    assert_eq!(
        explicit.heat_capacity_per_area(),
        Some(0.23 * 988.0 * 1346.0)
    );

    let defaults = result
        .report
        .defaults_applied
        .iter()
        .filter(|default| {
            default.object_type == OBJECT_TYPE && default.object_name == "A Default Vegetation"
        })
        .map(|default| default.field.as_str())
        .collect::<BTreeSet<_>>();
    let expected_defaults = [
        "height_of_plants",
        "leaf_area_index",
        "leaf_reflectivity",
        "leaf_emissivity",
        "minimum_stomatal_resistance",
        "soil_layer_name",
        "roughness",
        "thickness",
        "conductivity_of_dry_soil",
        "density_of_dry_soil",
        "specific_heat_of_dry_soil",
        "thermal_absorptance",
        "solar_absorptance",
        "visible_absorptance",
        "saturation_volumetric_moisture_content_of_the_soil_layer",
        "residual_volumetric_moisture_content_of_the_soil_layer",
        "initial_volumetric_moisture_content_of_the_soil_layer",
        "moisture_diffusion_calculation_method",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(defaults, expected_defaults);
    Ok(())
}

#[test]
fn roof_vegetation_enforces_every_numeric_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let accepted_lower = compile_roof_vegetation(
        "Accepted Lower Endpoints",
        &[
            ("height_of_plants", "0.0051"),
            ("leaf_area_index", "0.0011"),
            ("leaf_reflectivity", "0.05"),
            ("leaf_emissivity", "0.8"),
            ("minimum_stomatal_resistance", "50.0"),
            ("thickness", "0.0501"),
            ("conductivity_of_dry_soil", "0.2"),
            ("density_of_dry_soil", "300.0"),
            ("specific_heat_of_dry_soil", "500.1"),
            ("thermal_absorptance", "0.8001"),
            ("solar_absorptance", "0.4"),
            ("visible_absorptance", "0.5001"),
            (
                "saturation_volumetric_moisture_content_of_the_soil_layer",
                "0.1001",
            ),
            (
                "residual_volumetric_moisture_content_of_the_soil_layer",
                "0.01",
            ),
            (
                "initial_volumetric_moisture_content_of_the_soil_layer",
                "0.0501",
            ),
        ],
    )?;
    assert!(
        !accepted_lower.has_errors(),
        "{:?}",
        accepted_lower.report.diagnostics
    );

    let accepted_upper = compile_roof_vegetation(
        "Accepted Upper Endpoints",
        &[
            ("height_of_plants", "1.0"),
            ("leaf_area_index", "5.0"),
            ("leaf_reflectivity", "0.5"),
            ("leaf_emissivity", "1.0"),
            ("minimum_stomatal_resistance", "300.0"),
            ("thickness", "0.7"),
            ("conductivity_of_dry_soil", "1.5"),
            ("density_of_dry_soil", "2000.0"),
            ("specific_heat_of_dry_soil", "2000.0"),
            ("thermal_absorptance", "1.0"),
            ("solar_absorptance", "0.9"),
            ("visible_absorptance", "1.0"),
            (
                "saturation_volumetric_moisture_content_of_the_soil_layer",
                "0.5",
            ),
            (
                "residual_volumetric_moisture_content_of_the_soil_layer",
                "0.1",
            ),
            (
                "initial_volumetric_moisture_content_of_the_soil_layer",
                "0.5",
            ),
        ],
    )?;
    assert!(
        !accepted_upper.has_errors(),
        "{:?}",
        accepted_upper.report.diagnostics
    );

    let rejected_lower = [
        ("height_of_plants", "0.005"),
        ("leaf_area_index", "0.001"),
        ("leaf_reflectivity", "0.049"),
        ("leaf_emissivity", "0.79"),
        ("minimum_stomatal_resistance", "49.0"),
        ("thickness", "0.05"),
        ("conductivity_of_dry_soil", "0.19"),
        ("density_of_dry_soil", "299.0"),
        ("specific_heat_of_dry_soil", "500.0"),
        ("thermal_absorptance", "0.8"),
        ("solar_absorptance", "0.39"),
        ("visible_absorptance", "0.5"),
        (
            "saturation_volumetric_moisture_content_of_the_soil_layer",
            "0.1",
        ),
        (
            "residual_volumetric_moisture_content_of_the_soil_layer",
            "0.009",
        ),
        (
            "initial_volumetric_moisture_content_of_the_soil_layer",
            "0.05",
        ),
    ];
    for (field, value) in rejected_lower {
        let result = compile_roof_vegetation("Rejected Lower Endpoint", &[(field, value)])?;
        assert!(
            has_diagnostic(
                &result,
                DiagnosticSeverity::Error,
                "InvalidNumericRange",
                "Rejected Lower Endpoint",
                Some(field)
            ),
            "missing lower-bound rejection for {field}={value}: {:?}",
            result.report.diagnostics
        );
    }

    let rejected_upper = [
        ("height_of_plants", "1.01"),
        ("leaf_area_index", "5.01"),
        ("leaf_reflectivity", "0.51"),
        ("leaf_emissivity", "1.01"),
        ("minimum_stomatal_resistance", "301.0"),
        ("thickness", "0.71"),
        ("conductivity_of_dry_soil", "1.51"),
        ("density_of_dry_soil", "2001.0"),
        ("specific_heat_of_dry_soil", "2001.0"),
        ("thermal_absorptance", "1.01"),
        ("solar_absorptance", "0.91"),
        ("visible_absorptance", "1.01"),
        (
            "saturation_volumetric_moisture_content_of_the_soil_layer",
            "0.51",
        ),
        (
            "residual_volumetric_moisture_content_of_the_soil_layer",
            "0.11",
        ),
        (
            "initial_volumetric_moisture_content_of_the_soil_layer",
            "0.51",
        ),
    ];
    for (field, value) in rejected_upper {
        let result = compile_roof_vegetation("Rejected Upper Endpoint", &[(field, value)])?;
        assert!(
            has_diagnostic(
                &result,
                DiagnosticSeverity::Error,
                "InvalidNumericRange",
                "Rejected Upper Endpoint",
                Some(field)
            ),
            "missing upper-bound rejection for {field}={value}: {:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn roof_vegetation_rejects_invalid_enums_and_ignored_soil_label_type()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:RoofVegetation": {
                "Bad Roughness": {"roughness": "Pebbly"},
                "Bad Method": {"moisture_diffusion_calculation_method": "Intermediate"},
                "Bad Ignored Soil Label": {"soil_layer_name": 42}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(result.has_errors());
    for (code, object_name, field) in [
        ("InvalidEnumValue", "Bad Roughness", "roughness"),
        (
            "InvalidEnumValue",
            "Bad Method",
            "moisture_diffusion_calculation_method",
        ),
        (
            "InvalidFieldType",
            "Bad Ignored Soil Label",
            "soil_layer_name",
        ),
    ] {
        assert!(
            has_diagnostic(
                &result,
                DiagnosticSeverity::Error,
                code,
                object_name,
                Some(field)
            ),
            "missing {code} for {object_name}/{field}: {:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn roof_vegetation_clamps_only_initial_moisture_above_saturation()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:RoofVegetation": {
                "A Clamp": {
                    "saturation_volumetric_moisture_content_of_the_soil_layer": 0.2,
                    "residual_volumetric_moisture_content_of_the_soil_layer": 0.1,
                    "initial_volumetric_moisture_content_of_the_soil_layer": 0.3
                },
                "B Initial Below Residual": {
                    "saturation_volumetric_moisture_content_of_the_soil_layer": 0.2,
                    "residual_volumetric_moisture_content_of_the_soil_layer": 0.1,
                    "initial_volumetric_moisture_content_of_the_soil_layer": 0.06
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Warning,
        "RoofVegetationInitialMoistureReset",
        "A Clamp",
        Some("initial_volumetric_moisture_content_of_the_soil_layer")
    ));
    assert!(!result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "RoofVegetationInitialMoistureReset"
            && diagnostic.object_name.as_deref() == Some("B Initial Below Residual")
    }));
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected moisture-recovered model"))?;
    assert_eq!(
        model.materials[0]
            .as_roof_vegetation()
            .map(|material| material.initial_volumetric_moisture_content),
        Some(0.2)
    );
    assert_eq!(
        model.materials[1]
            .as_roof_vegetation()
            .map(|material| material.initial_volumetric_moisture_content),
        Some(0.06)
    );
    Ok(())
}

#[test]
fn roof_vegetation_uses_shared_material_identity() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:RoofVegetation": {"Shared Material": {}},
            "Material": {
                "shared material": {
                    "roughness": "MediumRough",
                    "thickness": 0.1,
                    "conductivity": 0.5,
                    "density": 800.0,
                    "specific_heat": 900.0
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(result.has_errors());
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "DuplicateName",
        "Shared Material",
        None
    ));
    Ok(())
}

#[test]
fn roof_vegetation_construction_is_outside_layer_only_and_fails_closed_on_source_hole()
-> Result<(), Box<dyn std::error::Error>> {
    let accepted_raw = parse_epjson_str(
        r#"{
            "Material": {
                "Inner Soil Support": {
                    "roughness": "MediumRough",
                    "thickness": 0.1,
                    "conductivity": 0.5,
                    "density": 800.0,
                    "specific_heat": 900.0
                }
            },
            "Material:RoofVegetation": {"Eco Outside": {}},
            "Construction": {
                "Accepted Vegetated Roof": {
                    "outside_layer": "Eco Outside",
                    "layer_2": "Inner Soil Support"
                }
            }
        }"#,
    )?;
    let accepted = compile_raw_model(&accepted_raw);
    assert!(!accepted.has_errors(), "{:?}", accepted.report.diagnostics);
    let accepted_model = accepted
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected accepted vegetated construction"))?;
    assert_eq!(accepted_model.constructions.len(), 1);
    assert_eq!(
        accepted_model.constructions[0].kind,
        ConstructionKind::Opaque
    );
    assert_eq!(
        accepted_model.materials[1].kind(),
        MaterialKind::RoofVegetation
    );

    let rejected_raw = parse_epjson_str(
        r#"{
            "Material": {
                "Regular Outside": {
                    "roughness": "MediumRough",
                    "thickness": 0.1,
                    "conductivity": 0.5,
                    "density": 800.0,
                    "specific_heat": 900.0
                }
            },
            "Material:RoofVegetation": {"Eco Layer": {}},
            "Construction": {
                "Interior Only Source Hole": {
                    "outside_layer": "Regular Outside",
                    "layer_2": "Eco Layer"
                },
                "Repeated Eco Layer": {
                    "outside_layer": "Eco Layer",
                    "layer_2": "Eco Layer"
                }
            }
        }"#,
    )?;
    let rejected = compile_raw_model(&rejected_raw);
    assert!(rejected.has_errors());
    for construction_name in ["Interior Only Source Hole", "Repeated Eco Layer"] {
        assert!(rejected.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "InvalidRoofVegetationLayerPosition"
                && diagnostic.object_type == "Construction"
                && diagnostic.object_name.as_deref() == Some(construction_name)
                && diagnostic.field.as_deref() == Some("layer_2")
        }));
    }
    Ok(())
}
