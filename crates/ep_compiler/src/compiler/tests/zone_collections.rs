use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{TypedModel, ZoneGroupId, ZoneId, ZoneListId};
use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

fn has_error(result: &CompileResult, code: &str, object_type: &str) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == object_type
    })
}

#[test]
fn get_zone_data_collection_phase_materializes_lists_groups_and_zone_side_effects()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone": {
                "Alpha Zone": {},
                "Beta Zone": {},
                "Gamma Zone": {}
            },
            "ZoneList": {
                "Alpha List": {
                    "zones": [{"zone_name":"Gamma Zone"}]
                },
                "Zulu List": {
                    "zones": [
                        {"zone_name":"Beta Zone"},
                        {"zone_name":"Alpha Zone"}
                    ]
                }
            },
            "ZoneGroup": {
                "Alpha Group": {"zone_list_name":"Alpha List"},
                "Zulu Group": {
                    "zone_list_name":"Zulu List",
                    "zone_list_multiplier":4
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status("ZoneList"),
        ObjectCoverageStatus::Typed
    );
    assert_eq!(
        typed_coverage_status("ZoneGroup"),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|coverage| {
        coverage.object_type == "ZoneList"
            && coverage.object_count == 2
            && coverage.status == ObjectCoverageStatus::Typed
    }));
    assert!(result.report.coverage.iter().any(|coverage| {
        coverage.object_type == "ZoneGroup"
            && coverage.object_count == 2
            && coverage.status == ObjectCoverageStatus::Typed
    }));

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed zone collections"))?;
    assert_eq!(model.object_count(), 8);
    assert_eq!(model.zone_lists.len(), 2);
    assert_eq!(model.zone_lists[0].id, ZoneListId(0));
    assert_eq!(model.zone_lists[0].name.0, "ALPHA LIST");
    assert_eq!(model.zone_lists[0].zones, vec![ZoneId(2)]);
    assert_eq!(model.zone_lists[0].max_zone_name_length, 10);
    assert_eq!(model.zone_lists[1].id, ZoneListId(1));
    assert_eq!(model.zone_lists[1].zones, vec![ZoneId(1), ZoneId(0)]);

    assert_eq!(model.zone_groups.len(), 2);
    assert_eq!(model.zone_groups[0].id, ZoneGroupId(0));
    assert_eq!(model.zone_groups[0].zone_list, ZoneListId(0));
    assert_eq!(model.zone_groups[0].multiplier, 1);
    assert_eq!(model.zone_groups[1].id, ZoneGroupId(1));
    assert_eq!(model.zone_groups[1].zone_list, ZoneListId(1));
    assert_eq!(model.zone_groups[1].multiplier, 4);

    assert_eq!(model.zones[0].list_multiplier, 4);
    assert_eq!(model.zones[0].list_group, Some(ZoneListId(1)));
    assert_eq!(model.zones[1].list_multiplier, 4);
    assert_eq!(model.zones[1].list_group, Some(ZoneListId(1)));
    assert_eq!(model.zones[2].list_multiplier, 1);
    assert_eq!(model.zones[2].list_group, Some(ZoneListId(0)));
    assert!(result.report.defaults_applied.iter().any(|default| {
        default.object_type == "ZoneGroup"
            && default.object_name == "Alpha Group"
            && default.field == "zone_list_multiplier"
            && default.value == "1"
    }));
    // Unit100 Source groups preserve exact names and still assign ZoneList IDs,
    // even when an unused earlier list makes the real Group/List indices differ.
    for (multiplier_field, expected) in [
        ("", 1_u32),
        (",\"zone_list_multiplier\":2", 2),
        (",\"zone_list_multiplier\":2147483647", i32::MAX as u32),
    ] {
        let text = format!(
            r#"{{"Zone":{{"A":{{}},"B":{{}}}},"ZoneList":{{
            "A unused":{{"zones":[{{"zone_name":"A"}}]}},
            "Both":{{"zones":[{{"zone_name":"B"}},{{"zone_name":"A"}}]}}}},
            "ZoneGroup":{{" G ":{{"zone_list_name":"Both"{multiplier_field}}}}}}}"#
        );
        let raw = parse_epjson_str(&text)?;
        let source = super::super::compile_raw_model_for_source(&raw);
        assert!(!source.has_errors(), "{:?}", source.report.diagnostics);
        let source = source.model.ok_or("actual exact Source Group")?;
        assert_eq!(source.zone_groups.len(), 1);
        let group = &source.zone_groups[0];
        assert_eq!(group.id, ZoneGroupId(0));
        assert_eq!(group.zone_list, ZoneListId(1));
        assert_ne!(group.id.0, group.zone_list.0);
        assert_eq!(group.name.0, " G ");
        assert_eq!(group.multiplier, expected);
        assert_eq!(source.zone_group_names.resolve(" g "), Some(group.id));
        assert!(source.zone_group_names.resolve("G").is_none());
        for zone in &source.zones {
            assert_eq!(zone.list_group, Some(group.zone_list));
            assert_eq!(zone.list_multiplier, expected);
        }
        let legacy = compile_raw_model(&raw)
            .model
            .ok_or("legacy Group normalization")?;
        assert_eq!(legacy.zone_groups[0].name.0, "G");
        assert_eq!(legacy.zone_group_names.resolve("G"), Some(ZoneGroupId(0)));
    }
    Ok(())
}

