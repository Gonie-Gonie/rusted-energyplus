use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "ConstructionProperty:InternalHeatSource";
const UNSUPPORTED_NOTE: &str = "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported.";

#[test]
fn typed_internal_heat_sources_are_all_definition_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier":"26.1"}},
            "Zone": {"Zone One": {"volume":100.0}},
            "Material:NoMass": {
                "Outside Layer": {"roughness":"Rough", "thermal_resistance":1.0},
                "Inside Layer": {"roughness":"Rough", "thermal_resistance":0.5}
            },
            "Construction": {
                "Used Source Construction": {
                    "outside_layer":"Outside Layer",
                    "layer_2":"Inside Layer"
                },
                "Unused Source Construction": {
                    "outside_layer":"Inside Layer",
                    "layer_2":"Outside Layer"
                }
            },
            "ConstructionProperty:InternalHeatSource": {
                "Used Source": {
                    "construction_name":"Used Source Construction",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":1,
                    "tube_spacing":0.20
                },
                "Unused Source": {
                    "construction_name":"Unused Source Construction",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":2,
                    "tube_spacing":0.30,
                    "two_dimensional_temperature_calculation_position":0.5
                }
            },
            "BuildingSurface:Detailed": {
                "Used Wall": {
                    "surface_type":"Wall",
                    "construction_name":"Used Source Construction",
                    "zone_name":"Zone One",
                    "outside_boundary_condition":"Outdoors",
                    "vertices":[
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":1},
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":1}
                    ]
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed internal heat sources"))?;
    assert_eq!(
        model
            .constructions
            .iter()
            .filter(|construction| construction.has_internal_heat_source())
            .count(),
        2
    );
    assert_eq!(model.surfaces.len(), 1);

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
    let boundary_diagnostics = assessment
        .diagnostics
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "UnsupportedSurfaceBoundary"
                && diagnostic.stage == "support"
                && diagnostic.object_type.as_deref() == Some(OBJECT_TYPE)
                && diagnostic.blocking
        })
        .collect::<Vec<_>>();
    assert_eq!(boundary_diagnostics.len(), 1);
    Ok(())
}
