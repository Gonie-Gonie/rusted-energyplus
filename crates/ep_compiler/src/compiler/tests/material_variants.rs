use super::super::{DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model};
use ep_model::{MaterialDefinition, MaterialId, MaterialKind, MaterialSurfaceRoughness};
use ep_raw_model::parse_epjson_str;

fn regular_fields(overrides: &str) -> String {
    let suffix = if overrides.is_empty() {
        String::new()
    } else {
        format!(",{overrides}")
    };
    format!(
        r#""roughness":"MediumRough","thickness":0.1,"conductivity":2.0,
           "density":2000.0,"specific_heat":100.0{suffix}"#
    )
}

fn nomass_fields(overrides: &str) -> String {
    let suffix = if overrides.is_empty() {
        String::new()
    } else {
        format!(",{overrides}")
    };
    format!(r#""roughness":"Rough","thermal_resistance":0.001{suffix}"#)
}

#[test]
fn material_variants_materialize_required_fields_and_defaults()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "Material:InfraredTransparent": {{"IRT": {{}}}},
            "Material:AirGap": {{"Air Gap": {{"thermal_resistance":0.18}}}},
            "Material:NoMass": {{"R-Min": {{{}}}}},
            "Material": {{"Regular": {{{}}}}}
        }}"#,
        nomass_fields(""),
        regular_fields("")
    );
    let raw_model = parse_epjson_str(&epjson).expect("material epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .expect("valid material variants should compile");
    assert_eq!(model.materials.len(), 4);
    assert_eq!(model.materials[0].id, MaterialId(0));
    assert_eq!(model.materials[0].name.0, "REGULAR");
    assert_eq!(model.materials[1].id, MaterialId(1));
    assert_eq!(model.materials[1].name.0, "R-MIN");
    assert_eq!(model.materials[2].id, MaterialId(2));
    assert_eq!(model.materials[2].name.0, "AIR GAP");
    assert_eq!(model.materials[3].id, MaterialId(3));
    assert_eq!(model.materials[3].name.0, "IRT");

    let MaterialDefinition::Regular(regular) = &model.materials[0].definition else {
        return Err(std::io::Error::other("Material must compile to the regular variant").into());
    };
    assert_eq!(regular.roughness, MaterialSurfaceRoughness::MediumRough);
    assert_eq!(regular.thickness_m, 0.1);
    assert_eq!(regular.conductivity_w_per_m_k, 2.0);
    assert_eq!(regular.density_kg_per_m3, 2000.0);
    assert_eq!(regular.specific_heat_j_per_kg_k, 100.0);
    assert_eq!(regular.surface.thermal_absorptance, 0.9);
    assert_eq!(regular.surface.solar_absorptance, 0.7);
    assert_eq!(regular.surface.visible_absorptance, 0.7);

    let MaterialDefinition::NoMass(nomass) = &model.materials[1].definition else {
        return Err(
            std::io::Error::other("Material:NoMass must compile to the no-mass variant").into(),
        );
    };
    assert_eq!(nomass.roughness, MaterialSurfaceRoughness::Rough);
    assert_eq!(nomass.thermal_resistance_m2_k_per_w, 0.001);
    assert_eq!(nomass.surface.thermal_absorptance, 0.9);
    assert_eq!(nomass.surface.solar_absorptance, 0.7);
    assert_eq!(nomass.surface.visible_absorptance, 0.7);
    assert_eq!(model.materials[0].thermal_resistance(), Some(0.05));
    assert_eq!(model.materials[0].heat_capacity_per_area(), Some(20_000.0));
    assert_eq!(model.materials[1].thermal_resistance(), Some(0.001));
    assert_eq!(model.materials[1].heat_capacity_per_area(), None);

    let MaterialDefinition::AirGap(air_gap) = &model.materials[2].definition else {
        return Err(
            std::io::Error::other("Material:AirGap must compile to the air-gap variant").into(),
        );
    };
    assert_eq!(air_gap.thermal_resistance_m2_k_per_w, 0.18);
    assert_eq!(model.materials[2].kind(), MaterialKind::AirGap);
    assert_eq!(
        model.materials[2].roughness(),
        Some(MaterialSurfaceRoughness::MediumRough)
    );
    assert_eq!(model.materials[2].is_resistance_only(), Some(true));
    assert_eq!(model.materials[2].thermal_resistance(), Some(0.18));
    assert_eq!(model.materials[2].heat_capacity_per_area(), None);
    assert_eq!(model.materials[2].thermal_absorptance(), Some(0.0));
    assert_eq!(model.materials[2].solar_absorptance(), Some(0.0));
    assert_eq!(model.materials[2].visible_absorptance(), Some(0.0));

    let MaterialDefinition::InfraredTransparent(_) = &model.materials[3].definition else {
        return Err(std::io::Error::other(
            "Material:InfraredTransparent must compile to its dedicated variant",
        )
        .into());
    };
    assert_eq!(model.materials[3].kind(), MaterialKind::InfraredTransparent);
    assert_eq!(model.materials[3].roughness(), None);
    assert_eq!(model.materials[3].is_resistance_only(), Some(true));
    assert_eq!(model.materials[3].thermal_resistance(), Some(0.01));
    assert_eq!(model.materials[3].heat_capacity_per_area(), None);
    assert_eq!(model.materials[3].thermal_absorptance(), Some(0.9999));
    assert_eq!(model.materials[3].solar_absorptance(), Some(1.0));
    assert_eq!(model.materials[3].visible_absorptance(), Some(1.0));

    for object_type in ["Material:AirGap", "Material:InfraredTransparent"] {
        let coverage = result
            .report
            .coverage
            .iter()
            .find(|entry| entry.object_type == object_type)
            .ok_or_else(|| {
                std::io::Error::other(format!("missing compiler coverage for {object_type}"))
            })?;
        assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    }

    let defaulted_fields = result
        .report
        .defaults_applied
        .iter()
        .map(|application| application.field.as_str())
        .collect::<Vec<_>>();
    for field in [
        "thermal_absorptance",
        "solar_absorptance",
        "visible_absorptance",
    ] {
        assert_eq!(
            defaulted_fields
                .iter()
                .filter(|candidate| **candidate == field)
                .count(),
            2,
            "expected one default per material variant for {field}"
        );
    }

    Ok(())
}

