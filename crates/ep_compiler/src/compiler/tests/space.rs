use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{AutoOrNumber, SpaceId, SpaceOrigin, SpaceTypeId, TypedModel};
use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

fn has_error(result: &CompileResult, object_type: &str, code: &str) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.object_type == object_type
            && diagnostic.code == code
    })
}

#[test]
fn get_space_data_materializes_spaces_lists_types_and_zone_defaults()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone": {"Zone 1": {}, "Zone 2": {}},
            "Space": {
                "Space 1a": {"zone_name":"Zone 1"},
                "Space 1b": {
                    "zone_name":"Zone 1",
                    "ceiling_height":3.2,
                    "volume":125.0,
                    "floor_area":100.0,
                    "space_type":"Office",
                    "tags":[
                        {"tag":"Tag1"},
                        {},
                        {"tag":""},
                        {"tag":"Tag2"}
                    ]
                }
            },
            "SpaceList": {
                "Some Spaces": {
                    "spaces":[
                        {"space_name":"Space 1a"},
                        {"space_name":"Space 1b"}
                    ]
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(typed_coverage_status("Space"), ObjectCoverageStatus::Typed);
    assert_eq!(
        typed_coverage_status("SpaceList"),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|coverage| {
        coverage.object_type == "Space"
            && coverage.object_count == 2
            && coverage.status == ObjectCoverageStatus::Typed
    }));

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed spaces"))?;
    assert_eq!(model.object_count(), 6);
    assert_eq!(model.spaces.len(), 3);
    assert_eq!(model.authored_space_names.len(), 2);
    assert_eq!(
        model
            .space_type_names
            .names()
            .iter()
            .map(|name| name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["GENERAL", "OFFICE"]
    );

    let defaulted = &model.spaces[0];
    assert_eq!(defaulted.id, SpaceId(0));
    assert_eq!(defaulted.name.0, "SPACE 1A");
    assert_eq!(defaulted.zone.0, 0);
    assert_eq!(defaulted.ceiling_height, AutoOrNumber::AutoCalculate);
    assert_eq!(defaulted.volume, AutoOrNumber::AutoCalculate);
    assert_eq!(defaulted.floor_area, AutoOrNumber::AutoCalculate);
    assert_eq!(defaulted.space_type.0, "GENERAL");
    assert_eq!(defaulted.space_type_id, SpaceTypeId(0));
    assert!(defaulted.tags.is_empty());
    assert_eq!(defaulted.origin, SpaceOrigin::Authored);

    let office = &model.spaces[1];
    assert_eq!(office.ceiling_height, AutoOrNumber::Value(3.2));
    assert_eq!(office.volume, AutoOrNumber::Value(125.0));
    assert_eq!(office.floor_area, AutoOrNumber::Value(100.0));
    assert_eq!(office.space_type.0, "OFFICE");
    assert_eq!(office.space_type_id, SpaceTypeId(1));
    assert_eq!(
        office
            .tags
            .iter()
            .map(|tag| tag.0.as_str())
            .collect::<Vec<_>>(),
        vec!["TAG1", "", "", "TAG2"]
    );

    let zone_default = &model.spaces[2];
    assert_eq!(zone_default.id, SpaceId(2));
    assert_eq!(zone_default.name.0, "ZONE 2");
    assert_eq!(zone_default.zone.0, 1);
    assert_eq!(zone_default.space_type_id, SpaceTypeId(0));
    assert_eq!(zone_default.origin, SpaceOrigin::AutoZoneDefault);
    assert_eq!(model.authored_space_names.resolve("Zone 2"), None);
    assert_eq!(model.zones[0].spaces, vec![SpaceId(0), SpaceId(1)]);
    assert_eq!(model.zones[1].spaces, vec![SpaceId(2)]);

    assert_eq!(model.space_lists.len(), 1);
    assert_eq!(model.space_lists[0].spaces, vec![SpaceId(0), SpaceId(1)]);
    assert_eq!(model.space_lists[0].max_space_name_length, 8);
    Ok(())
}

#[test]
fn spaces_and_space_lists_stay_lexical_even_with_staged_idf_order()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = r#"{
        "Zone":{"Only Zone":{}},
        "Space":{
            "Zulu Space":{"zone_name":"Only Zone","space_type":"Zulu Type"},
            "Alpha Space":{"zone_name":"Only Zone","space_type":"Alpha Type"}
        },
        "SpaceList":{
            "Zulu List":{},
            "Alpha List":{}
        }
    }"#;
    let idf = r#"
        Zone, Only Zone;
        Space, Zulu Space, Only Zone, Autocalculate, Autocalculate, Autocalculate, Zulu Type;
        Space, Alpha Space, Only Zone, Autocalculate, Autocalculate, Autocalculate, Alpha Type;
        SpaceList, Zulu List;
        SpaceList, Alpha List;
    "#;

    let staged_raw = parse_epjson_str_with_idf_order(epjson, idf)?;
    assert!(!staged_raw.has_idf_declaration_order("Space"));
    assert!(!staged_raw.has_idf_declaration_order("SpaceList"));
    for raw in [&staged_raw, &parse_epjson_str(epjson)?] {
        let result = compile_raw_model(raw);
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        let model = result
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected lexical spaces"))?;
        assert_eq!(
            model
                .spaces
                .iter()
                .map(|space| space.name.0.as_str())
                .collect::<Vec<_>>(),
            vec!["ALPHA SPACE", "ZULU SPACE"]
        );
        assert_eq!(
            model
                .space_lists
                .iter()
                .map(|list| list.name.0.as_str())
                .collect::<Vec<_>>(),
            vec!["ALPHA LIST", "ZULU LIST"]
        );
        assert_eq!(
            model
                .space_type_names
                .names()
                .iter()
                .map(|name| name.0.as_str())
                .collect::<Vec<_>>(),
            vec!["ALPHA TYPE", "ZULU TYPE"]
        );
    }
    Ok(())
}

