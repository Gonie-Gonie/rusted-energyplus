use super::super::{CompileResult, ObjectCoverageStatus, compile_raw_model};
use ep_model::{
    ConstructionKind, MaterialFamily, MaterialKind, MaterialSurfaceRoughness,
    WindowScreenBeamReflectanceModel, WindowScreenTransmittanceMapResolution,
};
use ep_raw_model::parse_epjson_str;

const SCREEN_REQUIRED_FIELDS: &str = r#"
    "diffuse_solar_reflectance":0.2,
    "diffuse_visible_reflectance":0.3,
    "screen_material_spacing":0.002,
    "screen_material_diameter":0.0005
"#;
const GLASS_FIELDS: &str = r#""optical_data_type":"SpectralAverage","thickness":0.003"#;
const SHADE_FIELDS: &str = r#"
    "solar_transmittance":0.15,
    "solar_reflectance":0.35,
    "visible_transmittance":0.10,
    "visible_reflectance":0.40,
    "infrared_hemispherical_emissivity":0.80,
    "infrared_transmittance":0.10,
    "thickness":0.002,
    "conductivity":0.20
"#;

fn assert_close(actual: f64, expected: f64) {
    let tolerance = f64::EPSILON * expected.abs().max(1.0) * 32.0;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}

fn has_diagnostic(
    result: &CompileResult,
    code: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == code
            && diagnostic.object_name.as_deref() == Some(object_name)
            && field.is_none_or(|field| diagnostic.field.as_deref() == Some(field))
    })
}

