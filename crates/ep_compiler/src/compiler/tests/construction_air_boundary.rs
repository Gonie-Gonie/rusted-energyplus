use super::super::{
    Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model, typed_coverage_status,
};
use ep_model::{
    AirBoundaryAirExchange, AirBoundaryMixingSchedule, ConstructionId, ConstructionKind,
    ModelGraph, NormalizedName, ScheduleId, TypedModel,
};
use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

const OBJECT_TYPE: &str = "Construction:AirBoundary";

fn has_error(compiler: &Compiler<'_>, code: &str, object_name: &str, field: Option<&str>) -> bool {
    compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

#[test]
fn air_boundaries_materialize_after_fc_with_zero_layers_and_mixing_state()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Layer": {"roughness":"Rough", "thermal_resistance":1.0}
            },
            "Construction": {
                "Ordinary": {"outside_layer":"Layer"}
            },
            "Construction:FfactorGroundFloor": {
                "F Ground": {"f_factor":0.5, "area":100.0, "perimeterexposed":20.0}
            },
            "Construction:CfactorUndergroundWall": {
                "C Ground": {"c_factor":0.5, "height":1.0}
            },
            "Schedule:Constant": {
                "Mixing Schedule": {"hourly_value":2.0}
            },
            "Construction:AirBoundary": {
                "Zulu User Mixing": {
                    "air_exchange_method":"simplemixing",
                    "simple_mixing_air_changes_per_hour":0.0,
                    "simple_mixing_schedule_name":"mixing schedule"
                },
                "Alpha None": {
                    "air_exchange_method":"None",
                    "simple_mixing_air_changes_per_hour":7.0,
                    "simple_mixing_schedule_name":"Unknown But Inactive"
                },
                "Beta Default Mixing": {
                    "air_exchange_method":"SimpleMixing"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed air boundaries"))?;

    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert_eq!(
        model
            .constructions
            .iter()
            .map(|construction| construction.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "ORDINARY",
            "F GROUND",
            "C GROUND",
            "ALPHA NONE",
            "BETA DEFAULT MIXING",
            "ZULU USER MIXING",
        ]
    );

    let air_boundaries = &model.constructions[3..];
    for construction in air_boundaries {
        assert_eq!(construction.kind, ConstructionKind::AirBoundary);
        assert!(construction.is_air_boundary());
        assert!(!construction.is_ordinary_opaque());
        assert_eq!(construction.outside_layer, None);
        assert!(construction.layers.is_empty());
        assert!(construction.effective_layers().is_empty());
        assert_eq!(construction.thermochromic_master, None);
        assert_eq!(construction.ground_factor, None);
    }
    assert_eq!(
        air_boundaries[0]
            .air_boundary
            .ok_or_else(|| std::io::Error::other("missing None metadata"))?
            .air_exchange,
        AirBoundaryAirExchange::None
    );
    assert_eq!(
        air_boundaries[1]
            .air_boundary
            .ok_or_else(|| std::io::Error::other("missing default mixing metadata"))?
            .air_exchange,
        AirBoundaryAirExchange::SimpleMixing {
            air_changes_per_hour: 0.5,
            schedule: AirBoundaryMixingSchedule::AlwaysOn,
        }
    );
    assert_eq!(
        model.schedule_names.resolve("Mixing Schedule"),
        Some(ScheduleId(0))
    );
    assert_eq!(
        air_boundaries[2]
            .air_boundary
            .ok_or_else(|| std::io::Error::other("missing user mixing metadata"))?
            .air_exchange,
        AirBoundaryAirExchange::SimpleMixing {
            air_changes_per_hour: 0.0,
            schedule: AirBoundaryMixingSchedule::User(ScheduleId(0)),
        }
    );

    let graph = ModelGraph::from_typed(&model);
    assert!(air_boundaries.iter().all(|construction| {
        graph
            .construction_materials
            .iter()
            .all(|edge| edge.construction != construction.id)
    }));
    Ok(())
}

#[test]
fn air_boundary_family_uses_epjson_lexical_order_not_idf_declaration_order()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = r#"{
        "Construction:AirBoundary": {
            "Alpha Boundary": {"air_exchange_method":"None"},
            "Zulu Boundary": {"air_exchange_method":"None"}
        }
    }"#;
    let idf = r#"
        Construction:AirBoundary, Zulu Boundary, None, , ;
        Construction:AirBoundary, Alpha Boundary, None, , ;
    "#;
    let raw = parse_epjson_str_with_idf_order(epjson, idf)?;
    assert!(!raw.has_idf_declaration_order(OBJECT_TYPE));

    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected lexical air boundaries"))?;
    assert_eq!(
        model
            .constructions
            .iter()
            .map(|construction| construction.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ALPHA BOUNDARY", "ZULU BOUNDARY"]
    );
    assert_eq!(model.constructions[0].id, ConstructionId(0));
    assert_eq!(model.constructions[1].id, ConstructionId(1));
    Ok(())
}

