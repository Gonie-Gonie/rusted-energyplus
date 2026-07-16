use super::super::{ObjectCoverageStatus, compile_raw_model};
use ep_model::{ConstructionKind, MaterialFamily, MaterialKind, MaterialSurfaceRoughness};
use ep_raw_model::parse_epjson_str;

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
    let tolerance = f64::EPSILON * expected.abs().max(1.0) * 16.0;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}

fn has_diagnostic(
    result: &super::super::CompileResult,
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
fn shade_materializes_fields_defaults_and_source_derivations()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Shade": {{
                "Default Shade": {{{SHADE_FIELDS}}},
                "Explicit Openings": {{
                    {SHADE_FIELDS},
                    "shade_to_glass_distance":0.001,
                    "top_opening_multiplier":0.0,
                    "bottom_opening_multiplier":1.0,
                    "left_side_opening_multiplier":0.25,
                    "right_side_opening_multiplier":0.75,
                    "airflow_permeability":0.8
                }}
            }}
        }}"#
    );
    let raw_model = parse_epjson_str(&epjson)?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed shade"))?;
    assert_eq!(model.materials.len(), 2);
    let material = model
        .materials
        .iter()
        .find(|material| material.name.0 == "DEFAULT SHADE")
        .ok_or_else(|| std::io::Error::other("missing default Shade"))?;
    assert_eq!(material.name.0, "DEFAULT SHADE");
    assert_eq!(material.kind(), MaterialKind::WindowShade);
    assert_eq!(material.family(), MaterialFamily::Fenestration);
    assert!(material.as_opaque().is_none());
    let shade = material
        .as_window_shade()
        .ok_or_else(|| std::io::Error::other("expected shade payload"))?;
    assert_eq!(shade.roughness, MaterialSurfaceRoughness::MediumRough);
    assert_eq!(shade.solar_transmittance, 0.15);
    assert_eq!(shade.solar_reflectance, 0.35);
    assert_eq!(shade.visible_transmittance, 0.10);
    assert_eq!(shade.visible_reflectance, 0.40);
    assert_eq!(shade.infrared_hemispherical_emissivity, 0.80);
    assert_eq!(shade.infrared_transmittance, 0.10);
    assert_eq!(shade.thickness_m, 0.002);
    assert_eq!(shade.conductivity_w_per_m_k, 0.20);
    assert_close(shade.solar_absorptance, 0.50);
    assert_eq!(
        shade.visible_absorptance, 0.0,
        "EnergyPlus 26.1 leaves shade visible absorptance at its initialized zero"
    );
    assert_eq!(shade.shade_to_glass_distance_m, 0.05);
    assert_eq!(shade.top_opening_multiplier, 0.5);
    assert_eq!(shade.bottom_opening_multiplier, 0.5);
    assert_eq!(shade.left_side_opening_multiplier, 0.5);
    assert_eq!(shade.right_side_opening_multiplier, 0.5);
    assert_eq!(shade.airflow_permeability, 0.0);
    assert_close(
        shade
            .nominal_thermal_resistance_m2_k_per_w()
            .ok_or_else(|| std::io::Error::other("expected nominal shade resistance"))?,
        0.01,
    );

    let explicit = model
        .materials
        .iter()
        .find(|material| material.name.0 == "EXPLICIT OPENINGS")
        .and_then(|material| material.as_window_shade())
        .ok_or_else(|| std::io::Error::other("missing explicit Shade"))?;
    assert_eq!(explicit.shade_to_glass_distance_m, 0.001);
    assert_eq!(explicit.top_opening_multiplier, 0.0);
    assert_eq!(explicit.bottom_opening_multiplier, 1.0);
    assert_eq!(explicit.left_side_opening_multiplier, 0.25);
    assert_eq!(explicit.right_side_opening_multiplier, 0.75);
    assert_eq!(explicit.airflow_permeability, 0.8);

    for (field, value) in [
        ("shade_to_glass_distance", "0.05"),
        ("top_opening_multiplier", "0.5"),
        ("bottom_opening_multiplier", "0.5"),
        ("left_side_opening_multiplier", "0.5"),
        ("right_side_opening_multiplier", "0.5"),
        ("airflow_permeability", "0.0"),
    ] {
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == "WindowMaterial:Shade"
                && default.object_name == "Default Shade"
                && default.field == field
                && default.value == value
        }));
    }
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == "WindowMaterial:Shade")
        .ok_or_else(|| std::io::Error::other("missing Shade coverage"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    Ok(())
}

#[test]
fn shade_enforces_required_fields_schema_bounds_and_strict_optical_sums()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade": {
                "Missing Required": {},
                "Bad Bounds": {
                    "solar_transmittance":1.0,
                    "solar_reflectance":-0.01,
                    "visible_transmittance":1.0,
                    "visible_reflectance":-0.01,
                    "infrared_hemispherical_emissivity":0.0,
                    "infrared_transmittance":1.0,
                    "thickness":0.0,
                    "conductivity":0.0,
                    "shade_to_glass_distance":0.0009,
                    "top_opening_multiplier":-0.01,
                    "bottom_opening_multiplier":1.01,
                    "left_side_opening_multiplier":-0.01,
                    "right_side_opening_multiplier":1.01,
                    "airflow_permeability":0.81
                },
                "Solar Sum": {
                    "solar_transmittance":0.6, "solar_reflectance":0.4,
                    "visible_transmittance":0.1, "visible_reflectance":0.4,
                    "infrared_hemispherical_emissivity":0.8,
                    "infrared_transmittance":0.1,
                    "thickness":0.002, "conductivity":0.2
                },
                "Visible Sum": {
                    "solar_transmittance":0.1, "solar_reflectance":0.4,
                    "visible_transmittance":0.6, "visible_reflectance":0.4,
                    "infrared_hemispherical_emissivity":0.8,
                    "infrared_transmittance":0.1,
                    "thickness":0.002, "conductivity":0.2
                },
                "Infrared Sum": {
                    "solar_transmittance":0.1, "solar_reflectance":0.4,
                    "visible_transmittance":0.1, "visible_reflectance":0.4,
                    "infrared_hemispherical_emissivity":0.8,
                    "infrared_transmittance":0.2,
                    "thickness":0.002, "conductivity":0.2
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for field in [
        "solar_transmittance",
        "solar_reflectance",
        "visible_transmittance",
        "visible_reflectance",
        "infrared_hemispherical_emissivity",
        "infrared_transmittance",
        "thickness",
        "conductivity",
    ] {
        assert!(
            has_diagnostic(
                &result,
                "MissingRequiredField",
                "Missing Required",
                Some(field)
            ),
            "missing required-field diagnostic for {field}"
        );
    }
    for field in [
        "solar_transmittance",
        "solar_reflectance",
        "visible_transmittance",
        "visible_reflectance",
        "infrared_hemispherical_emissivity",
        "infrared_transmittance",
        "thickness",
        "conductivity",
        "shade_to_glass_distance",
        "top_opening_multiplier",
        "bottom_opening_multiplier",
        "left_side_opening_multiplier",
        "right_side_opening_multiplier",
        "airflow_permeability",
    ] {
        assert!(
            has_diagnostic(&result, "InvalidNumericRange", "Bad Bounds", Some(field)),
            "missing range diagnostic for {field}"
        );
    }
    for object_name in ["Solar Sum", "Visible Sum", "Infrared Sum"] {
        assert!(has_diagnostic(
            &result,
            "InvalidWindowShadeOpticalSum",
            object_name,
            None
        ));
    }
    Ok(())
}

#[test]
fn shade_follows_gas_mixture_in_family_order_and_shares_the_name_registry()
-> Result<(), Box<dyn std::error::Error>> {
    let ordered = format!(
        r#"{{
            "WindowMaterial:Shade": {{"A Shade": {{{SHADE_FIELDS}}}}},
            "WindowMaterial:GasMixture": {{
                "Z Mixture": {{
                    "thickness":0.01,
                    "number_of_gases_in_mixture":1,
                    "gas_1_type":"Air",
                    "gas_1_fraction":1.0,
                    "gas_2_type":"Argon",
                    "gas_2_fraction":0.5
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
        .ok_or_else(|| std::io::Error::other("expected ordered materials"))?;
    assert_eq!(
        ordered_model
            .materials
            .iter()
            .map(|material| material.kind())
            .collect::<Vec<_>>(),
        vec![MaterialKind::WindowGasMixture, MaterialKind::WindowShade]
    );

    let duplicate = format!(
        r#"{{
            "WindowMaterial:GasMixture": {{
                "Shared": {{
                    "thickness":0.01,
                    "number_of_gases_in_mixture":1,
                    "gas_1_type":"Air",
                    "gas_1_fraction":1.0,
                    "gas_2_type":"Argon",
                    "gas_2_fraction":0.5
                }}
            }},
            "WindowMaterial:Shade": {{"Shared": {{{SHADE_FIELDS}}}}}
        }}"#
    );
    let duplicate_result = compile_raw_model(&parse_epjson_str(&duplicate)?);
    assert!(duplicate_result.has_errors());
    assert!(has_diagnostic(
        &duplicate_result,
        "DuplicateName",
        "Shared",
        None
    ));
    Ok(())
}

#[test]
fn shade_constructions_accept_safe_end_and_between_glass_placements()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing": {{"Glass": {{{GLASS_FIELDS}}}}},
            "WindowMaterial:Gas": {{
                "Air Thin": {{"gas_type":"Air","thickness":0.001}},
                "Air Threshold": {{"gas_type":"Air","thickness":0.0015}},
                "Custom Left": {{
                    "gas_type":"Custom","thickness":0.01,
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":44.0
                }},
                "Custom Right": {{
                    "gas_type":"Custom","thickness":0.01,
                    "conductivity_coefficient_a":0.03,
                    "viscosity_coefficient_a":0.00002,
                    "specific_heat_coefficient_a":1100.0,
                    "molecular_weight":50.0
                }}
            }},
            "WindowMaterial:GasMixture": {{
                "One Air": {{
                    "thickness":0.001,
                    "number_of_gases_in_mixture":1,
                    "gas_1_type":"Air","gas_1_fraction":1.0,
                    "gas_2_type":"Argon","gas_2_fraction":0.25
                }},
                "One Air Copy": {{
                    "thickness":0.001,
                    "number_of_gases_in_mixture":1,
                    "gas_1_type":"Air","gas_1_fraction":1.0,
                    "gas_2_type":"Krypton","gas_2_fraction":0.75
                }}
            }},
            "WindowMaterial:Shade": {{"Shade": {{{SHADE_FIELDS}}}}},
            "Construction": {{
                "Exterior Four Pane": {{
                    "outside_layer":"Shade","layer_2":"Glass","layer_3":"Air Thin",
                    "layer_4":"Glass","layer_5":"Air Thin","layer_6":"Glass",
                    "layer_7":"Air Thin","layer_8":"Glass"
                }},
                "Interior Four Pane": {{
                    "outside_layer":"Glass","layer_2":"Air Thin","layer_3":"Glass",
                    "layer_4":"Air Thin","layer_5":"Glass","layer_6":"Air Thin",
                    "layer_7":"Glass","layer_8":"Shade"
                }},
                "Between Double Threshold": {{
                    "outside_layer":"Glass","layer_2":"Air Thin","layer_3":"Shade",
                    "layer_4":"Air Threshold","layer_5":"Glass"
                }},
                "Between Triple": {{
                    "outside_layer":"Glass","layer_2":"Air Thin","layer_3":"Glass",
                    "layer_4":"Air Thin","layer_5":"Shade","layer_6":"Air Thin",
                    "layer_7":"Glass"
                }},
                "Matching One Gas Mixtures": {{
                    "outside_layer":"Glass","layer_2":"One Air","layer_3":"Shade",
                    "layer_4":"One Air Copy","layer_5":"Glass"
                }},
                "Custom Records Ignored": {{
                    "outside_layer":"Glass","layer_2":"Custom Left","layer_3":"Shade",
                    "layer_4":"Custom Right","layer_5":"Glass"
                }},
                "Bare Mixture Preserved": {{
                    "outside_layer":"Glass","layer_2":"One Air","layer_3":"Glass"
                }}
            }}
        }}"#
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected safe shade constructions"))?;
    assert_eq!(model.constructions.len(), 7);
    assert!(
        model
            .constructions
            .iter()
            .all(|construction| construction.kind == ConstructionKind::Fenestration)
    );
    assert_eq!(
        model
            .constructions
            .iter()
            .find(|construction| construction.name.0 == "EXTERIOR FOUR PANE")
            .ok_or_else(|| std::io::Error::other("missing four-pane exterior shade"))?
            .layers
            .len(),
        8
    );
    Ok(())
}

#[test]
fn shade_constructions_reject_multiple_diffusing_misplaced_and_unsafe_layers()
-> Result<(), Box<dyn std::error::Error>> {
    let diffusing_glass = format!(r#"{GLASS_FIELDS},"solar_diffusing":"Yes""#);
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing": {{
                "Glass": {{{GLASS_FIELDS}}},
                "Diffusing Glass": {{{diffusing_glass}}}
            }},
            "WindowMaterial:Gas": {{"Air": {{"gas_type":"Air","thickness":0.01}}}},
            "WindowMaterial:Shade": {{
                "Shade One": {{{SHADE_FIELDS}}},
                "Shade Two": {{{SHADE_FIELDS}}}
            }},
            "Construction": {{
                "Two Shades": {{
                    "outside_layer":"Shade One","layer_2":"Glass","layer_3":"Shade Two"
                }},
                "Diffusing With Shade": {{
                    "outside_layer":"Shade One","layer_2":"Diffusing Glass"
                }},
                "Misplaced Shade": {{
                    "outside_layer":"Glass","layer_2":"Shade One",
                    "layer_3":"Air","layer_4":"Glass"
                }},
                "Unsafe Exterior Hole": {{
                    "outside_layer":"Shade One","layer_2":"Air","layer_3":"Glass"
                }},
                "Unsafe Interior Hole": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Shade One"
                }},
                "Shade Alone": {{"outside_layer":"Shade One"}}
            }}
        }}"#
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(result.has_errors());
    assert!(has_diagnostic(
        &result,
        "InvalidWindowShadeCount",
        "Two Shades",
        None
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidSolarDiffusingGlazingWithShade",
        "Diffusing With Shade",
        None
    ));
    for object_name in ["Misplaced Shade", "Shade Alone"] {
        assert!(has_diagnostic(
            &result,
            "InvalidWindowShadeConstructionLayering",
            object_name,
            None
        ));
    }
    for object_name in ["Unsafe Exterior Hole", "Unsafe Interior Hole"] {
        assert!(has_diagnostic(
            &result,
            "UnsafeWindowShadeEndLayering",
            object_name,
            None
        ));
    }
    Ok(())
}

