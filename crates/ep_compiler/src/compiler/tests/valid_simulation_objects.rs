use super::super::{CompileResult, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model};
use ep_raw_model::{RawModel, parse_epjson_str};

const ERROR_CODE: &str = "InvalidSimulationWithoutZones";
const SURFACE_WITNESSES: [&str; 2] = ["Shading:Site:Detailed", "Shading:Building:Detailed"];
const VALID_NO_ZONE_OBJECT_TYPES: [&str; 8] = [
    "SolarCollector:FlatPlate:Water",
    "Generator:Photovoltaic",
    "Generator:InternalCombustionEngine",
    "Generator:CombustionTurbine",
    "Generator:FuelCell",
    "Generator:MicroCHP",
    "Generator:MicroTurbine",
    "Generator:WindTurbine",
];

fn raw_with_object_types(object_types: &[&str]) -> Result<RawModel, Box<dyn std::error::Error>> {
    let objects = object_types
        .iter()
        .enumerate()
        .map(|(index, object_type)| format!(r#""{object_type}":{{"Object {index}":{{}}}}"#))
        .collect::<Vec<_>>()
        .join(",");
    Ok(parse_epjson_str(&format!("{{{objects}}}"))?)
}

fn invalid_no_zone_errors(result: &CompileResult) -> Vec<&super::super::ModelDiagnostic> {
    result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == ERROR_CODE)
        .collect()
}

#[test]
fn no_surface_witness_is_silent() -> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&raw_with_object_types(&[])?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.model.is_some());
    assert!(invalid_no_zone_errors(&result).is_empty());
    Ok(())
}

#[test]
fn each_bounded_detached_shading_witness_emits_one_error() -> Result<(), Box<dyn std::error::Error>>
{
    for witness in SURFACE_WITNESSES {
        let result = compile_raw_model(&raw_with_object_types(&[witness])?);
        let errors = invalid_no_zone_errors(&result);

        assert!(
            result.has_errors(),
            "{witness}: {:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
        assert_eq!(
            errors.len(),
            1,
            "{witness}: {:?}",
            result.report.diagnostics
        );
        assert_eq!(errors[0].severity, DiagnosticSeverity::Error);
        assert_eq!(errors[0].object_type, "GetHeatBalanceInput");
        assert_eq!(errors[0].object_name, None);
        assert_eq!(errors[0].field, None);
        assert_eq!(
            errors[0].message,
            "There are surfaces in input but no zones found. Invalid simulation."
        );
        assert_eq!(
            result
                .report
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
                .count(),
            1,
            "{witness}: {:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn every_source_allowed_object_suppresses_the_no_zone_error()
-> Result<(), Box<dyn std::error::Error>> {
    for allowed in VALID_NO_ZONE_OBJECT_TYPES {
        let result =
            compile_raw_model(&raw_with_object_types(&["Shading:Site:Detailed", allowed])?);

        assert!(
            !result.has_errors(),
            "{allowed}: {:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_some(), "{allowed}");
        assert!(
            invalid_no_zone_errors(&result).is_empty(),
            "{allowed}: {:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn retained_zone_suppresses_the_no_zone_error() -> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&raw_with_object_types(&[
        "Zone",
        "Shading:Building:Detailed",
    ])?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.zones.len(), 1);
    assert!(invalid_no_zone_errors(&result).is_empty());
    Ok(())
}

#[test]
fn existing_error_suppresses_the_no_zone_error() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Timestep":{"Broken":{"number_of_timesteps_per_hour":"not a number"}},
            "Shading:Site:Detailed":{"Detached":{}}
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    assert!(result.model.is_none());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error && diagnostic.object_type == "Timestep"
    }));
    assert!(invalid_no_zone_errors(&result).is_empty());
    Ok(())
}

#[test]
fn rectangular_site_shading_is_not_a_detailed_surface_witness()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&raw_with_object_types(&["Shading:Site"])?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.model.is_some());
    assert!(invalid_no_zone_errors(&result).is_empty());
    Ok(())
}

#[test]
fn error_free_raw_only_objects_retain_counts_and_coverage() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = raw_with_object_types(&["Shading:Building:Detailed", "Generator:WindTurbine"])?;
    let raw_object_count = raw.object_count();
    let result = compile_raw_model(&raw);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(result.report.raw_object_count, raw_object_count);
    assert_eq!(result.report.raw_object_count, 2);
    assert_eq!(result.report.coverage.len(), 2);
    for object_type in ["Shading:Building:Detailed", "Generator:WindTurbine"] {
        let coverage = result
            .report
            .coverage
            .iter()
            .find(|coverage| coverage.object_type == object_type)
            .ok_or_else(|| std::io::Error::other(format!("missing coverage for {object_type}")))?;
        assert_eq!(coverage.object_count, 1);
        assert_eq!(coverage.status, ObjectCoverageStatus::RawOnly);
    }
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.object_count(), result.report.typed_object_count);
    Ok(())
}
