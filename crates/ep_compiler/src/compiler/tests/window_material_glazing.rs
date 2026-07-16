use super::super::{DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model};
use ep_model::{ConstructionKind, MaterialDefinition, MaterialFamily, MaterialId, MaterialKind};
use ep_raw_model::parse_epjson_str;

fn spectral_average_fields(overrides: &str) -> String {
    let suffix = if overrides.is_empty() {
        String::new()
    } else {
        format!(",{overrides}")
    };
    format!(
        r#""optical_data_type":"SpectralAverage","thickness":0.003,
           "solar_transmittance_at_normal_incidence":0.75,
           "front_side_solar_reflectance_at_normal_incidence":0.07,
           "back_side_solar_reflectance_at_normal_incidence":0.08,
           "visible_transmittance_at_normal_incidence":0.80,
           "front_side_visible_reflectance_at_normal_incidence":0.08,
           "back_side_visible_reflectance_at_normal_incidence":0.09{suffix}"#
    )
}

#[test]
fn spectral_average_glazing_materializes_source_fields_and_defaults()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "Material": {{
                "Opaque": {{
                    "roughness":"MediumRough","thickness":0.1,"conductivity":1.0,
                    "density":1000.0,"specific_heat":1000.0
                }}
            }},
            "WindowMaterial:Glazing": {{
                "Clear 3mm": {{{}}}
            }}
        }}"#,
        spectral_average_fields(
            r#""infrared_transmittance_at_normal_incidence":0.05,
               "front_side_infrared_hemispherical_emissivity":0.84,
               "back_side_infrared_hemispherical_emissivity":0.85,
               "conductivity":1.1,
               "dirt_correction_factor_for_solar_and_visible_transmittance":0.95,
               "solar_diffusing":"Yes",
               "young_s_modulus":73000000000.0,
               "poisson_s_ratio":0.23"#,
        )
    );
    let raw_model = parse_epjson_str(&epjson)?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed glazing material"))?;
    assert_eq!(model.materials.len(), 2);
    let glazing = &model.materials[1];
    assert_eq!(glazing.id, MaterialId(1));
    assert_eq!(glazing.name.0, "CLEAR 3MM");
    assert_eq!(glazing.kind(), MaterialKind::WindowGlazing);
    assert_eq!(glazing.family(), MaterialFamily::Fenestration);
    assert!(glazing.as_opaque().is_none());
    assert_eq!(glazing.thickness_m(), None);
    assert_eq!(glazing.conductivity_w_per_m_k(), None);
    assert_eq!(glazing.thermal_resistance(), None);
    assert_eq!(glazing.heat_capacity_per_area(), None);
    assert_eq!(glazing.surface_properties(), None);

    let MaterialDefinition::WindowGlazingSpectralAverage(fields) = glazing.definition else {
        return Err(std::io::Error::other("expected SpectralAverage glazing variant").into());
    };
    assert_eq!(fields.thickness_m, 0.003);
    assert_eq!(fields.solar_transmittance_at_normal_incidence, 0.75);
    assert_eq!(
        fields.front_side_solar_reflectance_at_normal_incidence,
        0.07
    );
    assert_eq!(fields.back_side_solar_reflectance_at_normal_incidence, 0.08);
    assert_eq!(fields.visible_transmittance_at_normal_incidence, 0.80);
    assert_eq!(
        fields.front_side_visible_reflectance_at_normal_incidence,
        0.08
    );
    assert_eq!(
        fields.back_side_visible_reflectance_at_normal_incidence,
        0.09
    );
    assert_eq!(fields.infrared_transmittance_at_normal_incidence, 0.05);
    assert_eq!(fields.front_side_infrared_hemispherical_emissivity, 0.84);
    assert_eq!(fields.back_side_infrared_hemispherical_emissivity, 0.85);
    assert_eq!(fields.conductivity_w_per_m_k, 1.1);
    assert_eq!(
        fields.dirt_correction_factor_for_solar_and_visible_transmittance,
        0.95
    );
    assert!(fields.solar_diffusing);
    assert_eq!(fields.youngs_modulus_pa, 73_000_000_000.0);
    assert_eq!(fields.poissons_ratio, 0.23);

    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == "WindowMaterial:Glazing")
        .ok_or_else(|| std::io::Error::other("missing glazing coverage row"))?;
    assert_eq!(
        coverage.status,
        ObjectCoverageStatus::RawOnly,
        "partial optical-mode support must not promote whole-object coverage"
    );
    Ok(())
}

