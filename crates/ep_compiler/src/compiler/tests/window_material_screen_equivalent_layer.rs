use super::super::{CompileResult, ObjectCoverageStatus, compile_raw_model, typed_coverage_status};
use ep_model::{
    AutoOrNumber, MaterialFamily, MaterialKind, MaterialSurfaceRoughness,
    WindowScreenEquivalentLayerSolarProperties, WindowScreenEquivalentLayerVisibleProperties,
};
use ep_raw_model::parse_epjson_str;
use std::collections::BTreeSet;

const OBJECT_TYPE: &str = "WindowMaterial:Screen:EquivalentLayer";

fn has_diagnostic(
    result: &CompileResult,
    code: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && field.is_none_or(|field| diagnostic.field.as_deref() == Some(field))
    })
}

#[test]
fn screen_equivalent_layer_materializes_all_inputs_defaults_and_source_quirks()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Screen": {
                "Screen Before Equivalent Screen": {
                    "diffuse_solar_reflectance": 0.1,
                    "diffuse_visible_reflectance": 0.1,
                    "screen_material_spacing": 0.002,
                    "screen_material_diameter": 0.0005
                }
            },
            "WindowMaterial:Screen:EquivalentLayer": {
                "Z Full Equivalent Screen": {
                    "screen_beam_beam_solar_transmittance": 0.64,
                    "screen_beam_diffuse_solar_transmittance": 0.10,
                    "screen_beam_diffuse_solar_reflectance": 0.20,
                    "screen_beam_beam_visible_transmittance": 0.30,
                    "screen_beam_diffuse_visible_transmittance": 0.40,
                    "screen_beam_diffuse_visible_reflectance": 0.25,
                    "screen_infrared_transmittance": 0.20,
                    "screen_infrared_emissivity": 0.90,
                    "screen_wire_spacing": 0.01,
                    "screen_wire_diameter": 0.002
                },
                "A Default Equivalent Screen": {
                    "screen_beam_diffuse_solar_transmittance": 0.11,
                    "screen_beam_diffuse_solar_reflectance": 0.12,
                    "screen_beam_beam_visible_transmittance": 0.13,
                    "screen_beam_diffuse_visible_transmittance": 0.14,
                    "screen_beam_diffuse_visible_reflectance": 0.15
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed equivalent-layer screens"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "SCREEN BEFORE EQUIVALENT SCREEN",
            "A DEFAULT EQUIVALENT SCREEN",
            "Z FULL EQUIVALENT SCREEN"
        ],
        "equivalent-layer screens must follow ordinary screens in source order"
    );

    let material = &model.materials[2];
    assert_eq!(material.kind(), MaterialKind::WindowScreenEquivalentLayer);
    assert_eq!(material.family(), MaterialFamily::EquivalentLayer);
    assert!(material.as_opaque().is_none());
    assert!(material.as_window_screen().is_none());
    assert_eq!(material.roughness(), None);
    assert_eq!(material.thickness_m(), None);
    assert_eq!(material.conductivity_w_per_m_k(), None);
    assert_eq!(material.density_kg_per_m3(), None);
    assert_eq!(material.specific_heat_j_per_kg_k(), None);
    assert_eq!(material.thermal_resistance(), None);
    assert_eq!(material.heat_capacity_per_area(), None);
    assert_eq!(material.thermal_absorptance(), None);
    assert_eq!(material.solar_absorptance(), None);
    assert_eq!(material.visible_absorptance(), None);

    let screen = material
        .as_window_screen_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer screen payload"))?;
    assert_eq!(screen.roughness, MaterialSurfaceRoughness::MediumRough);
    assert!(screen.is_resistance_only());
    assert_eq!(screen.nominal_thermal_resistance_m2_k_per_w(), None);
    assert_eq!(
        screen.front_solar,
        WindowScreenEquivalentLayerSolarProperties {
            beam_beam_transmittance: AutoOrNumber::Value(0.64),
            beam_diffuse_transmittance: 0.10,
            beam_diffuse_reflectance: 0.20,
        }
    );
    assert_eq!(screen.back_solar, screen.front_solar);
    assert_eq!(
        screen.front_visible,
        WindowScreenEquivalentLayerVisibleProperties {
            beam_beam_transmittance: 0.30,
            beam_diffuse_transmittance: 0.40,
            beam_diffuse_reflectance: 0.0,
            diffuse_diffuse_reflectance: 0.25,
        },
        "N6 belongs to the front diffuse-diffuse reflectance slot"
    );
    assert_eq!(
        screen.back_visible,
        WindowScreenEquivalentLayerVisibleProperties::default(),
        "EnergyPlus 26.1 leaves the entire back visible record at zero"
    );
    assert_eq!(screen.infrared_transmittance, 0.20);
    assert_eq!(screen.front_infrared_emissivity, 0.90);
    assert_eq!(screen.back_infrared_emissivity, 0.90);
    assert_eq!(screen.front_thermal_absorptance, 0.90);
    assert_eq!(screen.back_thermal_absorptance, 0.90);
    assert_eq!(screen.thermal_transmittance, 0.20);
    assert_eq!(screen.base_thermal_absorptance, 0.0);
    assert_eq!(screen.wire_spacing_m, 0.01);
    assert_eq!(screen.wire_diameter_m, 0.002);

    let default = model.materials[1]
        .as_window_screen_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing default equivalent-layer screen"))?;
    assert_eq!(
        default.front_solar.beam_beam_transmittance,
        AutoOrNumber::AutoCalculate
    );
    assert_eq!(default.back_solar, default.front_solar);
    assert_eq!(default.infrared_transmittance, 0.02);
    assert_eq!(default.front_infrared_emissivity, 0.93);
    assert_eq!(default.back_infrared_emissivity, 0.93);
    assert_eq!(default.wire_spacing_m, 0.0);
    assert_eq!(default.wire_diameter_m, 0.0);
    for (field, value) in [
        ("screen_beam_beam_solar_transmittance", "Autocalculate"),
        ("screen_infrared_transmittance", "0.02"),
        ("screen_infrared_emissivity", "0.93"),
        ("screen_wire_spacing", "0.0"),
        ("screen_wire_diameter", "0.0"),
    ] {
        assert!(
            result.report.defaults_applied.iter().any(|default| {
                default.object_type == OBJECT_TYPE
                    && default.object_name == "A Default Equivalent Screen"
                    && default.field == field
                    && default.value == value
            }),
            "missing source-effective default record for {field}={value}"
        );
    }

    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer screen coverage"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    assert_eq!(coverage.object_count, 2);
    Ok(())
}

