use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

#[test]
fn typed_roof_vegetation_materials_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "Material:RoofVegetation": {
                "Defaulted Roof Vegetation": {},
                "Unused Explicit Roof Vegetation": {
                    "height_of_plants": 0.45,
                    "leaf_area_index": 4.5,
                    "leaf_reflectivity": 0.23,
                    "leaf_emissivity": 0.92,
                    "minimum_stomatal_resistance": 235.0,
                    "soil_layer_name": "Explicit Soil Label",
                    "roughness": "VerySmooth",
                    "thickness": 0.23,
                    "conductivity_of_dry_soil": 0.45,
                    "density_of_dry_soil": 988.0,
                    "specific_heat_of_dry_soil": 1346.0,
                    "thermal_absorptance": 0.93,
                    "solar_absorptance": 0.81,
                    "visible_absorptance": 0.83,
                    "saturation_volumetric_moisture_content_of_the_soil_layer": 0.45,
                    "residual_volumetric_moisture_content_of_the_soil_layer": 0.08,
                    "initial_volumetric_moisture_content_of_the_soil_layer": 0.34,
                    "moisture_diffusion_calculation_method": "Simple"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        result.model.as_ref().map(|model| model.materials.len()),
        Some(2)
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
        entry.object_type == "Material:RoofVegetation"
            && entry.count == 2
            && entry.note
                == "Fenestration, daylighting, shading, and advanced surface boundary objects are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("Material:RoofVegetation")
    }));
    Ok(())
}
