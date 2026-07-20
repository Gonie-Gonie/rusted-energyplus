use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "Construction:WindowEquivalentLayer";
const UNSUPPORTED_NOTE: &str = "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported.";

#[test]
fn typed_window_equivalent_layer_constructions_are_all_definition_run_blocked()
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
                "Referenced Only By Definition": {"outside_layer":"EQL Shade"},
                "Second Unused Definition": {
                    "outside_layer":"EQL Shade",
                    "layer_2":"EQL Shade"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed equivalent-layer constructions"))?;
    assert_eq!(
        model
            .constructions
            .iter()
            .filter(|construction| construction.is_window_equivalent_layer())
            .count(),
        2
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
        entry.object_type == OBJECT_TYPE
            && entry.count == 2
            && entry.status == "unsupported"
            && entry.note == UNSUPPORTED_NOTE
    }));
    let diagnostics = assessment
        .diagnostics
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "UnsupportedSurfaceBoundary"
                && diagnostic.stage == "support"
                && diagnostic.object_type.as_deref() == Some(OBJECT_TYPE)
                && diagnostic.blocking
        })
        .count();
    assert_eq!(diagnostics, 1);
    Ok(())
}
