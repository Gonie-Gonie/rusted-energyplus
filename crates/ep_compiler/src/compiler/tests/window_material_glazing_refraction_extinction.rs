use super::super::{ObjectCoverageStatus, compile_raw_model};
use ep_model::{ConstructionKind, MaterialDefinition, MaterialFamily, MaterialId, MaterialKind};
use ep_raw_model::parse_epjson_str;

fn refraction_extinction_fields(overrides: &str) -> String {
    let suffix = if overrides.is_empty() {
        String::new()
    } else {
        format!(",{overrides}")
    };
    format!(
        r#""thickness":0.003,
           "solar_index_of_refraction":1.5,
           "solar_extinction_coefficient":20.0,
           "visible_index_of_refraction":1.7,
           "visible_extinction_coefficient":100.0{suffix}"#
    )
}

#[test]
fn refraction_extinction_materializes_inputs_derives_v261_optics_and_preserves_source_order()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {{
                "Alternative Glass": {{{}}}
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
            }},
            "Construction": {{
                "Alternative Window": {{"outside_layer":"Alternative Glass"}}
            }}
        }}"#,
        refraction_extinction_fields(
            r#""infrared_transmittance_at_normal_incidence":0.03,
               "infrared_hemispherical_emissivity":0.81,
               "conductivity":1.2,
               "dirt_correction_factor_for_solar_and_visible_transmittance":0.93,
               "solar_diffusing":"Yes""#,
        )
    );
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed refraction glazing"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["OPAQUE", "DIRECT GLASS", "ALTERNATIVE GLASS"],
        "material IDs must follow EnergyPlus family source order"
    );
    let glazing = &model.materials[2];
    assert_eq!(glazing.id, MaterialId(2));
    assert_eq!(
        glazing.kind(),
        MaterialKind::WindowGlazingRefractionExtinction
    );
    assert_eq!(glazing.family(), MaterialFamily::Fenestration);
    assert!(glazing.as_opaque().is_none());
    assert_eq!(glazing.thickness_m(), None);
    assert_eq!(glazing.conductivity_w_per_m_k(), None);
    assert_eq!(glazing.thermal_resistance(), None);
    assert_eq!(glazing.heat_capacity_per_area(), None);

    let fields = glazing
        .as_window_glazing_refraction_extinction()
        .ok_or_else(|| std::io::Error::other("expected refraction/extinction payload"))?;
    assert_eq!(fields.thickness_m, 0.003);
    assert_eq!(fields.solar_index_of_refraction, 1.5);
    assert_eq!(fields.solar_extinction_coefficient_per_m, 20.0);
    assert_eq!(fields.visible_index_of_refraction, 1.7);
    assert_eq!(fields.visible_extinction_coefficient_per_m, 100.0);
    assert_eq!(fields.infrared_transmittance_at_normal_incidence, 0.03);
    assert_eq!(fields.infrared_hemispherical_emissivity, 0.81);
    assert_eq!(fields.conductivity_w_per_m_k, 1.2);
    assert_eq!(
        fields.dirt_correction_factor_for_solar_and_visible_transmittance,
        0.93
    );
    assert!(fields.solar_diffusing);

    let optics = fields.normal_incidence_optical_properties();
    assert!((optics.solar_transmittance_at_normal_incidence - 0.86916).abs() < 0.00001);
    assert!((optics.front_side_solar_reflectance_at_normal_incidence - 0.0727419).abs() < 0.00001);
    assert_eq!(
        optics.back_side_solar_reflectance_at_normal_incidence,
        optics.front_side_solar_reflectance_at_normal_incidence
    );
    assert!((optics.visible_transmittance_at_normal_incidence - 0.64618).abs() < 0.00001);
    assert!(
        (optics.front_side_visible_reflectance_at_normal_incidence - 0.0993914).abs() < 0.00001
    );
    assert_eq!(
        optics.back_side_visible_reflectance_at_normal_incidence,
        optics.front_side_solar_reflectance_at_normal_incidence,
        "EnergyPlus 26.1 copies solar-front reflectance into visible-back"
    );

    assert_eq!(model.constructions[0].kind, ConstructionKind::Fenestration);
    assert_eq!(model.constructions[0].layers, vec![MaterialId(2)]);
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == "WindowMaterial:Glazing:RefractionExtinctionMethod")
        .ok_or_else(|| std::io::Error::other("missing refraction coverage row"))?;
    assert_eq!(
        coverage.status,
        ObjectCoverageStatus::Typed,
        "the complete RefractionExtinctionMethod object is typed"
    );
    Ok(())
}

#[test]
fn refraction_extinction_applies_energyplus_defaults() -> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {{
                "Defaults": {{{}}}
            }}
        }}"#,
        refraction_extinction_fields("")
    );
    let result = compile_raw_model(
        &parse_epjson_str(&epjson).expect("default refraction glazing should parse"),
    );

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .expect("default refraction glazing should compile");
    let MaterialDefinition::WindowGlazingRefractionExtinction(fields) =
        model.materials[0].definition
    else {
        return Err(std::io::Error::other("expected refraction/extinction glazing variant").into());
    };
    assert_eq!(fields.infrared_transmittance_at_normal_incidence, 0.0);
    assert_eq!(fields.infrared_hemispherical_emissivity, 0.84);
    assert_eq!(fields.conductivity_w_per_m_k, 0.9);
    assert_eq!(
        fields.dirt_correction_factor_for_solar_and_visible_transmittance,
        1.0
    );
    assert!(!fields.solar_diffusing);
    Ok(())
}

