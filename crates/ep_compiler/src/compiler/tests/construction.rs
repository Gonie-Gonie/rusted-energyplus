use super::super::{Compiler, DiagnosticSeverity, compile_raw_model};
use ep_model::{
    ConstructionId, ConstructionKind, MaterialDefinition, MaterialId, ModelGraph, NormalizedName,
    TypedModel,
};
use ep_raw_model::parse_epjson_str;

fn has_diagnostic(
    compiler: &Compiler<'_>,
    code: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == "Construction"
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

fn material_id(model: &TypedModel, name: &str) -> Result<MaterialId, Box<dyn std::error::Error>> {
    model
        .material_names
        .resolve(name)
        .ok_or_else(|| std::io::Error::other(format!("missing typed material {name}")))
        .map_err(Into::into)
}

#[test]
fn construction_rejects_blank_outer_name() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Layer": {"roughness":"Rough","thermal_resistance":1.0}
            },
            "Construction": {"   ": {"outside_layer":"Layer"}}
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);

    assert!(has_diagnostic(
        &compiler,
        "MissingRequiredField",
        "   ",
        Some("name")
    ));
    assert!(model.constructions.is_empty());
    assert!(model.construction_names.is_empty());
    Ok(())
}

#[test]
fn invalid_construction_does_not_reserve_casefolded_name_or_id()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "reserve me": {"roughness":"Rough","thermal_resistance":1.0}
            },
            "Construction": {
                "Reserve Me": {"outside_layer":"Missing"},
                "reserve me": {"outside_layer":"reserve me"}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);

    assert!(has_diagnostic(
        &compiler,
        "MissingReference",
        "Reserve Me",
        Some("outside_layer")
    ));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "DuplicateName")
    );
    assert_eq!(model.constructions.len(), 1);
    assert_eq!(model.constructions[0].id, ConstructionId(0));
    assert_eq!(
        model.constructions[0].name,
        NormalizedName::new("reserve me")
    );
    assert_eq!(
        model.construction_names.resolve("RESERVE ME"),
        Some(ConstructionId(0))
    );
    assert_eq!(
        model.material_names.resolve("RESERVE ME"),
        Some(MaterialId(0))
    );
    Ok(())
}

#[test]
fn construction_accumulates_required_optional_contiguity_and_reference_diagnostics()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Layer": {"roughness":"Rough","thermal_resistance":1.0}
            },
            "Construction": {
                "Broken Optional": {
                    "outside_layer":"Layer",
                    "layer_2":2,
                    "layer_3":"Missing Three",
                    "layer_5":"Missing Five"
                },
                "Missing Outside": {"layer_2":"Missing Two"}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);

    assert!(has_diagnostic(
        &compiler,
        "InvalidFieldType",
        "Broken Optional",
        Some("layer_2")
    ));
    for field in ["layer_3", "layer_5"] {
        assert!(has_diagnostic(
            &compiler,
            "MissingReference",
            "Broken Optional",
            Some(field)
        ));
    }
    assert!(has_diagnostic(
        &compiler,
        "NonContiguousConstructionLayers",
        "Broken Optional",
        Some("layer_2")
    ));

    assert!(has_diagnostic(
        &compiler,
        "MissingRequiredField",
        "Missing Outside",
        Some("outside_layer")
    ));
    assert!(has_diagnostic(
        &compiler,
        "NonContiguousConstructionLayers",
        "Missing Outside",
        Some("outside_layer")
    ));
    assert!(has_diagnostic(
        &compiler,
        "MissingReference",
        "Missing Outside",
        Some("layer_2")
    ));
    assert!(model.constructions.is_empty());
    Ok(())
}