#[test]
fn window_screen_materializes_defaults_and_source_derived_properties()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Screen": {{
                "Default Screen": {{
                    "reflected_beam_transmittance_accounting_method":"",
                    {SCREEN_REQUIRED_FIELDS},
                    "angle_of_resolution_for_screen_transmittance_output_map":""
                }},
                "Explicit Screen": {{
                    "reflected_beam_transmittance_accounting_method":"ModelAsDirectBeam",
                    {SCREEN_REQUIRED_FIELDS},
                    "thermal_hemispherical_emissivity":0.8,
                    "conductivity":200.0,
                    "screen_to_glass_distance":0.001,
                    "top_opening_multiplier":0.1,
                    "bottom_opening_multiplier":0.2,
                    "left_side_opening_multiplier":0.3,
                    "right_side_opening_multiplier":0.4,
                    "angle_of_resolution_for_screen_transmittance_output_map":5
                }}
            }}
        }}"#
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed screen model"))?;
    assert_eq!(model.materials.len(), 2);

    let material = model
        .materials
        .iter()
        .find(|material| material.name.0 == "DEFAULT SCREEN")
        .ok_or_else(|| std::io::Error::other("missing default screen"))?;
    assert_eq!(material.kind(), MaterialKind::WindowScreen);
    assert_eq!(material.family(), MaterialFamily::Fenestration);
    assert!(material.as_opaque().is_none());
    assert_eq!(material.roughness(), None);
    assert_eq!(material.thickness_m(), None);
    assert_eq!(material.thermal_resistance(), None);
    let screen = material
        .as_window_screen()
        .ok_or_else(|| std::io::Error::other("expected screen payload"))?;
    assert_eq!(screen.roughness, MaterialSurfaceRoughness::MediumRough);
    assert_eq!(
        screen.beam_reflectance_model,
        WindowScreenBeamReflectanceModel::ModelAsDiffuse
    );
    assert_eq!(screen.diffuse_solar_reflectance_input, 0.2);
    assert_eq!(screen.diffuse_visible_reflectance_input, 0.3);
    assert_eq!(screen.thermal_hemispherical_emissivity_input, 0.9);
    assert_eq!(screen.conductivity_w_per_m_k, 221.0);
    assert_eq!(screen.screen_material_spacing_m, 0.002);
    assert_eq!(screen.screen_material_diameter_m, 0.0005);
    assert_eq!(screen.screen_to_glass_distance_m, 0.025);
    assert_eq!(screen.top_opening_multiplier, 0.0);
    assert_eq!(screen.bottom_opening_multiplier, 0.0);
    assert_eq!(screen.left_side_opening_multiplier, 0.0);
    assert_eq!(screen.right_side_opening_multiplier, 0.0);
    assert_eq!(
        screen.transmittance_map_resolution,
        WindowScreenTransmittanceMapResolution::Disabled
    );
    assert_eq!(screen.transmittance_map_resolution.degrees(), 0);
    assert_close(screen.diameter_to_spacing_ratio(), 0.25);
    assert_eq!(screen.thickness_m(), 0.0005);
    assert!(screen.is_resistance_only());
    assert_close(screen.direct_normal_transmittance, 0.5625);
    assert_close(screen.solar_reflectance, 0.0875);
    assert_close(screen.visible_reflectance, 0.13125);
    assert_close(screen.visible_transmittance, 0.5625);
    assert_close(screen.thermal_transmittance, 0.5625);
    assert_close(screen.airflow_permeability, 0.5625);
    assert_close(screen.solar_absorptance, 0.35);
    assert_close(screen.visible_absorptance, 0.30625);
    assert_close(screen.thermal_absorptance, 0.39375);
    assert_close(
        screen
            .nominal_thermal_resistance_m2_k_per_w()
            .ok_or_else(|| std::io::Error::other("expected nominal screen resistance"))?,
        0.4375 * 0.0005 / 221.0,
    );

    let explicit = model
        .materials
        .iter()
        .find(|material| material.name.0 == "EXPLICIT SCREEN")
        .and_then(|material| material.as_window_screen())
        .ok_or_else(|| std::io::Error::other("missing explicit screen"))?;
    assert_eq!(
        explicit.beam_reflectance_model,
        WindowScreenBeamReflectanceModel::ModelAsDirectBeam
    );
    assert_eq!(explicit.thermal_hemispherical_emissivity_input, 0.8);
    assert_eq!(explicit.conductivity_w_per_m_k, 200.0);
    assert_eq!(explicit.screen_to_glass_distance_m, 0.001);
    assert_eq!(explicit.top_opening_multiplier, 0.1);
    assert_eq!(explicit.bottom_opening_multiplier, 0.2);
    assert_eq!(explicit.left_side_opening_multiplier, 0.3);
    assert_eq!(explicit.right_side_opening_multiplier, 0.4);
    assert_eq!(
        explicit.transmittance_map_resolution,
        WindowScreenTransmittanceMapResolution::Degrees5
    );
    assert_close(explicit.thermal_absorptance, 0.35);
    assert_close(
        explicit
            .nominal_thermal_resistance_m2_k_per_w()
            .ok_or_else(|| std::io::Error::other("expected explicit resistance"))?,
        1.09375e-6,
    );

    for (field, value) in [
        (
            "reflected_beam_transmittance_accounting_method",
            "ModelAsDiffuse",
        ),
        ("thermal_hemispherical_emissivity", "0.9"),
        ("conductivity", "221.0"),
        ("screen_to_glass_distance", "0.025"),
        ("top_opening_multiplier", "0.0"),
        ("bottom_opening_multiplier", "0.0"),
        ("left_side_opening_multiplier", "0.0"),
        ("right_side_opening_multiplier", "0.0"),
        (
            "angle_of_resolution_for_screen_transmittance_output_map",
            "0",
        ),
    ] {
        assert!(
            result.report.defaults_applied.iter().any(|default| {
                default.object_type == "WindowMaterial:Screen"
                    && default.object_name == "Default Screen"
                    && default.field == field
                    && default.value == value
            }),
            "missing default record for {field}={value}"
        );
    }
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == "WindowMaterial:Screen")
        .ok_or_else(|| std::io::Error::other("missing screen coverage"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    assert_eq!(coverage.object_count, 2);
    Ok(())
}

#[test]
fn window_screen_accepts_all_string_and_numeric_enums() -> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Screen": {{
                "A Disabled": {{{SCREEN_REQUIRED_FIELDS},
                    "reflected_beam_transmittance_accounting_method":"DoNotModel",
                    "angle_of_resolution_for_screen_transmittance_output_map":0}},
                "B One": {{{SCREEN_REQUIRED_FIELDS},
                    "reflected_beam_transmittance_accounting_method":"modelasdirectbeam",
                    "angle_of_resolution_for_screen_transmittance_output_map":1}},
                "C Two": {{{SCREEN_REQUIRED_FIELDS},
                    "reflected_beam_transmittance_accounting_method":"ModelAsDiffuse",
                    "angle_of_resolution_for_screen_transmittance_output_map":2}},
                "D Three": {{{SCREEN_REQUIRED_FIELDS},
                    "reflected_beam_transmittance_accounting_method":"DoNotModel",
                    "angle_of_resolution_for_screen_transmittance_output_map":3}},
                "E Five": {{{SCREEN_REQUIRED_FIELDS},
                    "reflected_beam_transmittance_accounting_method":"ModelAsDirectBeam",
                    "angle_of_resolution_for_screen_transmittance_output_map":5}}
            }}
        }}"#
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected enum model"))?;
    let expected = [
        (
            "A DISABLED",
            WindowScreenBeamReflectanceModel::DoNotModel,
            WindowScreenTransmittanceMapResolution::Disabled,
        ),
        (
            "B ONE",
            WindowScreenBeamReflectanceModel::ModelAsDirectBeam,
            WindowScreenTransmittanceMapResolution::Degrees1,
        ),
        (
            "C TWO",
            WindowScreenBeamReflectanceModel::ModelAsDiffuse,
            WindowScreenTransmittanceMapResolution::Degrees2,
        ),
        (
            "D THREE",
            WindowScreenBeamReflectanceModel::DoNotModel,
            WindowScreenTransmittanceMapResolution::Degrees3,
        ),
        (
            "E FIVE",
            WindowScreenBeamReflectanceModel::ModelAsDirectBeam,
            WindowScreenTransmittanceMapResolution::Degrees5,
        ),
    ];
    for (name, beam_model, map_resolution) in expected {
        let screen = model
            .materials
            .iter()
            .find(|material| material.name.0 == name)
            .and_then(|material| material.as_window_screen())
            .ok_or_else(|| std::io::Error::other(format!("missing {name}")))?;
        assert_eq!(screen.beam_reflectance_model, beam_model);
        assert_eq!(screen.transmittance_map_resolution, map_resolution);
    }
    Ok(())
}