#[test]
fn screen_equivalent_layer_requires_n2_through_n6() -> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "Missing Required": {}
            }
        }"#,
    )?);

    assert!(result.has_errors());
    let required_fields = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "MissingRequiredField"
                && diagnostic.object_type == OBJECT_TYPE
                && diagnostic.object_name.as_deref() == Some("Missing Required")
        })
        .filter_map(|diagnostic| diagnostic.field.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        required_fields,
        BTreeSet::from([
            "screen_beam_beam_visible_transmittance".to_string(),
            "screen_beam_diffuse_solar_reflectance".to_string(),
            "screen_beam_diffuse_solar_transmittance".to_string(),
            "screen_beam_diffuse_visible_reflectance".to_string(),
            "screen_beam_diffuse_visible_transmittance".to_string(),
        ])
    );
    Ok(())
}

#[test]
fn screen_equivalent_layer_enforces_schema_bounds_and_accepts_trimmed_autocalculate()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "Below Bounds": {
                    "screen_beam_beam_solar_transmittance": -0.01,
                    "screen_beam_diffuse_solar_transmittance": -0.01,
                    "screen_beam_diffuse_solar_reflectance": -0.01,
                    "screen_beam_beam_visible_transmittance": -0.01,
                    "screen_beam_diffuse_visible_transmittance": -0.01,
                    "screen_beam_diffuse_visible_reflectance": -0.01,
                    "screen_infrared_transmittance": -0.01,
                    "screen_infrared_emissivity": 0.0,
                    "screen_wire_spacing": 0.00001,
                    "screen_wire_diameter": 0.00001
                },
                "Above Bounds": {
                    "screen_beam_beam_solar_transmittance": 1.0,
                    "screen_beam_diffuse_solar_transmittance": 1.0,
                    "screen_beam_diffuse_solar_reflectance": 1.0,
                    "screen_beam_beam_visible_transmittance": 1.0,
                    "screen_beam_diffuse_visible_transmittance": 1.0,
                    "screen_beam_diffuse_visible_reflectance": 1.0,
                    "screen_infrared_transmittance": 1.0,
                    "screen_infrared_emissivity": 1.0
                },
                "Bad Auto Token": {
                    "screen_beam_beam_solar_transmittance": "Autosize",
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1
                }
            }
        }"#,
    )?);
    assert!(invalid.has_errors());
    for object_name in ["Below Bounds", "Above Bounds"] {
        for field in [
            "screen_beam_beam_solar_transmittance",
            "screen_beam_diffuse_solar_transmittance",
            "screen_beam_diffuse_solar_reflectance",
            "screen_beam_beam_visible_transmittance",
            "screen_beam_diffuse_visible_transmittance",
            "screen_beam_diffuse_visible_reflectance",
            "screen_infrared_transmittance",
            "screen_infrared_emissivity",
        ] {
            assert!(
                has_diagnostic(&invalid, "InvalidNumericRange", object_name, Some(field)),
                "missing {object_name}/{field} range diagnostic"
            );
        }
    }
    for field in ["screen_wire_spacing", "screen_wire_diameter"] {
        assert!(has_diagnostic(
            &invalid,
            "InvalidNumericRange",
            "Below Bounds",
            Some(field)
        ));
    }
    assert!(has_diagnostic(
        &invalid,
        "InvalidEnumValue",
        "Bad Auto Token",
        Some("screen_beam_beam_solar_transmittance")
    ));

    let accepted = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "Trimmed Mixed Case Auto": {
                    "screen_beam_beam_solar_transmittance": "  aUtOcAlCuLaTe  ",
                    "screen_beam_diffuse_solar_transmittance": 0.0,
                    "screen_beam_diffuse_solar_reflectance": 0.0,
                    "screen_beam_beam_visible_transmittance": 0.0,
                    "screen_beam_diffuse_visible_transmittance": 0.0,
                    "screen_beam_diffuse_visible_reflectance": 0.0,
                    "screen_infrared_transmittance": "",
                    "screen_infrared_emissivity": "",
                    "screen_wire_spacing": "",
                    "screen_wire_diameter": ""
                }
            }
        }"#,
    )?);
    assert!(!accepted.has_errors(), "{:?}", accepted.report.diagnostics);
    let screen = accepted
        .model
        .as_ref()
        .and_then(|model| model.materials[0].as_window_screen_equivalent_layer())
        .ok_or_else(|| std::io::Error::other("missing mixed-case Auto screen"))?;
    assert_eq!(
        screen.front_solar.beam_beam_transmittance,
        AutoOrNumber::AutoCalculate
    );
    assert_eq!(screen.infrared_transmittance, 0.02);
    assert_eq!(screen.front_infrared_emissivity, 0.93);
    assert_eq!((screen.wire_spacing_m, screen.wire_diameter_m), (0.0, 0.0));
    Ok(())
}

