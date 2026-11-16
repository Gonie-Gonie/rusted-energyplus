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

    // Genuine Source entry, with exact material names and ordinary opaque
    // Construction references sharing that namespace; no new test context helper.
    let mut source_value = r#"{
        "Material":{" Opaque ":{"roughness":"Rough","thickness":0.1,"conductivity":1.0,"density":1000.0,"specific_heat":1000.0}},
        "Construction":{"Opaque Ref":{"outside_layer":" OPAQUE "}},
        "WindowMaterial:Glazing":{
            " Glass ":{"optical_data_type":"SpectralAverage","thickness":0.003,
                "solar_transmittance_at_normal_incidence":0.75,"front_side_solar_reflectance_at_normal_incidence":0.75,"back_side_solar_reflectance_at_normal_incidence":0.75,
                "visible_transmittance_at_normal_incidence":0.75,"front_side_visible_reflectance_at_normal_incidence":0.75,"back_side_visible_reflectance_at_normal_incidence":0.75,
                "infrared_transmittance_at_normal_incidence":0.75},
            "\u00a0":{"optical_data_type":"SpectralAverage","thickness":0.003},
            "\u000b":{"optical_data_type":"SpectralAverage","thickness":0.003}
        }
    }"#.to_string();
    let raw = parse_epjson_str(&source_value).unwrap();
    let source = super::super::compile_raw_model_for_source(&raw);
    assert!(!source.has_errors(), "{:?}", source.report.diagnostics);
    let model = source.model.as_ref().unwrap();
    assert!(
        model.material_names.is_source_exact_namespace()
            && model.material_names.resolve("Opaque").is_none()
            && model.constructions[0].layers[0]
                == model.material_names.resolve(" Opaque ").unwrap()
    );
    for name in [" Glass ", "\u{a0}", "\u{b}"] {
        let id = model.material_names.resolve(name).unwrap();
        assert!(
            model
                .ctf_report_names
                .materials
                .contains(&(id, name.to_ascii_uppercase()))
                && model.materials.iter().any(|row| row.id == id
                    && matches!(
                        row.definition,
                        MaterialDefinition::WindowGlazingSpectralAverage(_)
                    ))
        );
    }
    let legacy = compile_raw_model(&raw);
    assert!(
        legacy
            .report
            .diagnostics
            .iter()
            .any(|d| d.code == "InvalidWindowGlazingOpticalSum")
    );
    // A Named neighbour selects SourceExact, but is never a waiver for a
    // used, schema-invalid or different optical-mode Glazing declaration.
    source_value = source_value.replacen(
        r#""Material":"#,
        r#""WindowMaterial:Gas":{"Named":{"gas_type":"Air","thickness":0.012}},"Material":"#,
        1,
    );
    let used = source_value.replacen(
        r#""Opaque Ref":"#,
        r#""Glass Used":{"outside_layer":" Glass "},"Opaque Ref":"#,
        1,
    );
    let invalid = source_value.replacen(r#""thickness":0.003"#, r#""thickness":0.0"#, 1);
    let deferred_optical = source_value.replacen(
        r#""optical_data_type":"SpectralAverage"#,
        r#""optical_data_type":"BSDF"#,
        1,
    );
    for (value, code) in [
        (used, "InvalidWindowGlazingOpticalSum"),
        (invalid, "InvalidNumericRange"),
        (deferred_optical, "UnsupportedWindowGlazingOpticalDataType"),
    ] {
        let result = super::super::compile_raw_model_for_source(&parse_epjson_str(&value).unwrap());
        assert!(result.report.diagnostics.iter().any(|d| d.code == code));
    }
    let no_glass = super::super::compile_raw_model_for_source(&parse_epjson_str(r#"{"Material":{" Opaque ":{"roughness":"Rough","thickness":0.1,"conductivity":1.0,"density":1000.0,"specific_heat":1000.0}}}"#).unwrap());
    assert!(
        !no_glass
            .model
            .unwrap()
            .material_names
            .is_source_exact_namespace()
    );

    // Angle is an honest Source-only tag. Raw strings remain Native getter
    // operands, so blank/missing/unresolved Curve names do not become typed IDs.
    let angle_value = r#"{
        "Material":{" Opaque ":{"roughness":"Rough","thickness":0.1,"conductivity":1.0,"density":1000.0,"specific_heat":1000.0}},
        "Construction":{"Opaque Ref":{"outside_layer":" OPAQUE "}},
        "WindowMaterial:Glazing":{
            " Angle ":{"optical_data_type":"SpectralAndAngle","thickness":0.003,
                "solar_transmittance_at_normal_incidence":0.75,"front_side_solar_reflectance_at_normal_incidence":0.75,"back_side_solar_reflectance_at_normal_incidence":0.75,
                "visible_transmittance_at_normal_incidence":0.75,"front_side_visible_reflectance_at_normal_incidence":0.75,"back_side_visible_reflectance_at_normal_incidence":0.75,
                "infrared_transmittance_at_normal_incidence":0.75,
                "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name":" mixed curve ",
                "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name":"",
                "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name":"MISSING"},
            "\u00a0":{"optical_data_type":"SpectralAndAngle","thickness":0.003},
            "\u000b":{"optical_data_type":"SpectralAndAngle","thickness":0.003}
        }
    }"#;
    let angle_raw = parse_epjson_str(angle_value).unwrap();
    let angle_source = super::super::compile_raw_model_for_source(&angle_raw);
    assert!(
        !angle_source.has_errors(),
        "{:?}",
        angle_source.report.diagnostics
    );
    let angle_model = angle_source.model.as_ref().unwrap();
    for name in [" Angle ", "\u{a0}", "\u{b}"] {
        let id = angle_model.material_names.resolve(name).unwrap();
        let row = angle_model
            .materials
            .iter()
            .find(|row| row.id == id)
            .unwrap();
        assert!(
            angle_model.material_names.is_source_exact_namespace()
                && angle_model.material_names.resolve("Opaque").is_none()
                && angle_model.constructions[0].layers[0]
                    == angle_model.material_names.resolve(" Opaque ").unwrap()
                && angle_model
                    .ctf_report_names
                    .materials
                    .contains(&(id, name.to_ascii_uppercase()))
                && row.kind() == MaterialKind::WindowGlazing
                && matches!(
                    row.definition,
                    MaterialDefinition::WindowGlazingSpectralAndAngle(_)
                )
                && row.as_window_glazing_spectral_average().is_none()
        );
    }
    let angle_id = angle_model.material_names.resolve(" Angle ").unwrap();
    let angle_material = angle_model
        .materials
        .iter()
        .find(|row| row.id == angle_id)
        .unwrap();
    let angle_fields = match angle_material.definition {
        MaterialDefinition::WindowGlazingSpectralAndAngle(fields) => Some(fields),
        _ => None,
    }
    .expect("Angle must not materialize an Average or Spectral placeholder");
    assert_eq!(
        (
            angle_fields.thickness_m,
            angle_fields.solar_transmittance_at_normal_incidence,
            angle_fields.visible_transmittance_at_normal_incidence,
            angle_fields.infrared_transmittance_at_normal_incidence,
            angle_fields.conductivity_w_per_m_k,
            angle_fields.youngs_modulus_pa,
            angle_fields.poissons_ratio,
            angle_fields.solar_diffusing
        ),
        (0.003, 0.75, 0.75, 0.75, 0.9, 72_000_000_000.0, 0.22, false)
    );
    let angle_declaration = &angle_raw.objects
        [&ep_raw_model::ObjectType("WindowMaterial:Glazing".into())]
        [&ep_raw_model::ObjectName(" Angle ".into())];
    assert!(matches!(
        angle_declaration.fields.get(&ep_raw_model::FieldName("thickness".into())),
        Some(ep_raw_model::RawValue::Number(token))
            if matches!(ep_model::classify_source_json_number(token), Ok(ep_model::SourceJsonNumber::Float(value))
                if value.to_bits() == 0.003_f64.to_bits())
    ));
    // A Named neighbour cannot waive the independent unused/original-schema
    // qualifier, even though it selects the same SourceExact namespace.
    let angle_with_named = angle_value.replacen(
        r#""Material":"#,
        r#""WindowMaterial:Gas":{"Named":{"gas_type":"Air","thickness":0.012}},"Material":"#,
        1,
    );
    let angle_used = angle_with_named.replacen(
        r#""Opaque Ref":"#,
        r#""Angle Used":{"outside_layer":" Angle "},"Opaque Ref":"#,
        1,
    );
    let used = super::super::compile_raw_model_for_source(&parse_epjson_str(&angle_used).unwrap());
    assert!(
        used.report
            .diagnostics
            .iter()
            .any(|d| d.code == "UnsupportedWindowGlazingOpticalDataType")
    );
    let angle_invalid = angle_with_named.replacen(r#""thickness":0.003"#, r#""thickness":0.0"#, 1);
    let invalid =
        super::super::compile_raw_model_for_source(&parse_epjson_str(&angle_invalid).unwrap());
    assert!(
        invalid
            .report
            .diagnostics
            .iter()
            .any(|d| d.code == "InvalidNumericRange")
    );
    let angle_bsdf = angle_with_named.replacen(
        r#""optical_data_type":"SpectralAndAngle"#,
        r#""optical_data_type":"BSDF"#,
        1,
    );
    let held = super::super::compile_raw_model_for_source(&parse_epjson_str(&angle_bsdf).unwrap());
    assert!(
        held.report
            .diagnostics
            .iter()
            .any(|d| d.code == "UnsupportedWindowGlazingOpticalDataType")
    );
    let angle_wrong_kind = angle_with_named.replacen(
        r#""window_glass_spectral_and_incident_angle_transmittance_data_set_table_name":" mixed curve ""#,
        r#""window_glass_spectral_and_incident_angle_transmittance_data_set_table_name":false"#,
        1,
    );
    let wrong_kind =
        super::super::compile_raw_model_for_source(&parse_epjson_str(&angle_wrong_kind).unwrap());
    assert!(
        wrong_kind
            .report
            .diagnostics
            .iter()
            .any(|d| d.code == "UnsupportedWindowGlazingOpticalDataType")
    );
    let legacy_angle = compile_raw_model(&angle_raw);
    assert!(
        legacy_angle
            .report
            .diagnostics
            .iter()
            .any(|d| d.code == "UnsupportedWindowGlazingOpticalDataType")
    );
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