#[test]
fn window_screen_enforces_required_fields_and_exact_schema_bounds()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid = r#"{
        "WindowMaterial:Screen": {
            "Missing Required": {},
            "Bad Lower Bounds": {
                "diffuse_solar_reflectance":-0.01,
                "diffuse_visible_reflectance":-0.01,
                "thermal_hemispherical_emissivity":0.0,
                "conductivity":0.0,
                "screen_material_spacing":0.0,
                "screen_material_diameter":0.0,
                "screen_to_glass_distance":0.0009,
                "top_opening_multiplier":-0.01,
                "bottom_opening_multiplier":-0.01,
                "left_side_opening_multiplier":-0.01,
                "right_side_opening_multiplier":-0.01,
                "angle_of_resolution_for_screen_transmittance_output_map":4
            },
            "Bad Upper Bounds": {
                "reflected_beam_transmittance_accounting_method":"Unsupported",
                "diffuse_solar_reflectance":1.0,
                "diffuse_visible_reflectance":1.0,
                "thermal_hemispherical_emissivity":1.0,
                "conductivity":221.0,
                "screen_material_spacing":0.002,
                "screen_material_diameter":0.0005,
                "screen_to_glass_distance":1.01,
                "top_opening_multiplier":1.01,
                "bottom_opening_multiplier":1.01,
                "left_side_opening_multiplier":1.01,
                "right_side_opening_multiplier":1.01,
                "angle_of_resolution_for_screen_transmittance_output_map":"5"
            },
            "Bad Map Negative": {
                "diffuse_solar_reflectance":0.2,
                "diffuse_visible_reflectance":0.3,
                "screen_material_spacing":0.002,
                "screen_material_diameter":0.0005,
                "angle_of_resolution_for_screen_transmittance_output_map":-1
            },
            "Bad Map Fraction": {
                "diffuse_solar_reflectance":0.2,
                "diffuse_visible_reflectance":0.3,
                "screen_material_spacing":0.002,
                "screen_material_diameter":0.0005,
                "angle_of_resolution_for_screen_transmittance_output_map":2.5
            },
            "Bad Map High": {
                "diffuse_solar_reflectance":0.2,
                "diffuse_visible_reflectance":0.3,
                "screen_material_spacing":0.002,
                "screen_material_diameter":0.0005,
                "angle_of_resolution_for_screen_transmittance_output_map":6
            }
        }
    }"#;
    let result = compile_raw_model(&parse_epjson_str(invalid)?);
    assert!(result.has_errors());

    for field in [
        "diffuse_solar_reflectance",
        "diffuse_visible_reflectance",
        "screen_material_spacing",
        "screen_material_diameter",
    ] {
        assert!(has_diagnostic(
            &result,
            "MissingRequiredField",
            "Missing Required",
            Some(field)
        ));
    }
    for field in [
        "diffuse_solar_reflectance",
        "diffuse_visible_reflectance",
        "thermal_hemispherical_emissivity",
        "conductivity",
        "screen_material_spacing",
        "screen_material_diameter",
        "screen_to_glass_distance",
        "top_opening_multiplier",
        "bottom_opening_multiplier",
        "left_side_opening_multiplier",
        "right_side_opening_multiplier",
    ] {
        assert!(
            has_diagnostic(
                &result,
                "InvalidNumericRange",
                "Bad Lower Bounds",
                Some(field)
            ) || has_diagnostic(
                &result,
                "InvalidNumericRange",
                "Bad Upper Bounds",
                Some(field)
            ),
            "missing bound diagnostic for {field}"
        );
    }
    assert!(has_diagnostic(
        &result,
        "InvalidNumericEnumValue",
        "Bad Lower Bounds",
        Some("angle_of_resolution_for_screen_transmittance_output_map")
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidFieldType",
        "Bad Upper Bounds",
        Some("angle_of_resolution_for_screen_transmittance_output_map")
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidEnumValue",
        "Bad Upper Bounds",
        Some("reflected_beam_transmittance_accounting_method")
    ));
    for name in ["Bad Map Negative", "Bad Map Fraction", "Bad Map High"] {
        assert!(has_diagnostic(
            &result,
            "InvalidNumericEnumValue",
            name,
            Some("angle_of_resolution_for_screen_transmittance_output_map")
        ));
    }

    let valid_edges = r#"{
        "WindowMaterial:Screen": {
            "Valid Edges": {
                "diffuse_solar_reflectance":0.0,
                "diffuse_visible_reflectance":0.999999,
                "thermal_hemispherical_emissivity":0.000001,
                "conductivity":0.000001,
                "screen_material_spacing":0.002,
                "screen_material_diameter":0.001999,
                "screen_to_glass_distance":1.0,
                "top_opening_multiplier":0.0,
                "bottom_opening_multiplier":1.0,
                "left_side_opening_multiplier":0.0,
                "right_side_opening_multiplier":1.0,
                "angle_of_resolution_for_screen_transmittance_output_map":5
            }
        }
    }"#;
    let edge_result = compile_raw_model(&parse_epjson_str(valid_edges)?);
    assert!(
        !edge_result.has_errors(),
        "{:?}",
        edge_result.report.diagnostics
    );
    Ok(())
}

