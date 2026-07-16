use super::super::{ObjectCoverageStatus, compile_raw_model};
use ep_model::{AutoOrNumber, MaterialDefinition, MaterialFamily, MaterialId, MaterialKind};
use ep_raw_model::parse_epjson_str;

fn equivalent_layer_fields(overrides: &str) -> String {
    let suffix = if overrides.is_empty() {
        String::new()
    } else {
        format!(",{overrides}")
    };
    format!(
        r#""front_side_beam_beam_solar_transmittance":0.61,
           "back_side_beam_beam_solar_transmittance":0.62,
           "front_side_beam_beam_solar_reflectance":0.21,
           "back_side_beam_beam_solar_reflectance":0.22{suffix}"#
    )
}

#[test]
fn equivalent_layer_materializes_all_inputs_and_preserves_source_order()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "Equivalent Glass": {{{}}}
            }},
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {{
                "Alternative Glass": {{
                    "thickness":0.003,
                    "solar_index_of_refraction":1.5,
                    "solar_extinction_coefficient":20.0,
                    "visible_index_of_refraction":1.6,
                    "visible_extinction_coefficient":30.0
                }}
            }},
            "WindowMaterial:Glazing": {{
                "Direct Glass": {{
                    "optical_data_type":"SpectralAverage",
                    "thickness":0.004
                }}
            }},
            "Material": {{
                "Opaque": {{
                    "roughness":"Rough","thickness":0.1,"conductivity":1.0,
                    "density":1000.0,"specific_heat":1000.0
                }}
            }}
        }}"#,
        equivalent_layer_fields(
            r#""optical_data_type":"SpectralAverage",
               "window_glass_spectral_data_set_name":"Ignored For SpectralAverage",
               "front_side_beam_beam_visible_solar_transmittance":0.71,
               "back_side_beam_beam_visible_solar_transmittance":0.72,
               "front_side_beam_beam_visible_solar_reflectance":0.11,
               "back_side_beam_beam_visible_solar_reflectance":0.12,
               "front_side_beam_diffuse_solar_transmittance":0.31,
               "back_side_beam_diffuse_solar_transmittance":0.32,
               "front_side_beam_diffuse_solar_reflectance":0.23,
               "back_side_beam_diffuse_solar_reflectance":0.24,
               "front_side_beam_diffuse_visible_solar_transmittance":0.41,
               "back_side_beam_diffuse_visible_solar_transmittance":0.42,
               "front_side_beam_diffuse_visible_solar_reflectance":0.13,
               "back_side_beam_diffuse_visible_solar_reflectance":0.14,
               "diffuse_diffuse_solar_transmittance":0.51,
               "front_side_diffuse_diffuse_solar_reflectance":0.25,
               "back_side_diffuse_diffuse_solar_reflectance":0.26,
               "diffuse_diffuse_visible_solar_transmittance":0.52,
               "front_side_diffuse_diffuse_visible_solar_reflectance":0.15,
               "back_side_diffuse_diffuse_visible_solar_reflectance":0.16,
               "infrared_transmittance_applies_to_front_and_back_":0.03,
               "front_side_infrared_emissivity":0.81,
               "back_side_infrared_emissivity":0.82,
               "thermal_resistance":0.17"#,
        )
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed equivalent-layer glazing"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "OPAQUE",
            "DIRECT GLASS",
            "ALTERNATIVE GLASS",
            "EQUIVALENT GLASS"
        ],
        "material IDs must follow EnergyPlus family source order"
    );
    let glazing = &model.materials[3];
    assert_eq!(glazing.id, MaterialId(3));
    assert_eq!(glazing.kind(), MaterialKind::WindowGlazingEquivalentLayer);
    assert_eq!(glazing.family(), MaterialFamily::EquivalentLayer);
    assert!(glazing.as_opaque().is_none());
    assert_eq!(glazing.thickness_m(), None);
    assert_eq!(glazing.conductivity_w_per_m_k(), None);
    assert_eq!(glazing.thermal_resistance(), None);
    assert_eq!(glazing.heat_capacity_per_area(), None);

    let fields = glazing
        .as_window_glazing_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("expected equivalent-layer payload"))?;
    assert_eq!(fields.solar.beam_beam.front_transmittance, 0.61);
    assert_eq!(fields.solar.beam_beam.back_transmittance, 0.62);
    assert_eq!(fields.solar.beam_beam.front_reflectance, 0.21);
    assert_eq!(fields.solar.beam_beam.back_reflectance, 0.22);
    assert_eq!(fields.visible.beam_beam.front_transmittance, 0.71);
    assert_eq!(fields.visible.beam_beam.back_transmittance, 0.72);
    assert_eq!(fields.visible.beam_beam.front_reflectance, 0.11);
    assert_eq!(fields.visible.beam_beam.back_reflectance, 0.12);
    assert_eq!(fields.solar.beam_diffuse.front_transmittance, 0.31);
    assert_eq!(fields.solar.beam_diffuse.back_transmittance, 0.32);
    assert_eq!(fields.solar.beam_diffuse.front_reflectance, 0.23);
    assert_eq!(fields.solar.beam_diffuse.back_reflectance, 0.24);
    assert_eq!(fields.visible.beam_diffuse.front_transmittance, 0.41);
    assert_eq!(fields.visible.beam_diffuse.back_transmittance, 0.42);
    assert_eq!(fields.visible.beam_diffuse.front_reflectance, 0.13);
    assert_eq!(fields.visible.beam_diffuse.back_reflectance, 0.14);
    assert_eq!(
        fields.solar.diffuse_diffuse.transmittance,
        AutoOrNumber::Value(0.51)
    );
    assert_eq!(
        fields.solar.diffuse_diffuse.front_reflectance,
        AutoOrNumber::Value(0.25)
    );
    assert_eq!(
        fields.solar.diffuse_diffuse.back_reflectance,
        AutoOrNumber::Value(0.26)
    );
    assert_eq!(
        fields.visible.diffuse_diffuse.transmittance,
        AutoOrNumber::Value(0.52)
    );
    assert_eq!(
        fields.visible.diffuse_diffuse.front_reflectance,
        AutoOrNumber::Value(0.15)
    );
    assert_eq!(
        fields.visible.diffuse_diffuse.back_reflectance,
        AutoOrNumber::Value(0.16)
    );
    assert_eq!(fields.infrared_transmittance, 0.03);
    assert_eq!(fields.front_infrared_emissivity, 0.81);
    assert_eq!(fields.back_infrared_emissivity, 0.82);
    assert_eq!(fields.thermal_resistance_m2_k_per_w, 0.17);

    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == "WindowMaterial:Glazing:EquivalentLayer")
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer coverage row"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    Ok(())
}