#[test]
fn refraction_extinction_enforces_required_fields_bounds_and_infrared_sum() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {
                "Missing": {},
                "Bad Bounds": {
                    "thickness":0.0,
                    "solar_index_of_refraction":1.0,
                    "solar_extinction_coefficient":0.0,
                    "visible_index_of_refraction":1.0,
                    "visible_extinction_coefficient":0.0,
                    "infrared_transmittance_at_normal_incidence":1.0,
                    "infrared_hemispherical_emissivity":0.0,
                    "conductivity":0.0,
                    "dirt_correction_factor_for_solar_and_visible_transmittance":0.0,
                    "solar_diffusing":"Maybe"
                },
                "Bad Opposite Bounds": {
                    "thickness":0.003,
                    "solar_index_of_refraction":1.5,
                    "solar_extinction_coefficient":20.0,
                    "visible_index_of_refraction":1.7,
                    "visible_extinction_coefficient":100.0,
                    "infrared_transmittance_at_normal_incidence":-0.1,
                    "infrared_hemispherical_emissivity":1.0,
                    "dirt_correction_factor_for_solar_and_visible_transmittance":1.1
                },
                "Bad Sum": {
                    "thickness":0.003,
                    "solar_index_of_refraction":1.5,
                    "solar_extinction_coefficient":20.0,
                    "visible_index_of_refraction":1.7,
                    "visible_extinction_coefficient":100.0,
                    "infrared_transmittance_at_normal_incidence":0.2,
                    "infrared_hemispherical_emissivity":0.8
                },
                "Bad Sum Above": {
                    "thickness":0.003,
                    "solar_index_of_refraction":1.5,
                    "solar_extinction_coefficient":20.0,
                    "visible_index_of_refraction":1.7,
                    "visible_extinction_coefficient":100.0,
                    "infrared_transmittance_at_normal_incidence":0.3,
                    "infrared_hemispherical_emissivity":0.8
                }
            }
        }"#,
    )
    .expect("invalid refraction glazing should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for field in [
        "thickness",
        "solar_index_of_refraction",
        "solar_extinction_coefficient",
        "visible_index_of_refraction",
        "visible_extinction_coefficient",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "MissingRequiredField"
                    && diagnostic.object_name.as_deref() == Some("Missing")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing required-field diagnostic for {field}"
        );
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidNumericRange"
                    && diagnostic.object_name.as_deref() == Some("Bad Bounds")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing required range diagnostic for {field}"
        );
    }
    for field in [
        "infrared_transmittance_at_normal_incidence",
        "infrared_hemispherical_emissivity",
        "conductivity",
        "dirt_correction_factor_for_solar_and_visible_transmittance",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidNumericRange"
                    && diagnostic.object_name.as_deref() == Some("Bad Bounds")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing optional range diagnostic for {field}"
        );
    }
    for field in [
        "infrared_transmittance_at_normal_incidence",
        "infrared_hemispherical_emissivity",
        "dirt_correction_factor_for_solar_and_visible_transmittance",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidNumericRange"
                    && diagnostic.object_name.as_deref() == Some("Bad Opposite Bounds")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing opposite-bound diagnostic for {field}"
        );
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_name.as_deref() == Some("Bad Bounds")
            && diagnostic.field.as_deref() == Some("solar_diffusing")
    }));
    for object_name in ["Bad Sum", "Bad Sum Above"] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidWindowGlazingOpticalSum"
                && diagnostic.object_name.as_deref() == Some(object_name)
                && diagnostic.field.as_deref() == Some("infrared_hemispherical_emissivity")
        }));
    }
}

#[test]
fn refraction_extinction_accepts_a_strictly_subunit_infrared_sum() {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {{
                "Below One": {{{}}}
            }}
        }}"#,
        refraction_extinction_fields(
            r#""infrared_transmittance_at_normal_incidence":0.2,
               "infrared_hemispherical_emissivity":0.799999"#,
        )
    );
    let result =
        compile_raw_model(&parse_epjson_str(&epjson).expect("subunit infrared sum should parse"));
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
}

#[test]
fn refraction_extinction_shares_names_with_prior_material_families() {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing": {{
                "Shared": {{
                    "optical_data_type":"SpectralAverage",
                    "thickness":0.004
                }}
            }},
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {{
                "shared": {{{}}}
            }}
        }}"#,
        refraction_extinction_fields("")
    );
    let result = compile_raw_model(
        &parse_epjson_str(&epjson).expect("duplicate refraction glazing should parse"),
    );
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == "WindowMaterial:Glazing:RefractionExtinctionMethod"
            && diagnostic.object_name.as_deref() == Some("shared")
    }));
}

#[test]
fn refraction_extinction_construction_fails_closed_for_opaque_surfaces() {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {{
                "Alternative Glass": {{{}}}
            }},
            "Construction": {{
                "Alternative Window": {{"outside_layer":"Alternative Glass"}}
            }},
            "BuildingSurface:Detailed": {{
                "Wrong Surface": {{
                    "surface_type":"Wall",
                    "construction_name":"Alternative Window"
                }}
            }}
        }}"#,
        refraction_extinction_fields("")
    );
    let result = compile_raw_model(
        &parse_epjson_str(&epjson).expect("refraction surface boundary should parse"),
    );
    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidBuildingSurfaceConstructionKind"
            && diagnostic.object_type == "BuildingSurface:Detailed"
            && diagnostic.object_name.as_deref() == Some("Wrong Surface")
            && diagnostic.field.as_deref() == Some("construction_name")
    }));
}
