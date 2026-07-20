use super::super::{
    CompileResult, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{ConstructionKind, MaterialFamily, ModelGraph};
use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

const OBJECT_TYPE: &str = "Construction:WindowEquivalentLayer";

fn has_error(result: &CompileResult, code: &str, object_name: &str, field: Option<&str>) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && field.is_none_or(|field| diagnostic.field.as_deref() == Some(field))
    })
}

fn shade_model(constructions: &str) -> String {
    format!(
        r#"{{
            "WindowMaterial:Shade:EquivalentLayer": {{
                "EQL Shade": {{
                    "front_side_shade_beam_diffuse_solar_transmittance":0.10,
                    "back_side_shade_beam_diffuse_solar_transmittance":0.20,
                    "front_side_shade_beam_diffuse_solar_reflectance":0.30,
                    "back_side_shade_beam_diffuse_solar_reflectance":0.40
                }}
            }},
            "{OBJECT_TYPE}": {{{constructions}}}
        }}"#
    )
}

#[test]
fn window_equivalent_layer_constructions_materialize_after_internal_heat_sources()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Opaque Outside": {"roughness":"Rough", "thermal_resistance":1.0},
                "Opaque Inside": {"roughness":"Rough", "thermal_resistance":0.5}
            },
            "WindowMaterial:Glazing:EquivalentLayer": {
                "EQL Glass": {
                    "front_side_beam_beam_solar_transmittance":0.61,
                    "back_side_beam_beam_solar_transmittance":0.62,
                    "front_side_beam_beam_solar_reflectance":0.21,
                    "back_side_beam_beam_solar_reflectance":0.22
                }
            },
            "WindowMaterial:Gap:EquivalentLayer": {
                "EQL Gap": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                }
            },
            "WindowMaterial:Shade:EquivalentLayer": {
                "EQL Shade": {
                    "front_side_shade_beam_diffuse_solar_transmittance":0.10,
                    "back_side_shade_beam_diffuse_solar_transmittance":0.20,
                    "front_side_shade_beam_diffuse_solar_reflectance":0.30,
                    "back_side_shade_beam_diffuse_solar_reflectance":0.40
                }
            },
            "Construction": {
                "Opaque Wall": {
                    "outside_layer":"Opaque Outside",
                    "layer_2":"Opaque Inside"
                }
            },
            "ConstructionProperty:InternalHeatSource": {
                "Wall Source": {
                    "construction_name":"Opaque Wall",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":1,
                    "tube_spacing":0.20
                }
            },
            "Construction:WindowEquivalentLayer": {
                "EQL Window": {
                    "outside_layer":"EQL Glass",
                    "layer_2":"EQL Gap",
                    "layer_3":"EQL Glass",
                    "layer_4":"EQL Shade"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|coverage| {
        coverage.object_type == OBJECT_TYPE
            && coverage.object_count == 1
            && coverage.status == ObjectCoverageStatus::Typed
    }));

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed equivalent-layer construction"))?;
    assert_eq!(model.constructions.len(), 2);
    assert!(model.constructions[0].has_internal_heat_source());
    let construction = &model.constructions[1];
    assert_eq!(construction.name.0, "EQL WINDOW");
    assert_eq!(construction.kind, ConstructionKind::WindowEquivalentLayer);
    assert!(construction.is_window_equivalent_layer());
    assert!(!construction.is_ordinary_opaque());
    assert_eq!(
        construction
            .window_equivalent_layer
            .ok_or_else(|| std::io::Error::other("missing equivalent-layer metadata"))?
            .source_index,
        0
    );
    assert_eq!(construction.layers.len(), 4);
    assert_eq!(
        construction.outside_layer,
        construction.layers.first().copied()
    );
    assert!(construction.layers.iter().all(|material_id| {
        model.materials[material_id.0 as usize].family() == MaterialFamily::EquivalentLayer
    }));
    assert_eq!(model.object_count(), 9);
    assert_eq!(result.report.typed_object_count, model.object_count());

    let graph = ModelGraph::from_typed(model);
    assert_eq!(
        graph
            .construction_materials
            .iter()
            .filter(|edge| edge.construction == construction.id)
            .count(),
        4
    );
    Ok(())
}

