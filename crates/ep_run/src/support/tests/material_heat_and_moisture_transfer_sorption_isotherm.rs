use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::MaterialHeatAndMoistureTransferSorptionIsothermId;
use ep_raw_model::parse_epjson_str;

#[test]
fn sorption_isotherm_blocks_used_and_unused_targets_before_runtime()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "Material": {
                "Used": {"roughness":"MediumRough","thickness":0.1,"conductivity":1,"density":900,"specific_heat":1000},
                "Unused": {"roughness":"Rough","thickness":0.2,"conductivity":0.5,"density":800,"specific_heat":900}
            },
            "MaterialProperty:HeatAndMoistureTransfer:Settings": {
                "Used Settings": {"material_name":"Used","porosity":0.75,"initial_water_content_ratio":0.01},
                "Unused Settings": {"material_name":"Unused","porosity":0.5}
            },
            "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm": {
                "Used Curve": {"material_name":"Used","number_of_isotherm_coordinates":1,"relative_humidity_fraction_1":0.5,"moisture_content_1":10},
                "Unused Curve": {"material_name":"Unused","number_of_isotherm_coordinates":1,"relative_humidity_fraction_1":0.5,"moisture_content_1":10}
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
    assert_eq!(
        model
            .material_heat_and_moisture_transfer_sorption_isotherms
            .len(),
        2
    );
    assert_eq!(
        model.material_heat_and_moisture_transfer_sorption_isotherms[0].id,
        MaterialHeatAndMoistureTransferSorptionIsothermId(0)
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
        entry.object_type == "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm"
            && entry.count == 2
            && entry.status == "typed"
    }));
    assert!(assessment.unsupported_objects.iter().any(|entry| {
        entry.object_type == "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm"
            && entry.count == 2
            && entry.status == "unsupported"
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref()
                == Some("MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm")
    }));
    Ok(())
}
