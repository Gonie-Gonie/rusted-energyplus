use super::super::thermochromic_construction::format_energyplus_round_zero;
use super::super::{Compiler, DiagnosticSeverity, compile_raw_model};
use ep_model::{
    ConstructionId, MaterialId, ModelGraph, ThermochromicConstructionChildId, TypedModel,
};
use ep_raw_model::{RawModel, parse_epjson_str};

fn material_id(model: &TypedModel, name: &str) -> Result<MaterialId, Box<dyn std::error::Error>> {
    model
        .material_names
        .resolve(name)
        .ok_or_else(|| std::io::Error::other(format!("missing material {name}")).into())
}

fn two_master_raw() -> Result<RawModel, Box<dyn std::error::Error>> {
    Ok(parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "A Cold":{"optical_data_type":"SpectralAverage","thickness":0.003},
                "A Warm":{"optical_data_type":"SpectralAverage","thickness":0.003},
                "B Cold":{"optical_data_type":"SpectralAverage","thickness":0.003},
                "B Warm":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "WindowMaterial:Gas": {
                "Air Gap":{"gas_type":"Air","thickness":0.012}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "TC A":{"temperature_data":[
                    {"optical_data_temperature":10.0,"window_material_glazing_name":"A Cold"},
                    {"optical_data_temperature":20.0,"window_material_glazing_name":"A Warm"}
                ]},
                "TC B":{"temperature_data":[
                    {"optical_data_temperature":5.0,"window_material_glazing_name":"B Cold"},
                    {"optical_data_temperature":5.0,"window_material_glazing_name":"B Cold"},
                    {"optical_data_temperature":6.0,"window_material_glazing_name":"B Warm"}
                ]}
            },
            "Construction": {
                "Zulu Master":{"outside_layer":"TC A","layer_2":"Air Gap","layer_3":"TC B"},
                "Alpha Master":{"outside_layer":"TC A"}
            }
        }"#,
    )?)
}

#[test]
fn thermochromic_projection_is_empty_without_a_master() -> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Opaque":{"roughness":"Rough","thermal_resistance":1.0}
            },
            "WindowMaterial:Glazing": {
                "Unused Glass":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "Unused TC":{"temperature_data":[
                    {"optical_data_temperature":10.0,"window_material_glazing_name":"Unused Glass"}
                ]}
            },
            "Construction": {"Opaque Construction":{"outside_layer":"Opaque"}}
        }"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert!(model.construction_thermochromic_series.is_empty());
    assert!(model.construction_thermochromic_children.is_empty());
    Ok(())
}

