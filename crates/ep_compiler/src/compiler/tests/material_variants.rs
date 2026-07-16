use super::super::{DiagnosticSeverity, compile_raw_model};
use ep_model::{MaterialDefinition, MaterialId, MaterialSurfaceRoughness};
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
    assert_eq!(model.materials.len(), 2);
    assert_eq!(model.materials[0].id, MaterialId(0));
    assert_eq!(model.materials[0].name.0, "REGULAR");
    assert_eq!(model.materials[1].id, MaterialId(1));
    assert_eq!(model.materials[1].name.0, "R-MIN");

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
            "Material:NoMass": {"NoMass Missing": {}}
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
            "Material:NoMass": {{"Bad NoMass": {{{}}}}}
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
            "Material:NoMass": {{"shared": {{{}}}}},
            "Material": {{"Shared": {{{}}}}}
        }}"#,
        nomass_fields(""),
        regular_fields("")
    );
    let raw_model = parse_epjson_str(&epjson).expect("duplicate-name epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == "Material:NoMass"
            && diagnostic.object_name.as_deref() == Some("shared")
    }));
}
