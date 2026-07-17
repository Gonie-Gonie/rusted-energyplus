use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::{AirBoundaryAirExchange, AirBoundaryMixingSchedule, ScheduleId};
use ep_raw_model::parse_epjson_str;

#[test]
fn typed_air_boundaries_are_all_definition_run_blocked() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier":"26.1"}},
            "Zone": {"Zone One": {"volume":100.0}},
            "Schedule:Constant": {
                "Mixing Fraction": {"hourly_value":0.75}
            },
            "Construction:AirBoundary": {
                "Unused None": {"air_exchange_method":"None"},
                "Unused Mixing": {
                    "air_exchange_method":"SimpleMixing",
                    "simple_mixing_air_changes_per_hour":0.4,
                    "simple_mixing_schedule_name":"Mixing Fraction"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed air boundaries"))?;
    let air_boundaries = model
        .constructions
        .iter()
        .filter(|construction| construction.is_air_boundary())
        .collect::<Vec<_>>();
    assert_eq!(air_boundaries.len(), 2);
    let none_boundary = air_boundaries
        .iter()
        .find(|construction| construction.name.0 == "UNUSED NONE")
        .ok_or_else(|| std::io::Error::other("missing None boundary"))?;
    let mixing_boundary = air_boundaries
        .iter()
        .find(|construction| construction.name.0 == "UNUSED MIXING")
        .ok_or_else(|| std::io::Error::other("missing mixing boundary"))?;
    assert_eq!(
        none_boundary
            .air_boundary
            .ok_or_else(|| std::io::Error::other("missing None metadata"))?
            .air_exchange,
        AirBoundaryAirExchange::None
    );
    assert_eq!(
        mixing_boundary
            .air_boundary
            .ok_or_else(|| std::io::Error::other("missing mixing metadata"))?
            .air_exchange,
        AirBoundaryAirExchange::SimpleMixing {
            air_changes_per_hour: 0.4,
            schedule: AirBoundaryMixingSchedule::User(ScheduleId(0)),
        }
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
        entry.object_type == "Construction:AirBoundary"
            && entry.count == 2
            && entry.status == "unsupported"
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.stage == "support"
            && diagnostic.object_type.as_deref() == Some("Construction:AirBoundary")
            && diagnostic.blocking
    }));
    Ok(())
}
