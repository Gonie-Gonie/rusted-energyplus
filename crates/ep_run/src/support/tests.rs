use super::{RunResultState, RuntimeClass, SelectedAlgorithmLane, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

mod glazing_spectral_data;
mod material_heat_and_moisture_transfer_settings;
mod material_heat_and_moisture_transfer_sorption_isotherm;
mod material_heat_and_moisture_transfer_suction;
mod material_moisture_penetration_depth_settings;
mod material_phase_change;
mod material_phase_change_hysteresis;
mod material_roof_vegetation;
mod material_variable_absorptance;
mod material_variable_thermal_conductivity;
mod window_glazing_thermochromic_group;
mod window_material_blind_equivalent_layer;
mod window_material_complex_shade;
mod window_material_gap;
mod window_simple_glazing;

#[test]
fn simple_one_zone_model_is_supported() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
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
        assessment.runtime_class,
        RuntimeClass::OneZoneHeatBalanceCompatibility
    );
    assert_eq!(
        assessment.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );
    assert_eq!(
        assessment.matched_capability_ids,
        vec!["official_1zone_uncontrolled_declared_heat_balance"]
    );
    assert_eq!(assessment.matched_capabilities.len(), 1);
    assert_eq!(assessment.matched_capabilities[0].domain, "heat_balance");
    assert_eq!(
        assessment.matched_capabilities[0].evidence_cases,
        vec!["official_1zone_uncontrolled_dynamic_conformance_candidate_001"]
    );
    assert!(assessment.failed_capability_ids.is_empty());
    assert!(assessment.capability_registry_loaded);
    Ok(())
}

#[test]
fn typed_refraction_extinction_glazing_remains_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Glazing:RefractionExtinctionMethod": {
                    "Alternative Glass": {
                        "thickness": 0.006,
                        "solar_index_of_refraction": 1.5,
                        "solar_extinction_coefficient": 20.0,
                        "visible_index_of_refraction": 1.6,
                        "visible_extinction_coefficient": 10.0
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
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
        entry.object_type == "WindowMaterial:Glazing:RefractionExtinctionMethod"
            && entry.count == 1
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref()
                == Some("WindowMaterial:Glazing:RefractionExtinctionMethod")
    }));
    Ok(())
}

#[test]
fn typed_equivalent_layer_glazing_remains_run_blocked() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Glazing:EquivalentLayer": {
                    "Equivalent Glass": {
                        "front_side_beam_beam_solar_transmittance": 0.61,
                        "back_side_beam_beam_solar_transmittance": 0.62,
                        "front_side_beam_beam_solar_reflectance": 0.21,
                        "back_side_beam_beam_solar_reflectance": 0.22
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
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
        entry.object_type == "WindowMaterial:Glazing:EquivalentLayer"
            && entry.count == 1
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Glazing:EquivalentLayer")
    }));
    Ok(())
}

#[test]
fn typed_window_gas_remains_run_blocked() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Gas": {
                    "Air Gap": {
                        "gas_type": "Air",
                        "thickness": 0.0127
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
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
        entry.object_type == "WindowMaterial:Gas"
            && entry.count == 1
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Gas")
    }));
    Ok(())
}

#[test]
fn typed_window_gap_equivalent_layer_remains_run_blocked() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Gap:EquivalentLayer": {
                    "Vented Gap": {
                        "gas_type": "AIR",
                        "thickness": 0.0127,
                        "gap_vent_type": "VentedOutdoor"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
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
        entry.object_type == "WindowMaterial:Gap:EquivalentLayer"
            && entry.count == 1
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Gap:EquivalentLayer")
    }));
    Ok(())
}

#[test]
fn typed_window_gas_mixture_remains_run_blocked() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:GasMixture": {
                    "Argon Air Gap": {
                        "thickness": 0.0127,
                        "number_of_gases_in_mixture": 2,
                        "gas_1_type": "Argon",
                        "gas_1_fraction": 0.9,
                        "gas_2_type": "Air",
                        "gas_2_fraction": 0.1
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
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
        entry.object_type == "WindowMaterial:GasMixture"
            && entry.count == 1
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:GasMixture")
    }));
    Ok(())
}

