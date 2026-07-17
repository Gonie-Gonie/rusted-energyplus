use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::MaterialVariableThermalConductivityId;
use ep_raw_model::parse_epjson_str;

#[test]
fn variable_thermal_conductivity_blocks_used_and_unused_targets_before_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "Material": {
                "Used": {
                    "roughness":"MediumRough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":900.0,
                    "specific_heat":1000.0
                }
            },
            "Material:NoMass": {
                "Unused": {"roughness":"MediumRough","thermal_resistance":1.5}
            },
            "MaterialProperty:VariableThermalConductivity": {
                "Used": {
                    "values":[
                        {"temperature":-20,"thermal_conductivity":0.5},
                        {"temperature":20,"thermal_conductivity":1.0},
                        {"temperature":21,"thermal_conductivity":1.5}
                    ]
                },
                "Unused": {
                    "values":[{"temperature":0,"thermal_conductivity":-1}]
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
    assert_eq!(model.material_variable_thermal_conductivities.len(), 2);
    assert_eq!(
        model.material_variable_thermal_conductivities[0].id,
        MaterialVariableThermalConductivityId(0)
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
        entry.object_type == "MaterialProperty:VariableThermalConductivity"
            && entry.count == 2
            && entry.status == "typed"
    }));
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == "MaterialProperty:VariableThermalConductivity"
            && entry.count == 2
            && entry.status == "unsupported"
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref()
                == Some("MaterialProperty:VariableThermalConductivity")
    }));
    Ok(())
}