#[test]
fn construction_preserves_ten_layer_order_graph_and_family_classification()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "M01":{"roughness":"Rough","thermal_resistance":0.1},
                "M02":{"roughness":"Rough","thermal_resistance":0.2},
                "M03":{"roughness":"Rough","thermal_resistance":0.3},
                "M04":{"roughness":"Rough","thermal_resistance":0.4},
                "M05":{"roughness":"Rough","thermal_resistance":0.5},
                "M06":{"roughness":"Rough","thermal_resistance":0.6},
                "M07":{"roughness":"Rough","thermal_resistance":0.7},
                "M08":{"roughness":"Rough","thermal_resistance":0.8},
                "M09":{"roughness":"Rough","thermal_resistance":0.9},
                "M10":{"roughness":"Rough","thermal_resistance":1.0}
            },
            "WindowMaterial:Glazing": {
                "Clear":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "Construction": {
                "Opaque Ten": {
                    "outside_layer":"M01","layer_2":"M02","layer_3":"M03",
                    "layer_4":"M04","layer_5":"M05","layer_6":"M06",
                    "layer_7":"M07","layer_8":"M08","layer_9":"M09",
                    "layer_10":"M10"
                },
                "Window": {"outside_layer":"Clear"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed constructions"))?;

    let opaque = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "OPAQUE TEN")
        .ok_or_else(|| std::io::Error::other("missing ten-layer construction"))?;
    let expected = (1..=10)
        .map(|index| material_id(&model, &format!("M{index:02}")))
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(opaque.kind, ConstructionKind::Opaque);
    assert_eq!(opaque.outside_layer, expected[0]);
    assert_eq!(opaque.layers, expected);
    assert_eq!(opaque.thermochromic_master, None);

    let window = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "WINDOW")
        .ok_or_else(|| std::io::Error::other("missing window construction"))?;
    assert_eq!(window.kind, ConstructionKind::Fenestration);
    assert_eq!(window.layers, vec![material_id(&model, "Clear")?]);

    let graph = ModelGraph::from_typed(&model);
    let opaque_edges = graph
        .construction_materials
        .iter()
        .filter(|edge| edge.construction == opaque.id)
        .collect::<Vec<_>>();
    assert_eq!(opaque_edges.len(), 10);
    for (index, edge) in opaque_edges.into_iter().enumerate() {
        assert_eq!(edge.layer_index, u32::try_from(index)?);
        assert_eq!(edge.material, opaque.layers[index]);
    }
    Ok(())
}

#[test]
fn thermochromic_constructions_substitute_every_parent_and_retain_last_metadata()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass 10":{"optical_data_type":"SpectralAverage","thickness":0.003},
                "Glass 20":{"optical_data_type":"SpectralAverage","thickness":0.003},
                "Lead Glass":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "WindowMaterial:Gas": {
                "Air Gap":{"gas_type":"Air","thickness":0.012}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "TC A":{"temperature_data":[
                    {"optical_data_temperature":10.0,"window_material_glazing_name":"Glass 10"},
                    {"optical_data_temperature":20.0,"window_material_glazing_name":"Glass 20"}
                ]},
                "TC B":{"temperature_data":[
                    {"optical_data_temperature":5.0,"window_material_glazing_name":"Glass 20"}
                ]}
            },
            "Construction": {
                "TC Outside":{"outside_layer":"TC A"},
                "Preceded":{"outside_layer":"Lead Glass","layer_2":"Air Gap","layer_3":"TC A"},
                "Two Parents":{"outside_layer":"TC A","layer_2":"Air Gap","layer_3":"TC B"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected thermochromic masters"))?;
    let glass_10 = material_id(&model, "Glass 10")?;
    let glass_20 = material_id(&model, "Glass 20")?;
    let lead_glass = material_id(&model, "Lead Glass")?;
    let air_gap = material_id(&model, "Air Gap")?;
    let tc_a = material_id(&model, "TC A")?;
    let tc_b = material_id(&model, "TC B")?;

    let tc_outside = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "TC OUTSIDE")
        .ok_or_else(|| std::io::Error::other("missing outside TC construction"))?;
    assert_eq!(tc_outside.layers, vec![glass_10]);
    assert_eq!(tc_outside.outside_layer, glass_10);
    let outside_master = tc_outside
        .thermochromic_master
        .ok_or_else(|| std::io::Error::other("missing outside TC metadata"))?;
    assert_eq!(outside_master.parent_material, tc_a);
    assert_eq!(outside_master.layer_index, 0);
    assert_eq!(outside_master.glazing_layer_index, 0);

    let preceded = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "PRECEDED")
        .ok_or_else(|| std::io::Error::other("missing preceded TC construction"))?;
    assert_eq!(preceded.layers, vec![lead_glass, air_gap, glass_10]);
    let preceded_master = preceded
        .thermochromic_master
        .ok_or_else(|| std::io::Error::other("missing preceded TC metadata"))?;
    assert_eq!(preceded_master.parent_material, tc_a);
    assert_eq!(preceded_master.layer_index, 2);
    assert_eq!(preceded_master.glazing_layer_index, 1);

    let two_parents = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "TWO PARENTS")
        .ok_or_else(|| std::io::Error::other("missing two-parent TC construction"))?;
    assert_eq!(two_parents.layers, vec![glass_10, air_gap, glass_20]);
    let last_master = two_parents
        .thermochromic_master
        .ok_or_else(|| std::io::Error::other("missing last-parent TC metadata"))?;
    assert_eq!(last_master.parent_material, tc_b);
    assert_eq!(last_master.layer_index, 2);
    assert_eq!(last_master.glazing_layer_index, 1);

    let graph = ModelGraph::from_typed(&model);
    let effective_edges = graph
        .construction_materials
        .iter()
        .filter(|edge| edge.construction == two_parents.id)
        .map(|edge| edge.material)
        .collect::<Vec<_>>();
    assert_eq!(effective_edges, vec![glass_10, air_gap, glass_20]);
    assert!(
        effective_edges
            .iter()
            .all(|material| *material != tc_a && *material != tc_b)
    );
    assert_eq!(model.constructions.len(), 3, "TC children remain deferred");
    Ok(())
}

