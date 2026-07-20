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

#[test]
fn authored_spaces_and_space_lists_fail_closed_before_partition_consumers()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone":{"Zone One":{}},
            "Space":{"Room One":{"zone_name":"Zone One"}},
            "SpaceList":{"All Rooms":{"spaces":[{"space_name":"Room One"}]}}
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
            .any(|entry| { entry.object_type == "Space" && entry.count == 1 })
    );
    assert!(
        assessment
            .unsupported_objects
            .iter()
            .any(|entry| { entry.object_type == "SpaceList" && entry.count == 1 })
    );
    assert_eq!(
        assessment
            .diagnostics
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "UnsupportedSpacePartitioning")
            .count(),
        2
    );
    Ok(())
}

#[test]
fn generated_default_spaces_do_not_add_a_runtime_boundary() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(r#"{"Zone":{"Zone One":{}}}"#)?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected generated default space"))?;
    assert_eq!(model.spaces.len(), 1);

    let assessment = assess_support(
        &raw,
        &result.report,
        Some(model),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert!(
        !assessment
            .unsupported_objects
            .iter()
            .any(|entry| { entry.object_type == "Space" || entry.object_type == "SpaceList" })
    );
    assert!(
        !assessment
            .diagnostics
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "UnsupportedSpacePartitioning")
    );
    Ok(())
}

#[test]
fn explicit_surface_space_assignments_fail_closed_until_geometry_resolution()
-> Result<(), Box<dyn std::error::Error>> {
    const EPJSON: &str = r#"{
        "Zone":{"Zone One":{"volume":100}},
        "Material:NoMass":{
            "R13":{"roughness":"MediumRough","thermal_resistance":2.29}
        },
        "Construction":{"Wall":{"outside_layer":"R13"}},
        "BuildingSurface:Detailed":{
            "Wall One":{
                "surface_type":"Wall",
                "construction_name":"Wall",
                "zone_name":"Zone One",
                "space_name":__SPACE_NAME__,
                "outside_boundary_condition":"Outdoors",
                "vertices":[
                    {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                    {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                    {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":1},
                    {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":1}
                ]
            }
        }
    }"#;

    for value in ["\"Zone One\"", "\"Missing\"", "3"] {
        let raw = parse_epjson_str(&EPJSON.replace("__SPACE_NAME__", value))?;
        let result = compile_raw_model(&raw);
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        let model = result
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected generated default space"))?;
        assert_eq!(model.spaces.len(), 1);

        let assessment = assess_support(
            &raw,
            &result.report,
            Some(model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
        );
        assert_eq!(assessment.status, SupportStatus::Unsupported);
        assert_eq!(assessment.runtime_class, RuntimeClass::None);
        assert_eq!(assessment.run_result_state, RunResultState::RunBlocked);
        assert!(assessment.unsupported_objects.iter().any(|entry| {
            entry.object_type == "BuildingSurface:Detailed space_name" && entry.count == 1
        }));
        assert!(
            assessment
                .diagnostics
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "UnsupportedSpacePartitioning")
        );
    }

    let blank_raw = parse_epjson_str(&EPJSON.replace("__SPACE_NAME__", "\"\""))?;
    let blank_result = compile_raw_model(&blank_raw);
    assert!(
        !blank_result.has_errors(),
        "{:?}",
        blank_result.report.diagnostics
    );
    let blank_assessment = assess_support(
        &blank_raw,
        &blank_result.report,
        blank_result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );
    assert!(
        !blank_assessment
            .unsupported_objects
            .iter()
            .any(|entry| { entry.object_type == "BuildingSurface:Detailed space_name" })
    );
    assert!(
        !blank_assessment
            .diagnostics
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "UnsupportedSpacePartitioning")
    );
    Ok(())
}