#[test]
fn zones_without_authored_spaces_receive_general_defaults_outside_authored_name_map()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{"Zone":{"Zulu Zone":{},"Alpha Zone":{}}}"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected default spaces"))?;
    assert_eq!(model.object_count(), 3);
    assert!(model.authored_space_names.is_empty());
    assert_eq!(model.space_type_names.names()[0].0, "GENERAL");
    assert_eq!(
        model
            .spaces
            .iter()
            .map(|space| (space.name.0.as_str(), space.origin))
            .collect::<Vec<_>>(),
        vec![
            ("ALPHA ZONE", SpaceOrigin::AutoZoneDefault),
            ("ZULU ZONE", SpaceOrigin::AutoZoneDefault)
        ]
    );
    assert_eq!(model.zones[0].spaces, vec![SpaceId(0)]);
    assert_eq!(model.zones[1].spaces, vec![SpaceId(1)]);
    Ok(())
}

#[test]
fn default_spaces_follow_staged_idf_zone_order() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str_with_idf_order(
        r#"{"Zone":{"Alpha Zone":{},"Zulu Zone":{}}}"#,
        r#"
            Zone, Zulu Zone;
            Zone, Alpha Zone;
        "#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected staged default spaces"))?;
    assert_eq!(
        model
            .spaces
            .iter()
            .map(|space| space.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ZULU ZONE", "ALPHA ZONE"]
    );
    assert_eq!(model.zones[0].spaces, vec![SpaceId(0)]);
    assert_eq!(model.zones[1].spaces, vec![SpaceId(1)]);
    Ok(())
}

#[test]
fn authored_spaces_preserve_zero_and_negative_geometry_and_default_blank_type()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "Zone":{"Only Zone":{}},
            "Space":{
                "Only Space":{
                    "zone_name":"Only Zone",
                    "ceiling_height":0.0,
                    "volume":-125.0,
                    "floor_area":-100.0,
                    "space_type":""
                }
            }
        }"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected authored space"))?;
    assert_eq!(model.spaces.len(), 1);
    assert_eq!(model.spaces[0].ceiling_height, AutoOrNumber::Value(0.0));
    assert_eq!(model.spaces[0].volume, AutoOrNumber::Value(-125.0));
    assert_eq!(model.spaces[0].floor_area, AutoOrNumber::Value(-100.0));
    assert_eq!(model.spaces[0].space_type.0, "GENERAL");
    assert_eq!(model.spaces[0].space_type_id, SpaceTypeId(0));
    assert_eq!(model.spaces[0].origin, SpaceOrigin::Authored);
    Ok(())
}

