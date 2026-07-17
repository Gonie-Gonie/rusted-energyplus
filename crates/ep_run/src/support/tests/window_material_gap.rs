use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::MaterialKind;
use ep_raw_model::parse_epjson_str;

#[test]
fn typed_window_material_gaps_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "WindowMaterial:Gas": {
                "Air Source": {
                    "gas_type": "Air",
                    "thickness": 0.012
                }
            },
            "WindowMaterial:GasMixture": {
                "Air Argon Source": {
                    "thickness": 0.010,
                    "number_of_gases_in_mixture": 2,
                    "gas_1_type": "Air",
                    "gas_1_fraction": 0.2,
                    "gas_2_type": "Argon",
                    "gas_2_fraction": 0.8
                }
            },
            "WindowMaterial:Gap": {
                "Unused Single Gas Gap": {
                    "thickness": 0.012,
                    "gas_or_gas_mixture_": "Air Source"
                },
                "Unused Mixture Gap": {
                    "thickness": 0.010,
                    "gas_or_gas_mixture_": "Air Argon Source",
                    "pressure": 87654.321
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed window gap materials"))?;
    assert_eq!(model.materials.len(), 4);
    assert_eq!(
        model
            .materials
            .iter()
            .filter(|material| material.kind() == MaterialKind::WindowComplexGap)
            .count(),
        2,
        "all WindowMaterial:Gap definitions must remain typed even when unused"
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
        entry.object_type == "WindowMaterial:Gap"
            && entry.count == 2
            && entry.status == "unsupported"
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.stage == "support"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Gap")
            && diagnostic.message
                == "WindowMaterial:Gap is typed for graph/source-map diagnostics but not executable in arbitrary-run compatibility mode"
            && diagnostic.blocking
    }));
    Ok(())
}
