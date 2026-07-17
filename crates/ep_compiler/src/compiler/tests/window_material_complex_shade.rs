use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    MaterialDefinition, MaterialFamily, MaterialKind, MaterialSurfaceRoughness, TypedModel,
    WindowComplexShadeLayerType,
};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "WindowMaterial:ComplexShade";

fn has_diagnostic(
    result: &CompileResult,
    code: &str,
    object_type: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == object_type
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

fn compile_fields(
    name: &str,
    fields: &[(&str, &str)],
) -> Result<CompileResult, Box<dyn std::error::Error>> {
    let fields = fields
        .iter()
        .map(|(field, value)| format!(r#""{field}":{value}"#))
        .collect::<Vec<_>>()
        .join(",");
    let raw = parse_epjson_str(&format!(r#"{{"{OBJECT_TYPE}":{{"{name}":{{{fields}}}}}}}"#))?;
    Ok(compile_raw_model(&raw))
}

#[test]
fn window_complex_shade_materializes_source_defaults_and_source_order()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Earlier Gas": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:SimpleGlazingSystem": {
                "Earlier Simple": {"u_factor":3.0, "solar_heat_gain_coefficient":0.5}
            },
            "WindowMaterial:Gap": {
                "Earlier Complex Gap": {
                    "thickness":0.010, "gas_or_gas_mixture_":"Earlier Gas"
                }
            },
            "WindowMaterial:ComplexShade": {
                "A Defaults": {},
                "B Explicit Blanks": {
                    "layer_type":"", "thickness":"", "conductivity":"",
                    "ir_transmittance":"", "front_emissivity":"", "back_emissivity":"",
                    "top_opening_multiplier":"", "bottom_opening_multiplier":"",
                    "left_side_opening_multiplier":"", "right_side_opening_multiplier":"",
                    "front_opening_multiplier":"", "slat_width":"", "slat_spacing":"",
                    "slat_thickness":"", "slat_angle":"", "slat_conductivity":"",
                    "slat_curve":""
                }
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
        .ok_or_else(|| std::io::Error::other("expected typed complex shades"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "EARLIER GAS",
            "EARLIER SIMPLE",
            "EARLIER COMPLEX GAP",
            "A DEFAULTS",
            "B EXPLICIT BLANKS",
        ],
        "ComplexShade must be the twenty-second source-order material parser, after Gap"
    );

    for material in &model.materials[3..] {
        assert_eq!(material.kind(), MaterialKind::WindowComplexShade);
        assert_eq!(material.family(), MaterialFamily::ComplexFenestration);
        assert!(material.as_opaque().is_none());
        assert_eq!(material.thickness_m(), None);
        assert_eq!(material.thermal_resistance(), None);

        let shade = material
            .as_window_complex_shade()
            .ok_or_else(|| std::io::Error::other("missing complex-shade payload"))?;
        assert_eq!(shade.roughness, MaterialSurfaceRoughness::Rough);
        assert_eq!(
            shade.layer_type,
            WindowComplexShadeLayerType::OtherShadingType
        );
        assert_eq!(shade.thickness_m, 0.002);
        assert_eq!(shade.conductivity_w_per_m_k, 1.0);
        assert_eq!(shade.infrared_transmittance, 0.0);
        assert_eq!(shade.front_infrared_emissivity, 0.84);
        assert_eq!(shade.back_infrared_emissivity, 0.84);
        assert_eq!(shade.thermal_absorptance, 0.84);
        assert_eq!(shade.front_thermal_absorptance, 0.84);
        assert_eq!(shade.back_thermal_absorptance, 0.84);
        assert_eq!(shade.top_opening_multiplier, 0.0);
        assert_eq!(shade.bottom_opening_multiplier, 0.0);
        assert_eq!(shade.left_side_opening_multiplier, 0.0);
        assert_eq!(shade.right_side_opening_multiplier, 0.0);
        assert_eq!(shade.front_opening_multiplier, 0.05);
        assert_eq!(shade.slat_width_m, 0.016);
        assert_eq!(shade.slat_spacing_m, 0.012);
        assert_eq!(shade.slat_thickness_m, 0.0006);
        assert_eq!(shade.slat_angle_deg, 90.0);
        assert_eq!(shade.slat_conductivity_w_per_m_k, 160.0);
        assert_eq!(shade.slat_curvature_radius_m, 0.0);
        assert_eq!(shade.density_kg_per_m3, 0.0);
        assert_eq!(shade.specific_heat_j_per_kg_k, 0.0);
        assert_eq!(shade.base_thermal_resistance_m2_k_per_w, 0.0);
        assert_eq!(shade.base_nominal_thermal_resistance_m2_k_per_w, 0.0);
        assert_eq!(shade.solar_absorptance, 0.0);
        assert_eq!(shade.visible_absorptance, 0.0);
        assert!(shade.is_resistance_only());
        assert_eq!(shade.nominal_thermal_resistance_m2_k_per_w(), None);
    }
    Ok(())
}

#[test]
fn window_complex_shade_accepts_all_six_layer_types_and_copies_source_effective_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:ComplexShade": {
                "A Other": {"layer_type":"OtherShadingType"},
                "B Horizontal": {"layer_type":"venetianhorizontal"},
                "C Vertical": {"layer_type":"VENETIANVERTICAL"},
                "D Woven": {"layer_type":"Woven"},
                "E Perforated": {"layer_type":"perforated"},
                "F Bsdf": {
                    "layer_type":"bsdf", "thickness":0.003, "conductivity":2.0,
                    "ir_transmittance":0.9, "front_emissivity":0.7,
                    "back_emissivity":0.6, "top_opening_multiplier":0.1,
                    "bottom_opening_multiplier":0.2, "left_side_opening_multiplier":0.3,
                    "right_side_opening_multiplier":0.4, "front_opening_multiplier":0.5,
                    "slat_width":0.020, "slat_spacing":0.015,
                    "slat_thickness":0.001, "slat_angle":-45.0,
                    "slat_conductivity":100.0, "slat_curve":0.010
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected all layer types"))?;
    for (name, expected) in [
        ("A OTHER", WindowComplexShadeLayerType::OtherShadingType),
        (
            "B HORIZONTAL",
            WindowComplexShadeLayerType::VenetianHorizontal,
        ),
        ("C VERTICAL", WindowComplexShadeLayerType::VenetianVertical),
        ("D WOVEN", WindowComplexShadeLayerType::Woven),
        ("E PERFORATED", WindowComplexShadeLayerType::Perforated),
        ("F BSDF", WindowComplexShadeLayerType::Bsdf),
    ] {
        let layer_type = model
            .materials
            .iter()
            .find(|material| material.name.0 == name)
            .and_then(ep_model::Material::as_window_complex_shade)
            .map(|shade| shade.layer_type);
        assert_eq!(layer_type, Some(expected), "missing {name}");
    }

    let explicit = model.materials[5]
        .as_window_complex_shade()
        .ok_or_else(|| std::io::Error::other("missing explicit BSDF shade"))?;
    assert_eq!(explicit.roughness, MaterialSurfaceRoughness::Rough);
    assert_eq!(explicit.thickness_m, 0.003);
    assert_eq!(explicit.conductivity_w_per_m_k, 2.0);
    assert_eq!(explicit.infrared_transmittance, 0.9);
    assert_eq!(explicit.front_infrared_emissivity, 0.7);
    assert_eq!(explicit.back_infrared_emissivity, 0.6);
    assert_eq!(explicit.thermal_absorptance, 0.6);
    assert_eq!(explicit.front_thermal_absorptance, 0.7);
    assert_eq!(explicit.back_thermal_absorptance, 0.6);
    assert_eq!(explicit.top_opening_multiplier, 0.1);
    assert_eq!(explicit.bottom_opening_multiplier, 0.2);
    assert_eq!(explicit.left_side_opening_multiplier, 0.3);
    assert_eq!(explicit.right_side_opening_multiplier, 0.4);
    assert_eq!(explicit.front_opening_multiplier, 0.5);
    assert_eq!(explicit.slat_width_m, 0.020);
    assert_eq!(explicit.slat_spacing_m, 0.015);
    assert_eq!(explicit.slat_thickness_m, 0.001);
    assert_eq!(explicit.slat_angle_deg, -45.0);
    assert_eq!(explicit.slat_conductivity_w_per_m_k, 100.0);
    assert_eq!(explicit.slat_curvature_radius_m, 0.010);

    for (name, value, code) in [
        ("Bad Token", r#""Venetian""#, "InvalidEnumValue"),
        ("Bad Type", "3", "InvalidFieldType"),
    ] {
        let invalid = compile_fields(name, &[("layer_type", value)])?;
        assert!(has_diagnostic(
            &invalid,
            code,
            OBJECT_TYPE,
            name,
            Some("layer_type")
        ));
    }
    Ok(())
}

#[test]
fn window_complex_shade_enforces_source_numeric_boundaries()
-> Result<(), Box<dyn std::error::Error>> {
    for (name, fields) in [
        (
            "Lower Endpoints",
            vec![
                ("ir_transmittance", "0.0"),
                ("top_opening_multiplier", "0.0"),
                ("bottom_opening_multiplier", "0.0"),
                ("left_side_opening_multiplier", "0.0"),
                ("right_side_opening_multiplier", "0.0"),
                ("front_opening_multiplier", "0.0"),
                ("slat_angle", "-90.0"),
                ("slat_curve", "0.0"),
            ],
        ),
        (
            "Upper Endpoints",
            vec![
                ("ir_transmittance", "1.0"),
                ("front_emissivity", "1.0"),
                ("back_emissivity", "1.0"),
                ("top_opening_multiplier", "1.0"),
                ("bottom_opening_multiplier", "1.0"),
                ("left_side_opening_multiplier", "1.0"),
                ("right_side_opening_multiplier", "1.0"),
                ("front_opening_multiplier", "1.0"),
                ("slat_angle", "90.0"),
            ],
        ),
    ] {
        let result = compile_fields(name, &fields)?;
        assert!(
            !result.has_errors(),
            "{name} rejected: {:?}",
            result.report.diagnostics
        );
    }

    for (name, field, value) in [
        ("Zero Thickness", "thickness", "0.0"),
        ("Zero Conductivity", "conductivity", "0.0"),
        ("IR Below", "ir_transmittance", "-0.0001"),
        ("IR Above", "ir_transmittance", "1.0001"),
        ("Zero Front Emissivity", "front_emissivity", "0.0"),
        ("Front Emissivity Above", "front_emissivity", "1.0001"),
        ("Zero Back Emissivity", "back_emissivity", "0.0"),
        ("Back Emissivity Above", "back_emissivity", "1.0001"),
        ("Top Below", "top_opening_multiplier", "-0.1"),
        ("Bottom Above", "bottom_opening_multiplier", "1.1"),
        ("Left Below", "left_side_opening_multiplier", "-0.1"),
        ("Right Above", "right_side_opening_multiplier", "1.1"),
        ("Front Above", "front_opening_multiplier", "1.1"),
        ("Zero Slat Width", "slat_width", "0.0"),
        ("Zero Slat Spacing", "slat_spacing", "0.0"),
        ("Zero Slat Thickness", "slat_thickness", "0.0"),
        ("Angle Below", "slat_angle", "-90.0001"),
        ("Angle Above", "slat_angle", "90.0001"),
        ("Zero Slat Conductivity", "slat_conductivity", "0.0"),
        ("Negative Slat Curve", "slat_curve", "-0.0001"),
    ] {
        let result = compile_fields(name, &[(field, value)])?;
        assert!(
            has_diagnostic(
                &result,
                "InvalidNumericRange",
                OBJECT_TYPE,
                name,
                Some(field)
            ),
            "missing range diagnostic for {name}: {:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn window_complex_shade_applies_curve_relation_only_to_venetian_layers()
-> Result<(), Box<dyn std::error::Error>> {
    for (name, layer_type, curve) in [
        ("Horizontal Flat", "VenetianHorizontal", "0.0"),
        ("Horizontal Exact Half", "VenetianHorizontal", "0.008"),
        ("Vertical Exact Half", "VenetianVertical", "0.008"),
    ] {
        let result = compile_fields(
            name,
            &[
                ("layer_type", &format!(r#""{layer_type}""#)),
                ("slat_width", "0.016"),
                ("slat_curve", curve),
            ],
        )?;
        assert!(
            !result.has_errors(),
            "{name} rejected: {:?}",
            result.report.diagnostics
        );
    }

    for layer_type in ["VenetianHorizontal", "VenetianVertical"] {
        let name = format!("{layer_type} Below Half");
        let layer_value = format!(r#""{layer_type}""#);
        let result = compile_fields(
            &name,
            &[
                ("layer_type", layer_value.as_str()),
                ("slat_width", "0.016"),
                ("slat_curve", "0.007999999"),
            ],
        )?;
        assert!(has_diagnostic(
            &result,
            "InvalidWindowComplexShadeSlatCurve",
            OBJECT_TYPE,
            &name,
            Some("slat_curve")
        ));
    }

    let non_venetian = compile_fields(
        "No Invented Relations",
        &[
            ("layer_type", r#""OtherShadingType""#),
            ("ir_transmittance", "0.9"),
            ("front_emissivity", "0.9"),
            ("back_emissivity", "0.9"),
            ("top_opening_multiplier", "0.8"),
            ("bottom_opening_multiplier", "0.8"),
            ("left_side_opening_multiplier", "0.8"),
            ("right_side_opening_multiplier", "0.8"),
            ("front_opening_multiplier", "0.8"),
            ("slat_width", "0.016"),
            ("slat_spacing", "0.001"),
            ("slat_thickness", "0.020"),
            ("slat_curve", "0.001"),
        ],
    )?;
    assert!(
        !non_venetian.has_errors(),
        "non-Venetian layers must not gain optical-sum, opening-sum, or slat-geometry relations: {:?}",
        non_venetian.report.diagnostics
    );
    Ok(())
}

#[test]
fn window_complex_shade_shares_identity_and_reserves_it_after_validation()
-> Result<(), Box<dyn std::error::Error>> {
    let collision_raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Shared": {"roughness":"Rough", "thermal_resistance":1.0}
            },
            "WindowMaterial:ComplexShade": {"shared": {}}
        }"#,
    )?;
    let collision = compile_raw_model(&collision_raw);
    assert!(has_diagnostic(
        &collision,
        "DuplicateName",
        OBJECT_TYPE,
        "shared",
        None
    ));

    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:ComplexShade": {
                "A Shared": {
                    "layer_type":"VenetianHorizontal",
                    "slat_width":0.016, "slat_curve":0.001
                },
                "a shared": {
                    "layer_type":"VenetianHorizontal",
                    "slat_width":0.016, "slat_curve":0.008
                }
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);

    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidWindowComplexShadeSlatCurve"
            && diagnostic.object_name.as_deref() == Some("A Shared")
    }));
    assert!(!compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName" && diagnostic.object_type == OBJECT_TYPE
    }));
    assert_eq!(
        model
            .materials
            .iter()
            .filter(|material| matches!(
                material.definition,
                MaterialDefinition::WindowComplexShade(_)
            ))
            .count(),
        1
    );
    assert_eq!(model.materials[0].name.0, "A SHARED");
    assert_eq!(
        model.materials[0]
            .as_window_complex_shade()
            .map(|shade| shade.slat_curvature_radius_m),
        Some(0.008)
    );

    for (suffix, invalid_field, expected_code) in [
        ("Enum", r#""layer_type":"Venetian""#, "InvalidEnumValue"),
        ("Type", r#""layer_type":3"#, "InvalidFieldType"),
        ("Range", r#""thickness":0.0"#, "InvalidNumericRange"),
    ] {
        let first_name = format!("A {suffix} Shared");
        let second_name = format!("a {suffix} shared");
        let raw = parse_epjson_str(&format!(
            r#"{{"{OBJECT_TYPE}":{{"{first_name}":{{{invalid_field}}},"{second_name}":{{}}}}}}"#
        ))?;
        let mut compiler = Compiler::new(&raw, None);
        let mut model = TypedModel::default();
        compiler.parse_materials(&mut model);

        assert!(compiler.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == expected_code
                && diagnostic.object_type == OBJECT_TYPE
                && diagnostic.object_name.as_deref() == Some(first_name.as_str())
        }));
        assert!(!compiler.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "DuplicateName" && diagnostic.object_type == OBJECT_TYPE
        }));
        assert_eq!(model.materials.len(), 1);
        assert_eq!(model.materials[0].name.0, second_name.to_ascii_uppercase());
        assert_eq!(
            model.materials[0]
                .as_window_complex_shade()
                .map(|shade| shade.thickness_m),
            Some(0.002)
        );
    }
    Ok(())
}

#[test]
fn ordinary_construction_rejects_complex_shade_in_every_plausible_position()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage", "thickness":0.004}
            },
            "WindowMaterial:Gas": {
                "Air": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:ComplexShade": {"Complex Shade": {}},
            "Construction": {
                "A Sole": {"outside_layer":"Complex Shade"},
                "B Exterior": {"outside_layer":"Complex Shade", "layer_2":"Glass"},
                "C Middle": {
                    "outside_layer":"Glass", "layer_2":"Complex Shade", "layer_3":"Glass"
                },
                "D Interior": {
                    "outside_layer":"Glass", "layer_2":"Air", "layer_3":"Complex Shade"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    for (name, field) in [
        ("A Sole", "outside_layer"),
        ("B Exterior", "outside_layer"),
        ("C Middle", "layer_2"),
        ("D Interior", "layer_3"),
    ] {
        assert!(has_diagnostic(
            &result,
            "UnsupportedComplexFenestrationConstruction",
            "Construction",
            name,
            Some(field)
        ));
    }
    assert_eq!(
        result
            .report
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code == "UnsupportedComplexFenestrationConstruction"
                    && diagnostic.message.contains("WindowMaterial:Gap")
                    && diagnostic.message.contains("WindowMaterial:ComplexShade")
                    && diagnostic
                        .message
                        .contains("Construction:ComplexFenestrationState")
            })
            .count(),
        4
    );
    Ok(())
}