#[test]
fn typed_window_shades_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Shade": {
                    "Interior Shade": {
                        "solar_transmittance": 0.1,
                        "solar_reflectance": 0.6,
                        "visible_transmittance": 0.1,
                        "visible_reflectance": 0.6,
                        "infrared_hemispherical_emissivity": 0.8,
                        "infrared_transmittance": 0.0,
                        "thickness": 0.001,
                        "conductivity": 0.2
                    },
                    "Unused Exterior Shade": {
                        "solar_transmittance": 0.2,
                        "solar_reflectance": 0.5,
                        "visible_transmittance": 0.2,
                        "visible_reflectance": 0.5,
                        "infrared_hemispherical_emissivity": 0.7,
                        "infrared_transmittance": 0.1,
                        "thickness": 0.002,
                        "conductivity": 0.3
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
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
        entry.object_type == "WindowMaterial:Shade"
            && entry.count == 2
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Shade")
    }));
    Ok(())
}

#[test]
fn typed_equivalent_layer_window_shades_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Shade:EquivalentLayer": {
                    "Defaulted Equivalent Shade": {
                        "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                        "back_side_shade_beam_diffuse_solar_transmittance": 0.2,
                        "front_side_shade_beam_diffuse_solar_reflectance": 0.3,
                        "back_side_shade_beam_diffuse_solar_reflectance": 0.2
                    },
                    "Unused Asymmetric Equivalent Shade": {
                        "shade_beam_beam_solar_transmittance": 0.1,
                        "front_side_shade_beam_diffuse_solar_transmittance": 0.2,
                        "back_side_shade_beam_diffuse_solar_transmittance": 0.3,
                        "front_side_shade_beam_diffuse_solar_reflectance": 0.3,
                        "back_side_shade_beam_diffuse_solar_reflectance": 0.2,
                        "shade_beam_beam_visible_transmittance_at_normal_incidence": 0.1,
                        "shade_beam_diffuse_visible_transmittance_at_normal_incidence": 0.2,
                        "shade_beam_diffuse_visible_reflectance_at_normal_incidence": 0.3,
                        "shade_material_infrared_transmittance": 0.1,
                        "front_side_shade_material_infrared_emissivity": 0.7,
                        "back_side_shade_material_infrared_emissivity": 0.6
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
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
        entry.object_type == "WindowMaterial:Shade:EquivalentLayer"
            && entry.count == 2
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Shade:EquivalentLayer")
    }));
    Ok(())
}

#[test]
fn typed_equivalent_layer_window_drapes_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Drape:EquivalentLayer": {
                    "Defaulted Equivalent Drape": {
                        "front_side_drape_beam_diffuse_solar_transmittance": 0.1,
                        "back_side_drape_beam_diffuse_solar_transmittance": 0.2,
                        "front_side_drape_beam_diffuse_solar_reflectance": 0.3,
                        "back_side_drape_beam_diffuse_solar_reflectance": 0.2
                    },
                    "Unused Pleated Equivalent Drape": {
                        "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.1,
                        "front_side_drape_beam_diffuse_solar_transmittance": 0.2,
                        "back_side_drape_beam_diffuse_solar_transmittance": 0.3,
                        "front_side_drape_beam_diffuse_solar_reflectance": 0.3,
                        "back_side_drape_beam_diffuse_solar_reflectance": 0.2,
                        "drape_beam_beam_visible_transmittance": 0.1,
                        "drape_beam_diffuse_visible_transmittance": 0.2,
                        "drape_beam_diffuse_visible_reflectance": 0.3,
                        "drape_material_infrared_transmittance": 0.1,
                        "front_side_drape_material_infrared_emissivity": 0.7,
                        "back_side_drape_material_infrared_emissivity": 0.6,
                        "width_of_pleated_fabric": 0.02,
                        "length_of_pleated_fabric": 0.03
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
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
        entry.object_type == "WindowMaterial:Drape:EquivalentLayer"
            && entry.count == 2
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Drape:EquivalentLayer")
    }));
    Ok(())
}

