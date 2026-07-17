use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::SurfaceVaporCoefficientsId;
use ep_raw_model::parse_epjson_str;

#[test]
fn surface_vapor_coefficients_block_every_definition_before_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "Material:NoMass": {
                "R13": {"roughness":"MediumRough","thermal_resistance":2.29}
            },
            "Construction": {"Wall": {"outside_layer":"R13"}},
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
                },
                "Wall Two": {
                    "surface_type":"Wall",
                    "construction_name":"Wall",
                    "zone_name":"Zone One",
                    "outside_boundary_condition":"Outdoors",
                    "vertices":[
                        {"vertex_x_coordinate":2,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":3,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":3,"vertex_y_coordinate":0,"vertex_z_coordinate":1},
                        {"vertex_x_coordinate":2,"vertex_y_coordinate":0,"vertex_z_coordinate":1}
                    ]
                }
            },
            "SurfaceProperties:VaporCoefficients": {
                "A zero override": {
                    "surface_name":"Wall One",
                    "constant_external_vapor_transfer_coefficient":"Yes",
                    "external_vapor_coefficient_value":0,
                    "constant_internal_vapor_transfer_coefficient":"Yes",
                    "internal_vapor_coefficient_value":0
                },
                "B inactive values": {
                    "surface_name":"Wall Two",
                    "constant_external_vapor_transfer_coefficient":"No",
                    "external_vapor_coefficient_value":0.125,
                    "constant_internal_vapor_transfer_coefficient":"No",
                    "internal_vapor_coefficient_value":0.25
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
    assert_eq!(model.surface_vapor_coefficients.len(), 2);
    assert_eq!(
        model.surface_vapor_coefficients[0].id,
        SurfaceVaporCoefficientsId(0)
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
    let typed_entries = assessment
        .typed_objects
        .iter()
        .filter(|entry| entry.object_type == "SurfaceProperties:VaporCoefficients")
        .collect::<Vec<_>>();
    assert_eq!(typed_entries.len(), 1);
    assert_eq!(typed_entries[0].count, 2);
    assert_eq!(typed_entries[0].status, "typed");

    let unsupported_entries = assessment
        .unsupported_objects
        .iter()
        .filter(|entry| entry.object_type == "SurfaceProperties:VaporCoefficients")
        .collect::<Vec<_>>();
    assert_eq!(unsupported_entries.len(), 1);
    assert_eq!(unsupported_entries[0].count, 2);
    assert_eq!(unsupported_entries[0].status, "unsupported");

    let boundary_diagnostics = assessment
        .diagnostics
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code == "UnsupportedSurfaceBoundary"
                && diagnostic.object_type.as_deref() == Some("SurfaceProperties:VaporCoefficients")
        })
        .collect::<Vec<_>>();
    assert_eq!(boundary_diagnostics.len(), 1);
    Ok(())
}