#[test]
fn window_screen_requires_diameter_below_spacing_and_checks_effective_sums()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid_geometry = r#"{
        "WindowMaterial:Screen": {
            "Equal Geometry": {
                "diffuse_solar_reflectance":0.2,
                "diffuse_visible_reflectance":0.3,
                "screen_material_spacing":0.002,
                "screen_material_diameter":0.002
            },
            "Greater Geometry": {
                "diffuse_solar_reflectance":0.2,
                "diffuse_visible_reflectance":0.3,
                "screen_material_spacing":0.002,
                "screen_material_diameter":0.003
            }
        }
    }"#;
    let geometry_result = compile_raw_model(&parse_epjson_str(invalid_geometry)?);
    for name in ["Equal Geometry", "Greater Geometry"] {
        assert!(has_diagnostic(
            &geometry_result,
            "InvalidWindowScreenGeometry",
            name,
            Some("screen_material_diameter")
        ));
    }

    // The source scales raw reflectance by the solid fraction before checking
    // the sum. A raw T + R greater than one must therefore remain valid.
    let raw_sum_allowed = r#"{
        "WindowMaterial:Screen": {
            "Raw Sum Allowed": {
                "diffuse_solar_reflectance":0.99,
                "diffuse_visible_reflectance":0.99,
                "screen_material_spacing":0.002,
                "screen_material_diameter":0.000002
            }
        }
    }"#;
    let raw_sum_result = compile_raw_model(&parse_epjson_str(raw_sum_allowed)?);
    assert!(
        !raw_sum_result.has_errors(),
        "{:?}",
        raw_sum_result.report.diagnostics
    );
    let screen = raw_sum_result
        .model
        .as_ref()
        .and_then(|model| model.materials.first())
        .and_then(|material| material.as_window_screen())
        .ok_or_else(|| std::io::Error::other("missing raw-sum screen"))?;
    assert!(screen.direct_normal_transmittance + screen.diffuse_solar_reflectance_input > 1.0);
    assert!(screen.direct_normal_transmittance + screen.solar_reflectance < 1.0);

    // At this extreme ratio, subtraction rounds to one exactly. EnergyPlus's
    // final effective-property check still rejects the material.
    let rounded_open = r#"{
        "WindowMaterial:Screen": {
            "Rounded Open": {
                "diffuse_solar_reflectance":0.2,
                "diffuse_visible_reflectance":0.3,
                "screen_material_spacing":1.0,
                "screen_material_diameter":1e-20
            }
        }
    }"#;
    let rounded_result = compile_raw_model(&parse_epjson_str(rounded_open)?);
    assert!(has_diagnostic(
        &rounded_result,
        "InvalidWindowScreenOpticalSum",
        "Rounded Open",
        None
    ));
    Ok(())
}