#[test]
fn typed_window_screens_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Glazing": {
                    "Screen Test Glass": {
                        "optical_data_type": "SpectralAverage",
                        "thickness": 0.003
                    }
                },
                "WindowMaterial:Screen": {
                    "Defaulted Screen": {
                        "diffuse_solar_reflectance": 0.1,
                        "diffuse_visible_reflectance": 0.2,
                        "screen_material_spacing": 0.01,
                        "screen_material_diameter": 0.002
                    },
                    "Unused High Precision Screen": {
                        "reflected_beam_transmittance_accounting_method": "ModelAsDirectBeam",
                        "diffuse_solar_reflectance": 0.123456,
                        "diffuse_visible_reflectance": 0.234567,
                        "thermal_hemispherical_emissivity": 0.812345,
                        "conductivity": 17.2345,
                        "screen_material_spacing": 0.0123456,
                        "screen_material_diameter": 0.0034567,
                        "screen_to_glass_distance": 0.0345678,
                        "top_opening_multiplier": 0.1111,
                        "bottom_opening_multiplier": 0.2222,
                        "left_side_opening_multiplier": 0.3333,
                        "right_side_opening_multiplier": 0.4444,
                        "angle_of_resolution_for_screen_transmittance_output_map": 0
                    }
                },
                "Construction": {
                    "Used Screen Construction": {
                        "outside_layer": "Defaulted Screen",
                        "layer_2": "Screen Test Glass"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
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
        entry.object_type == "WindowMaterial:Screen"
            && entry.count == 2
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Screen")
    }));
    Ok(())
}

#[test]
fn typed_equivalent_layer_window_screens_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Screen:EquivalentLayer": {
                    "Defaulted Equivalent Screen": {
                        "screen_beam_diffuse_solar_transmittance": 0.1,
                        "screen_beam_diffuse_solar_reflectance": 0.2,
                        "screen_beam_beam_visible_transmittance": 0.2,
                        "screen_beam_diffuse_visible_transmittance": 0.3,
                        "screen_beam_diffuse_visible_reflectance": 0.1
                    },
                    "Unused Explicit Equivalent Screen": {
                        "screen_beam_beam_solar_transmittance": 0.6,
                        "screen_beam_diffuse_solar_transmittance": 0.7,
                        "screen_beam_diffuse_solar_reflectance": 0.1,
                        "screen_beam_beam_visible_transmittance": 0.2,
                        "screen_beam_diffuse_visible_transmittance": 0.7,
                        "screen_beam_diffuse_visible_reflectance": 0.1,
                        "screen_infrared_transmittance": 0.2,
                        "screen_infrared_emissivity": 0.9,
                        "screen_wire_spacing": 0.01,
                        "screen_wire_diameter": 0.002
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
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
        entry.object_type == "WindowMaterial:Screen:EquivalentLayer"
            && entry.count == 2
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Screen:EquivalentLayer")
    }));
    Ok(())
}