#[test]
fn get_space_data_rejects_invalid_space_fields() -> Result<(), Box<dyn std::error::Error>> {
    for (epjson, code) in [
        (
            r#"{"Zone":{"Only Zone":{}},"Space":{"Missing Zone":{}}}"#,
            "MissingRequiredField",
        ),
        (
            r#"{"Zone":{"Only Zone":{}},"Space":{"Unknown Zone":{"zone_name":"No Zone"}}}"#,
            "MissingReference",
        ),
        (
            r#"{"Zone":{"Only Zone":{}},"Space":{"Bad Number":{"zone_name":"Only Zone","volume":"Autosize"}}}"#,
            "InvalidFieldType",
        ),
        (
            r#"{"Zone":{"Only Zone":{}},"Space":{"Bad Type":{"zone_name":"Only Zone","space_type":3}}}"#,
            "InvalidFieldType",
        ),
        (
            r#"{"Zone":{"Only Zone":{}},"Space":{"Bad Tags":{"zone_name":"Only Zone","tags":{}}}}"#,
            "InvalidFieldType",
        ),
        (
            r#"{"Zone":{"Only Zone":{}},"Space":{"Bad Tag":{"zone_name":"Only Zone","tags":[{"tag":3}]}}}"#,
            "InvalidFieldType",
        ),
    ] {
        let result = compile_raw_model(&parse_epjson_str(epjson)?);
        assert!(
            has_error(&result, "Space", code),
            "code={code}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
    }
    Ok(())
}

#[test]
fn invalid_space_does_not_reserve_identity_type_or_zone_link()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone":{"Only Zone":{}},
            "Space":{
                "Shared Space":{"zone_name":"Missing Zone","space_type":"Invalid Type"},
                "shared space":{"zone_name":"Only Zone","space_type":"Valid Type"}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_zones(&mut model);
    compiler.parse_space_data(&mut model);

    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference" && diagnostic.object_type == "Space"
    }));
    assert_eq!(model.spaces.len(), 1);
    assert_eq!(model.spaces[0].id, SpaceId(0));
    assert_eq!(model.spaces[0].name.0, "SHARED SPACE");
    assert_eq!(model.spaces[0].space_type.0, "VALID TYPE");
    assert_eq!(model.space_type_names.resolve("Invalid Type"), None);
    assert_eq!(model.zones[0].spaces, vec![SpaceId(0)]);
    Ok(())
}

#[test]
fn space_lists_allow_empty_input_and_reject_invalid_members_and_name_collisions()
-> Result<(), Box<dyn std::error::Error>> {
    let valid = compile_raw_model(&parse_epjson_str(
        r#"{
            "Zone":{"Only Zone":{}},
            "Space":{"Only Space":{"zone_name":"Only Zone"}},
            "SpaceList":{"Absent Members":{},"Empty Members":{"spaces":[]}}
        }"#,
    )?);
    assert!(!valid.has_errors(), "{:?}", valid.report.diagnostics);
    let valid_model = valid
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected empty space lists"))?;
    assert_eq!(valid_model.space_lists.len(), 2);
    assert!(
        valid_model
            .space_lists
            .iter()
            .all(|list| list.spaces.is_empty())
    );

    for (epjson, code) in [
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "Space":{"Only Space":{"zone_name":"Only Zone"}},
                "SpaceList":{"Unknown":{"spaces":[{"space_name":"Missing"}]}}
            }"#,
            "MissingReference",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "SpaceList":{"Generated Default Is Not Authored":{"spaces":[
                    {"space_name":"Only Zone"}
                ]}}
            }"#,
            "MissingReference",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "Space":{"Only Space":{"zone_name":"Only Zone"}},
                "SpaceList":{"Duplicate":{"spaces":[
                    {"space_name":"Only Space"},{"space_name":"only space"}
                ]}}
            }"#,
            "DuplicateSpaceListMember",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "Space":{"Only Space":{"zone_name":"Only Zone"}},
                "SpaceList":{"Only Zone":{}}
            }"#,
            "SpaceListNameMatchesZone",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "Space":{"Only Space":{"zone_name":"Only Zone"}},
                "SpaceList":{"Only Space":{}}
            }"#,
            "SpaceListNameMatchesSpace",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "Space":{"Only Space":{"zone_name":"Only Zone"}},
                "SpaceList":{"Malformed":{"spaces":[3]}}
            }"#,
            "InvalidFieldType",
        ),
    ] {
        let result = compile_raw_model(&parse_epjson_str(epjson)?);
        assert!(
            has_error(&result, "SpaceList", code),
            "code={code}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
    }
    Ok(())
}

#[test]
fn normalized_duplicate_space_and_space_list_names_are_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    for (epjson, object_type) in [
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "Space":{
                    "Duplicate Name":{"zone_name":"Only Zone"},
                    "duplicate name":{"zone_name":"Only Zone"}
                }
            }"#,
            "Space",
        ),
        (
            r#"{
                "Zone":{"Only Zone":{}},
                "SpaceList":{"Duplicate Name":{},"duplicate name":{}}
            }"#,
            "SpaceList",
        ),
    ] {
        let result = compile_raw_model(&parse_epjson_str(epjson)?);
        assert!(has_error(&result, object_type, "DuplicateName"));
        assert!(result.model.is_none());
    }
    Ok(())
}
