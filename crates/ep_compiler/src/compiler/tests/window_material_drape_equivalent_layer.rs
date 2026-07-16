use super::super::{CompileResult, ObjectCoverageStatus, compile_raw_model, typed_coverage_status};
use ep_model::{
    MaterialFamily, MaterialKind, MaterialSurfaceRoughness,
    WindowShadeEquivalentLayerSideOpticalProperties,
};
use ep_raw_model::parse_epjson_str;
use std::collections::BTreeSet;

const OBJECT_TYPE: &str = "WindowMaterial:Drape:EquivalentLayer";

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
fn drape_equivalent_layer_materializes_all_inputs_defaults_and_source_quirks()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Shade Before Drape": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1
                }
            },
            "WindowMaterial:Drape:EquivalentLayer": {
                "Z Full EQL Drape": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.15,
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.10,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.70,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.12,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.20,
                    "drape_beam_beam_visible_transmittance": 0.21,
                    "drape_beam_diffuse_visible_transmittance": 0.22,
                    "drape_beam_diffuse_visible_reflectance": 0.23,
                    "drape_material_infrared_transmittance": 0.10,
                    "front_side_drape_material_infrared_emissivity": 0.90,
                    "back_side_drape_material_infrared_emissivity": 0.95,
                    "width_of_pleated_fabric": 0.03,
                    "length_of_pleated_fabric": 0.05
                },
                "A Default EQL Drape": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.10,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.20,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.30,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.40
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
        .ok_or_else(|| std::io::Error::other("expected typed equivalent-layer drapes"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "SHADE BEFORE DRAPE",
            "A DEFAULT EQL DRAPE",
            "Z FULL EQL DRAPE"
        ],
        "equivalent-layer drapes must follow equivalent-layer shades"
    );

    let material = &model.materials[2];
    assert_eq!(material.kind(), MaterialKind::WindowDrapeEquivalentLayer);
    assert_eq!(material.family(), MaterialFamily::EquivalentLayer);
    assert!(material.as_opaque().is_none());
    assert!(material.as_window_shade_equivalent_layer().is_none());
    assert_eq!(material.thickness_m(), None);
    assert_eq!(material.conductivity_w_per_m_k(), None);
    assert_eq!(material.density_kg_per_m3(), None);
    assert_eq!(material.specific_heat_j_per_kg_k(), None);
    assert_eq!(material.thermal_resistance(), None);
    assert_eq!(material.heat_capacity_per_area(), None);
    assert_eq!(material.thermal_absorptance(), None);
    assert_eq!(material.solar_absorptance(), None);
    assert_eq!(material.visible_absorptance(), None);
    let drape = material
        .as_window_drape_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer drape payload"))?;
    assert_eq!(drape.roughness, MaterialSurfaceRoughness::MediumRough);
    assert!(drape.is_resistance_only());
    assert_eq!(drape.nominal_thermal_resistance_m2_k_per_w(), None);
    assert_eq!(
        drape.front_solar,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.15,
            beam_diffuse_transmittance: 0.10,
            beam_diffuse_reflectance: 0.12,
        }
    );
    assert_eq!(
        drape.back_solar,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.15,
            beam_diffuse_transmittance: 0.70,
            beam_diffuse_reflectance: 0.20,
        },
        "EnergyPlus 26.1 does not validate the back-side solar sum"
    );
    assert_eq!(
        drape.front_visible,
        WindowShadeEquivalentLayerSideOpticalProperties {
            beam_beam_transmittance: 0.21,
            beam_diffuse_transmittance: 0.22,
            beam_diffuse_reflectance: 0.23,
        }
    );
    assert_eq!(
        drape.back_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default(),
        "EnergyPlus 26.1 leaves all back-side visible TAR slots at zero"
    );
    assert_eq!(drape.infrared_transmittance, 0.10);
    assert_eq!(drape.front_infrared_emissivity, 0.90);
    assert_eq!(drape.back_infrared_emissivity, 0.95);
    assert_eq!(drape.front_thermal_absorptance, 0.90);
    assert_eq!(drape.back_thermal_absorptance, 0.95);
    assert_eq!(drape.thermal_transmittance, 0.10);
    assert_eq!(drape.pleated_width_m, 0.03);
    assert_eq!(drape.pleated_length_m, 0.05);
    assert!(drape.is_pleated());

    let default = model.materials[1]
        .as_window_drape_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("missing defaulted equivalent-layer drape"))?;
    assert_eq!(default.front_solar.beam_beam_transmittance, 0.0);
    assert_eq!(default.back_solar.beam_beam_transmittance, 0.0);
    assert_eq!(
        default.front_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(
        default.back_visible,
        WindowShadeEquivalentLayerSideOpticalProperties::default()
    );
    assert_eq!(default.infrared_transmittance, 0.05);
    assert_eq!(default.front_infrared_emissivity, 0.87);
    assert_eq!(default.back_infrared_emissivity, 0.87);
    assert_eq!(default.pleated_width_m, 0.0);
    assert_eq!(default.pleated_length_m, 0.0);
    assert!(!default.is_pleated());
    for (field, value) in [
        (
            "drape_beam_beam_solar_transmittance_at_normal_incidence",
            "0.0",
        ),
        ("drape_beam_beam_visible_transmittance", "0.0"),
        ("drape_beam_diffuse_visible_transmittance", "0.0"),
        ("drape_beam_diffuse_visible_reflectance", "0.0"),
        ("drape_material_infrared_transmittance", "0.05"),
        ("front_side_drape_material_infrared_emissivity", "0.87"),
        ("back_side_drape_material_infrared_emissivity", "0.87"),
        ("width_of_pleated_fabric", "0.0"),
        ("length_of_pleated_fabric", "0.0"),
    ] {
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == OBJECT_TYPE
                && default.object_name == "A Default EQL Drape"
                && default.field == field
                && default.value == value
        }));
    }

    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer drape coverage"))?;
    assert_eq!(coverage.object_count, 2);
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    Ok(())
}

