use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::MaterialKind;
use ep_raw_model::parse_epjson_str;

#[test]
fn typed_window_complex_shades_including_all_unused_definitions_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "WindowMaterial:ComplexShade": {
                "Unused Other": {"layer_type":"OtherShadingType"},
                "Unused Horizontal": {"layer_type":"VenetianHorizontal"},
                "Unused Vertical": {"layer_type":"VenetianVertical"},
                "Unused Woven": {"layer_type":"Woven"},
                "Unused Perforated": {"layer_type":"Perforated"},
                "Unused Bsdf": {"layer_type":"BSDF"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed complex shades"))?;
    assert_eq!(model.materials.len(), 6);
    assert_eq!(
        model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowComplexShade)
            .count(),
        6,
        "every WindowMaterial:ComplexShade definition must remain typed even when unused"
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
        entry.object_type == "WindowMaterial:ComplexShade"
            && entry.count == 6
            && entry.status == "unsupported"
            && entry.note
                == "Fenestration, daylighting, shading, and advanced surface boundary objects are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.stage == "support"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:ComplexShade")
            && diagnostic.message
                == "WindowMaterial:ComplexShade is typed for graph/source-map diagnostics but not executable in arbitrary-run compatibility mode"
            && diagnostic.blocking
    }));
    Ok(())
}