#[test]
fn spectral_average_glazing_applies_energyplus_defaults() -> Result<(), Box<dyn std::error::Error>>
{
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Defaults": {
                    "optical_data_type":"SpectralAverage",
                    "thickness":0.003
                }
            }
        }"#,
    )
    .expect("default glazing epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result.model.expect("default glazing should compile");
    let MaterialDefinition::WindowGlazingSpectralAverage(fields) = model.materials[0].definition
    else {
        return Err(std::io::Error::other("expected SpectralAverage glazing").into());
    };
    assert_eq!(fields.solar_transmittance_at_normal_incidence, 0.0);
    assert_eq!(fields.front_side_solar_reflectance_at_normal_incidence, 0.0);
    assert_eq!(fields.back_side_solar_reflectance_at_normal_incidence, 0.0);
    assert_eq!(fields.visible_transmittance_at_normal_incidence, 0.0);
    assert_eq!(
        fields.front_side_visible_reflectance_at_normal_incidence,
        0.0
    );
    assert_eq!(
        fields.back_side_visible_reflectance_at_normal_incidence,
        0.0
    );
    assert_eq!(fields.infrared_transmittance_at_normal_incidence, 0.0);
    assert_eq!(fields.front_side_infrared_hemispherical_emissivity, 0.84);
    assert_eq!(fields.back_side_infrared_hemispherical_emissivity, 0.84);
    assert_eq!(fields.conductivity_w_per_m_k, 0.9);
    assert_eq!(
        fields.dirt_correction_factor_for_solar_and_visible_transmittance,
        1.0
    );
    assert!(!fields.solar_diffusing);
    assert_eq!(fields.youngs_modulus_pa, 72_000_000_000.0);
    assert_eq!(fields.poissons_ratio, 0.22);
    Ok(())
}

#[test]
fn glazing_rejects_deferred_optical_modes_without_approximating_them() {
    for optical_data_type in ["Spectral", "SpectralAndAngle", "BSDF"] {
        let epjson = format!(
            r#"{{
                "WindowMaterial:Glazing": {{
                    "Deferred": {{
                        "optical_data_type":"{optical_data_type}",
                        "thickness":0.003
                    }}
                }}
            }}"#
        );
        let raw_model = parse_epjson_str(&epjson).expect("deferred glazing epJSON should parse");
        let result = compile_raw_model(&raw_model);
        assert!(result.has_errors());
        assert!(result.model.is_none());
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "UnsupportedWindowGlazingOpticalDataType"
                && diagnostic.object_type == "WindowMaterial:Glazing"
                && diagnostic.object_name.as_deref() == Some("Deferred")
                && diagnostic.field.as_deref() == Some("optical_data_type")
        }));
    }
}

#[test]
fn glazing_enforces_required_fields_bounds_and_energy_conservation() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Missing": {},
                "Bad Bounds": {
                    "optical_data_type":"SpectralAverage",
                    "thickness":0.0,
                    "solar_transmittance_at_normal_incidence":1.1,
                    "front_side_solar_reflectance_at_normal_incidence":-0.1,
                    "infrared_transmittance_at_normal_incidence":1.1,
                    "front_side_infrared_hemispherical_emissivity":0.0,
                    "back_side_infrared_hemispherical_emissivity":1.0,
                    "conductivity":0.0,
                    "dirt_correction_factor_for_solar_and_visible_transmittance":0.0,
                    "young_s_modulus":0.0,
                    "poisson_s_ratio":1.0
                },
                "Bad Sums": {
                    "optical_data_type":"SpectralAverage",
                    "thickness":0.003,
                    "solar_transmittance_at_normal_incidence":0.8,
                    "front_side_solar_reflectance_at_normal_incidence":0.3,
                    "visible_transmittance_at_normal_incidence":0.8,
                    "back_side_visible_reflectance_at_normal_incidence":0.3,
                    "infrared_transmittance_at_normal_incidence":0.2,
                    "front_side_infrared_hemispherical_emissivity":0.9
                }
            }
        }"#,
    )
    .expect("invalid glazing epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for field in ["optical_data_type", "thickness"] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "MissingRequiredField"
                && diagnostic.object_name.as_deref() == Some("Missing")
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    for field in [
        "thickness",
        "solar_transmittance_at_normal_incidence",
        "front_side_solar_reflectance_at_normal_incidence",
        "infrared_transmittance_at_normal_incidence",
        "front_side_infrared_hemispherical_emissivity",
        "back_side_infrared_hemispherical_emissivity",
        "conductivity",
        "dirt_correction_factor_for_solar_and_visible_transmittance",
        "young_s_modulus",
        "poisson_s_ratio",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidNumericRange"
                    && diagnostic.object_name.as_deref() == Some("Bad Bounds")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing range diagnostic for {field}"
        );
    }
    for field in [
        "front_side_solar_reflectance_at_normal_incidence",
        "back_side_visible_reflectance_at_normal_incidence",
        "front_side_infrared_hemispherical_emissivity",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidWindowGlazingOpticalSum"
                    && diagnostic.object_name.as_deref() == Some("Bad Sums")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing optical-sum diagnostic for {field}"
        );
    }
}