#[test]
fn screen_equivalent_layer_preserves_blank_wire_geometry_and_rejects_recovery_cases()
-> Result<(), Box<dyn std::error::Error>> {
    let valid = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "A Omitted Geometry": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1
                },
                "B Blank Geometry": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_wire_spacing": "",
                    "screen_wire_diameter": ""
                },
                "C Blank Diameter": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_wire_spacing": 0.01,
                    "screen_wire_diameter": ""
                }
            }
        }"#,
    )?);
    assert!(!valid.has_errors(), "{:?}", valid.report.diagnostics);
    let model = valid
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected valid blank geometry cases"))?;
    for name in ["A OMITTED GEOMETRY", "B BLANK GEOMETRY"] {
        let screen = model
            .materials
            .iter()
            .find(|material| material.name.0 == name)
            .and_then(|material| material.as_window_screen_equivalent_layer())
            .ok_or_else(|| std::io::Error::other(format!("missing {name}")))?;
        assert_eq!((screen.wire_spacing_m, screen.wire_diameter_m), (0.0, 0.0));
    }
    let blank_diameter = model.materials[2]
        .as_window_screen_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing blank-diameter screen"))?;
    assert_eq!(
        (
            blank_diameter.wire_spacing_m,
            blank_diameter.wire_diameter_m
        ),
        (0.01, 0.0)
    );

    let invalid = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "Blank Spacing Explicit Diameter": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_wire_spacing": "",
                    "screen_wire_diameter": 0.002
                },
                "Equal Dimensions": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_wire_spacing": 0.002,
                    "screen_wire_diameter": 0.002
                }
            }
        }"#,
    )?);
    assert!(invalid.has_errors());
    for name in ["Blank Spacing Explicit Diameter", "Equal Dimensions"] {
        assert!(has_diagnostic(
            &invalid,
            "InvalidWindowScreenEquivalentLayerGeometry",
            name,
            Some("screen_wire_diameter")
        ));
    }
    Ok(())
}

