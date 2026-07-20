use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    AutoOrNumber, InsideSurfaceConvectionAlgorithm, OutsideSurfaceConvectionAlgorithm, TypedModel,
    ZoneConvectionAlgorithm, ZoneId,
};
use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

const OBJECT_TYPE: &str = "Zone";

fn has_error(result: &CompileResult, code: &str, object_name: &str, field: Option<&str>) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && field.is_none_or(|field| diagnostic.field.as_deref() == Some(field))
    })
}

#[test]
fn zone_materializes_complete_public_inputs_and_source_defaults()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "SurfaceConvectionAlgorithm:Inside": {
                "Inside Defaults": {"algorithm":"CeilingDiffuser"}
            },
            "SurfaceConvectionAlgorithm:Outside": {
                "Outside Defaults": {"algorithm":"TARP"}
            },
            "Zone": {
                "Authored Zone": {
                    "direction_of_relative_north":12.5,
                    "x_origin":1.0,
                    "y_origin":-2.0,
                    "z_origin":3.0,
                    "type":1,
                    "multiplier":3,
                    "ceiling_height":0.0,
                    "volume":-5.0,
                    "floor_area":45.5,
                    "zone_inside_convection_algorithm":"TrombeWall",
                    "zone_outside_convection_algorithm":"MoWiTT",
                    "part_of_total_floor_area":"No"
                },
                "Inherited Zone": {}
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
            && coverage.object_count == 2
            && coverage.status == ObjectCoverageStatus::Typed
    }));

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed zones"))?;
    assert_eq!(model.zones.len(), 2);
    let authored = &model.zones[0];
    assert_eq!(authored.id, ZoneId(0));
    assert_eq!(authored.name.0, "AUTHORED ZONE");
    assert_eq!(authored.direction_of_relative_north_deg, 12.5);
    assert_eq!(
        (
            authored.origin.x_m,
            authored.origin.y_m,
            authored.origin.z_m
        ),
        (1.0, -2.0, 3.0)
    );
    assert_eq!(authored.zone_type, 1);
    assert_eq!(authored.multiplier, 3);
    assert_eq!(authored.ceiling_height, AutoOrNumber::Value(0.0));
    assert_eq!(authored.volume, AutoOrNumber::Value(-5.0));
    assert_eq!(authored.floor_area, AutoOrNumber::Value(45.5));
    assert_eq!(
        authored.inside_convection_algorithm,
        ZoneConvectionAlgorithm::Override(InsideSurfaceConvectionAlgorithm::TrombeWall)
    );
    assert_eq!(
        authored.outside_convection_algorithm,
        ZoneConvectionAlgorithm::Override(OutsideSurfaceConvectionAlgorithm::MoWitt)
    );
    assert!(!authored.is_part_of_total_floor_area);

    let inherited = &model.zones[1];
    assert_eq!(inherited.id, ZoneId(1));
    assert_eq!(inherited.ceiling_height, AutoOrNumber::AutoCalculate);
    assert_eq!(inherited.volume, AutoOrNumber::AutoCalculate);
    assert_eq!(inherited.floor_area, AutoOrNumber::AutoCalculate);
    assert_eq!(
        inherited.inside_convection_algorithm,
        ZoneConvectionAlgorithm::Inherited(InsideSurfaceConvectionAlgorithm::CeilingDiffuser)
    );
    assert_eq!(
        inherited.outside_convection_algorithm,
        ZoneConvectionAlgorithm::Inherited(OutsideSurfaceConvectionAlgorithm::Tarp)
    );
    assert!(inherited.is_part_of_total_floor_area);
    Ok(())
}

#[test]
fn zone_blank_auto_and_alpha_fields_use_energyplus_defaults()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "Zone": {
                "Default Zone": {
                    "ceiling_height":"",
                    "volume":"Autocalculate",
                    "floor_area":"",
                    "zone_inside_convection_algorithm":"",
                    "zone_outside_convection_algorithm":"",
                    "part_of_total_floor_area":""
                }
            }
        }"#,
    )?);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let zone = &result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected defaulted zone"))?
        .zones[0];
    assert_eq!(zone.direction_of_relative_north_deg, 0.0);
    assert_eq!(
        (zone.origin.x_m, zone.origin.y_m, zone.origin.z_m),
        (0.0, 0.0, 0.0)
    );
    assert_eq!(zone.zone_type, 1);
    assert_eq!(zone.multiplier, 1);
    assert_eq!(zone.ceiling_height, AutoOrNumber::AutoCalculate);
    assert_eq!(zone.volume, AutoOrNumber::AutoCalculate);
    assert_eq!(zone.floor_area, AutoOrNumber::AutoCalculate);
    assert_eq!(
        zone.inside_convection_algorithm,
        ZoneConvectionAlgorithm::Inherited(InsideSurfaceConvectionAlgorithm::Tarp)
    );
    assert_eq!(
        zone.outside_convection_algorithm,
        ZoneConvectionAlgorithm::Inherited(OutsideSurfaceConvectionAlgorithm::Doe2)
    );
    assert!(zone.is_part_of_total_floor_area);
    Ok(())
}

