use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::ConstructionGroundFactor;
use ep_raw_model::parse_epjson_str;

#[test]
fn typed_ground_factor_constructions_are_all_definition_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier":"26.1"}},
            "Zone": {"Zone One": {"volume":100.0}},
            "Construction:FfactorGroundFloor": {
                "Slab A": {"f_factor":0.5, "area":100.0, "perimeterexposed":20.0},
                "Slab B Unused": {"f_factor":0.4, "area":50.0, "perimeterexposed":0.0}
            },
            "Construction:CfactorUndergroundWall": {
                "Basement Wall Unused": {"c_factor":0.5, "height":1.0}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed ground-factor constructions"))?;
    assert_eq!(model.constructions.len(), 3);
    assert_eq!(model.materials.len(), 4);
    assert_eq!(
        model
            .constructions
            .iter()
            .filter(|construction| matches!(
                construction.ground_factor,
                Some(ConstructionGroundFactor::FfactorGroundFloor { .. })
            ))
            .count(),
        2
    );
    assert_eq!(
        model
            .constructions
            .iter()
            .filter(|construction| matches!(
                construction.ground_factor,
                Some(ConstructionGroundFactor::CfactorUndergroundWall { .. })
            ))
            .count(),
        1
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
    for (object_type, count) in [
        ("Construction:FfactorGroundFloor", 2),
        ("Construction:CfactorUndergroundWall", 1),
    ] {
        assert!(assessment.unsupported_objects.iter().any(|entry| {
            entry.object_type == object_type
                && entry.count == count
                && entry.status == "unsupported"
                && entry.note
                    == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
        }));
        assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "UnsupportedSurfaceBoundary"
                && diagnostic.stage == "support"
                && diagnostic.object_type.as_deref() == Some(object_type)
                && diagnostic.blocking
        }));
    }
    Ok(())
}