#[test]
fn thermochromic_glazing_ordinal_excludes_simple_glazing_materials()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "TC":{"temperature_data":[
                    {"optical_data_temperature":10.0,"window_material_glazing_name":"Glass"}
                ]}
            },
            "WindowMaterial:SimpleGlazingSystem": {
                "Simple":{"u_factor":2.0,"solar_heat_gain_coefficient":0.5}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    let simple = material_id(&model, "Simple")?;
    let parent = material_id(&model, "TC")?;
    let glass = material_id(&model, "Glass")?;
    assert!(matches!(
        model.materials[simple.0 as usize].definition,
        MaterialDefinition::WindowSimpleGlazing(_)
    ));

    let mut layers = vec![simple, parent];
    let metadata = compiler
        .substitute_construction_thermochromic_layers(&model, "Ordinal Probe", &mut layers)
        .ok_or_else(|| std::io::Error::other("expected valid TC substitution"))?
        .ok_or_else(|| std::io::Error::other("expected TC metadata"))?;
    assert_eq!(layers, vec![simple, glass]);
    assert_eq!(metadata.layer_index, 1);
    assert_eq!(metadata.glazing_layer_index, 0);
    Ok(())
}

#[test]
fn thermochromic_substitution_rejects_invalid_effective_topology_and_empty_state_range()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid_topology = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass A":{"optical_data_type":"SpectralAverage","thickness":0.003},
                "Glass B":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "TC":{"temperature_data":[
                    {"optical_data_temperature":10.0,"window_material_glazing_name":"Glass A"}
                ]}
            },
            "Construction": {
                "Adjacent Glass":{"outside_layer":"TC","layer_2":"Glass B"}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&invalid_topology, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    assert!(has_diagnostic(
        &compiler,
        "InvalidWindowConstructionLayering",
        "Adjacent Glass",
        Some("layer_2")
    ));
    assert!(model.constructions.is_empty());

    let empty_state = parse_epjson_str(
        r#"{
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "Empty TC":{"temperature_data":[]}
            },
            "Construction": {"Empty Master":{"outside_layer":"Empty TC"}}
        }"#,
    )?;
    let mut compiler = Compiler::new(&empty_state, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    assert!(has_diagnostic(
        &compiler,
        "InvalidThermochromicGlazingGroupStateRange",
        "Empty Master",
        Some("outside_layer")
    ));
    assert!(model.constructions.is_empty());
    Ok(())
}