#[test]
fn glazing_shares_material_names_and_classifies_single_layer_constructions() {
    let epjson = format!(
        r#"{{
            "WindowMaterial:Glazing": {{
                "Clear": {{{}}},
                "Second": {{{}}}
            }},
            "Construction": {{
                "Window": {{"outside_layer":"Clear"}}
            }}
        }}"#,
        spectral_average_fields(""),
        spectral_average_fields("")
    );
    let raw_model = parse_epjson_str(&epjson).expect("glazing construction epJSON should parse");
    let result = compile_raw_model(&raw_model);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .expect("single glazing construction should compile");
    assert_eq!(model.constructions[0].kind, ConstructionKind::Fenestration);
    assert_eq!(model.constructions[0].layers, vec![MaterialId(0)]);

    let duplicate = format!(
        r#"{{
            "Material": {{
                "Shared": {{
                    "roughness":"Rough","thickness":0.1,"conductivity":1.0,
                    "density":1000.0,"specific_heat":1000.0
                }}
            }},
            "WindowMaterial:Glazing": {{"shared": {{{}}}}}
        }}"#,
        spectral_average_fields("")
    );
    let result = compile_raw_model(
        &parse_epjson_str(&duplicate).expect("duplicate material epJSON should parse"),
    );
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == "WindowMaterial:Glazing"
            && diagnostic.object_name.as_deref() == Some("shared")
    }));
}

#[test]
fn constructions_and_building_surfaces_fail_closed_at_family_boundaries() {
    let epjson = format!(
        r#"{{
            "Material": {{
                "Opaque": {{
                    "roughness":"Rough","thickness":0.1,"conductivity":1.0,
                    "density":1000.0,"specific_heat":1000.0
                }}
            }},
            "WindowMaterial:Glazing": {{
                "Glass A": {{{}}},
                "Glass B": {{{}}}
            }},
            "Construction": {{
                "Mixed": {{"outside_layer":"Opaque","layer_2":"Glass A"}},
                "Multiple Glass": {{"outside_layer":"Glass A","layer_2":"Glass B"}},
                "Window": {{"outside_layer":"Glass A"}}
            }},
            "BuildingSurface:Detailed": {{
                "Wrong Surface": {{
                    "surface_type":"Wall",
                    "construction_name":"Window"
                }}
            }}
        }}"#,
        spectral_average_fields(""),
        spectral_average_fields("")
    );
    let raw_model = parse_epjson_str(&epjson).expect("family-boundary epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for (code, object_type, object_name, field) in [
        (
            "MixedConstructionMaterialFamilies",
            "Construction",
            "Mixed",
            None,
        ),
        (
            "InvalidWindowConstructionLayering",
            "Construction",
            "Multiple Glass",
            Some("layer_2"),
        ),
        (
            "InvalidBuildingSurfaceConstructionKind",
            "BuildingSurface:Detailed",
            "Wrong Surface",
            Some("construction_name"),
        ),
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.severity == DiagnosticSeverity::Error
                    && diagnostic.code == code
                    && diagnostic.object_type == object_type
                    && diagnostic.object_name.as_deref() == Some(object_name)
                    && diagnostic.field.as_deref() == field
            }),
            "missing {code} diagnostic"
        );
    }
}