#[test]
fn typed_window_blinds_including_unused_remain_run_blocked()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "WindowMaterial:Blind": {
                    "Defaulted Blind": {
                        "slat_width": 0.02,
                        "slat_separation": 0.02,
                        "front_side_slat_beam_solar_reflectance": 0.2,
                        "back_side_slat_beam_solar_reflectance": 0.3,
                        "front_side_slat_diffuse_solar_reflectance": 0.2,
                        "back_side_slat_diffuse_solar_reflectance": 0.3,
                        "slat_beam_visible_transmittance": 0.0
                    },
                    "Unused Vertical Blind": {
                        "slat_orientation": "Vertical",
                        "slat_width": 0.03,
                        "slat_separation": 0.02,
                        "slat_thickness": 0.001,
                        "slat_angle": 60.0,
                        "slat_conductivity": 15.0,
                        "slat_beam_solar_transmittance": 0.11,
                        "front_side_slat_beam_solar_reflectance": 0.22,
                        "back_side_slat_beam_solar_reflectance": 0.33,
                        "slat_diffuse_solar_transmittance": 0.11,
                        "front_side_slat_diffuse_solar_reflectance": 0.22,
                        "back_side_slat_diffuse_solar_reflectance": 0.33,
                        "slat_beam_visible_transmittance": 0.14,
                        "front_side_slat_beam_visible_reflectance": 0.25,
                        "back_side_slat_beam_visible_reflectance": 0.35,
                        "slat_diffuse_visible_transmittance": 0.14,
                        "front_side_slat_diffuse_visible_reflectance": 0.25,
                        "back_side_slat_diffuse_visible_reflectance": 0.35,
                        "slat_infrared_hemispherical_transmittance": 0.12,
                        "front_side_slat_infrared_hemispherical_emissivity": 0.70,
                        "back_side_slat_infrared_hemispherical_emissivity": 0.60,
                        "blind_to_glass_distance": 0.02,
                        "blind_top_opening_multiplier": 0.1,
                        "blind_bottom_opening_multiplier": 0.2,
                        "blind_left_side_opening_multiplier": 0.3,
                        "blind_right_side_opening_multiplier": 0.4,
                        "minimum_slat_angle": 11.0,
                        "maximum_slat_angle": 169.0
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
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
        entry.object_type == "WindowMaterial:Blind"
            && entry.count == 2
            && entry.note
                == "Fenestration, daylighting, shading, and advanced material or surface runtime semantics are not ported."
    }));
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedSurfaceBoundary"
            && diagnostic.object_type.as_deref() == Some("WindowMaterial:Blind")
    }));
    Ok(())
}

#[test]
fn missing_registry_capability_blocks_runtime_selection() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}}
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let (status, runtime_class, matched_capability_ids, missing_capability_ids) =
        super::runtime_boundaries::runtime_status_for_typed_model(
            result.model.as_ref(),
            &crate::support_registry::CapabilityRegistrySpec::default(),
        );

    assert_eq!(status, SupportStatus::Unsupported);
    assert_eq!(runtime_class, RuntimeClass::None);
    assert!(matched_capability_ids.is_empty());
    assert_eq!(
        missing_capability_ids,
        vec!["official_1zone_uncontrolled_declared_heat_balance"]
    );
    Ok(())
}

#[test]
fn runtime_class_reports_selected_algorithm_lane_metadata() {
    let heat_balance_compat =
        SelectedAlgorithmLane::from_runtime_class(RuntimeClass::OneZoneHeatBalanceCompatibility);
    assert_eq!(heat_balance_compat.id, "compatibility-source-order");
    assert!(!heat_balance_compat.diagnostic_probe_used);
    assert!(heat_balance_compat.conformance_promotion_allowed);

    let ideal_loads_compat = SelectedAlgorithmLane::from_runtime_class(
        RuntimeClass::IdealLoadsHumiditySelectedBranchesCompatibility,
    );
    assert_eq!(ideal_loads_compat.id, "compatibility-source-order");
    assert!(!ideal_loads_compat.diagnostic_probe_used);
    assert!(ideal_loads_compat.conformance_promotion_allowed);

    let heat_balance_diagnostic =
        SelectedAlgorithmLane::from_runtime_class(RuntimeClass::HeatBalanceZoneAirDiagnostic);
    assert_eq!(heat_balance_diagnostic.id, "diagnostic-probe");
    assert!(heat_balance_diagnostic.diagnostic_probe_used);
    assert!(!heat_balance_diagnostic.conformance_promotion_allowed);

    let ideal_loads_projection =
        SelectedAlgorithmLane::from_runtime_class(RuntimeClass::IdealLoadsNodeStateProjection);
    assert_eq!(ideal_loads_projection.id, "diagnostic-probe");
    assert!(ideal_loads_projection.diagnostic_probe_used);
    assert!(!ideal_loads_projection.conformance_promotion_allowed);

    assert_eq!(
        SelectedAlgorithmLane::none(),
        SelectedAlgorithmLane::from_runtime_class(RuntimeClass::None)
    );
}

