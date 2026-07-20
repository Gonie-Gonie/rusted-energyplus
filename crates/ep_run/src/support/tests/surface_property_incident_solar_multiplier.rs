use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "SurfaceProperty:IncidentSolarMultiplier";

#[test]
fn incident_solar_multiplier_requests_are_all_definition_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "SurfaceProperty:IncidentSolarMultiplier": {
                "Unused Request": {
                    "surface_name": "Deferred Exterior Window",
                    "incident_solar_multiplier": 0.6
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed incident-solar request"))?;
    assert_eq!(model.surface_incident_solar_multiplier_requests.len(), 1);
    assert!(
        model.surfaces.is_empty(),
        "request-only typing must not create a surface"
    );

    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );
    assert_eq!(assessment.status, SupportStatus::Unsupported);
    assert_eq!(assessment.run_result_state, RunResultState::RunBlocked);
    assert_eq!(assessment.runtime_class, RuntimeClass::None);
    assert!(assessment.typed_objects.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE && entry.count == 1 && entry.status == "typed"
    }));
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE && entry.count == 1 && entry.status == "unsupported"
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some(OBJECT_TYPE)
            && diagnostic.blocking
    }));
    Ok(())
}