#[test]
fn material_variants_require_their_schema_fields() {
    let raw_model = parse_epjson_str(
        r#"{
            "Material": {"Regular Missing": {}},
            "Material:NoMass": {"NoMass Missing": {}},
            "Material:AirGap": {"Air Gap Missing": {}},
            "Material:InfraredTransparent": {"IRT Needs Only A Name": {}}
        }"#,
    )
    .expect("missing-field epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.model.is_none());
    for (object_type, fields) in [
        (
            "Material",
            &[
                "roughness",
                "thickness",
                "conductivity",
                "density",
                "specific_heat",
            ][..],
        ),
        ("Material:NoMass", &["roughness", "thermal_resistance"][..]),
        ("Material:AirGap", &["thermal_resistance"][..]),
    ] {
        for field in fields {
            assert!(
                result.report.diagnostics.iter().any(|diagnostic| {
                    diagnostic.severity == DiagnosticSeverity::Error
                        && diagnostic.code == "MissingRequiredField"
                        && diagnostic.object_type == object_type
                        && diagnostic.field.as_deref() == Some(field)
                }),
                "missing required diagnostic for {object_type}.{field}"
            );
        }
    }
}

#[test]
fn material_variants_enforce_energyplus_numeric_bounds() {
    let epjson = format!(
        r#"{{
            "Material": {{"Bad Regular": {{{}}}}},
            "Material:NoMass": {{"Bad NoMass": {{{}}}}},
            "Material:AirGap": {{"Bad Air Gap": {{"thermal_resistance":0.0}}}}
        }}"#,
        regular_fields(
            r#""thickness":0.0,"conductivity":0.0,"density":0.0,
               "specific_heat":99.0,"thermal_absorptance":0.0,
               "solar_absorptance":1.01,"visible_absorptance":-0.01"#,
        ),
        nomass_fields(r#""thermal_resistance":0.0009"#),
    );
    let raw_model = parse_epjson_str(&epjson).expect("range-check epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for (object_type, field) in [
        ("Material", "thickness"),
        ("Material", "conductivity"),
        ("Material", "density"),
        ("Material", "specific_heat"),
        ("Material", "thermal_absorptance"),
        ("Material", "solar_absorptance"),
        ("Material", "visible_absorptance"),
        ("Material:NoMass", "thermal_resistance"),
        ("Material:AirGap", "thermal_resistance"),
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidNumericRange"
                    && diagnostic.object_type == object_type
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing range diagnostic for {object_type}.{field}"
        );
    }
}

