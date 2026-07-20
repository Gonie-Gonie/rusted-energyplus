use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::{MaterialVariableAbsorptanceId, SurfaceId, VariableAbsorptanceSurfaceBinding};
use ep_raw_model::parse_epjson_str;

#[test]
fn variable_absorptance_blocks_used_and_unused_targets_before_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "Material:NoMass": {
                "Used": {"roughness":"MediumRough","thermal_resistance":2.29},
                "Unused": {"roughness":"MediumRough","thermal_resistance":1.5}
            },
            "MaterialProperty:VariableAbsorptance": {
                "Used Overlay": {
                    "reference_material_name":"Used",
                    "control_signal":"Scheduled",
                    "thermal_absorptance_schedule_name":"Constant-0.0"
                },
                "Unused Overlay": {
                    "reference_material_name":"Unused",
                    "control_signal":"Scheduled",
                    "solar_absorptance_schedule_name":"Constant-1.0"
                }
            },
            "Construction": {"Wall": {"outside_layer":"Used"}},
            "BuildingSurface:Detailed": {
                "Wall One": {
                    "surface_type":"Wall",
                    "construction_name":"Wall",
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
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.material_variable_absorptances.len(), 2);
    assert_eq!(
        model.material_variable_absorptances[0].id,
        MaterialVariableAbsorptanceId(0)
    );
    let used_overlay = model
        .material_variable_absorptance_names
        .resolve("Used Overlay")
        .ok_or_else(|| std::io::Error::other("expected used variable-absorptance overlay"))?;
    assert_eq!(
        model.variable_absorptance_surface_bindings,
        vec![VariableAbsorptanceSurfaceBinding {
            surface: SurfaceId(0),
            variable_absorptance: used_overlay,
        }]
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
        entry.object_type == "MaterialProperty:VariableAbsorptance"
            && entry.count == 2
            && entry.status == "typed"
    }));
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == "MaterialProperty:VariableAbsorptance"
            && entry.count == 2
            && entry.status == "unsupported"
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("MaterialProperty:VariableAbsorptance")
    }));
    Ok(())
}