#[test]
fn window_screen_follows_drape_source_order_and_shares_material_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let ordered = format!(
        r#"{{
            "WindowMaterial:Screen": {{"A Screen": {{{SCREEN_REQUIRED_FIELDS}}}}},
            "WindowMaterial:Drape:EquivalentLayer": {{
                "Z Drape": {{
                    "front_side_drape_beam_diffuse_solar_transmittance":0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance":0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance":0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance":0.1
                }}
            }}
        }}"#
    );
    let ordered_result = compile_raw_model(&parse_epjson_str(&ordered)?);
    assert!(
        !ordered_result.has_errors(),
        "{:?}",
        ordered_result.report.diagnostics
    );
    let ordered_model = ordered_result
        .model
        .ok_or_else(|| std::io::Error::other("expected ordered model"))?;
    assert_eq!(
        ordered_model
            .materials
            .iter()
            .map(|material| material.kind())
            .collect::<Vec<_>>(),
        vec![
            MaterialKind::WindowDrapeEquivalentLayer,
            MaterialKind::WindowScreen
        ]
    );

    let identity = format!(
        r#"{{
            "WindowMaterial:Glazing": {{"Shared": {{{GLASS_FIELDS}}}}},
            "WindowMaterial:Screen": {{
                "Shared": {{{SCREEN_REQUIRED_FIELDS}}},
                "": {{{SCREEN_REQUIRED_FIELDS}}}
            }}
        }}"#
    );
    let identity_result = compile_raw_model(&parse_epjson_str(&identity)?);
    assert!(has_diagnostic(
        &identity_result,
        "DuplicateName",
        "Shared",
        None
    ));
    assert!(has_diagnostic(
        &identity_result,
        "MissingRequiredField",
        "",
        Some("name")
    ));
    Ok(())
}

