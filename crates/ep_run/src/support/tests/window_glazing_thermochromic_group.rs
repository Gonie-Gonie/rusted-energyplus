use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_model::MaterialKind;
use ep_raw_model::parse_epjson_str;

#[test]
fn typed_thermochromic_groups_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "WindowMaterial:Glazing": {
                "Spectral Average Glass": {
                    "optical_data_type": "SpectralAverage",
                    "thickness": 0.006
                }
            },
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {
                "Refraction Extinction Glass": {
                    "thickness": 0.007,
                    "solar_index_of_refraction": 1.5,
                    "solar_extinction_coefficient": 20.0,
                    "visible_index_of_refraction": 1.6,
                    "visible_extinction_coefficient": 10.0
                }
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "Mixed Ordinary Glazing Group": {
                    "temperature_data": [
                        {
                            "optical_data_temperature": 35.0,
                            "window_material_glazing_name": "Spectral Average Glass"
                        },
                        {
                            "optical_data_temperature": 5.0,
                            "window_material_glazing_name": "Refraction Extinction Glass"
                        }
                    ]
                },
                "Unused Spectral Glazing Group": {
                    "temperature_data": [
                        {
                            "optical_data_temperature": 20.0,
                            "window_material_glazing_name": "Spectral Average Glass"
                        }
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
        .ok_or_else(|| std::io::Error::other("expected typed thermochromic groups"))?;
    assert_eq!(model.materials.len(), 4);
    assert_eq!(
        model
            .materials
            .iter()
            .filter(|material| { material.kind() == MaterialKind::WindowGlazingThermochromicGroup })
            .count(),
        2,
        "all thermochromic definitions must remain typed even when unused"
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
        entry.object_type == "WindowMaterial:GlazingGroup:Thermochromic"
            && entry.count == 2
            && entry.note
                == "Fenestration, daylighting, shading, and advanced surface boundary objects are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref()
                == Some("WindowMaterial:GlazingGroup:Thermochromic")
    }));
    Ok(())
}
