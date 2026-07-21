use super::super::{Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model};
use ep_model::{
    Construction, ConstructionGroundFactor, ConstructionId, ConstructionKind, ModelGraph,
    NormalizedName, TypedModel,
};
use ep_raw_model::parse_epjson_str;

const RAW_REFERENCE_TYPES: [&str; 6] = [
    "Pipe:Indoor",
    "Pipe:Outdoor",
    "Pipe:Underground",
    "GroundHeatExchanger:Surface",
    "DaylightingDevice:Tubular",
    "EnergyManagementSystem:ConstructionIndexVariable",
];

fn construction(id: u32, name: &str, kind: ConstructionKind) -> Construction {
    Construction {
        id: ConstructionId(id),
        name: NormalizedName::new(name),
        kind,
        outside_layer: None,
        layers: Vec::new(),
        thermochromic_master: None,
        ground_factor: None,
        air_boundary: None,
        complex_fenestration: None,
        window_equivalent_layer: None,
        internal_heat_source: None,
    }
}

fn model_with_kinds(entries: &[(&str, ConstructionKind)]) -> TypedModel {
    let mut model = TypedModel::default();
    for (index, (name, kind)) in entries.iter().enumerate() {
        let id = ConstructionId(u32::try_from(index).expect("test construction ID should fit"));
        assert!(model.construction_names.insert(name, id).is_none());
        model.constructions.push(construction(id.0, name, *kind));
    }
    model
}

fn construction_ids(
    model: &TypedModel,
    names: &[&str],
) -> Result<Vec<ConstructionId>, Box<dyn std::error::Error>> {
    let mut ids = names
        .iter()
        .map(|name| {
            model
                .construction_names
                .resolve(name)
                .ok_or_else(|| std::io::Error::other(format!("missing typed construction {name}")))
        })
        .collect::<Result<Vec<_>, _>>()?;
    ids.sort_unstable();
    ids.dedup();
    Ok(ids)
}