#[test]
fn nominal_control_scan_is_case_insensitive_and_independent_of_full_connection_parsing()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone": {
                "Alpha Zone": {},
                "Beta Zone": {},
                "Gamma Zone": {}
            },
            "ZoneHVAC:EquipmentConnections": {
                "Incomplete Connection": {"zone_name":"bEtA zOnE"},
                "Unmatched Connection": {"zone_name":"Missing Zone"}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_zones(&mut model);
    compiler.mark_nominal_controlled_zones(&mut model);

    assert!(!model.zones[0].is_nominal_controlled);
    assert!(model.zones[1].is_nominal_controlled);
    assert!(!model.zones[2].is_nominal_controlled);
    assert!(compiler.diagnostics.is_empty());
    Ok(())
}

#[test]
fn zone_collections_preserve_staged_idf_order_and_native_lexical_order()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = r#"{
        "Zone": {
            "Zulu Zone": {},
            "Alpha Zone": {}
        },
        "ZoneList": {
            "Zulu List": {"zones":[{"zone_name":"Zulu Zone"}]},
            "Alpha List": {"zones":[{"zone_name":"Alpha Zone"}]}
        },
        "ZoneGroup": {
            "Zulu Group": {"zone_list_name":"Zulu List", "zone_list_multiplier":2},
            "Alpha Group": {"zone_list_name":"Alpha List", "zone_list_multiplier":3}
        }
    }"#;
    let idf = r#"
        Zone, Zulu Zone;
        Zone, Alpha Zone;
        ZoneList, Zulu List, Zulu Zone;
        ZoneList, Alpha List, Alpha Zone;
        ZoneGroup, Zulu Group, Zulu List, 2;
        ZoneGroup, Alpha Group, Alpha List, 3;
    "#;

    let staged_raw = parse_epjson_str_with_idf_order(epjson, idf)?;
    assert!(staged_raw.has_idf_declaration_order("ZoneList"));
    assert!(staged_raw.has_idf_declaration_order("ZoneGroup"));
    let staged = compile_raw_model(&staged_raw);
    assert!(!staged.has_errors(), "{:?}", staged.report.diagnostics);
    let staged_model = staged
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected staged zone collections"))?;
    assert_eq!(
        staged_model
            .zone_lists
            .iter()
            .map(|list| (list.id, list.name.0.as_str()))
            .collect::<Vec<_>>(),
        vec![(ZoneListId(0), "ZULU LIST"), (ZoneListId(1), "ALPHA LIST")]
    );
    assert_eq!(
        staged_model
            .zone_groups
            .iter()
            .map(|group| (group.id, group.name.0.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (ZoneGroupId(0), "ZULU GROUP"),
            (ZoneGroupId(1), "ALPHA GROUP")
        ]
    );

    let native = compile_raw_model(&parse_epjson_str(epjson)?);
    assert!(!native.has_errors(), "{:?}", native.report.diagnostics);
    let native_model = native
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected native zone collections"))?;
    assert_eq!(native_model.zone_lists[0].name.0, "ALPHA LIST");
    assert_eq!(native_model.zone_groups[0].name.0, "ALPHA GROUP");
    let numeric = r#"{"Zone":{"A":{},"B":{}},"ZoneList":{
        "L10":{"zones":[{"zone_name":"A"}]},"L2":{"zones":[{"zone_name":"B"}]}},
        "ZoneGroup":{"G10":{"zone_list_name":"L10","zone_list_multiplier":2},
        "G2":{"zone_list_name":"L2"}}}"#;
    let numeric_idf =
        "Zone,A;Zone,B;ZoneList,L10,A;ZoneList,L2,B;ZoneGroup,G10,L10,2;ZoneGroup,G2,L2;";
    for staged in [false, true] {
        let raw = if staged {
            parse_epjson_str_with_idf_order(numeric, numeric_idf)?
        } else {
            parse_epjson_str(numeric)?
        };
        assert_eq!(raw.has_idf_declaration_order("ZoneGroup"), staged);
        let compiled = super::super::compile_raw_model_for_source(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let model = compiled.model.ok_or("Source group declaration order")?;
        let actual = model
            .zone_groups
            .iter()
            .map(|group| group.name.0.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            actual,
            if staged {
                vec!["G10", "G2"]
            } else {
                vec!["G2", "G10"]
            }
        );
        for (index, ((name, _), group)) in raw
            .source_ordered_instances("ZoneGroup")?
            .iter()
            .zip(&model.zone_groups)
            .enumerate()
        {
            assert_eq!(usize::try_from(group.id.0)?, index);
            assert_eq!(group.name.0, name.0.to_ascii_uppercase());
        }
    }
    let exact = parse_epjson_str(
        r#"{"Zone":{"A":{},"B":{}},"ZoneList":{
        " L ":{"zones":[{"zone_name":"A"}]},"L":{"zones":[{"zone_name":"B"}]}},
        "ZoneGroup":{" G ":{"zone_list_name":" l "},"G":{"zone_list_name":"l"}}}"#,
    )?;
    let exact = super::super::compile_raw_model_for_source(&exact);
    assert!(!exact.has_errors(), "{:?}", exact.report.diagnostics);
    let exact = exact.model.ok_or("exact Group and ZoneList namespaces")?;
    assert_eq!(exact.zone_groups.len(), 2);
    assert_ne!(
        exact.zone_group_names.resolve("G"),
        exact.zone_group_names.resolve(" G ")
    );
    assert_ne!(
        exact.zone_list_names.resolve("L"),
        exact.zone_list_names.resolve(" L ")
    );
    Ok(())
}