#[test]
fn drape_equivalent_layer_requires_only_source_required_n2_through_n5()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Missing Required": {}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

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
            "back_side_drape_beam_diffuse_solar_reflectance".to_string(),
            "back_side_drape_beam_diffuse_solar_transmittance".to_string(),
            "front_side_drape_beam_diffuse_solar_reflectance".to_string(),
            "front_side_drape_beam_diffuse_solar_transmittance".to_string(),
        ])
    );
    Ok(())
}

#[test]
fn drape_equivalent_layer_enforces_all_schema_bounds() -> Result<(), Box<dyn std::error::Error>> {
    let inclusive = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Inclusive Endpoints": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.2,
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.0,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.0,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.0,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.0,
                    "drape_beam_beam_visible_transmittance": 0.0,
                    "drape_beam_diffuse_visible_transmittance": 0.0,
                    "drape_beam_diffuse_visible_reflectance": 0.0,
                    "drape_material_infrared_transmittance": 0.0,
                    "front_side_drape_material_infrared_emissivity": 0.5,
                    "back_side_drape_material_infrared_emissivity": 0.5,
                    "width_of_pleated_fabric": 0.0,
                    "length_of_pleated_fabric": 0.0
                }
            }
        }"#,
    )?;
    let inclusive_result = compile_raw_model(&inclusive);
    assert!(
        !inclusive_result.has_errors(),
        "inclusive zero and N1=0.2 endpoints must compile: {:?}",
        inclusive_result.report.diagnostics
    );

    let invalid = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Below Bounds": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": -0.01,
                    "front_side_drape_beam_diffuse_solar_transmittance": -0.01,
                    "back_side_drape_beam_diffuse_solar_transmittance": -0.01,
                    "front_side_drape_beam_diffuse_solar_reflectance": -0.01,
                    "back_side_drape_beam_diffuse_solar_reflectance": -0.01,
                    "drape_beam_beam_visible_transmittance": -0.01,
                    "drape_beam_diffuse_visible_transmittance": -0.01,
                    "drape_beam_diffuse_visible_reflectance": -0.01,
                    "drape_material_infrared_transmittance": -0.01,
                    "front_side_drape_material_infrared_emissivity": 0.0,
                    "back_side_drape_material_infrared_emissivity": 0.0,
                    "width_of_pleated_fabric": -0.01,
                    "length_of_pleated_fabric": -0.01
                },
                "Above Bounds": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.200001,
                    "front_side_drape_beam_diffuse_solar_transmittance": 1.0,
                    "back_side_drape_beam_diffuse_solar_transmittance": 1.0,
                    "front_side_drape_beam_diffuse_solar_reflectance": 1.0,
                    "back_side_drape_beam_diffuse_solar_reflectance": 1.0,
                    "drape_beam_beam_visible_transmittance": 1.0,
                    "drape_beam_diffuse_visible_transmittance": 1.0,
                    "drape_beam_diffuse_visible_reflectance": 1.0,
                    "drape_material_infrared_transmittance": 1.0,
                    "front_side_drape_material_infrared_emissivity": 1.0,
                    "back_side_drape_material_infrared_emissivity": 1.0,
                    "width_of_pleated_fabric": 0.0,
                    "length_of_pleated_fabric": 0.0
                }
            }
        }"#,
    )?;
    let invalid_result = compile_raw_model(&invalid);
    assert!(invalid_result.has_errors());
    let bounded_above_and_below_fields = [
        "drape_beam_beam_solar_transmittance_at_normal_incidence",
        "front_side_drape_beam_diffuse_solar_transmittance",
        "back_side_drape_beam_diffuse_solar_transmittance",
        "front_side_drape_beam_diffuse_solar_reflectance",
        "back_side_drape_beam_diffuse_solar_reflectance",
        "drape_beam_beam_visible_transmittance",
        "drape_beam_diffuse_visible_transmittance",
        "drape_beam_diffuse_visible_reflectance",
        "drape_material_infrared_transmittance",
        "front_side_drape_material_infrared_emissivity",
        "back_side_drape_material_infrared_emissivity",
    ];
    for object_name in ["Below Bounds", "Above Bounds"] {
        for field in bounded_above_and_below_fields {
            assert!(
                has_diagnostic(
                    &invalid_result,
                    "InvalidNumericRange",
                    object_name,
                    Some(field)
                ),
                "missing {object_name}/{field} range diagnostic"
            );
        }
    }
    for field in ["width_of_pleated_fabric", "length_of_pleated_fabric"] {
        assert!(
            has_diagnostic(
                &invalid_result,
                "InvalidNumericRange",
                "Below Bounds",
                Some(field)
            ),
            "missing Below Bounds/{field} range diagnostic"
        );
    }
    Ok(())
}

