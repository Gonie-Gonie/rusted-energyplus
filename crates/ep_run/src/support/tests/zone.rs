use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

#[test]
fn zone_local_convection_override_is_typed_and_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone": {
                "Zone One": {
                    "zone_inside_convection_algorithm":"TrombeWall",
                    "zone_outside_convection_algorithm":"MoWiTT"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);

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
    assert_eq!(assessment.runtime_class, RuntimeClass::None);
    assert_eq!(assessment.run_result_state, RunResultState::RunBlocked);
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == "Zone convection algorithm override" && entry.count == 1
    }));
    assert!(
        assessment
            .diagnostics
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "UnsupportedZoneConvectionOverride" })
    );
    Ok(())
}

#[test]
fn inherited_zone_convection_selection_does_not_add_a_runtime_boundary()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "SurfaceConvectionAlgorithm:Inside": {
                "Inside Selection": {"algorithm":"CeilingDiffuser"}
            },
            "SurfaceConvectionAlgorithm:Outside": {
                "Outside Selection": {"algorithm":"TARP"}
            },
            "Zone": {"Zone One": {}}
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);

    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert!(
        !assessment
            .unsupported_objects
            .iter()
            .any(|entry| { entry.object_type == "Zone convection algorithm override" })
    );
    assert!(
        !assessment
            .diagnostics
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "UnsupportedZoneConvectionOverride" })
    );
    Ok(())
}

#[test]
fn typed_zone_lists_and_groups_fail_closed_before_unconsumed_runtime_semantics()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone": {"Zone One": {}},
            "ZoneList": {
                "Zone Collection": {"zones":[{"zone_name":"Zone One"}]}
            },
            "ZoneGroup": {
                "Repeated Floor": {
                    "zone_list_name":"Zone Collection",
                    "zone_list_multiplier":3
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);

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
    assert_eq!(assessment.runtime_class, RuntimeClass::None);
    assert_eq!(assessment.run_result_state, RunResultState::RunBlocked);
    assert!(
        assessment
            .unsupported_objects
            .iter()
            .any(|entry| { entry.object_type == "ZoneList" && entry.count == 1 })
    );
    assert!(
        assessment
            .unsupported_objects
            .iter()
            .any(|entry| { entry.object_type == "ZoneGroup" && entry.count == 1 })
    );
    assert_eq!(
        assessment
            .diagnostics
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "UnsupportedZoneGrouping")
            .count(),
        2
    );
    Ok(())
}

#[test]
fn typed_zone_local_environment_fails_closed_before_local_weather_consumption()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone": {"Zone One": {}},
            "ZoneProperty:LocalEnvironment": {
                "Zone Weather": {
                    "zone_name":"Zone One",
                    "outdoor_air_node_name":"Local Outdoor Node"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);

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
    assert_eq!(assessment.runtime_class, RuntimeClass::None);
    assert_eq!(assessment.run_result_state, RunResultState::RunBlocked);
    assert!(
        assessment.unsupported_objects.iter().any(|entry| {
            entry.object_type == "ZoneProperty:LocalEnvironment" && entry.count == 1
        })
    );
    assert_eq!(
        assessment
            .diagnostics
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "UnsupportedZoneLocalEnvironment")
            .count(),
        1
    );
    Ok(())
}