#[test]
fn material_names_share_one_source_order_registry() {
    let epjson = format!(
        r#"{{
            "Material:InfraredTransparent": {{"SHARED": {{}}}},
            "Material:AirGap": {{"Shared": {{"thermal_resistance":0.18}}}},
            "Material:NoMass": {{"shared": {{{}}}}},
            "Material": {{"Shared": {{{}}}}}
        }}"#,
        nomass_fields(""),
        regular_fields("")
    );
    let raw_model = parse_epjson_str(&epjson).expect("duplicate-name epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for (object_type, object_name) in [
        ("Material:NoMass", "shared"),
        ("Material:AirGap", "Shared"),
        ("Material:InfraredTransparent", "SHARED"),
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "DuplicateName"
                    && diagnostic.object_type == object_type
                    && diagnostic.object_name.as_deref() == Some(object_name)
            }),
            "missing duplicate-name diagnostic for {object_type}/{object_name}"
        );
    }
}

#[test]
fn construction_material_variants_preserve_resistance_only_layer_semantics()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = format!(
        r#"{{
            "Material": {{
                "Outside Solid": {{{}}},
                "Inside Solid": {{{}}}
            }},
            "Material:AirGap": {{
                "Gap One": {{"thermal_resistance":0.18}},
                "Gap Two": {{"thermal_resistance":0.12}}
            }},
            "Material:InfraredTransparent": {{"IRT": {{}}}},
            "Construction": {{
                "Mixed Opaque": {{
                    "outside_layer":"Outside Solid",
                    "layer_2":"Gap One",
                    "layer_3":"Gap Two",
                    "layer_4":"Inside Solid"
                }},
                "IRT Only": {{"outside_layer":"IRT"}}
            }}
        }}"#,
        regular_fields(""),
        regular_fields("")
    );
    let raw_model = parse_epjson_str(&epjson).expect("construction variant epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed material constructions"))?;
    let mixed = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "MIXED OPAQUE")
        .ok_or_else(|| std::io::Error::other("missing mixed opaque construction"))?;
    assert_eq!(mixed.layers.len(), 4);

    let mut total_resistance = 0.0;
    let mut total_heat_capacity = 0.0;
    for material_id in &mixed.layers {
        let material = model
            .materials
            .iter()
            .find(|material| material.id == *material_id)
            .ok_or_else(|| std::io::Error::other("missing mixed construction material"))?;
        total_resistance += material
            .thermal_resistance()
            .ok_or_else(|| std::io::Error::other("missing material thermal resistance"))?;
        total_heat_capacity += material.heat_capacity_per_area().unwrap_or(0.0);
    }
    assert!((total_resistance - 0.4).abs() <= f64::EPSILON);
    assert!((total_heat_capacity - 40_000.0).abs() <= f64::EPSILON);

    let irt = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "IRT ONLY")
        .ok_or_else(|| std::io::Error::other("missing IRT-only construction"))?;
    assert_eq!(irt.layers.len(), 1);
    let irt_material = model
        .materials
        .iter()
        .find(|material| material.id == irt.layers[0])
        .ok_or_else(|| std::io::Error::other("missing IRT material"))?;
    assert_eq!(irt_material.kind(), MaterialKind::InfraredTransparent);
    assert_eq!(irt_material.thermal_resistance(), Some(0.01));

    Ok(())
}

#[test]
fn construction_rejects_air_gap_boundaries_and_multilayer_irt() {
    let epjson = format!(
        r#"{{
            "Material": {{"Solid": {{{}}}}},
            "Material:AirGap": {{"Gap": {{"thermal_resistance":0.18}}}},
            "Material:InfraredTransparent": {{"IRT": {{}}}},
            "Construction": {{
                "Air Outside": {{"outside_layer":"Gap","layer_2":"Solid"}},
                "Air Inside": {{"outside_layer":"Solid","layer_2":"Gap"}},
                "IRT Outside Multi": {{"outside_layer":"IRT","layer_2":"Solid"}},
                "IRT Inside Multi": {{"outside_layer":"Solid","layer_2":"IRT"}}
            }}
        }}"#,
        regular_fields("")
    );
    let raw_model = parse_epjson_str(&epjson).expect("invalid construction epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.model.is_none());
    for (construction_name, field) in [("Air Outside", "outside_layer"), ("Air Inside", "layer_2")]
    {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidAirGapLayerPosition"
                && diagnostic.object_type == "Construction"
                && diagnostic.object_name.as_deref() == Some(construction_name)
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    for (construction_name, field) in [
        ("IRT Outside Multi", "outside_layer"),
        ("IRT Inside Multi", "layer_2"),
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidInfraredTransparentConstruction"
                && diagnostic.object_type == "Construction"
                && diagnostic.object_name.as_deref() == Some(construction_name)
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
}