#[test]
fn drape_equivalent_layer_enforces_only_three_source_optical_sums()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Front Solar Sum": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.2,
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.4,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.4,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                },
                "Visible Sum": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "drape_beam_beam_visible_transmittance": 0.3,
                    "drape_beam_diffuse_visible_transmittance": 0.3,
                    "drape_beam_diffuse_visible_reflectance": 0.4
                },
                "Front Infrared Sum": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "drape_material_infrared_transmittance": 0.2,
                    "front_side_drape_material_infrared_emissivity": 0.81,
                    "back_side_drape_material_infrared_emissivity": 0.5
                }
            }
        }"#,
    )?;
    let invalid_result = compile_raw_model(&invalid);
    assert!(invalid_result.has_errors());
    let diagnostics = invalid_result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "InvalidWindowDrapeEquivalentLayerOpticalSum"
                && diagnostic.object_type == OBJECT_TYPE
        })
        .collect::<Vec<_>>();
    assert_eq!(diagnostics.len(), 3, "{diagnostics:?}");
    assert_eq!(
        diagnostics
            .iter()
            .filter_map(|diagnostic| diagnostic.object_name.as_deref())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["Front Infrared Sum", "Front Solar Sum", "Visible Sum"])
    );

    let unchecked_back_and_front_equality = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Unchecked Back And Front Equality": {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.2,
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.2,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.7,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.7,
                    "drape_material_infrared_transmittance": 0.1,
                    "front_side_drape_material_infrared_emissivity": 0.9,
                    "back_side_drape_material_infrared_emissivity": 0.95
                }
            }
        }"#,
    )?;
    let accepted_result = compile_raw_model(&unchecked_back_and_front_equality);
    assert!(
        !accepted_result.has_errors(),
        "front IR equality and unchecked back-side sums must compile: {:?}",
        accepted_result.report.diagnostics
    );
    Ok(())
}

#[test]
fn drape_equivalent_layer_applies_all_or_nothing_effective_pleat_geometry()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "A Both Positive": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "width_of_pleated_fabric": 0.02,
                    "length_of_pleated_fabric": 0.03
                },
                "B Both Default": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                },
                "C Width Only": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "width_of_pleated_fabric": 0.02
                },
                "D Length Only": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "length_of_pleated_fabric": 0.03
                },
                "E Zero Width": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "width_of_pleated_fabric": 0.0,
                    "length_of_pleated_fabric": 0.03
                },
                "F Zero Length": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "width_of_pleated_fabric": 0.02,
                    "length_of_pleated_fabric": 0.0
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed pleat cases"))?;

    for (name, width, length, is_pleated) in [
        ("A BOTH POSITIVE", 0.02, 0.03, true),
        ("B BOTH DEFAULT", 0.0, 0.0, false),
        ("C WIDTH ONLY", 0.0, 0.0, false),
        ("D LENGTH ONLY", 0.0, 0.0, false),
        ("E ZERO WIDTH", 0.0, 0.0, false),
        ("F ZERO LENGTH", 0.0, 0.0, false),
    ] {
        let drape = model
            .materials
            .iter()
            .find(|material| material.name.0 == name)
            .and_then(|material| material.as_window_drape_equivalent_layer())
            .ok_or_else(|| std::io::Error::other(format!("missing pleat case {name}")))?;
        assert_eq!(drape.pleated_width_m, width, "{name}");
        assert_eq!(drape.pleated_length_m, length, "{name}");
        assert_eq!(drape.is_pleated(), is_pleated, "{name}");
    }
    Ok(())
}

#[test]
fn drape_equivalent_layer_uses_shared_material_namespace_and_rejects_blank_names()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "Shared": {
                    "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_shade_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_shade_beam_diffuse_solar_reflectance": 0.1
                }
            },
            "WindowMaterial:Drape:EquivalentLayer": {
                "shared": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                },
                "": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

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
fn drape_equivalent_layer_fails_closed_in_ordinary_construction()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Drape:EquivalentLayer": {
                "Equivalent Drape": {
                    "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance": 0.1,
                    "front_side_drape_beam_diffuse_solar_reflectance": 0.1,
                    "back_side_drape_beam_diffuse_solar_reflectance": 0.1
                }
            },
            "Construction": {
                "Wrong Window": {"outside_layer": "Equivalent Drape"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEquivalentLayerConstruction"
            && diagnostic.object_type == "Construction"
            && diagnostic.object_name.as_deref() == Some("Wrong Window")
            && diagnostic.field.as_deref() == Some("outside_layer")
    }));
    Ok(())
}
