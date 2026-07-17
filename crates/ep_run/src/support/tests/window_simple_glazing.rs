use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::MaterialKind;
use ep_raw_model::parse_epjson_str;

#[test]
fn typed_simple_glazing_materials_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "WindowMaterial:SimpleGlazingSystem": {
                "Default Visible Transmittance": {
                    "u_factor": 2.7,
                    "solar_heat_gain_coefficient": 0.38
                },
                "Unused Explicit Visible Transmittance": {
                    "u_factor": 4.2,
                    "solar_heat_gain_coefficient": 0.57,
                    "visible_transmittance": 0.61
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed simple glazing materials"))?;
    assert_eq!(model.materials.len(), 2);
    assert_eq!(
        model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowSimpleGlazing)
            .count(),
        2,
        "all simple glazing definitions must remain typed even when unused"
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
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == "WindowMaterial:SimpleGlazingSystem"
            && entry.count == 2
            && entry.status == "unsupported"
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.stage == "support"
            && diagnostic.object_type.as_deref()
                == Some("WindowMaterial:SimpleGlazingSystem")
            && diagnostic.message
                == "WindowMaterial:SimpleGlazingSystem is typed for graph/source-map diagnostics but not executable in arbitrary-run compatibility mode"
            && diagnostic.blocking
    }));
    Ok(())
}