#[test]
fn thermochromic_children_include_first_state_without_global_construction_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Cold":{"optical_data_type":"SpectralAverage","thickness":0.003},
                "Warm":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "TC":{"temperature_data":[
                    {"optical_data_temperature":10.0,"window_material_glazing_name":"Cold"},
                    {"optical_data_temperature":20.0,"window_material_glazing_name":"Warm"}
                ]}
            },
            "Construction": {"Master":{"outside_layer":"TC"}}
        }"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    let cold = material_id(&model, "Cold")?;
    let warm = material_id(&model, "Warm")?;

    assert_eq!(model.constructions.len(), 1);
    assert_eq!(model.constructions[0].id, ConstructionId(0));
    assert_eq!(model.constructions[0].layers, vec![cold]);
    assert_eq!(model.construction_names.len(), 1);
    assert_eq!(
        model.construction_names.resolve("Master"),
        Some(ConstructionId(0))
    );
    assert_eq!(model.construction_names.resolve("Master_TC_10"), None);

    assert_eq!(model.construction_thermochromic_series.len(), 1);
    let series = model.construction_thermochromic_series[0];
    assert_eq!(series.master_construction, ConstructionId(0));
    assert_eq!(series.initial_specification_temperature_c, 10.0);
    assert_eq!(series.first_child, ThermochromicConstructionChildId(0));
    assert_eq!(series.child_count, 2);
    let children = model
        .construction_thermochromic_children(series)
        .ok_or_else(|| std::io::Error::other("expected child range"))?;
    assert_eq!(
        children.iter().map(|child| child.id).collect::<Vec<_>>(),
        vec![
            ThermochromicConstructionChildId(0),
            ThermochromicConstructionChildId(1)
        ]
    );
    assert_eq!(
        children
            .iter()
            .map(|child| child.state_index)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
    assert_eq!(
        children
            .iter()
            .map(|child| child.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["MASTER_TC_10", "MASTER_TC_20"]
    );
    assert_eq!(children[0].layers, vec![cold]);
    assert_eq!(children[0].outside_layer, cold);
    assert_eq!(children[1].layers, vec![warm]);
    assert_eq!(children[1].outside_layer, warm);

    let count_with_projection = model.object_count();
    let mut without_projection = model.clone();
    without_projection.construction_thermochromic_series.clear();
    without_projection
        .construction_thermochromic_children
        .clear();
    assert_eq!(count_with_projection, without_projection.object_count());
    assert_eq!(result.report.typed_object_count, count_with_projection);
    let graph = ModelGraph::from_typed(&model);
    assert_eq!(graph.construction_materials.len(), 1);
    assert_eq!(
        graph.construction_materials[0].construction,
        ConstructionId(0)
    );
    assert_eq!(graph.construction_materials[0].material, cold);
    Ok(())
}

#[test]
fn thermochromic_series_follow_master_id_and_state_order_with_duplicates_preserved()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&two_master_raw()?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    let a_cold = material_id(&model, "A Cold")?;
    let a_warm = material_id(&model, "A Warm")?;
    let b_cold = material_id(&model, "B Cold")?;
    let b_warm = material_id(&model, "B Warm")?;
    let air_gap = material_id(&model, "Air Gap")?;

    assert_eq!(
        model
            .construction_thermochromic_series
            .iter()
            .map(|series| series.master_construction)
            .collect::<Vec<_>>(),
        vec![ConstructionId(0), ConstructionId(1)]
    );
    let alpha = model.construction_thermochromic_series[0];
    assert_eq!(alpha.first_child, ThermochromicConstructionChildId(0));
    assert_eq!(alpha.child_count, 2);
    let alpha_children = model
        .construction_thermochromic_children(alpha)
        .ok_or_else(|| std::io::Error::other("expected alpha children"))?;
    assert_eq!(alpha_children[0].layers, vec![a_cold]);
    assert_eq!(alpha_children[1].layers, vec![a_warm]);

    let zulu = model.construction_thermochromic_series[1];
    assert_eq!(zulu.first_child, ThermochromicConstructionChildId(2));
    assert_eq!(zulu.child_count, 3);
    let zulu_children = model
        .construction_thermochromic_children(zulu)
        .ok_or_else(|| std::io::Error::other("expected zulu children"))?;
    assert_eq!(
        zulu_children
            .iter()
            .map(|child| child.id)
            .collect::<Vec<_>>(),
        vec![
            ThermochromicConstructionChildId(2),
            ThermochromicConstructionChildId(3),
            ThermochromicConstructionChildId(4)
        ]
    );
    assert_eq!(
        zulu_children
            .iter()
            .map(|child| child.specification_temperature_c)
            .collect::<Vec<_>>(),
        vec![5.0, 5.0, 6.0]
    );
    assert_eq!(
        zulu_children
            .iter()
            .map(|child| child.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ZULU MASTER_TC_5", "ZULU MASTER_TC_5", "ZULU MASTER_TC_6"]
    );
    assert_eq!(zulu_children[0].layers, vec![a_cold, air_gap, b_cold]);
    assert_eq!(zulu_children[1].layers, vec![a_cold, air_gap, b_cold]);
    assert_eq!(zulu_children[2].layers, vec![a_cold, air_gap, b_warm]);
    assert_eq!(zulu_children[0].outside_layer, a_cold);
    Ok(())
}

#[test]
fn shared_group_projects_each_master_and_preserves_authored_generated_name_collision()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Cold":{"optical_data_type":"SpectralAverage","thickness":0.003},
                "Lead":{"optical_data_type":"SpectralAverage","thickness":0.003},
                "Warm":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "WindowMaterial:Gas": {
                "Air Gap":{"gas_type":"Air","thickness":0.012}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "Shared TC":{"temperature_data":[
                    {"optical_data_temperature":10.0,"window_material_glazing_name":"Cold"},
                    {"optical_data_temperature":20.0,"window_material_glazing_name":"Warm"}
                ]}
            },
            "Construction": {
                "Alpha Master":{"outside_layer":"Shared TC"},
                "Alpha Master_TC_10":{"outside_layer":"Lead"},
                "Zulu Master":{"outside_layer":"Lead","layer_2":"Air Gap","layer_3":"Shared TC"}
            }
        }"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.report.diagnostics.is_empty());
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    let cold = material_id(&model, "Cold")?;
    let lead = material_id(&model, "Lead")?;
    let warm = material_id(&model, "Warm")?;
    let air_gap = material_id(&model, "Air Gap")?;

    assert_eq!(model.constructions.len(), 3);
    assert_eq!(model.construction_names.len(), 3);
    assert_eq!(
        model.construction_names.resolve("Alpha Master_TC_10"),
        Some(ConstructionId(1))
    );
    assert_eq!(
        model
            .construction_thermochromic_series
            .iter()
            .map(|series| series.master_construction)
            .collect::<Vec<_>>(),
        vec![ConstructionId(0), ConstructionId(2)]
    );

    let alpha = model.construction_thermochromic_series[0];
    let alpha_children = model
        .construction_thermochromic_children(alpha)
        .ok_or_else(|| std::io::Error::other("expected alpha children"))?;
    assert_eq!(alpha.child_count, 2);
    assert_eq!(alpha_children[0].name.0, "ALPHA MASTER_TC_10");
    assert_eq!(alpha_children[0].layers, vec![cold]);
    assert_eq!(alpha_children[1].layers, vec![warm]);

    let zulu = model.construction_thermochromic_series[1];
    let zulu_children = model
        .construction_thermochromic_children(zulu)
        .ok_or_else(|| std::io::Error::other("expected zulu children"))?;
    assert_eq!(zulu.child_count, 2);
    assert_eq!(zulu_children[0].layers, vec![lead, air_gap, cold]);
    assert_eq!(zulu_children[1].layers, vec![lead, air_gap, warm]);

    assert_eq!(
        model
            .construction_thermochromic_children
            .iter()
            .map(|child| child.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "ALPHA MASTER_TC_10",
            "ALPHA MASTER_TC_20",
            "ZULU MASTER_TC_10",
            "ZULU MASTER_TC_20"
        ]
    );
    assert_eq!(model.construction_names.len(), 3);
    assert_eq!(
        model.construction_names.resolve("Alpha Master_TC_10"),
        Some(ConstructionId(1))
    );
    Ok(())
}