#[test]
fn output_objects_use_partial_rule_from_registry() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Output:Variable": {
                    "Zone Mean Air Temperature": {
                        "key_value": "*",
                        "variable_name": "Zone Mean Air Temperature",
                        "reporting_frequency": "Hourly"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert!(
        assessment
            .ignored_raw_only_objects
            .iter()
            .any(|entry| entry.object_type == "Output:Variable"
                && entry.status == "ignored_reporting_objects")
    );
    Ok(())
}

#[test]
fn global_geometry_rules_are_consumed_without_ignored_semantics_warning()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "GlobalGeometryRules": {
                    "Rules": {
                        "starting_vertex_position": "UpperLeftCorner",
                        "vertex_entry_direction": "CounterClockWise",
                        "coordinate_system": "Relative",
                        "daylighting_reference_point_coordinate_system": "Relative",
                        "rectangular_surface_coordinate_system": "Relative"
                    }
                },
                "Zone": {"Zone One": {"volume": 100}}
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert!(
        result
            .model
            .as_ref()
            .and_then(|model| model.global_geometry_rules)
            .is_some()
    );
    assert!(
        assessment
            .ignored_raw_only_objects
            .iter()
            .all(|entry| entry.object_type != "GlobalGeometryRules")
    );
    assert!(assessment.diagnostics.diagnostics.iter().all(|diagnostic| {
        diagnostic.object_type.as_deref() != Some("GlobalGeometryRules")
            || diagnostic.code != "UnsupportedAlgorithmIgnored"
    }));
    Ok(())
}

#[test]
fn hvac_air_loop_uses_unsupported_rule_from_registry() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "AirLoopHVAC": {"Main Air Loop": {}}
            }"#,
    )?;
    let result = compile_raw_model(&raw);
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
    assert!(
        assessment
            .unsupported_objects
            .iter()
            .any(|entry| entry.object_type == "AirLoopHVAC"
                && entry.note == "Broad HVAC air-loop semantics are not ported.")
    );
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedHVACObject"
            && diagnostic.object_type.as_deref() == Some("AirLoopHVAC")
    }));
    Ok(())
}

#[test]
fn ideal_loads_no_oa_branch_matches_registry_capability() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
                    }
                },
                "ZoneControl:Thermostat": {
                    "Zone Thermostat": {
                        "zone_or_zonelist_name": "Zone One",
                        "control_type_schedule_name": "Control Type",
                        "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
                        "control_1_name": "Dual Setpoints"
                    }
                },
                "NodeList": {
                    "Zone Inlets": {
                        "nodes": [{"node_name": "Zone One Inlet"}]
                    }
                },
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone Inlets",
                        "dehumidification_control_type": "None",
                        "humidification_control_type": "None"
                    }
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone One": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
                        "zone_air_node_name": "Zone One Air Node",
                        "zone_return_air_node_or_nodelist_name": "Zone One Return"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Diagnostic,
        PartialRunPolicy::Allow,
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
        RuntimeClass::IdealLoadsNoOaSensibleCompatibility
    );
    assert_eq!(
        assessment.matched_capability_ids,
        vec!["ideal_loads_no_oa_sensible"]
    );
    assert_eq!(
        assessment.active_ideal_loads_branches,
        vec!["no_oa_sensible"]
    );
    assert!(
        assessment
            .inactive_ideal_loads_branches
            .contains(&"finite_capacity".to_string())
    );
    assert_eq!(assessment.matched_capabilities[0].domain, "ideal_loads");
    assert!(
        assessment.matched_capabilities[0]
            .evidence_cases
            .contains(&"ideal_loads_no_oa_sensible_conformance_001".to_string())
    );
    assert!(assessment.failed_capability_ids.is_empty());
    Ok(())
}

