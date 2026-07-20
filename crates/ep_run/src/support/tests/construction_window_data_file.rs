use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "Construction:WindowDataFile";
const UNSUPPORTED_NOTE: &str = "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported.";

#[test]
fn typed_window_data_file_requests_are_all_definition_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Construction:WindowDataFile": {
                "Default Missing Library": {},
                "Explicit Missing Library": {"file_name":"Missing/Library.dat"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed WINDOW5 requests"))?;
    assert_eq!(model.construction_window_data_file_requests.len(), 2);
    assert!(model.constructions.is_empty());
    assert!(model.materials.is_empty());
    assert!(model.window_frame_and_dividers.is_empty());

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
    assert_eq!(
        assessment
            .diagnostics
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code == "UnsupportedSurfaceBoundary"
                    && diagnostic.stage == "support"
                    && diagnostic.object_type.as_deref() == Some(OBJECT_TYPE)
                    && diagnostic.blocking
            })
            .count(),
        1
    );
    Ok(())
}