#[test]
fn equivalent_layer_applies_energyplus_defaults() -> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "Defaults": {{{}}}
            }}
        }}"#,
        equivalent_layer_fields("")
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected defaulted equivalent-layer glazing"))?;
    let MaterialDefinition::WindowGlazingEquivalentLayer(fields) = model.materials[0].definition
    else {
        return Err(std::io::Error::other("expected equivalent-layer glazing variant").into());
    };
    assert_eq!(fields.visible.beam_beam.front_transmittance, 0.0);
    assert_eq!(fields.visible.beam_beam.back_reflectance, 0.0);
    assert_eq!(fields.solar.beam_diffuse.front_transmittance, 0.0);
    assert_eq!(fields.visible.beam_diffuse.back_reflectance, 0.0);
    assert_eq!(
        fields.solar.diffuse_diffuse.transmittance,
        AutoOrNumber::AutoCalculate
    );
    assert_eq!(
        fields.visible.diffuse_diffuse.back_reflectance,
        AutoOrNumber::AutoCalculate
    );
    assert_eq!(fields.infrared_transmittance, 0.0);
    assert_eq!(fields.front_infrared_emissivity, 0.84);
    assert_eq!(fields.back_infrared_emissivity, 0.84);
    assert_eq!(fields.thermal_resistance_m2_k_per_w, 0.158);
    for field in [
        "optical_data_type",
        "front_side_beam_diffuse_solar_transmittance",
        "diffuse_diffuse_solar_transmittance",
        "front_side_infrared_emissivity",
        "thermal_resistance",
    ] {
        assert!(
            result.report.defaults_applied.iter().any(|entry| {
                entry.object_type == "WindowMaterial:Glazing:EquivalentLayer"
                    && entry.object_name == "Defaults"
                    && entry.field == field
            }),
            "missing default application for {field}"
        );
    }
    Ok(())
}

#[test]
fn equivalent_layer_preserves_explicit_auto_and_zero_without_inventing_sum_constraints()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "Source Actual": {{{}}}
            }}
        }}"#,
        equivalent_layer_fields(
            r#""front_side_beam_beam_visible_solar_transmittance":0.9,
               "front_side_beam_beam_visible_solar_reflectance":0.9,
               "diffuse_diffuse_solar_transmittance":"Autocalculate",
               "front_side_diffuse_diffuse_solar_reflectance":0.0,
               "infrared_transmittance_applies_to_front_and_back_":0.8,
               "front_side_infrared_emissivity":0.8"#,
        )
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(
        !result.has_errors(),
        "EquivalentLayer has individual bounds but no source optical-sum gate: {:?}",
        result.report.diagnostics
    );
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected source-actual equivalent layer"))?;
    let fields = model.materials[0]
        .as_window_glazing_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("expected equivalent-layer payload"))?;
    assert_eq!(
        fields.solar.diffuse_diffuse.transmittance,
        AutoOrNumber::AutoCalculate
    );
    assert_eq!(
        fields.solar.diffuse_diffuse.front_reflectance,
        AutoOrNumber::Value(0.0)
    );
    assert_eq!(fields.infrared_transmittance, 0.8);
    assert_eq!(fields.front_infrared_emissivity, 0.8);
    Ok(())
}

