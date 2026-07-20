use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{NodeId, TypedModel, ZoneLocalEnvironmentId};
use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

fn has_error(result: &CompileResult, code: &str) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == "ZoneProperty:LocalEnvironment"
    })
}

#[test]
fn get_zone_local_environment_materializes_optional_nodes_and_zone_links()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone": {
                "Alpha Zone": {},
                "Beta Zone": {}
            },
            "NodeList": {
                "One Node Alias": {
                    "nodes": [{"node_name":"Aliased Outdoor Node"}]
                }
            },
            "ZoneProperty:LocalEnvironment": {
                "Alpha Direct": {
                    "zone_name":"Beta Zone",
                    "outdoor_air_node_name":"Direct Outdoor Node"
                },
                "Beta Alias": {
                    "zone_name":"Alpha Zone",
                    "outdoor_air_node_name":"One Node Alias"
                },
                "Gamma Blank": {
                    "zone_name":"Alpha Zone"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status("ZoneProperty:LocalEnvironment"),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|coverage| {
        coverage.object_type == "ZoneProperty:LocalEnvironment"
            && coverage.object_count == 3
            && coverage.status == ObjectCoverageStatus::Typed
    }));

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed local environments"))?;
    assert_eq!(model.object_count(), 7);
    assert_eq!(model.nodes.len(), 2);
    assert_eq!(model.nodes[0].id, NodeId(0));
    assert_eq!(model.nodes[0].name.0, "ALIASED OUTDOOR NODE");
    assert_eq!(model.nodes[1].id, NodeId(1));
    assert_eq!(model.nodes[1].name.0, "DIRECT OUTDOOR NODE");

    assert_eq!(model.zone_local_environments.len(), 3);
    assert_eq!(
        model.zone_local_environments[0].id,
        ZoneLocalEnvironmentId(0)
    );
    assert_eq!(model.zone_local_environments[0].name.0, "ALPHA DIRECT");
    assert_eq!(model.zone_local_environments[0].zone.0, 1);
    assert_eq!(
        model.zone_local_environments[0].outdoor_air_node,
        Some(NodeId(1))
    );
    assert_eq!(
        model.zone_local_environments[1].outdoor_air_node,
        Some(NodeId(0))
    );
    assert_eq!(model.zone_local_environments[2].outdoor_air_node, None);
    assert_eq!(model.zones[0].linked_outdoor_air_node, Some(NodeId(0)));
    assert_eq!(model.zones[1].linked_outdoor_air_node, Some(NodeId(1)));
    Ok(())
}

#[test]
fn local_environment_order_controls_last_nonblank_zone_link()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = r#"{
        "Zone":{"Only Zone":{}},
        "ZoneProperty:LocalEnvironment": {
            "Zulu First": {
                "zone_name":"Only Zone",
                "outdoor_air_node_name":"First Node"
            },
            "Alpha Second": {
                "zone_name":"Only Zone",
                "outdoor_air_node_name":"Second Node"
            },
            "Omega Blank": {"zone_name":"Only Zone"}
        }
    }"#;
    let idf = r#"
        Zone, Only Zone;
        ZoneProperty:LocalEnvironment, Zulu First, Only Zone, First Node;
        ZoneProperty:LocalEnvironment, Alpha Second, Only Zone, Second Node;
        ZoneProperty:LocalEnvironment, Omega Blank, Only Zone, ;
    "#;

    let staged = compile_raw_model(&parse_epjson_str_with_idf_order(epjson, idf)?);
    assert!(!staged.has_errors(), "{:?}", staged.report.diagnostics);
    let staged_model = staged
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected staged local environments"))?;
    assert_eq!(
        staged_model
            .zone_local_environments
            .iter()
            .map(|environment| environment.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ZULU FIRST", "ALPHA SECOND", "OMEGA BLANK"]
    );
    let staged_link = staged_model.zones[0]
        .linked_outdoor_air_node
        .ok_or_else(|| std::io::Error::other("expected staged zone link"))?;
    assert_eq!(
        staged_model.nodes[staged_link.0 as usize].name.0,
        "SECOND NODE"
    );

    let native = compile_raw_model(&parse_epjson_str(epjson)?);
    assert!(!native.has_errors(), "{:?}", native.report.diagnostics);
    let native_model = native
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected native local environments"))?;
    assert_eq!(
        native_model
            .zone_local_environments
            .iter()
            .map(|environment| environment.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ALPHA SECOND", "OMEGA BLANK", "ZULU FIRST"]
    );
    let native_link = native_model.zones[0]
        .linked_outdoor_air_node
        .ok_or_else(|| std::io::Error::other("expected native zone link"))?;
    assert_eq!(
        native_model.nodes[native_link.0 as usize].name.0,
        "FIRST NODE"
    );
    Ok(())
}

#[test]
fn blank_outdoor_air_node_is_valid_and_does_not_create_a_node()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "Zone":{"Only Zone":{}},
            "ZoneProperty:LocalEnvironment": {
                "Blank Node":{"zone_name":"Only Zone","outdoor_air_node_name":""}
            }
        }"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected blank-node declaration"))?;
    assert!(model.nodes.is_empty());
    assert_eq!(model.zone_local_environments[0].outdoor_air_node, None);
    assert_eq!(model.zones[0].linked_outdoor_air_node, None);
    Ok(())
}