#[test]
fn window_equivalent_layer_preserves_staged_idf_order_and_native_lexical_order()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = shade_model(
        r#"
            "Zulu First In IDF": {"outside_layer":"EQL Shade"},
            "Alpha Second In IDF": {"outside_layer":"EQL Shade"}
        "#,
    );
    let idf = r#"
        Construction:WindowEquivalentLayer,
          Zulu First In IDF,
          EQL Shade;
        Construction:WindowEquivalentLayer,
          Alpha Second In IDF,
          EQL Shade;
    "#;

    let staged_raw = parse_epjson_str_with_idf_order(&epjson, idf)?;
    assert!(staged_raw.has_idf_declaration_order(OBJECT_TYPE));
    let staged = compile_raw_model(&staged_raw);
    assert!(!staged.has_errors(), "{:?}", staged.report.diagnostics);
    let staged_model = staged
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected staged equivalent layers"))?;
    assert_eq!(
        staged_model
            .constructions
            .iter()
            .map(|construction| construction.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ZULU FIRST IN IDF", "ALPHA SECOND IN IDF"]
    );
    assert_eq!(
        staged_model
            .constructions
            .iter()
            .map(|construction| construction.window_equivalent_layer.unwrap().source_index)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );

    let native = compile_raw_model(&parse_epjson_str(&epjson)?);
    assert!(!native.has_errors(), "{:?}", native.report.diagnostics);
    assert_eq!(
        native
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected native equivalent layers"))?
            .constructions
            .iter()
            .map(|construction| construction.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ALPHA SECOND IN IDF", "ZULU FIRST IN IDF"]
    );
    Ok(())
}

#[test]
fn window_equivalent_layer_accepts_one_to_eleven_family_layers_without_runtime_topology()
-> Result<(), Box<dyn std::error::Error>> {
    let inner_layers = (2..=11)
        .map(|layer| format!(r#""layer_{layer}":"EQL Shade""#))
        .collect::<Vec<_>>()
        .join(",");
    let epjson = shade_model(&format!(
        r#"
            "Eleven Layers": {{"outside_layer":"EQL Shade",{inner_layers}}},
            "Single Layer": {{"outside_layer":"EQL Shade"}}
        "#
    ));
    let result = compile_raw_model(&parse_epjson_str(&epjson)?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected bounded equivalent-layer stacks"))?;
    assert_eq!(model.constructions[0].layers.len(), 11);
    assert_eq!(model.constructions[1].layers.len(), 1);
    assert!(
        model
            .constructions
            .iter()
            .all(|construction| construction.is_window_equivalent_layer())
    );
    Ok(())
}

#[test]
fn window_equivalent_layer_rejects_invalid_fields_families_gaps_and_shared_names()
-> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        (
            shade_model(r#""Missing Outside": {}"#),
            "MissingRequiredField",
            "Missing Outside",
            Some("outside_layer"),
        ),
        (
            shade_model(r#""Missing Material": {"outside_layer":"Unknown"}"#),
            "MissingReference",
            "Missing Material",
            Some("outside_layer"),
        ),
        (
            shade_model(r#""Layer Gap": {"outside_layer":"EQL Shade", "layer_3":"EQL Shade"}"#),
            "NonContiguousEquivalentLayerConstructionLayers",
            "Layer Gap",
            Some("layer_2"),
        ),
        (
            shade_model(r#""Malformed Layer": {"outside_layer":"EQL Shade", "layer_2":2}"#),
            "InvalidFieldType",
            "Malformed Layer",
            Some("layer_2"),
        ),
        (
            shade_model(r#""": {"outside_layer":"EQL Shade"}"#),
            "MissingRequiredField",
            "",
            Some("name"),
        ),
        (
            r#"{
                "Material:NoMass": {
                    "Opaque": {"roughness":"Rough", "thermal_resistance":1.0}
                },
                "Construction:WindowEquivalentLayer": {
                    "Wrong Family": {"outside_layer":"Opaque"}
                }
            }"#
            .to_string(),
            "InvalidEquivalentLayerConstructionMaterial",
            "Wrong Family",
            Some("outside_layer"),
        ),
        (
            r#"{
                "Material:NoMass": {
                    "Opaque": {"roughness":"Rough", "thermal_resistance":1.0}
                },
                "WindowMaterial:Shade:EquivalentLayer": {
                    "EQL Shade": {
                        "front_side_shade_beam_diffuse_solar_transmittance":0.10,
                        "back_side_shade_beam_diffuse_solar_transmittance":0.20,
                        "front_side_shade_beam_diffuse_solar_reflectance":0.30,
                        "back_side_shade_beam_diffuse_solar_reflectance":0.40
                    }
                },
                "Construction": {
                    "Shared Name": {"outside_layer":"Opaque"}
                },
                "Construction:WindowEquivalentLayer": {
                    "shared name": {"outside_layer":"EQL Shade"}
                }
            }"#
            .to_string(),
            "DuplicateName",
            "shared name",
            None,
        ),
    ];

    for (epjson, code, name, field) in cases {
        let result = compile_raw_model(&parse_epjson_str(&epjson)?);
        assert!(
            has_error(&result, code, name, field),
            "code={code}, name={name}, field={field:?}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
    }
    Ok(())
}

#[test]
fn internal_heat_source_cannot_resolve_future_window_equivalent_layer_construction()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade:EquivalentLayer": {
                "EQL Shade": {
                    "front_side_shade_beam_diffuse_solar_transmittance":0.10,
                    "back_side_shade_beam_diffuse_solar_transmittance":0.20,
                    "front_side_shade_beam_diffuse_solar_reflectance":0.30,
                    "back_side_shade_beam_diffuse_solar_reflectance":0.40
                }
            },
            "ConstructionProperty:InternalHeatSource": {
                "Premature Source": {
                    "construction_name":"EQL Window",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":1,
                    "tube_spacing":0.20
                }
            },
            "Construction:WindowEquivalentLayer": {
                "EQL Window": {
                    "outside_layer":"EQL Shade",
                    "layer_2":"EQL Shade"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == "MissingReference"
            && diagnostic.object_type == "ConstructionProperty:InternalHeatSource"
            && diagnostic.object_name.as_deref() == Some("Premature Source")
            && diagnostic.field.as_deref() == Some("construction_name")
    }));
    assert!(result.model.is_none());
    Ok(())
}