#[test]
fn ideal_loads_constant_supply_humidity_branch_matches_registry_capability()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
                    }
                },
                "ZoneControl:Thermostat": {
                    "Zone Thermostat": {
                        "zone_or_zonelist_name": "Zone One",
                        "control_type_schedule_name": "Control Type",
                        "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
                        "control_1_name": "Dual Setpoints"
                    }
                },
                "NodeList": {
                    "Zone Inlets": {
                        "nodes": [{"node_name": "Zone One Inlet"}]
                    }
                },
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone Inlets",
                        "dehumidification_control_type": "ConstantSupplyHumidityRatio",
                        "humidification_control_type": "None",
                        "minimum_cooling_supply_air_humidity_ratio": 0.0077
                    }
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone One": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
                        "zone_air_node_name": "Zone One Air Node",
                        "zone_return_air_node_or_nodelist_name": "Zone One Return"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
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
        RuntimeClass::IdealLoadsHumiditySelectedBranchesCompatibility
    );
    assert_eq!(
        assessment.matched_capability_ids,
        vec!["ideal_loads_humidity_selected_branches"]
    );
    assert_eq!(
        assessment.active_ideal_loads_branches,
        vec!["constant_supply_humidity_cooling"]
    );
    assert!(
        assessment
            .inactive_ideal_loads_branches
            .contains(&"humidistat_dehumidification".to_string())
    );
    assert_eq!(assessment.matched_capabilities[0].domain, "ideal_loads");
    assert!(assessment.matched_capabilities[0].evidence_cases.contains(
        &"ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001".to_string()
    ));
    assert!(assessment.failed_capability_ids.is_empty());
    Ok(())
}
#[test]
fn ideal_loads_dual_humidity_controls_are_not_selected_branch_compatibility()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
                    }
                },
                "ZoneControl:Thermostat": {
                    "Zone Thermostat": {
                        "zone_or_zonelist_name": "Zone One",
                        "control_type_schedule_name": "Control Type",
                        "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
                        "control_1_name": "Dual Setpoints"
                    }
                },
                "NodeList": {
                    "Zone Inlets": {
                        "nodes": [{"node_name": "Zone One Inlet"}]
                    }
                },
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone Inlets",
                        "dehumidification_control_type": "ConstantSupplyHumidityRatio",
                        "humidification_control_type": "ConstantSupplyHumidityRatio"
                    }
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone One": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
                        "zone_air_node_name": "Zone One Air Node",
                        "zone_return_air_node_or_nodelist_name": "Zone One Return"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
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
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedHeatBalanceBranch"
            && diagnostic.message.contains("unsupported feature flags")
    }));
    Ok(())
}
#[test]
fn support_status_maps_to_public_run_result_state() {
    assert_eq!(
        RunResultState::from_support_status(
            SupportStatus::Unsupported,
            RunMode::Compatibility,
            PartialRunPolicy::Deny
        ),
        RunResultState::RunBlocked
    );
    assert_eq!(
        RunResultState::from_support_status(
            SupportStatus::SupportedCompatibility,
            RunMode::Compatibility,
            PartialRunPolicy::Deny
        ),
        RunResultState::SupportedCompatibilityRun
    );
    assert_eq!(
        RunResultState::from_support_status(
            SupportStatus::SupportedCompatibility,
            RunMode::Diagnostic,
            PartialRunPolicy::Allow
        ),
        RunResultState::SupportedCompatibilityRun
    );
    assert_eq!(
        RunResultState::from_support_status(
            SupportStatus::SupportedDiagnosticOnly,
            RunMode::Compatibility,
            PartialRunPolicy::Deny
        ),
        RunResultState::RunBlocked
    );
    assert_eq!(
        RunResultState::from_support_status(
            SupportStatus::SupportedDiagnosticOnly,
            RunMode::Diagnostic,
            PartialRunPolicy::Allow
        ),
        RunResultState::PartialSupportedRun
    );
    assert_eq!(
        RunResultState::from_support_status(
            SupportStatus::SupportedDiagnosticOnly,
            RunMode::Diagnostic,
            PartialRunPolicy::Deny
        ),
        RunResultState::RunBlocked
    );
}