#[test]
fn zone_list_rejects_empty_unknown_duplicate_and_malformed_members()
-> Result<(), Box<dyn std::error::Error>> {
    for (epjson, code) in [
        (
            r#"{"Zone":{"Zone One":{}},"ZoneList":{"Empty":{"zones":[]}}}"#,
            "MissingZoneListMember",
        ),
        (
            r#"{"Zone":{"Zone One":{}},"ZoneList":{"Unknown":{"zones":[{"zone_name":"Missing"}]}}}"#,
            "MissingReference",
        ),
        (
            r#"{"Zone":{"Zone One":{}},"ZoneList":{"Duplicate":{"zones":[{"zone_name":"Zone One"},{"zone_name":"zone one"}]}}}"#,
            "DuplicateZoneListMember",
        ),
        (
            r#"{"Zone":{"Zone One":{}},"ZoneList":{"Wrong Type":{"zones":{}}}}"#,
            "InvalidFieldType",
        ),
        (
            r#"{"Zone":{"Zone One":{}},"ZoneList":{"Wrong Entry":{"zones":[3]}}}"#,
            "InvalidFieldType",
        ),
    ] {
        let result = compile_raw_model(&parse_epjson_str(epjson)?);
        assert!(
            has_error(&result, code, "ZoneList"),
            "code={code}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
    }
    Ok(())
}

#[test]
fn zone_group_rejects_bad_references_multipliers_and_group_overlap()
-> Result<(), Box<dyn std::error::Error>> {
    for (epjson, code) in [
        (
            r#"{
                "Zone":{"Zone One":{}},
                "ZoneGroup":{"Unknown":{"zone_list_name":"Missing"}}
            }"#,
            "MissingReference",
        ),
        (
            r#"{
                "Zone":{"Zone One":{}},
                "ZoneList":{"List":{"zones":[{"zone_name":"Zone One"}]}},
                "ZoneGroup":{"Zero":{"zone_list_name":"List","zone_list_multiplier":0}}
            }"#,
            "InvalidNumericRange",
        ),
        (
            r#"{
                "Zone":{"Zone One":{}},
                "ZoneList":{"List":{"zones":[{"zone_name":"Zone One"}]}},
                "ZoneGroup":{"Fractional":{"zone_list_name":"List","zone_list_multiplier":1.5}}
            }"#,
            "InvalidInteger",
        ),
        (
            r#"{
                "Zone":{"Zone One":{}},
                "ZoneList":{"List":{"zones":[{"zone_name":"Zone One"}]}},
                "ZoneGroup":{
                    "Alpha":{"zone_list_name":"List"},
                    "Beta":{"zone_list_name":"List"}
                }
            }"#,
            "DuplicateZoneGroupList",
        ),
        (
            r#"{
                "Zone":{"Zone One":{}},
                "ZoneList":{
                    "Alpha List":{"zones":[{"zone_name":"Zone One"}]},
                    "Beta List":{"zones":[{"zone_name":"Zone One"}]}
                },
                "ZoneGroup":{
                    "Alpha":{"zone_list_name":"Alpha List"},
                    "Beta":{"zone_list_name":"Beta List"}
                }
            }"#,
            "ZoneInMultipleGroups",
        ),
    ] {
        let result = compile_raw_model(&parse_epjson_str(epjson)?);
        assert!(
            has_error(&result, code, "ZoneGroup"),
            "code={code}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
        let source = super::super::compile_raw_model_for_source(&parse_epjson_str(epjson)?);
        assert!(
            has_error(&source, code, "ZoneGroup"),
            "{:?}",
            source.report.diagnostics
        );
        assert!(source.model.is_none());
    }
    for multiplier in ["2.0", "2e0", "\"2\"", "null", "true", "2147483648"] {
        let text = format!(
            r#"{{"Zone":{{"Z":{{}}}},"ZoneList":{{"L":{{"zones":[{{"zone_name":"Z"}}]}}}},
            "ZoneGroup":{{"G":{{"zone_list_name":"L","zone_list_multiplier":{multiplier}}}}}}}"#
        );
        let raw = parse_epjson_str(&text)?;
        assert!(
            super::super::compile_raw_model_for_source(&raw).has_errors(),
            "Source Group requires the original integer JSON kind and defined signed range: {multiplier}"
        );
        if matches!(multiplier, "2.0" | "2e0") {
            assert!(
                !compile_raw_model(&raw).has_errors(),
                "legacy integral-double typing is unchanged"
            );
        }
    }
    for text in [
        r#"{"Zone":{"Z":{}},"ZoneList":{" L ":{"zones":[{"zone_name":"Z"}]}},"ZoneGroup":{"G":{"zone_list_name":"L"}}}"#,
        r#"{"Zone":{"Z":{}},"ZoneList":{"L":{"zones":[{"zone_name":"Z"}]}},"ZoneGroup":{"G\nX":{"zone_list_name":"L"}}}"#,
    ] {
        assert!(super::super::compile_raw_model_for_source(&parse_epjson_str(text)?).has_errors());
    }
    Ok(())
}

