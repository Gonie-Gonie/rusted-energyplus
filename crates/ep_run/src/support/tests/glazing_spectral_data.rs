use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::GlazingSpectralDataId;
use ep_raw_model::parse_epjson_str;

#[test]
fn valid_unused_glazing_spectral_dataset_is_inert_at_runtime_support_boundary()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "MaterialProperty:GlazingSpectralData": {
                "Unused Valid Dataset": {
                    "wavelength_1":0.3,
                    "transmittance_1":0.6,
                    "front_reflectance_1":0.2,
                    "back_reflectance_1":0.2
                }
            },
            "Zone": {"Zone One": {"volume": 100}},
            "Material:NoMass": {
                "R13": {
                    "roughness": "MediumRough",
                    "thermal_resistance": 2.29
                }
            },
            "Construction": {"Wall": {"outside_layer": "R13"}},
            "BuildingSurface:Detailed": {
                "Wall One": {
                    "surface_type": "Wall",
                    "construction_name": "Wall",
                    "zone_name": "Zone One",
                    "outside_boundary_condition": "Outdoors",
                    "vertices": [
                        {"vertex_x_coordinate": 0, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0},
                        {"vertex_x_coordinate": 1, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0},
                        {"vertex_x_coordinate": 1, "vertex_y_coordinate": 0, "vertex_z_coordinate": 1},
                        {"vertex_x_coordinate": 0, "vertex_y_coordinate": 0, "vertex_z_coordinate": 1}
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
        .ok_or_else(|| std::io::Error::other("expected typed supported model"))?;
    assert_eq!(model.glazing_spectral_data.len(), 1);
    assert_eq!(model.glazing_spectral_data[0].id, GlazingSpectralDataId(0));

    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert_eq!(assessment.status, SupportStatus::SupportedCompatibility);
    assert_eq!(
        assessment.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );
    assert_eq!(
        assessment.runtime_class,
        RuntimeClass::OneZoneHeatBalanceCompatibility
    );
    assert_eq!(
        assessment.matched_capability_ids,
        vec!["official_1zone_uncontrolled_declared_heat_balance"]
    );
    assert!(assessment.typed_objects.iter().any(|entry| {
        entry.object_type == "MaterialProperty:GlazingSpectralData"
            && entry.count == 1
            && entry.status == "typed"
    }));
    assert!(
        assessment
            .unsupported_objects
            .iter()
            .all(|entry| { entry.object_type != "MaterialProperty:GlazingSpectralData" })
    );
    assert!(assessment.diagnostics.diagnostics.iter().all(|diagnostic| {
        diagnostic.object_type.as_deref() != Some("MaterialProperty:GlazingSpectralData")
    }));
    Ok(())
}