#[test]
fn thermochromic_projection_sorts_reordered_storage_by_master_construction_id()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = two_master_raw()?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    assert!(compiler.diagnostics.is_empty());
    model.constructions.reverse();

    compiler.create_thermochromic_construction_projections(&mut model);
    assert!(compiler.diagnostics.is_empty());
    assert_eq!(
        model
            .construction_thermochromic_series
            .iter()
            .map(|series| series.master_construction)
            .collect::<Vec<_>>(),
        vec![ConstructionId(0), ConstructionId(1)]
    );
    Ok(())
}

#[test]
fn thermochromic_generated_names_pin_source_shaped_zero_r_boundaries()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "TC":{"temperature_data":[
                    {"optical_data_temperature":40.0,"window_material_glazing_name":"Glass"},
                    {"optical_data_temperature":-999.5,"window_material_glazing_name":"Glass"},
                    {"optical_data_temperature":0.0,"window_material_glazing_name":"Glass"},
                    {"optical_data_temperature":0.05,"window_material_glazing_name":"Glass"}
                ]}
            },
            "Construction": {"Rounding":{"outside_layer":"TC"}}
        }"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(
        model
            .construction_thermochromic_children
            .iter()
            .map(|child| child.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "ROUNDING_TC_40",
            "ROUNDING_TC_-1000",
            "ROUNDING_TC_0",
            "ROUNDING_TC_5E-002"
        ]
    );
    Ok(())
}