#[test]
fn invalid_zone_collections_do_not_reserve_normalized_identity_or_dense_id()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone":{"Zone One":{}},
            "ZoneList":{
                "Shared List":{"zones":[{"zone_name":"Missing"}]},
                "shared list":{"zones":[{"zone_name":"Zone One"}]}
            },
            "ZoneGroup":{
                "Shared Group":{"zone_list_name":"Missing"},
                "shared group":{"zone_list_name":"shared list","zone_list_multiplier":2}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_zones(&mut model);
    compiler.parse_zone_lists(&mut model);
    compiler.parse_zone_groups(&mut model);

    assert_eq!(model.zone_lists.len(), 1);
    assert_eq!(model.zone_lists[0].id, ZoneListId(0));
    assert_eq!(model.zone_lists[0].name.0, "SHARED LIST");
    assert_eq!(model.zone_groups.len(), 1);
    assert_eq!(model.zone_groups[0].id, ZoneGroupId(0));
    assert_eq!(model.zone_groups[0].name.0, "SHARED GROUP");
    assert_eq!(model.zones[0].list_multiplier, 2);
    assert_eq!(model.zones[0].list_group, Some(ZoneListId(0)));
    Ok(())
}

#[test]
fn zone_list_name_matching_zone_is_a_nonblocking_source_warning()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "Zone":{"Shared Name":{}},
            "ZoneList":{"shared name":{"zones":[{"zone_name":"Shared Name"}]}}
        }"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.model.is_some());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Warning
            && diagnostic.code == "ZoneListNameMatchesZone"
            && diagnostic.object_type == "ZoneList"
    }));
    // Unit94 Source list names share the exact namespace policy, not legacy
    // trimming, and only an actual Zone spelling produces the Native warning.
    let raw = parse_epjson_str(
        r#"{"Zone":{"A":{}},"ZoneList":{"A":{"zones":[{"zone_name":"A"}]}," A ":{"zones":[{"zone_name":"A"}]}}}"#,
    )?;
    let exact = super::super::compile_raw_model_for_source(&raw);
    assert!(!exact.has_errors(), "{:?}", exact.report.diagnostics);
    assert_eq!(
        exact
            .report
            .diagnostics
            .iter()
            .filter(|d| d.code == "ZoneListNameMatchesZone")
            .count(),
        1
    );
    let model = exact.model.ok_or("exact Source ZoneList names")?;
    assert_eq!(model.zone_lists.len(), 2);
    for list in &model.zone_lists {
        assert_eq!(
            model
                .zone_list_names
                .resolve(&list.name.0.to_ascii_lowercase()),
            Some(list.id)
        );
        assert_eq!(list.zones, vec![model.zones[0].id]);
    }
    assert_ne!(
        model.zone_list_names.resolve("A"),
        model.zone_list_names.resolve(" A ")
    );
    assert_eq!(model.zone_list_names.resolve(" A"), None);
    assert!(
        compile_raw_model(&raw).has_errors(),
        "legacy list namespace still trims"
    );
    for text in [
        r#"{"Zone":{"A":{}},"ZoneList":{"L":{"zones":[{"zone_name":" A "}]}}}"#,
        r#"{"Zone":{"A":{}},"ZoneList":{"A\nB":{"zones":[{"zone_name":"A"}]}}}"#,
    ] {
        assert!(super::super::compile_raw_model_for_source(&parse_epjson_str(text)?).has_errors());
    }
    Ok(())
}