#[test]
fn screen_equivalent_layer_matches_source_one_sided_openness_tolerance()
-> Result<(), Box<dyn std::error::Error>> {
    let accepted = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "A Lower Than Geometry": {
                    "screen_beam_beam_solar_transmittance": 0.50,
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_wire_spacing": 0.01,
                    "screen_wire_diameter": 0.002
                },
                "B Within One Percent": {
                    "screen_beam_beam_solar_transmittance": 0.645,
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_wire_spacing": 0.01,
                    "screen_wire_diameter": 0.002
                },
                "C Exact One Percent": {
                    "screen_beam_beam_solar_transmittance": 0.6464,
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_wire_spacing": 0.01,
                    "screen_wire_diameter": 0.002
                },
                "D Autocalculate Bypasses Mismatch": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_wire_spacing": 0.01,
                    "screen_wire_diameter": 0.002
                }
            }
        }"#,
    )?);
    assert!(
        !accepted.has_errors(),
        "one-sided accepted cases failed: {:?}",
        accepted.report.diagnostics
    );

    let rejected = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "Above One Percent": {
                    "screen_beam_beam_solar_transmittance": 0.65,
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_wire_spacing": 0.01,
                    "screen_wire_diameter": 0.002
                }
            }
        }"#,
    )?);
    assert!(rejected.has_errors());
    assert!(has_diagnostic(
        &rejected,
        "InvalidWindowScreenEquivalentLayerOpennessMismatch",
        "Above One Percent",
        Some("screen_beam_beam_solar_transmittance")
    ));
    Ok(())
}

#[test]
fn screen_equivalent_layer_enforces_only_source_solar_and_visible_sums()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "Solar Sum": {
                    "screen_beam_beam_solar_transmittance": 0.8,
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.2,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1
                },
                "Visible Sum": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.8,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.2
                }
            }
        }"#,
    )?);
    assert!(invalid.has_errors());
    let optical_sum_objects = invalid
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "InvalidWindowScreenEquivalentLayerOpticalSum"
                && diagnostic.object_type == OBJECT_TYPE
        })
        .filter_map(|diagnostic| diagnostic.object_name.as_deref())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        optical_sum_objects,
        BTreeSet::from(["Solar Sum", "Visible Sum"])
    );

    let accepted = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "Diffuse Transmittances And Thermal Sum Are Excluded": {
                    "screen_beam_beam_solar_transmittance": 0.2,
                    "screen_beam_diffuse_solar_transmittance": 0.9,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.2,
                    "screen_beam_diffuse_visible_transmittance": 0.9,
                    "screen_beam_diffuse_visible_reflectance": 0.1,
                    "screen_infrared_transmittance": 0.2,
                    "screen_infrared_emissivity": 0.9
                },
                "Autocalculate Bypasses Solar Sum": {
                    "screen_beam_diffuse_solar_transmittance": 0.99,
                    "screen_beam_diffuse_solar_reflectance": 0.99,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1
                }
            }
        }"#,
    )?);
    assert!(
        !accepted.has_errors(),
        "source-excluded sums must compile: {:?}",
        accepted.report.diagnostics
    );
    assert_eq!(
        accepted.model.as_ref().map(|model| model.materials.len()),
        Some(2)
    );
    Ok(())
}

#[test]
fn screen_equivalent_layer_uses_shared_material_namespace_and_rejects_blank_names()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen": {
                "Shared": {
                    "diffuse_solar_reflectance": 0.1,
                    "diffuse_visible_reflectance": 0.1,
                    "screen_material_spacing": 0.002,
                    "screen_material_diameter": 0.0005
                }
            },
            "WindowMaterial:Screen:EquivalentLayer": {
                "shared": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1
                },
                "": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1
                }
            }
        }"#,
    )?);

    assert!(result.has_errors());
    assert!(has_diagnostic(&result, "DuplicateName", "shared", None));
    assert!(has_diagnostic(
        &result,
        "MissingRequiredField",
        "",
        Some("name")
    ));
    Ok(())
}

#[test]
fn screen_equivalent_layer_fails_closed_in_ordinary_construction()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Screen:EquivalentLayer": {
                "Equivalent Screen": {
                    "screen_beam_diffuse_solar_transmittance": 0.1,
                    "screen_beam_diffuse_solar_reflectance": 0.1,
                    "screen_beam_beam_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_transmittance": 0.1,
                    "screen_beam_diffuse_visible_reflectance": 0.1
                }
            },
            "Construction": {
                "Wrong Window": {"outside_layer": "Equivalent Screen"}
            }
        }"#,
    )?);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEquivalentLayerConstruction"
            && diagnostic.object_type == "Construction"
            && diagnostic.object_name.as_deref() == Some("Wrong Window")
            && diagnostic.field.as_deref() == Some("outside_layer")
    }));
    Ok(())
}