#[test]
fn window_screen_constructions_allow_only_safe_exterior_direct_glazing()
-> Result<(), Box<dyn std::error::Error>> {
    let valid = format!(
        r#"{{
            "WindowMaterial:Glazing": {{"Glass": {{{GLASS_FIELDS}}}}},
            "WindowMaterial:Gas": {{"Air": {{"gas_type":"Air","thickness":0.01}}}},
            "WindowMaterial:Screen": {{"Screen": {{{SCREEN_REQUIRED_FIELDS}}}}},
            "Construction": {{
                "Exterior Single": {{
                    "outside_layer":"Screen","layer_2":"Glass"
                }},
                "Exterior Double": {{
                    "outside_layer":"Screen","layer_2":"Glass",
                    "layer_3":"Air","layer_4":"Glass"
                }},
                "Exterior Triple": {{
                    "outside_layer":"Screen","layer_2":"Glass",
                    "layer_3":"Air","layer_4":"Glass",
                    "layer_5":"Air","layer_6":"Glass"
                }},
                "Exterior Quad": {{
                    "outside_layer":"Screen","layer_2":"Glass",
                    "layer_3":"Air","layer_4":"Glass",
                    "layer_5":"Air","layer_6":"Glass",
                    "layer_7":"Air","layer_8":"Glass"
                }}
            }}
        }}"#
    );
    let valid_result = compile_raw_model(&parse_epjson_str(&valid)?);
    assert!(
        !valid_result.has_errors(),
        "{:?}",
        valid_result.report.diagnostics
    );
    let valid_model = valid_result
        .model
        .ok_or_else(|| std::io::Error::other("expected screen constructions"))?;
    assert_eq!(valid_model.constructions.len(), 4);
    assert!(
        valid_model
            .constructions
            .iter()
            .all(|construction| construction.kind == ConstructionKind::Fenestration)
    );
    for (name, layer_count) in [
        ("EXTERIOR SINGLE", 2),
        ("EXTERIOR DOUBLE", 4),
        ("EXTERIOR TRIPLE", 6),
        ("EXTERIOR QUAD", 8),
    ] {
        assert_eq!(
            valid_model
                .constructions
                .iter()
                .find(|construction| construction.name.0 == name)
                .ok_or_else(|| std::io::Error::other(format!("missing {name}")))?
                .layers
                .len(),
            layer_count
        );
    }

    let diffusing_glass = format!(r#"{GLASS_FIELDS},"solar_diffusing":"Yes""#);
    let invalid = format!(
        r#"{{
            "WindowMaterial:Glazing": {{
                "Glass": {{{GLASS_FIELDS}}},
                "Diffusing Glass": {{{diffusing_glass}}}
            }},
            "WindowMaterial:Gas": {{"Air": {{"gas_type":"Air","thickness":0.01}}}},
            "WindowMaterial:Screen": {{
                "Screen One": {{{SCREEN_REQUIRED_FIELDS}}},
                "Screen Two": {{{SCREEN_REQUIRED_FIELDS}}}
            }},
            "WindowMaterial:Shade": {{"Shade": {{{SHADE_FIELDS}}}}},
            "Construction": {{
                "Screen Alone": {{"outside_layer":"Screen One"}},
                "Interior Screen": {{
                    "outside_layer":"Glass","layer_2":"Screen One"
                }},
                "Between Glass Screen": {{
                    "outside_layer":"Glass","layer_2":"Air",
                    "layer_3":"Screen One","layer_4":"Air","layer_5":"Glass"
                }},
                "Unsafe Exterior Hole": {{
                    "outside_layer":"Screen One","layer_2":"Air","layer_3":"Glass"
                }},
                "Two Screens": {{
                    "outside_layer":"Screen One","layer_2":"Glass","layer_3":"Screen Two"
                }},
                "Screen And Shade": {{
                    "outside_layer":"Screen One","layer_2":"Glass","layer_3":"Shade"
                }},
                "Diffusing With Screen": {{
                    "outside_layer":"Screen One","layer_2":"Diffusing Glass"
                }}
            }}
        }}"#
    );
    let invalid_result = compile_raw_model(&parse_epjson_str(&invalid)?);
    for name in ["Screen Alone", "Interior Screen", "Between Glass Screen"] {
        assert!(has_diagnostic(
            &invalid_result,
            "InvalidWindowScreenConstructionLayering",
            name,
            None
        ));
    }
    assert!(has_diagnostic(
        &invalid_result,
        "UnsafeWindowScreenEndLayering",
        "Unsafe Exterior Hole",
        None
    ));
    for name in ["Two Screens", "Screen And Shade"] {
        assert!(has_diagnostic(
            &invalid_result,
            "InvalidWindowScreenCount",
            name,
            None
        ));
    }
    assert!(has_diagnostic(
        &invalid_result,
        "InvalidSolarDiffusingGlazingWithScreen",
        "Diffusing With Screen",
        None
    ));
    Ok(())
}