#[test]
fn equivalent_layer_enforces_required_and_optional_bounds() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing:EquivalentLayer": {
                "Missing": {},
                "Bad Required": {
                    "front_side_beam_beam_solar_transmittance":-0.1,
                    "back_side_beam_beam_solar_transmittance":1.1,
                    "front_side_beam_beam_solar_reflectance":-0.2,
                    "back_side_beam_beam_solar_reflectance":1.2
                },
                "Bad Optional": {
                    "front_side_beam_beam_solar_transmittance":0.5,
                    "back_side_beam_beam_solar_transmittance":0.5,
                    "front_side_beam_beam_solar_reflectance":0.2,
                    "back_side_beam_beam_solar_reflectance":0.2,
                    "front_side_beam_beam_visible_solar_transmittance":-0.1,
                    "back_side_beam_diffuse_solar_reflectance":1.1,
                    "diffuse_diffuse_solar_transmittance":-0.1,
                    "front_side_diffuse_diffuse_visible_solar_reflectance":1.1,
                    "infrared_transmittance_applies_to_front_and_back_":1.1,
                    "front_side_infrared_emissivity":0.0,
                    "back_side_infrared_emissivity":1.0,
                    "thermal_resistance":0.0
                },
                "Bad Autocalculate": {
                    "front_side_beam_beam_solar_transmittance":0.5,
                    "back_side_beam_beam_solar_transmittance":0.5,
                    "front_side_beam_beam_solar_reflectance":0.2,
                    "back_side_beam_beam_solar_reflectance":0.2,
                    "diffuse_diffuse_solar_transmittance":"Autosize"
                }
            }
        }"#,
    )
    .expect("invalid equivalent-layer inputs should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for field in [
        "front_side_beam_beam_solar_transmittance",
        "back_side_beam_beam_solar_transmittance",
        "front_side_beam_beam_solar_reflectance",
        "back_side_beam_beam_solar_reflectance",
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "MissingRequiredField"
                && diagnostic.object_name.as_deref() == Some("Missing")
                && diagnostic.field.as_deref() == Some(field)
        }));
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidNumericRange"
                && diagnostic.object_name.as_deref() == Some("Bad Required")
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    for field in [
        "front_side_beam_beam_visible_solar_transmittance",
        "back_side_beam_diffuse_solar_reflectance",
        "diffuse_diffuse_solar_transmittance",
        "front_side_diffuse_diffuse_visible_solar_reflectance",
        "infrared_transmittance_applies_to_front_and_back_",
        "front_side_infrared_emissivity",
        "back_side_infrared_emissivity",
        "thermal_resistance",
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidNumericRange"
                && diagnostic.object_name.as_deref() == Some("Bad Optional")
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_name.as_deref() == Some("Bad Autocalculate")
            && diagnostic.field.as_deref() == Some("diffuse_diffuse_solar_transmittance")
    }));
}

#[test]
fn equivalent_layer_rejects_spectral_and_invalid_optical_modes() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing:EquivalentLayer": {
                "Spectral": {"optical_data_type":"Spectral"},
                "Bad Enum": {"optical_data_type":"BSDF"},
                "Bad Type": {"optical_data_type":42}
            }
        }"#,
    )
    .expect("invalid optical modes should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedWindowGlazingEquivalentLayerOpticalDataType"
            && diagnostic.object_name.as_deref() == Some("Spectral")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_name.as_deref() == Some("Bad Enum")
            && diagnostic.field.as_deref() == Some("optical_data_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidFieldType"
            && diagnostic.object_name.as_deref() == Some("Bad Type")
            && diagnostic.field.as_deref() == Some("optical_data_type")
    }));
}

#[test]
fn equivalent_layer_shares_names_with_prior_material_families() {
    let epjson = format!(
        r#"{{
            "Material": {{
                "Shared": {{
                    "roughness":"Rough","thickness":0.1,"conductivity":1.0,
                    "density":1000.0,"specific_heat":1000.0
                }}
            }},
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "shared": {{{}}}
            }}
        }}"#,
        equivalent_layer_fields("")
    );
    let result = compile_raw_model(
        &parse_epjson_str(&epjson).expect("duplicate equivalent-layer name should parse"),
    );
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == "WindowMaterial:Glazing:EquivalentLayer"
            && diagnostic.object_name.as_deref() == Some("shared")
    }));
}

#[test]
fn equivalent_layer_fails_closed_outside_its_deferred_construction_type() {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:EquivalentLayer": {{
                "Equivalent Glass": {{{}}}
            }},
            "Construction": {{
                "Wrong Window": {{"outside_layer":"Equivalent Glass"}}
            }},
            "BuildingSurface:Detailed": {{
                "Wrong Surface": {{
                    "surface_type":"Wall",
                    "construction_name":"Wrong Window"
                }}
            }}
        }}"#,
        equivalent_layer_fields("")
    );
    let result = compile_raw_model(
        &parse_epjson_str(&epjson).expect("equivalent-layer boundary should parse"),
    );

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEquivalentLayerConstruction"
            && diagnostic.object_type == "Construction"
            && diagnostic.object_name.as_deref() == Some("Wrong Window")
            && diagnostic.field.as_deref() == Some("outside_layer")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_type == "BuildingSurface:Detailed"
            && diagnostic.object_name.as_deref() == Some("Wrong Surface")
            && diagnostic.field.as_deref() == Some("construction_name")
    }));
}