#[test]
fn zone_preserves_staged_idf_order_and_native_lexical_order()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = r#"{
        "Zone": {
            "Zulu First In IDF": {},
            "Alpha Second In IDF": {}
        }
    }"#;
    let idf = r#"
        Zone,
          Zulu First In IDF;
        Zone,
          Alpha Second In IDF;
    "#;

    let staged_raw = parse_epjson_str_with_idf_order(epjson, idf)?;
    assert!(staged_raw.has_idf_declaration_order(OBJECT_TYPE));
    let staged = compile_raw_model(&staged_raw);
    assert!(!staged.has_errors(), "{:?}", staged.report.diagnostics);
    let staged_model = staged
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected staged zones"))?;
    assert_eq!(
        staged_model
            .zones
            .iter()
            .map(|zone| (zone.id, zone.name.0.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (ZoneId(0), "ZULU FIRST IN IDF"),
            (ZoneId(1), "ALPHA SECOND IN IDF")
        ]
    );

    let native = compile_raw_model(&parse_epjson_str(epjson)?);
    assert!(!native.has_errors(), "{:?}", native.report.diagnostics);
    assert_eq!(
        native
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected native zones"))?
            .zones
            .iter()
            .map(|zone| (zone.id, zone.name.0.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (ZoneId(0), "ALPHA SECOND IN IDF"),
            (ZoneId(1), "ZULU FIRST IN IDF")
        ]
    );
    Ok(())
}

#[test]
fn zone_rejects_invalid_names_ranges_types_and_enums() -> Result<(), Box<dyn std::error::Error>> {
    for (epjson, code, name, field) in [
        (
            r#"{"Zone":{"":{}}}"#,
            "MissingRequiredField",
            "",
            Some("name"),
        ),
        (
            r#"{"Zone":{"Wrong Type":{"type":2}}}"#,
            "InvalidNumericRange",
            "Wrong Type",
            Some("type"),
        ),
        (
            r#"{"Zone":{"Zero Multiplier":{"multiplier":0}}}"#,
            "InvalidNumericRange",
            "Zero Multiplier",
            Some("multiplier"),
        ),
        (
            r#"{"Zone":{"Fractional Multiplier":{"multiplier":1.5}}}"#,
            "InvalidInteger",
            "Fractional Multiplier",
            Some("multiplier"),
        ),
        (
            r#"{"Zone":{"Bad Inside":{"zone_inside_convection_algorithm":"Unknown"}}}"#,
            "InvalidEnumValue",
            "Bad Inside",
            Some("zone_inside_convection_algorithm"),
        ),
        (
            r#"{"Zone":{"Bad Outside":{"zone_outside_convection_algorithm":"Unknown"}}}"#,
            "InvalidEnumValue",
            "Bad Outside",
            Some("zone_outside_convection_algorithm"),
        ),
        (
            r#"{"Zone":{"Bad Floor Flag":{"part_of_total_floor_area":"Maybe"}}}"#,
            "InvalidEnumValue",
            "Bad Floor Flag",
            Some("part_of_total_floor_area"),
        ),
        (
            r#"{"Zone":{"Bad Auto":{"floor_area":"Autosize"}}}"#,
            "InvalidFieldType",
            "Bad Auto",
            Some("floor_area"),
        ),
        (
            r#"{"Zone":{"Blank North":{"direction_of_relative_north":""}}}"#,
            "InvalidFieldType",
            "Blank North",
            Some("direction_of_relative_north"),
        ),
    ] {
        let result = compile_raw_model(&parse_epjson_str(epjson)?);
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
fn invalid_zone_does_not_reserve_normalized_identity_or_dense_id()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone": {
                "Shared Name": {"type":2},
                "shared name": {"floor_area":12.0}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_zones(&mut model);

    assert_eq!(compiler.diagnostics.len(), 1);
    assert_eq!(model.zones.len(), 1);
    assert_eq!(model.zone_names.len(), 1);
    assert_eq!(model.zones[0].id, ZoneId(0));
    assert_eq!(model.zones[0].name.0, "SHARED NAME");
    assert_eq!(model.zones[0].floor_area, AutoOrNumber::Value(12.0));
    Ok(())
}
