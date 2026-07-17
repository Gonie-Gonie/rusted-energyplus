use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::MaterialMoisturePenetrationDepthSettingsId;
use ep_raw_model::parse_epjson_str;

#[test]
fn moisture_penetration_depth_blocks_used_and_unused_targets_before_runtime()
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
                },
                "Unused": {
                    "roughness":"Rough",
                    "thickness":0.2,
                    "conductivity":0.5,
                    "density":800.0,
                    "specific_heat":900.0
                }
            },
            "MaterialProperty:MoisturePenetrationDepth:Settings": {
                "Used": {
                    "water_vapor_diffusion_resistance_factor":27.3,
                    "moisture_equation_coefficient_a":0.072549,
                    "moisture_equation_coefficient_b":0.397173,
                    "moisture_equation_coefficient_c":0.007774,
                    "moisture_equation_coefficient_d":11.7057,
                    "surface_layer_penetration_depth":0.004,
                    "deep_layer_penetration_depth":0,
                    "coating_layer_thickness":0,
                    "coating_layer_water_vapor_diffusion_resistance_factor":0
                },
                "Unused": {
                    "water_vapor_diffusion_resistance_factor":0,
                    "moisture_equation_coefficient_a":-1,
                    "moisture_equation_coefficient_b":0,
                    "moisture_equation_coefficient_c":1,
                    "moisture_equation_coefficient_d":2,
                    "coating_layer_thickness":0,
                    "coating_layer_water_vapor_diffusion_resistance_factor":0
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
    assert_eq!(model.material_moisture_penetration_depth_settings.len(), 2);
    assert_eq!(
        model.material_moisture_penetration_depth_settings[0].id,
        MaterialMoisturePenetrationDepthSettingsId(0)
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
        entry.object_type == "MaterialProperty:MoisturePenetrationDepth:Settings"
            && entry.count == 2
            && entry.status == "typed"
    }));
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == "MaterialProperty:MoisturePenetrationDepth:Settings"
            && entry.count == 2
            && entry.status == "unsupported"
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref()
                == Some("MaterialProperty:MoisturePenetrationDepth:Settings")
    }));
    Ok(())
}
