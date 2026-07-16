use super::super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

#[test]
fn typed_equivalent_layer_window_blinds_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Version": {"Version 1": {"version_identifier": "26.1"}},
            "Zone": {"Zone One": {"volume": 100}},
            "WindowMaterial:Blind:EquivalentLayer": {
                "Defaulted Equivalent Blind": {
                    "slat_width": 0.02,
                    "slat_separation": 0.02,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.3,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.3
                },
                "Unused Explicit Equivalent Blind": {
                    "slat_orientation": "Vertical",
                    "slat_width": 0.024,
                    "slat_separation": 0.018,
                    "slat_crown": 0.0006,
                    "slat_angle": 63.0,
                    "front_side_slat_beam_diffuse_solar_transmittance": 0.1,
                    "back_side_slat_beam_diffuse_solar_transmittance": 0.2,
                    "front_side_slat_beam_diffuse_solar_reflectance": 0.3,
                    "back_side_slat_beam_diffuse_solar_reflectance": 0.4,
                    "front_side_slat_beam_diffuse_visible_transmittance": 0.1,
                    "back_side_slat_beam_diffuse_visible_transmittance": 0.2,
                    "front_side_slat_beam_diffuse_visible_reflectance": 0.3,
                    "back_side_slat_beam_diffuse_visible_reflectance": 0.4,
                    "slat_diffuse_diffuse_solar_transmittance": 0.1,
                    "front_side_slat_diffuse_diffuse_solar_reflectance": 0.2,
                    "back_side_slat_diffuse_diffuse_solar_reflectance": 0.3,
                    "slat_diffuse_diffuse_visible_transmittance": 0.1,
                    "front_side_slat_diffuse_diffuse_visible_reflectance": 0.2,
                    "back_side_slat_diffuse_diffuse_visible_reflectance": 0.3,
                    "slat_infrared_transmittance": 0.1,
                    "front_side_slat_infrared_emissivity": 0.7,
                    "back_side_slat_infrared_emissivity": 0.6,
                    "slat_angle_control": "BlockBeamSolar"
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
        entry.object_type == "WindowMaterial:Blind:EquivalentLayer"
            && entry.count == 2
            && entry.note
                == "Fenestration, daylighting, shading, and advanced surface boundary objects are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Blind:EquivalentLayer")
    }));
    Ok(())
}
