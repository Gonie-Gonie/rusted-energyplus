use super::super::{
    CompileResult, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{ModelGraph, WindowDataFileSource};
use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

const OBJECT_TYPE: &str = "Construction:WindowDataFile";

fn has_error(result: &CompileResult, code: &str, object_name: &str, field: &str) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == Some(field)
    })
}

#[test]
fn window_data_file_requests_materialize_after_equivalent_layer_without_file_io()
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
            "Construction:WindowEquivalentLayer": {
                "EQL Window": {"outside_layer":"EQL Shade"}
            },
            "Construction:WindowDataFile": {
                "Zulu Default Request": {},
                "Alpha Explicit Request": {
                    "file_name":"Missing/MixedCase Window Library.dat"
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
            && coverage.object_count == 2
            && coverage.status == ObjectCoverageStatus::Typed
    }));

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed WINDOW5 requests"))?;
    assert_eq!(model.constructions.len(), 1);
    assert!(model.constructions[0].is_window_equivalent_layer());
    assert_eq!(model.construction_window_data_file_requests.len(), 2);
    let explicit = &model.construction_window_data_file_requests[0];
    assert_eq!(explicit.name.0, "ALPHA EXPLICIT REQUEST");
    assert_eq!(explicit.source_index, 0);
    assert_eq!(
        explicit.source,
        WindowDataFileSource::Explicit("Missing/MixedCase Window Library.dat".to_string())
    );
    assert_eq!(
        explicit.source.file_name(),
        "Missing/MixedCase Window Library.dat"
    );
    let defaulted = &model.construction_window_data_file_requests[1];
    assert_eq!(defaulted.name.0, "ZULU DEFAULT REQUEST");
    assert_eq!(defaulted.source_index, 1);
    assert!(defaulted.source.uses_default_working_directory());
    assert_eq!(defaulted.source.file_name(), "Window5DataFile.dat");
    assert_eq!(model.object_count(), 5);
    assert_eq!(result.report.typed_object_count, model.object_count());

    let graph = ModelGraph::from_typed(model);
    assert_eq!(graph.construction_materials.len(), 1);
    Ok(())
}

#[test]
fn window_data_file_requests_preserve_staged_idf_order_and_native_lexical_order()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = r#"{
        "Construction:WindowDataFile": {
            "Zulu First In IDF": {},
            "Alpha Second In IDF": {"file_name":"Library.dat"}
        }
    }"#;
    let idf = r#"
        Construction:WindowDataFile,
          Zulu First In IDF;
        Construction:WindowDataFile,
          Alpha Second In IDF,
          Library.dat;
    "#;

    let staged_raw = parse_epjson_str_with_idf_order(epjson, idf)?;
    assert!(staged_raw.has_idf_declaration_order(OBJECT_TYPE));
    let staged = compile_raw_model(&staged_raw);
    assert!(!staged.has_errors(), "{:?}", staged.report.diagnostics);
    let staged_model = staged
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected staged WINDOW5 requests"))?;
    assert_eq!(
        staged_model
            .construction_window_data_file_requests
            .iter()
            .map(|request| request.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ZULU FIRST IN IDF", "ALPHA SECOND IN IDF"]
    );
    assert_eq!(
        staged_model
            .construction_window_data_file_requests
            .iter()
            .map(|request| request.source_index)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );

    let native = compile_raw_model(&parse_epjson_str(epjson)?);
    assert!(!native.has_errors(), "{:?}", native.report.diagnostics);
    assert_eq!(
        native
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected native WINDOW5 requests"))?
            .construction_window_data_file_requests
            .iter()
            .map(|request| request.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["ALPHA SECOND IN IDF", "ZULU FIRST IN IDF"]
    );
    Ok(())
}

#[test]
fn window_data_file_request_defaults_blank_file_and_rejects_malformed_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let valid = compile_raw_model(&parse_epjson_str(
        r#"{
            "Construction:WindowDataFile": {
                "Blank File": {"file_name":"   "},
                "Missing File": {}
            }
        }"#,
    )?);
    assert!(!valid.has_errors(), "{:?}", valid.report.diagnostics);
    assert!(
        valid
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected defaulted WINDOW5 requests"))?
            .construction_window_data_file_requests
            .iter()
            .all(|request| request.source.uses_default_working_directory())
    );

    for (epjson, code, name, field) in [
        (
            r#"{"Construction:WindowDataFile":{"":{"file_name":"Library.dat"}}}"#,
            "MissingRequiredField",
            "",
            "name",
        ),
        (
            r#"{"Construction:WindowDataFile":{"Malformed":{"file_name":2}}}"#,
            "InvalidFieldType",
            "Malformed",
            "file_name",
        ),
    ] {
        let result = compile_raw_model(&parse_epjson_str(epjson)?);
        assert!(
            has_error(&result, code, name, field),
            "code={code}, name={name}, field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
    }
    Ok(())
}

#[test]
fn window_data_file_request_does_not_reserve_unmaterialized_construction_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Opaque": {"roughness":"Rough", "thermal_resistance":1.0}
            },
            "Construction": {
                "Shared Name": {"outside_layer":"Opaque"}
            },
            "Construction:WindowDataFile": {
                "shared name": {"file_name":"NotRead.dat"},
                "SHARED NAME": {}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected request-only WINDOW5 state"))?;
    assert_eq!(model.constructions.len(), 1);
    assert_eq!(model.construction_names.len(), 1);
    assert_eq!(model.construction_window_data_file_requests.len(), 2);
    assert!(
        model
            .construction_window_data_file_requests
            .iter()
            .all(|request| request.name.0 == "SHARED NAME")
    );
    Ok(())
}