#[test]
fn empty_model_has_no_positive_construction_use_evidence() -> Result<(), Box<dyn std::error::Error>>
{
    let result = compile_raw_model(&parse_epjson_str("{}")?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert!(model.known_used_constructions.is_empty());
    assert!(model.known_ctf_used_constructions.is_empty());
    Ok(())
}

#[test]
fn surfaces_and_all_six_source_families_publish_sorted_positive_ids_only()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Layer": {"roughness":"Rough","thermal_resistance":1.0}
            },
            "Construction": {
                "A Surface": {"outside_layer":"Layer"},
                "B Indoor": {"outside_layer":"Layer"},
                "C Outdoor": {"outside_layer":"Layer"},
                "D Underground": {"outside_layer":"Layer"},
                "E Ground": {"outside_layer":"Layer"},
                "F Tubular": {"outside_layer":"Layer"},
                "G Ems": {"outside_layer":"Layer"},
                "H Distractor": {"outside_layer":"Layer"},
                "Z Unused": {"outside_layer":"Layer"}
            },
            "Zone": {"Zone One":{}},
            "BuildingSurface:Detailed": {
                "A Surface One": {
                    "surface_type":"Wall",
                    "construction_name":"A Surface",
                    "zone_name":"Zone One",
                    "outside_boundary_condition":"Outdoors",
                    "vertices":[
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":1,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":1,"vertex_z_coordinate":1},
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":1}
                    ]
                },
                "B Surface Two": {
                    "surface_type":"Wall",
                    "construction_name":" a surface ",
                    "zone_name":"Zone One",
                    "outside_boundary_condition":"Outdoors",
                    "vertices":[
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":1},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":1}
                    ]
                }
            },
            "Pipe:Indoor": {
                "Indoor": {"construction_name":" b indoor "}
            },
            "Pipe:Outdoor": {
                "Outdoor": {"construction_name":"C OUTDOOR"}
            },
            "Pipe:Underground": {
                "Underground": {"construction_name":"d underground"}
            },
            "GroundHeatExchanger:Surface": {
                "Ground": {"construction_name":"E Ground"}
            },
            "DaylightingDevice:Tubular": {
                "Tube": {
                    "construction_name":"F Tubular",
                    "construction_object_name":"H Distractor",
                    "diameter":"H Distractor"
                }
            },
            "EnergyManagementSystem:ConstructionIndexVariable": {
                "Index": {
                    "construction_object_name":"g ems",
                    "construction_name":"H Distractor"
                }
            },
            "GroundHeatExchanger:Pond": {
                "Ignored Family": {"construction_name":"H Distractor"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(
        model.known_used_constructions,
        construction_ids(
            model,
            &[
                "A Surface",
                "B Indoor",
                "C Outdoor",
                "D Underground",
                "E Ground",
                "F Tubular",
                "G Ems",
            ],
        )?
    );
    assert_eq!(
        model.known_ctf_used_constructions,
        construction_ids(model, &["E Ground", "G Ems"])?
    );
    assert!(
        !model
            .known_used_constructions
            .contains(&model.construction_names.resolve("H Distractor").unwrap())
    );
    assert!(
        !model
            .known_used_constructions
            .contains(&model.construction_names.resolve("Z Unused").unwrap())
    );
    assert!(
        !model
            .known_ctf_used_constructions
            .contains(&model.construction_names.resolve("A Surface").unwrap())
    );

    let mut without_evidence = model.clone();
    without_evidence.known_used_constructions.clear();
    without_evidence.known_ctf_used_constructions.clear();
    assert_eq!(model.object_count(), without_evidence.object_count());
    assert_eq!(
        ModelGraph::from_typed(model),
        ModelGraph::from_typed(&without_evidence)
    );
    assert_eq!(result.report.typed_object_count, model.object_count());
    assert!(result.report.diagnostics.iter().all(|diagnostic| {
        !diagnostic.code.contains("UnusedConstruction")
            && !diagnostic
                .message
                .to_ascii_lowercase()
                .contains("unused construction")
    }));
    for object_type in RAW_REFERENCE_TYPES {
        let coverage = result
            .report
            .coverage
            .iter()
            .find(|coverage| coverage.object_type == object_type)
            .ok_or_else(|| std::io::Error::other(format!("missing coverage for {object_type}")))?;
        assert_eq!(coverage.status, ObjectCoverageStatus::RawOnly);
        assert_eq!(coverage.object_count, 1);
    }
    Ok(())
}

#[test]
fn only_ground_heat_exchanger_and_ems_non_window_references_publish_ctf_use_evidence()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "GroundHeatExchanger:Surface": {
                "A Opaque": {"construction_name":" opaque "},
                "B Air": {"construction_name":"AIR BOUNDARY"},
                "C Fenestration": {"construction_name":"Fenestration"},
                "D Complex": {"construction_name":"Complex"},
                "E Equivalent": {"construction_name":"Equivalent"}
            },
            "EnergyManagementSystem:ConstructionIndexVariable": {
                "A Opaque": {"construction_object_name":"OPAQUE"},
                "B Air": {"construction_object_name":" air boundary "},
                "C Fenestration": {"construction_object_name":"FENESTRATION"},
                "D Complex": {"construction_object_name":"complex"},
                "E Equivalent": {"construction_object_name":" equivalent "}
            }
        }"#,
    )?;
    let mut model = model_with_kinds(&[
        ("Opaque", ConstructionKind::Opaque),
        ("Air Boundary", ConstructionKind::AirBoundary),
        ("Fenestration", ConstructionKind::Fenestration),
        ("Complex", ConstructionKind::ComplexFenestration),
        ("Equivalent", ConstructionKind::WindowEquivalentLayer),
    ]);
    let compiler = Compiler::new(&raw, None);

    compiler.collect_known_construction_use_evidence(&mut model);

    assert_eq!(
        model.known_used_constructions,
        vec![
            ConstructionId(0),
            ConstructionId(1),
            ConstructionId(2),
            ConstructionId(3),
            ConstructionId(4),
        ]
    );
    assert_eq!(
        model.known_ctf_used_constructions,
        vec![ConstructionId(0), ConstructionId(1)]
    );
    assert!(compiler.diagnostics.is_empty());
    Ok(())
}

