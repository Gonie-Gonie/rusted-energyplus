use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "SurfaceProperty:SolarIncidentInside";

#[test]
fn scheduled_inside_solar_incidents_are_all_definition_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {"Layer": {"roughness":"Rough","thermal_resistance":1}},
            "Construction": {
                "Surface Construction": {"outside_layer":"Layer"},
                "Selected Construction": {"outside_layer":"Layer"}
            },
            "Schedule:Constant": {"Inside Solar": {"hourly_value":125}},
            "Zone": {"Zone One": {}},
            "BuildingSurface:Detailed": {
                "Wall One": {
                    "surface_type":"Wall","construction_name":"Surface Construction","zone_name":"Zone One","outside_boundary_condition":"Outdoors",
                    "vertices":[{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0},{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0},{"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":0}]
                }
            },
            "SurfaceProperty:SolarIncidentInside": {
                "Scheduled Inside Solar": {
                    "surface_name":"Wall One",
                    "construction_name":"Selected Construction",
                    "inside_surface_incident_sun_solar_radiation_schedule_name":"Inside Solar"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed scheduled solar incident"))?;
    assert_eq!(model.surface_solar_incidents.len(), 1);
    assert_ne!(
        model.surface_solar_incidents[0].construction, model.surfaces[0].construction,
        "the typed input boundary must retain source construction mismatch"
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
    assert!(assessment.typed_objects.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE && entry.count == 1 && entry.status == "typed"
    }));
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE && entry.count == 1 && entry.status == "unsupported"
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some(OBJECT_TYPE)
            && diagnostic.blocking
    }));
    Ok(())
}