#[test]
fn between_glass_shade_rejects_species_fraction_and_width_mismatches()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing": {{"Glass": {{{GLASS_FIELDS}}}}},
            "WindowMaterial:Gas": {{
                "Air": {{"gas_type":"Air","thickness":0.001}},
                "Argon": {{"gas_type":"Argon","thickness":0.001}},
                "Air Wide": {{"gas_type":"Air","thickness":0.00151}}
            }},
            "WindowMaterial:GasMixture": {{
                "Mixture A": {{
                    "thickness":0.001,"number_of_gases_in_mixture":2,
                    "gas_1_type":"Air","gas_1_fraction":0.5,
                    "gas_2_type":"Argon","gas_2_fraction":0.5
                }},
                "Mixture B": {{
                    "thickness":0.001,"number_of_gases_in_mixture":2,
                    "gas_1_type":"Air","gas_1_fraction":0.6,
                    "gas_2_type":"Argon","gas_2_fraction":0.4
                }},
                "One Air": {{
                    "thickness":0.001,"number_of_gases_in_mixture":1,
                    "gas_1_type":"Air","gas_1_fraction":1.0,
                    "gas_2_type":"Argon","gas_2_fraction":0.5
                }}
            }},
            "WindowMaterial:Shade": {{"Shade": {{{SHADE_FIELDS}}}}},
            "Construction": {{
                "Species Mismatch": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Shade",
                    "layer_4":"Argon","layer_5":"Glass"
                }},
                "Fraction Mismatch": {{
                    "outside_layer":"Glass","layer_2":"Mixture A","layer_3":"Shade",
                    "layer_4":"Mixture B","layer_5":"Glass"
                }},
                "Single Gas Versus One Mixture": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Shade",
                    "layer_4":"One Air","layer_5":"Glass"
                }},
                "Width Mismatch": {{
                    "outside_layer":"Glass","layer_2":"Air","layer_3":"Shade",
                    "layer_4":"Air Wide","layer_5":"Glass"
                }}
            }}
        }}"#
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(result.has_errors());
    for object_name in [
        "Species Mismatch",
        "Fraction Mismatch",
        "Single Gas Versus One Mixture",
    ] {
        assert!(has_diagnostic(
            &result,
            "InvalidBetweenGlassShadeGasComposition",
            object_name,
            None
        ));
    }
    assert!(has_diagnostic(
        &result,
        "InvalidBetweenGlassShadeGapThickness",
        "Width Mismatch",
        None
    ));
    Ok(())
}

#[test]
fn shade_construction_rejects_equivalent_layer_materials_and_keeps_bare_rules()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing": {{"Glass": {{{GLASS_FIELDS}}}}},
            "WindowMaterial:Gas": {{"Air": {{"gas_type":"Air","thickness":0.01}}}},
            "WindowMaterial:Gap:EquivalentLayer": {{
                "EQL Gap": {{
                    "gas_type":"AIR","thickness":0.01,"gap_vent_type":"Sealed"
                }}
            }},
            "WindowMaterial:Shade": {{"Shade": {{{SHADE_FIELDS}}}}},
            "Construction": {{
                "Shade With EQL": {{
                    "outside_layer":"Shade","layer_2":"Glass","layer_3":"EQL Gap"
                }},
                "Bare Trailing Gap": {{
                    "outside_layer":"Glass","layer_2":"Air"
                }}
            }}
        }}"#
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(result.has_errors());
    assert!(has_diagnostic(
        &result,
        "InvalidEquivalentLayerConstruction",
        "Shade With EQL",
        None
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidWindowConstructionLayering",
        "Bare Trailing Gap",
        None
    ));
    Ok(())
}