#[test]
fn local_environment_rejects_invalid_zone_fields_and_multi_node_lists()
-> Result<(), Box<dyn std::error::Error>> {
    for (epjson, code) in [
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "ZoneProperty:LocalEnvironment":{"Missing Zone":{}}
            }"#,
            "MissingRequiredField",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "ZoneProperty:LocalEnvironment":{
                    "Unknown Zone":{"zone_name":"Missing Zone"}
                }
            }"#,
            "MissingReference",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "ZoneProperty:LocalEnvironment":{
                    "Bad Zone Type":{"zone_name":3}
                }
            }"#,
            "InvalidFieldType",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "ZoneProperty:LocalEnvironment":{
                    "Bad Node Type":{"zone_name":"Only Zone","outdoor_air_node_name":3}
                }
            }"#,
            "InvalidFieldType",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "NodeList":{
                    "Two Nodes":{"nodes":[
                        {"node_name":"Node One"},
                        {"node_name":"Node Two"}
                    ]}
                },
                "ZoneProperty:LocalEnvironment":{
                    "List Alias":{
                        "zone_name":"Only Zone",
                        "outdoor_air_node_name":"Two Nodes"
                    }
                }
            }"#,
            "InvalidSingleNodeReference",
        ),
    ] {
        let result = compile_raw_model(&parse_epjson_str(epjson)?);
        assert!(
            has_error(&result, code),
            "code={code}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
    }
    Ok(())
}

#[test]
fn invalid_local_environment_does_not_reserve_identity_node_or_zone_link()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone":{"Only Zone":{}},
            "ZoneProperty:LocalEnvironment":{
                "Shared Environment":{
                    "zone_name":"Missing Zone",
                    "outdoor_air_node_name":"Should Not Register"
                },
                "shared environment":{
                    "zone_name":"Only Zone",
                    "outdoor_air_node_name":"Valid Node"
                }
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_zones(&mut model);
    compiler.parse_node_lists(&mut model);
    compiler.parse_zone_local_environments(&mut model);

    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_type == "ZoneProperty:LocalEnvironment"
    }));
    assert_eq!(model.zone_local_environments.len(), 1);
    assert_eq!(
        model.zone_local_environments[0].id,
        ZoneLocalEnvironmentId(0)
    );
    assert_eq!(
        model.zone_local_environments[0].name.0,
        "SHARED ENVIRONMENT"
    );
    assert!(model.node_names.resolve("Should Not Register").is_none());
    assert_eq!(model.node_names.resolve("Valid Node"), Some(NodeId(0)));
    assert_eq!(model.zones[0].linked_outdoor_air_node, Some(NodeId(0)));
    Ok(())
}

#[test]
fn valid_normalized_duplicate_local_environment_name_is_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "Zone":{"Only Zone":{}},
            "ZoneProperty:LocalEnvironment":{
                "Duplicate Name":{"zone_name":"Only Zone"},
                "duplicate name":{"zone_name":"Only Zone"}
            }
        }"#,
    )?);
    assert!(has_error(&result, "DuplicateName"));
    assert!(result.model.is_none());
    Ok(())
}