#[test]
fn ground_factor_opaque_construction_is_non_window_for_ctf_evidence()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "GroundHeatExchanger:Surface": {
                "Ground":{"construction_name":"F Factor"}
            },
            "EnergyManagementSystem:ConstructionIndexVariable": {
                "Index":{"construction_object_name":"F FACTOR"}
            }
        }"#,
    )?;
    let mut model = model_with_kinds(&[("F Factor", ConstructionKind::Opaque)]);
    model.constructions[0].ground_factor = Some(ConstructionGroundFactor::FfactorGroundFloor {
        f_factor_w_per_m_k: 1.0,
        area_m2: 10.0,
        perimeter_exposed_m: 4.0,
        effective_thermal_resistance_m2_k_per_w: 2.0,
        insulation_thermal_resistance_m2_k_per_w: 1.5,
    });
    assert!(!model.constructions[0].is_ordinary_opaque());
    let compiler = Compiler::new(&raw, None);

    compiler.collect_known_construction_use_evidence(&mut model);

    assert_eq!(model.known_used_constructions, vec![ConstructionId(0)]);
    assert_eq!(model.known_ctf_used_constructions, vec![ConstructionId(0)]);
    Ok(())
}

#[test]
fn malformed_blank_missing_and_unresolved_raw_references_are_silent()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Pipe:Indoor": {"Missing":{}},
            "Pipe:Outdoor": {"Blank":{"construction_name":"   "}},
            "Pipe:Underground": {"Number":{"construction_name":7}},
            "GroundHeatExchanger:Surface": {
                "Unresolved":{"construction_name":"Absent"}
            },
            "DaylightingDevice:Tubular": {
                "Array":{"construction_name":["Known"]}
            },
            "EnergyManagementSystem:ConstructionIndexVariable": {
                "Null":{"construction_object_name":null}
            }
        }"#,
    )?;
    let mut model = model_with_kinds(&[("Known", ConstructionKind::Opaque)]);
    let compiler = Compiler::new(&raw, None);

    compiler.collect_known_construction_use_evidence(&mut model);

    assert!(compiler.diagnostics.is_empty());
    assert!(model.known_used_constructions.is_empty());
    assert!(model.known_ctf_used_constructions.is_empty());
    Ok(())
}

#[test]
fn prior_error_prevents_any_construction_use_publication() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(
        r#"{
            "Timestep":{"Broken":{"number_of_timesteps_per_hour":"not a number"}},
            "Pipe:Indoor":{"Pipe":{"construction_name":"Known"}},
            "GroundHeatExchanger:Surface":{"Ground":{"construction_name":"Known"}}
        }"#,
    )?;
    let mut model = model_with_kinds(&[("Known", ConstructionKind::Opaque)]);
    let mut compiler = Compiler::new(&raw, None);
    compiler.parse_timestep(&mut model);
    assert!(
        compiler
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
    );

    compiler.collect_known_construction_use_evidence(&mut model);

    assert!(model.known_used_constructions.is_empty());
    assert!(model.known_ctf_used_constructions.is_empty());
    Ok(())
}

#[test]
fn private_thermochromic_children_and_window_data_requests_are_not_identities()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
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
            "Construction": {"Master":{"outside_layer":"TC"}},
            "Construction:WindowDataFile": {"Window Request":{}},
            "EnergyManagementSystem:ConstructionIndexVariable": {
                "A Master":{"construction_object_name":"Master"},
                "B Child":{"construction_object_name":"Master_TC_20"},
                "C Request":{"construction_object_name":"Window Request"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.constructions.len(), 1);
    assert_eq!(model.construction_thermochromic_children.len(), 2);
    assert_eq!(model.construction_window_data_file_requests.len(), 1);
    assert_eq!(
        model.known_used_constructions,
        vec![model.construction_names.resolve("Master").unwrap()]
    );
    assert!(model.known_ctf_used_constructions.is_empty());
    assert_eq!(model.construction_names.resolve("Master_TC_20"), None);
    assert_eq!(model.construction_names.resolve("Window Request"), None);
    Ok(())
}