#[test]
fn source_shaped_zero_r_helper_locks_normal_large_threshold_and_extreme_boundaries() {
    for (value, expected) in [
        (40.0, "40"),
        (-999.5, "-1000"),
        (0.0, "0"),
        (-0.0, "0"),
        (0.05, "5E-002"),
        (0.5, "1"),
        (-0.5, "-1"),
        (9.5, "10"),
        (-9.5, "-10"),
        (0.099, "1E-001"),
        (-0.099, "-1E-001"),
        (100_000.0, "100000"),
        (100_000.5, "100000"),
        (-100_000.5, "-100001"),
        (1.0e16, "10000000000000000"),
        (-1.0e16, "-10000000000000006"),
        (1.0e17, "100000000000000000."),
        (-1.0e17, "-100000000000000048"),
    ] {
        assert_eq!(
            format_energyplus_round_zero(value),
            expected,
            "value={value}"
        );
    }

    let threshold = f64::from_bits(0x3fb9_9999_9999_9999);
    let predecessor = f64::from_bits(threshold.to_bits() - 1);
    assert_eq!(format_energyplus_round_zero(threshold), "0");
    assert_eq!(format_energyplus_round_zero(predecessor), "1E-001");
    assert_eq!(format_energyplus_round_zero(-threshold), "-0");
    assert_eq!(format_energyplus_round_zero(-predecessor), "-1E-001");
    assert_eq!(
        format_energyplus_round_zero(f64::MAX),
        format!("{:.0}.", f64::MAX)
    );
    assert_eq!(
        format_energyplus_round_zero(f64::MIN),
        format!("{:.0}", f64::MIN)
    );
}

#[test]
fn thermochromic_projection_coexists_with_deferred_window5_requests()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass":{"optical_data_type":"SpectralAverage","thickness":0.003}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "TC":{"temperature_data":[
                    {"optical_data_temperature":10.0,"window_material_glazing_name":"Glass"}
                ]}
            },
            "Construction": {"Master":{"outside_layer":"TC"}},
            "Construction:WindowDataFile": {
                "Window5 Request":{"file_name":"Legacy Window.dat"}
            }
        }"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.constructions.len(), 1);
    assert_eq!(model.construction_names.len(), 1);
    assert_eq!(model.construction_window_data_file_requests.len(), 1);
    assert_eq!(model.construction_thermochromic_series.len(), 1);
    assert_eq!(model.construction_thermochromic_children.len(), 1);
    assert_eq!(
        model.construction_window_data_file_requests[0].name.0,
        "WINDOW5 REQUEST"
    );
    Ok(())
}

#[test]
fn upstream_errors_suppress_thermochromic_projection_publication()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = two_master_raw()?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    assert!(compiler.diagnostics.is_empty());
    compiler.error(
        "InjectedUpstreamError",
        "Construction",
        None,
        None,
        "test-only upstream error".to_string(),
    );

    compiler.create_thermochromic_construction_projections(&mut model);
    assert!(model.construction_thermochromic_series.is_empty());
    assert!(model.construction_thermochromic_children.is_empty());
    Ok(())
}

#[test]
fn thermochromic_projection_pass_is_transactional_on_invalid_later_master()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = two_master_raw()?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    assert!(compiler.diagnostics.is_empty());
    let ordinary_glazing = material_id(&model, "A Cold")?;
    model.constructions[1]
        .thermochromic_master
        .as_mut()
        .ok_or_else(|| std::io::Error::other("expected second master metadata"))?
        .parent_material = ordinary_glazing;

    compiler.create_thermochromic_construction_projections(&mut model);
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == "InvalidThermochromicConstructionProjection"
            && diagnostic.object_name.as_deref() == Some("ZULU MASTER")
    }));
    assert!(model.construction_thermochromic_series.is_empty());
    assert!(model.construction_thermochromic_children.is_empty());
    Ok(())
}