#[test]
fn invalid_air_boundaries_fail_before_identity_and_inactive_names_are_not_resolved()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Layer": {"roughness":"Rough", "thermal_resistance":1.0}
            },
            "Construction": {
                "Shared": {"outside_layer":"Layer"}
            },
            "Construction:AirBoundary": {
                "A Bad Method": {"air_exchange_method":"MixALot"},
                "B Negative": {
                    "air_exchange_method":"SimpleMixing",
                    "simple_mixing_air_changes_per_hour":-0.1
                },
                "C Missing Schedule": {
                    "air_exchange_method":"SimpleMixing",
                    "simple_mixing_schedule_name":"Missing"
                },
                "D Bad Schedule Type": {
                    "air_exchange_method":"None",
                    "simple_mixing_schedule_name":7
                },
                "Reserve Me": {
                    "air_exchange_method":"SimpleMixing",
                    "simple_mixing_schedule_name":"Missing"
                },
                "Shared": {"air_exchange_method":"None"},
                "reserve me": {
                    "air_exchange_method":"None",
                    "simple_mixing_schedule_name":"Unknown But Inactive"
                }
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    compiler.parse_air_boundary_constructions(&mut model);

    assert!(has_error(
        &compiler,
        "InvalidEnumValue",
        "A Bad Method",
        Some("air_exchange_method")
    ));
    assert!(has_error(
        &compiler,
        "InvalidNumericRange",
        "B Negative",
        Some("simple_mixing_air_changes_per_hour")
    ));
    assert!(has_error(
        &compiler,
        "MissingReference",
        "C Missing Schedule",
        Some("simple_mixing_schedule_name")
    ));
    assert!(has_error(
        &compiler,
        "InvalidFieldType",
        "D Bad Schedule Type",
        Some("simple_mixing_schedule_name")
    ));
    assert!(has_error(&compiler, "DuplicateName", "Shared", None));
    assert_eq!(model.constructions.len(), 2);
    assert_eq!(model.constructions[0].name, NormalizedName::new("Shared"));
    assert_eq!(
        model.constructions[1].name,
        NormalizedName::new("reserve me")
    );
    assert_eq!(model.constructions[1].id, ConstructionId(1));
    assert_eq!(
        model.construction_names.resolve("RESERVE ME"),
        Some(ConstructionId(1))
    );
    Ok(())
}

#[test]
fn building_surface_rejects_air_boundary_until_pairing_and_enclosures_are_ported()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Construction:AirBoundary": {
                "Open Boundary": {"air_exchange_method":"None"}
            },
            "BuildingSurface:Detailed": {
                "Air Wall": {
                    "surface_type":"Wall",
                    "construction_name":"Open Boundary"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidBuildingSurfaceConstructionKind"
            && diagnostic.object_type == "BuildingSurface:Detailed"
            && diagnostic.object_name.as_deref() == Some("Air Wall")
            && diagnostic.field.as_deref() == Some("construction_name")
    }));
    assert!(result.model.is_none());
    Ok(())
}
